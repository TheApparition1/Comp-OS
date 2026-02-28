using InstallerFlows.Models;

namespace InstallerFlows.Services
{
    public class InstallationService
    {
        public async Task<bool> InstallPackagesAsync(List<AppPackage> packages, IProgress<string> progress)
        {
            var selectedPackages = packages.Where(p => p.IsSelected).ToList();

            if (!selectedPackages.Any())
            {
                progress?.Report("No packages selected for installation.");
                return false;
            }

            progress?.Report($"Starting installation of {selectedPackages.Count} packages...");

            foreach (var package in selectedPackages)
            {
                try
                {
                    progress?.Report($"Installing {package.Name}...");

                    var process = new System.Diagnostics.Process
                    {
                        StartInfo = new System.Diagnostics.ProcessStartInfo
                        {
                            FileName = "/bin/bash",
                            Arguments = $"-c \"{package.InstallCommand}\"",
                            RedirectStandardOutput = true,
                            RedirectStandardError = true,
                            UseShellExecute = false,
                            CreateNoWindow = true
                        }
                    };

                    process.Start();
                    await process.WaitForExitAsync();

                    if (process.ExitCode == 0)
                    {
                        progress?.Report($"✓ Successfully installed {package.Name}");
                    }
                    else
                    {
                        var error = await process.StandardError.ReadToEndAsync();
                        progress?.Report($"✗ Failed to install {package.Name}: {error}");
                        return false;
                    }
                }
                catch (Exception ex)
                {
                    progress?.Report($"✗ Error installing {package.Name}: {ex.Message}");
                    return false;
                }
            }

            progress?.Report("Installation completed successfully!");
            return true;
        }
    }

