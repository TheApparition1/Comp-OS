// Prevents additional console window on Windows in release
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use tauri::command;
use std::process::Command;
use serde::Deserialize;
use serde_json::Value;

#[derive(serde::Serialize, Clone)]
struct InstallStep {
    name: String,
    description: String,
    explanation: String, // Technical deep-dive for Learn Mode
    command: String,
    requires_sudo: bool,
    category: String, // "disk", "user", "desktop", "system"
}

#[derive(serde::Serialize)]
struct CommandResult {
    success: bool,
    output: String,
    error: Option<String>,
}

#[derive(Deserialize)]
struct PartitionPlanItem {
    mountpoint: String,
    filesystem: String,
    #[serde(alias = "sizeGb")]
    size_gb: u32,
    encrypted: bool,
    efi: bool,
}

#[derive(Deserialize)]
struct NetworkState {
    #[serde(rename = "type")]
    network_type: String,
    #[serde(rename = "wifiNetwork")]
    wifi_network: String,
    #[serde(rename = "wifiPassword")]
    wifi_password: String,
}

#[derive(Deserialize)]
struct DiskState {
    #[serde(rename = "selectedDisk")]
    selected_disk: String,
    flow: String,
    #[serde(rename = "guidedAction")]
    guided_action: String,
    filesystem: String,
    #[serde(rename = "swapSizeGb")]
    swap_size_gb: u32,
    #[serde(rename = "encryptionEnabled")]
    encryption_enabled: bool,
    #[serde(rename = "encryptionPassword")]
    encryption_password: String,
}

#[derive(Deserialize)]
struct DesktopState {
    #[serde(rename = "selectedDesktop")]
    selected_desktop: String,
    #[serde(rename = "displayManager")]
    display_manager: String,
}

#[derive(Deserialize)]
struct UserState {
    username: String,
    password: String,
    hostname: String,
}

#[derive(Deserialize)]
struct InstallerState {
    network: NetworkState,
    disk: DiskState,
    desktop: DesktopState,
    user: UserState,
}

