use serde::Serialize;
use serde_json::Value;

#[derive(Serialize)]
struct SystemInfo {
    hostname: String,
}

#[derive(Serialize)]
struct SystemStatistics {
    cpu_usage: f64,
    memory_usage: f64,
}

#[derive(Serialize)]
struct BatteryInfo {
    percentage: u8,
    status: String,
    is_present: bool,
}

#[derive(Serialize)]
struct InstallStep {
    description: String,
    command: String,
    requires_sudo: bool,
    category: String,
    explanation: String,
}

#[derive(Serialize)]
struct CommandResult {
    success: bool,
    output: Option<String>,
    error: Option<String>,
}

#[derive(Serialize)]
struct BlockDeviceResponse {
    devices: Vec<BlockDevice>,
}

#[derive(Serialize)]
struct BlockDevice {
    name: String,
    model: String,
    size: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "type")]
    device_type: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tran: Option<String>,
    children: Vec<BlockDevicePartition>,
}

#[derive(Serialize)]
struct BlockDevicePartition {
    name: String,
    size: String,
    fstype: String,
    mountpoint: String,
}

#[derive(Serialize)]
struct WifiNetwork {
    ssid: String,
    frequency: String,
    security: String,
    strength: String,
}

#[tauri::command]
fn check_sudo_available() -> bool {
    cfg!(target_family = "unix")
}

#[tauri::command]
fn get_system_info() -> SystemInfo {
    let hostname = std::fs::read_to_string("/etc/hostname")
        .ok()
        .map(|value| value.trim().to_string())
        .filter(|value| !value.is_empty())
        .or_else(|| std::env::var("HOSTNAME").ok())
        .or_else(|| std::env::var("COMPUTERNAME").ok())
        .unwrap_or_else(|| "Unknown".to_string());
    SystemInfo { hostname }
}

#[tauri::command]
fn get_system_statistics() -> SystemStatistics {
    SystemStatistics {
        cpu_usage: 1.0,
        memory_usage: 1.0,
    }
}

#[tauri::command]
fn get_battery_info() -> BatteryInfo {
    BatteryInfo {
        percentage: 0,
        status: "Unavailable".to_string(),
        is_present: false,
    }
}

#[tauri::command]
fn get_install_steps(_state: Option<Value>) -> Vec<InstallStep> {
    vec![
        InstallStep {
            description: "Prepare package database".to_string(),
            command: "pacman -Sy".to_string(),
            requires_sudo: true,
            category: "system".to_string(),
            explanation: "Refresh package metadata before installation.".to_string(),
        },
        InstallStep {
            description: "Install base system".to_string(),
            command: "pacstrap /mnt base linux linux-firmware".to_string(),
            requires_sudo: true,
            category: "install".to_string(),
            explanation: "Install core packages to the target mount point.".to_string(),
        },
    ]
}

#[tauri::command]
fn execute_command(command: String, requires_sudo: Option<bool>) -> CommandResult {
    let prefix = if requires_sudo.unwrap_or(false) {
        "(sudo)"
    } else {
        "(user)"
    };
    CommandResult {
        success: true,
        output: Some(format!("Simulated execution {prefix}: {command}")),
        error: None,
    }
}

#[tauri::command]
fn list_block_devices(simulation: Option<bool>) -> BlockDeviceResponse {
    let use_mock = simulation.unwrap_or(true);
    if use_mock {
        return BlockDeviceResponse {
            devices: vec![BlockDevice {
                name: "sda".to_string(),
                model: "Simulated Disk".to_string(),
                size: "256G".to_string(),
                device_type: Some("disk".to_string()),
                tran: Some("sata".to_string()),
                children: vec![],
            }],
        };
    }

    BlockDeviceResponse {
        devices: vec![BlockDevice {
            name: "sda".to_string(),
            model: "Generic Disk".to_string(),
            size: "512G".to_string(),
            device_type: Some("disk".to_string()),
            tran: Some("sata".to_string()),
            children: vec![BlockDevicePartition {
                name: "sda1".to_string(),
                size: "512G".to_string(),
                fstype: "ext4".to_string(),
                mountpoint: "/".to_string(),
            }],
        }],
    }
}

#[tauri::command]
fn scan_wifi_networks(_simulation: Option<bool>) -> Vec<WifiNetwork> {
    vec![
        WifiNetwork {
            ssid: "CompOS-Network".to_string(),
            frequency: "5GHz".to_string(),
            security: "WPA2".to_string(),
            strength: "excellent".to_string(),
        },
        WifiNetwork {
            ssid: "Guest".to_string(),
            frequency: "2.4GHz".to_string(),
            security: "Open".to_string(),
            strength: "good".to_string(),
        },
    ]
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri::plugin::prelude::Builder::default().build())
        .invoke_handler(tauri::generate_handler![
            check_sudo_available,
            get_system_info,
            get_system_statistics,
            get_battery_info,
            get_install_steps,
            execute_command,
            list_block_devices,
            scan_wifi_networks
        ])
        .setup(|app| {
            #[cfg(debug_assertions)]
            {
                let window = app.get_webview_window("main").unwrap();
                window.open_devtools();
            }
            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