    public class ProfileService
    {
        public List<UserProfile> GetProfiles()
        {
            return new List<UserProfile>
            {
                new UserProfile
                {
                    Name = "Developer",
                    Description = "Software development tools and environments",
                    RecommendedApps = new List<AppPackage>
                    {
                        new AppPackage { Name = "Git", Description = "Version control system", InstallCommand = "pacman -S git --noconfirm" },
                        new AppPackage { Name = "Visual Studio Code", Description = "Code editor", InstallCommand = "pacman -S code --noconfirm" },
                        new AppPackage { Name = "Docker", Description = "Container platform", InstallCommand = "pacman -S docker --noconfirm" },
                        new AppPackage { Name = "Node.js", Description = "JavaScript runtime", InstallCommand = "pacman -S nodejs npm --noconfirm" },
                        new AppPackage { Name = "Python", Description = "Programming language", InstallCommand = "pacman -S python python-pip --noconfirm" },
                        new AppPackage { Name = "Java JDK", Description = "Java development kit", InstallCommand = "pacman -S jdk-openjdk --noconfirm" },
                        new AppPackage { Name = "Postman", Description = "API testing tool", InstallCommand = "pacman -S postman-bin --noconfirm" },
                        new AppPackage { Name = "GitHub CLI", Description = "GitHub command line tool", InstallCommand = "pacman -S github-cli --noconfirm" }
                    }
                },
                new UserProfile
                {
                    Name = "Artist",
                    Description = "Creative and design applications",
                    RecommendedApps = new List<AppPackage>
                    {
                        new AppPackage { Name = "GIMP", Description = "Image editing software", InstallCommand = "pacman -S gimp --noconfirm" },
                        new AppPackage { Name = "Inkscape", Description = "Vector graphics editor", InstallCommand = "pacman -S inkscape --noconfirm" },
                        new AppPackage { Name = "Blender", Description = "3D creation suite", InstallCommand = "pacman -S blender --noconfirm" },
                        new AppPackage { Name = "Krita", Description = "Digital painting software", InstallCommand = "pacman -S krita --noconfirm" },
                        new AppPackage { Name = "Audacity", Description = "Audio editor", InstallCommand = "pacman -S audacity --noconfirm" },
                        new AppPackage { Name = "OBS Studio", Description = "Video recording and streaming", InstallCommand = "pacman -S obs-studio --noconfirm" },
                        new AppPackage { Name = "Darktable", Description = "Photography workflow software", InstallCommand = "pacman -S darktable --noconfirm" }
                    }
                },
                new UserProfile
                {
                    Name = "General User",
                    Description = "Essential applications for everyday use",
                    RecommendedApps = new List<AppPackage>
                    {
                        new AppPackage { Name = "Firefox", Description = "Web browser", InstallCommand = "pacman -S firefox --noconfirm" },
                        new AppPackage { Name = "LibreOffice", Description = "Office suite", InstallCommand = "pacman -S libreoffice-fresh --noconfirm" },
                        new AppPackage { Name = "VLC Media Player", Description = "Media player", InstallCommand = "pacman -S vlc --noconfirm" },
                        new AppPackage { Name = "Thunderbird", Description = "Email client", InstallCommand = "pacman -S thunderbird --noconfirm" },
                        new AppPackage { Name = "FileZilla", Description = "FTP client", InstallCommand = "pacman -S filezilla --noconfirm" },
                        new AppPackage { Name = "VeraCrypt", Description = "Encryption software", InstallCommand = "pacman -S veracrypt --noconfirm" }
                    }
                },
                new UserProfile
                {
                    Name = "Gamer",
                    Description = "Gaming platforms and utilities",
                    RecommendedApps = new List<AppPackage>
                    {
                        new AppPackage { Name = "Steam", Description = "Gaming platform", InstallCommand = "pacman -S steam --noconfirm" },
                        new AppPackage { Name = "Lutris", Description = "Gaming platform for multiple stores", InstallCommand = "pacman -S lutris --noconfirm" },
                        new AppPackage { Name = "Wine", Description = "Windows compatibility layer", InstallCommand = "pacman -S wine --noconfirm" },
                        new AppPackage { Name = "Discord", Description = "Voice and text chat", InstallCommand = "pacman -S discord --noconfirm" },
                        new AppPackage { Name = "MangoHud", Description = "Gaming performance overlay", InstallCommand = "pacman -S mangohud --noconfirm" },
                        new AppPackage { Name = "ProtonUp-Qt", Description = "Proton version manager", InstallCommand = "pacman -S protonup-qt --noconfirm" }
                    }
                },
                new UserProfile
                {
                    Name = "Student",
                    Description = "Educational and productivity tools",
                    RecommendedApps = new List<AppPackage>
                    {
                        new AppPackage { Name = "LibreOffice", Description = "Office suite", InstallCommand = "pacman -S libreoffice-fresh --noconfirm" },
                        new AppPackage { Name = "Firefox", Description = "Web browser", InstallCommand = "pacman -S firefox --noconfirm" },
                        new AppPackage { Name = "Anki", Description = "Flashcard software", InstallCommand = "pacman -S anki --noconfirm" },
                        new AppPackage { Name = "Zotero", Description = "Reference management", InstallCommand = "pacman -S zotero --noconfirm" },
                        new AppPackage { Name = "Obsidian", Description = "Note-taking and knowledge management", InstallCommand = "pacman -S obsidian --noconfirm" },
                        new AppPackage { Name = "Python", Description = "Programming language for data science", InstallCommand = "pacman -S python python-pip --noconfirm" },
                        new AppPackage { Name = "LaTeX", Description = "Document preparation system", InstallCommand = "pacman -S texlive-most --noconfirm" }
                    }
                },
                new UserProfile
                {
                    Name = "System Administrator",
                    Description = "System management and monitoring tools",
                    RecommendedApps = new List<AppPackage>
                    {
                        new AppPackage { Name = "htop", Description = "Process viewer", InstallCommand = "pacman -S htop --noconfirm" },
                        new AppPackage { Name = "neofetch", Description = "System information tool", InstallCommand = "pacman -S neofetch --noconfirm" },
                        new AppPackage { Name = "tmux", Description = "Terminal multiplexer", InstallCommand = "pacman -S tmux --noconfirm" },
                        new AppPackage { Name = "rsync", Description = "File synchronization tool", InstallCommand = "pacman -S rsync --noconfirm" },
                        new AppPackage { Name = "nmap", Description = "Network scanner", InstallCommand = "pacman -S nmap --noconfirm" },
                        new AppPackage { Name = "Wireshark", Description = "Network protocol analyzer", InstallCommand = "pacman -S wireshark-qt --noconfirm" },
                        new AppPackage { Name = "Fail2ban", Description = "Intrusion prevention software", InstallCommand = "pacman -S fail2ban --noconfirm" }
                    }
                }
            };
        }
    }
}