#[command]
fn get_install_steps(state: InstallerState) -> Vec<InstallStep> {
    let mut steps = Vec::new();

    // --- 1. SYSTEM PREPARATION ---
    steps.push(InstallStep {
        name: "update_repos".to_string(),
        description: "Update system package repositories".to_string(),
        explanation: "Before installing software, we must sync the local package database with the remote servers. This ensures we download the latest versions and security patches.".to_string(),
        command: "apt update".to_string(),
        requires_sudo: true,
        category: "system".to_string(),
    });

    // --- 2. DISK & FILESYSTEM ---
    let fs_type = &state.disk.filesystem;
    let target_disk = &state.disk.selected_disk;
    
    steps.push(InstallStep {
        name: "prepare_disk".to_string(),
        description: format!("Initialize partition table on {}", target_disk),
        explanation: format!("We use 'parted' to create a GPT partition table. GPT is the modern standard for disk layouts, supporting larger drives and more partitions than the older MBR format."),
        command: format!("parted -s /dev/{} mklabel gpt", target_disk),
        requires_sudo: true,
        category: "disk".to_string(),
    });

    steps.push(InstallStep {
        name: "create_efi".to_string(),
        description: "Create EFI System Partition".to_string(),
        explanation: "The EFI partition (ESP) is where the bootloader (GRUB) resides. The BIOS looks here first to find the instructions needed to start the operating system.".to_string(),
        command: format!("parted -s /dev/{} mkpart primary fat32 1MiB 513MiB && parted -s /dev/{} set 1 esp on", target_disk, target_disk),
        requires_sudo: true,
        category: "disk".to_string(),
    });

    steps.push(InstallStep {
        name: "create_root".to_string(),
        description: format!("Create root (/) partition with {}", fs_type),
        explanation: format!("This partition will hold your entire operating system. Using {}, we ensure reliable data storage and efficient file access.", fs_type),
        command: format!("parted -s /dev/{} mkpart primary {} 513MiB 100%", target_disk, fs_type),
        requires_sudo: true,
        category: "disk".to_string(),
    });

    // --- 3. USER & SECURITY ---
    steps.push(InstallStep {
        name: "create_user".to_string(),
        description: format!("Create primary user: {}", state.user.username),
        explanation: "Linux is a multi-user system. We create your account and specify '/bin/bash' as your default shell—the environment where you interact with the system via text.".to_string(),
        command: format!("useradd -m -s /bin/bash {}", state.user.username),
        requires_sudo: true,
        category: "user".to_string(),
    });

    steps.push(InstallStep {
        name: "set_password".to_string(),
        description: "Secure user account".to_string(),
        explanation: "We securely pipe your password into 'chpasswd'. In a real environment, this password is encrypted (hashed) before being stored in '/etc/shadow'.".to_string(),
        command: format!("echo '{}:{}' | chpasswd", state.user.username.replace('\'', "'\\''"), state.user.password.replace('\'', "'\\''")),
        requires_sudo: true,
        category: "user".to_string(),
    });

    // --- 4. DESKTOP ENVIRONMENT ---
    let (de_name, de_pkgs) = match state.desktop.selected_desktop.as_str() {
        "gnome" => ("GNOME", "gnome-core gdm3 network-manager-gnome"),
        "kde" => ("KDE Plasma", "kde-plasma-desktop sddm"),
        "xfce" => ("XFCE", "xfce4 xfce4-goodies lightdm"),
        "hyprland" => ("Hyprland", "hyprland hyprpaper waybar wofi kitty"),
        _ => ("GNOME", "gnome-core gdm3"),
    };

    steps.push(InstallStep {
        name: "install_de".to_string(),
        description: format!("Install {} Graphical Interface", de_name),
        explanation: format!("This command downloads the Desktop Environment and Display Manager. The Display Manager (like GDM or SDDM) provides the login screen you see at boot."),
        command: format!("apt install -y {}", de_pkgs),
        requires_sudo: true,
        category: "desktop".to_string(),
    });

    // --- 5. NETWORK CONFIG ---
    if state.network.network_type == "wireless" && !state.network.wifi_network.is_empty() {
        steps.push(InstallStep {
            name: "setup_wifi".to_string(),
            description: format!("Configure WiFi: {}", state.network.wifi_network),
            explanation: "We use NetworkManager's CLI tool (nmcli) to securely save your wireless credentials. This allows the system to reconnect automatically after reboot.".to_string(),
            command: format!("nmcli dev wifi connect '{}' password '{}'", state.network.wifi_network.replace('\'', "'\\''"), state.network.wifi_password.replace('\'', "'\\''")),
            requires_sudo: true,
            category: "system".to_string(),
        });
    }

    steps
}

#[command]
fn download_with_curl(url: String, target_path: String) -> Result<CommandResult, String> {
    let curl_command = format!(
        "curl -L -fsSL --connect-timeout 30 --max-time 300 --retry 3 --retry-delay 5 -o '{}' '{}'",
        target_path, url
    );
    
    match Command::new("sh")
        .arg("-c")
        .arg(&curl_command)
        .output()
    {
        Ok(output) => {
            let stdout = String::from_utf8_lossy(&output.stdout).to_string();
            let stderr = String::from_utf8_lossy(&output.stderr).to_string();
            let success = output.status.success();
            
            // Additional verification: check if file exists and has content
            let file_verification = if success {
                match std::fs::metadata(&target_path) {
                    Ok(metadata) => {
                        let size = metadata.len();
                        if size == 0 {
                            format!("Downloaded file is empty (0 bytes)")
                        } else {
                            format!("Downloaded file size: {} bytes", size)
                        }
                    }
                    Err(e) => format!("Failed to verify downloaded file: {}", e),
                }
            } else {
                "Download failed".to_string()
            };
            
            Ok(CommandResult {
                success: success && std::path::Path::new(&target_path).exists(),
                output: format!("{}\n{}", stdout, file_verification),
                error: if stderr.is_empty() { None } else { Some(stderr) },
            })
        }
        Err(e) => Err(format!("Failed to execute curl download: {}", e))
    }
}

#[command]
fn verify_download(file_path: String) -> Result<serde_json::Value, String> {
    let path = std::path::Path::new(&file_path);
    
    if !path.exists() {
        return Ok(serde_json::json!({
            "exists": false,
            "error": "File does not exist"
        }));
    }
    
    match std::fs::metadata(&file_path) {
        Ok(metadata) => {
            let size = metadata.len();
            let modified = metadata.modified()
                .map(|t| format!("{}", t.duration_since(std::time::UNIX_EPOCH).unwrap_or_default().as_secs()))
                .unwrap_or_else(|_| "Unknown".to_string());
            
            // Try to get file type info
            let file_type = match Command::new("file")
                .arg(&file_path)
                .output()
            {
                Ok(output) => {
                    if output.status.success() {
                        String::from_utf8_lossy(&output.stdout).trim().to_string()
                    } else {
                        "Unknown".to_string()
                    }
                }
                Err(_) => "Unknown".to_string(),
            };
            
            // Check if it's a valid package file (basic checks)
            let is_valid_package = if file_path.ends_with(".deb") {
                size > 1000000 && file_type.contains("Debian") // > 1MB and Debian package
            } else if file_path.ends_with(".sh") {
                size > 1000 && file_type.contains("Bourne") // > 1KB and shell script
            } else if file_path.ends_with(".AppImage") {
                size > 10000000 && file_type.contains("ELF") // > 10MB and ELF executable
            } else {
                size > 0 // Just check if it's not empty for other files
            };
            
            Ok(serde_json::json!({
                "exists": true,
                "size": size,
                "size_mb": format!("{:.2}", size as f64 / 1024.0 / 1024.0),
                "modified": modified,
                "file_type": file_type,
                "is_valid_package": is_valid_package,
                "readable": metadata.permissions().readonly() == false
            }))
        }
        Err(e) => Ok(serde_json::json!({
            "exists": false,
            "error": format!("Failed to read file metadata: {}", e)
        }))
    }
}

#[command]
fn execute_command(command: String, requires_sudo: bool) -> Result<CommandResult, String> {
    let full_command = if requires_sudo {
        format!("sudo {}", command)
    } else {
        command
    };
    
    // Add timeout for long-running commands with proper escaping
    let escaped_command = full_command.replace("'", "'\\''");
    let timeout_command = format!(
        "timeout 300 sh -c '{}'",
        escaped_command
    );
    
    match Command::new("sh")
        .arg("-c")
        .arg(&timeout_command)
        .output()
    {
        Ok(output) => {
            let stdout = String::from_utf8_lossy(&output.stdout).to_string();
            let stderr = String::from_utf8_lossy(&output.stderr).to_string();
            let status_code = output.status.code().unwrap_or(-1);
            
            let success = output.status.success();
            let mut error_msg = None;
            
            if !success {
                if status_code == 124 {
                    error_msg = Some("Command timed out after 5 minutes".to_string());
                } else if requires_sudo && stderr.contains("sudo:") && stderr.contains("password") {
                    error_msg = Some("Sudo authentication failed - password may be required".to_string());
                } else if !stderr.is_empty() {
                    error_msg = Some(stderr);
                } else {
                    error_msg = Some(format!("Command failed with exit code {}", status_code));
                }
            }
            
            Ok(CommandResult {
                success,
                output: stdout,
                error: error_msg,
            })
        }
        Err(e) => Err(format!("Failed to execute command: {}", e))
    }
}

#[command]
fn check_sudo_available() -> bool {
    Command::new("sudo")
        .arg("-n")
        .arg("true")
        .output()
        .map(|output| output.status.success())
        .unwrap_or(false)
}

#[command]
fn get_system_info() -> Result<serde_json::Value, String> {
    let hostname = Command::new("hostname")
        .output()
        .and_then(|o| Ok(String::from_utf8_lossy(&o.stdout).trim().to_string()))
        .unwrap_or_else(|_| "Unknown".to_string());
    
    let uptime = Command::new("uptime")
        .output()
        .and_then(|o| Ok(String::from_utf8_lossy(&o.stdout).trim().to_string()))
        .unwrap_or_else(|_| "Unknown".to_string());

    Ok(serde_json::json!({
        "hostname": hostname,
        "uptime": uptime
    }))
}

fn run_cmd(program: &str, args: &[&str]) -> Result<String, String> {
    let out = Command::new(program)
        .args(args)
        .output()
        .map_err(|e| format!("{} failed: {}", program, e))?;
    if !out.status.success() {
        return Err(String::from_utf8_lossy(&out.stderr).trim().to_string());
    }
    Ok(String::from_utf8_lossy(&out.stdout).trim().to_string())
}

fn parse_lsblk_devices(payload: &str) -> Result<Vec<Value>, String> {
    let parsed: Value = serde_json::from_str(payload).map_err(|e| format!("Invalid lsblk JSON: {}", e))?;
    let devices = parsed
        .get("blockdevices")
        .and_then(|v| v.as_array())
        .ok_or_else(|| "lsblk JSON missing blockdevices".to_string())?;
    Ok(devices.clone())
}

#[command]
fn get_system_statistics() -> Result<serde_json::Value, String> {
    let cpu_model = run_cmd("sh", &["-c", "lscpu 2>/dev/null | awk -F: '/Model name/{print $2; exit}' | xargs"])
        .unwrap_or_else(|_| "Unknown CPU".to_string());
    let mem = run_cmd("sh", &["-c", "free -h 2>/dev/null | awk '/Mem:/{print $3\"/\"$2}'"])
        .unwrap_or_else(|_| "Unknown".to_string());
    let load = run_cmd("sh", &["-c", "uptime | awk -F'load average: ' '{print $2}'"])
        .unwrap_or_else(|_| "Unknown".to_string());
    let root_usage = run_cmd("sh", &["-c", "df -h / | awk 'NR==2{print $3\" used of \"$2\" (\"$5\")\"}'"])
        .unwrap_or_else(|_| "Unknown".to_string());

    Ok(serde_json::json!({
        "cpuModel": cpu_model,
        "memoryUsage": mem,
        "loadAverage": load,
        "rootUsage": root_usage
    }))
}

#[command]
fn list_block_devices(simulation: bool) -> Result<serde_json::Value, String> {
    if simulation {
        return Ok(serde_json::json!({
            "devices": [
                {"name":"sda","size":"500G","model":"Samsung SSD 870 EVO"},
                {"name":"sdb","size":"1T","model":"WD Blue"}
            ]
        }));
    }

    // Primary strategy: structured lsblk JSON.
    if let Ok(lsblk_json) = run_cmd("lsblk", &["-J", "-o", "NAME,SIZE,MODEL,TYPE,FSTYPE,MOUNTPOINT"]) {
        if let Ok(devices) = parse_lsblk_devices(&lsblk_json) {
            return Ok(serde_json::json!({ "source": "lsblk", "devices": devices }));
        }
    }

    // Fallback strategy: fdisk text output + minimal parsing.
    if let Ok(fdisk_text) = run_cmd("sh", &["-c", "fdisk -l 2>/dev/null | grep '^Disk /dev/'"]) {
        let mut devices: Vec<Value> = Vec::new();
        for line in fdisk_text.lines() {
            // Example: Disk /dev/sda: 465.8 GiB, ...
            let parts: Vec<&str> = line.split(':').collect();
            if parts.len() >= 2 {
                let name = parts[0].replace("Disk ", "").trim().to_string();
                let size = parts[1].split(',').next().unwrap_or("").trim().to_string();
                devices.push(serde_json::json!({
                    "name": name.trim_start_matches("/dev/"),
                    "size": size,
                    "model": "Unknown",
                    "type": "disk",
                    "children": []
                }));
            }
        }
        if !devices.is_empty() {
            return Ok(serde_json::json!({ "source": "fdisk", "devices": devices }));
        }
    }

    // Last fallback: mounted filesystems from df.
    if let Ok(df_text) = run_cmd("sh", &["-c", "df -h | awk 'NR>1{print $1\" \"$2\" \"$6}'"]) {
        let mut devices: Vec<Value> = Vec::new();
        for line in df_text.lines() {
            let cols: Vec<&str> = line.split_whitespace().collect();
            if cols.len() >= 3 && cols[0].starts_with("/dev/") {
                devices.push(serde_json::json!({
                    "name": cols[0].trim_start_matches("/dev/"),
                    "size": cols[1],
                    "model": "Mounted filesystem",
                    "type": "disk",
                    "children": [{"name": cols[0], "mountpoint": cols[2]}]
                }));
            }
        }
        if !devices.is_empty() {
            return Ok(serde_json::json!({ "source": "df", "devices": devices }));
        }
    }

    Err("Could not discover block devices using lsblk, fdisk, or df".to_string())
}

#[command]
fn probe_partitions(device: String, simulation: bool) -> Result<serde_json::Value, String> {
    if simulation {
        return Ok(serde_json::json!({
            "device": device,
            "partitions": [
                {"name":"sdc1","fstype":"fat32","size":"512M"},
                {"name":"sdc2","fstype":"ext4","size":"100G"}
            ]
        }));
    }

    // Primary strategy: blkid.
    if let Ok(raw) = run_cmd("blkid", &[&device]) {
        return Ok(serde_json::json!({
            "device": device,
            "source": "blkid",
            "raw": raw,
            "success": true
        }));
    }

    // Fallback strategy: lsblk on requested device.
    if let Ok(lsblk_json) = run_cmd("lsblk", &["-J", "-o", "NAME,SIZE,FSTYPE,MOUNTPOINT", &format!("/dev/{}", device)]) {
        return Ok(serde_json::json!({
            "device": device,
            "source": "lsblk",
            "raw": lsblk_json,
            "success": true
        }));
    }

    Err(format!("Could not probe partitions for {} using blkid or lsblk", device))
}

#[command]
fn apply_partition_plan(device: String, plan: Vec<PartitionPlanItem>, simulation: bool) -> Result<serde_json::Value, String> {
    if simulation {
        let steps: Vec<String> = plan
            .iter()
            .map(|p| format!("create {} {} {}GB encrypted={} efi={}", p.mountpoint, p.filesystem, p.size_gb, p.encrypted, p.efi))
            .collect();
        return Ok(serde_json::json!({
            "mode": "simulation",
            "device": device,
            "steps": steps
        }));
    }

    // Placeholder for dangerous mode implementation in Linux live environment.
    // This intentionally returns an explicit blocker until validated destructive flows are added.
    Err("Dangerous partition application is not enabled yet. Use simulation mode.".to_string())
}

#[command]
fn configure_time_sync(
    timezone: String,
    use_ntp: bool,
    ntp_server: String,
    simulation: bool
) -> Result<serde_json::Value, String> {
    if simulation {
        return Ok(serde_json::json!({
            "mode":"simulation",
            "timezone": timezone,
            "useNtp": use_ntp,
            "ntpServer": ntp_server
        }));
    }

    Err("Dangerous time-sync configuration is not enabled yet. Use simulation mode.".to_string())
}

#[command]
fn scan_wifi_networks(simulation: bool) -> Result<serde_json::Value, String> {
    if simulation {
        return Ok(serde_json::json!([
            { "ssid": "Home_WiFi_5G", "security": "WPA2", "strength": "excellent", "frequency": "5 GHz" },
            { "ssid": "Office_Guest", "security": "WPA3", "strength": "good", "frequency": "5 GHz" },
            { "ssid": "Starbucks_Free", "security": "Open", "strength": "fair", "frequency": "2.4 GHz" },
            { "ssid": "Neighbor_Network", "security": "WPA2", "strength": "poor", "frequency": "2.4 GHz" }
        ]));
    }

    // Use nmcli to scan for wifi networks
    // -t: terse output (no headers, simple format)
    // -f: specify fields
    match Command::new("nmcli")
        .args(&["-t", "-f", "SSID,SECURITY,SIGNAL,FREQ", "dev", "wifi"])
        .output()
    {
        Ok(output) => {
            if !output.status.success() {
                return Err(String::from_utf8_lossy(&output.stderr).trim().to_string());
            }

            let stdout = String::from_utf8_lossy(&output.stdout);
            let mut networks = Vec::new();

            for line in stdout.lines() {
                if line.trim().is_empty() {
                    continue;
                }

                // nmcli -t uses ':' as default separator
                // Format: SSID:SECURITY:SIGNAL:FREQ
                let parts: Vec<&str> = line.split(':').collect();
                if parts.len() >= 4 {
                    let ssid = parts[0].to_string();
                    if ssid.is_empty() {
                        continue; // Skip hidden/unnamed networks for now
                    }

                    let security = parts[1].to_string();
                    let signal_val: i32 = parts[2].parse().unwrap_or(0);
                    let freq = parts[3].to_string();

                    let strength = if signal_val >= 80 {
                        "excellent"
                    } else if signal_val >= 60 {
                        "good"
                    } else if signal_val >= 40 {
                        "fair"
                    } else {
                        "poor"
                    };

                    networks.push(serde_json::json!({
                        "ssid": ssid,
                        "security": security,
                        "strength": strength,
                        "frequency": freq
                    }));
                }
            }

            // Deduplicate SSIDs (nmcli often shows same SSID for different frequencies)
            networks.sort_by(|a, b| a["ssid"].as_str().unwrap().cmp(b["ssid"].as_str().unwrap()));
            networks.dedup_by(|a, b| a["ssid"] == b["ssid"]);

            Ok(serde_json::Value::Array(networks))
        }
        Err(e) => Err(format!("Failed to execute nmcli: {}. Is network-manager installed?", e))
    }
}

#[command]
fn get_battery_info() -> Result<serde_json::Value, String> {
    use std::fs;

    // Check common Linux power_supply paths
    let base_path = "/sys/class/power_supply";
    let entries = fs::read_dir(base_path).map_err(|e| e.to_string())?;

    for entry in entries {
        let entry = entry.map_err(|e| e.to_string())?;
        let path = entry.path();
        let type_path = path.join("type");
        
        if let Ok(p_type) = fs::read_to_string(type_path) {
            if p_type.trim() == "Battery" {
                let capacity = fs::read_to_string(path.join("capacity"))
                    .unwrap_or_else(|_| "0".to_string())
                    .trim()
                    .parse::<i32>()
                    .unwrap_or(0);
                
                let status = fs::read_to_string(path.join("status"))
                    .unwrap_or_else(|_| "Unknown".to_string())
                    .trim()
                    .to_string();

                return Ok(serde_json::json!({
                    "percentage": capacity,
                    "status": status,
                    "is_present": true
                }));
            }
        }
    }

    Ok(serde_json::json!({
        "percentage": 100,
        "status": "Full",
        "is_present": false
    }))
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
            get_install_steps,
            execute_command,
            download_with_curl,
            verify_download,
            check_sudo_available,
            get_system_info,
            get_system_statistics,
            list_block_devices,
            probe_partitions,
            apply_partition_plan,
            configure_time_sync,
            scan_wifi_networks,
            get_battery_info
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
