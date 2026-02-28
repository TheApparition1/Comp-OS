using InstallerFlows.Models;
using InstallerFlows.Services;

namespace InstallerFlows.UI
{
    public class MenuInterface
    {
        private readonly ProfileService _profileService;
        private readonly InstallationService _installationService;

        public MenuInterface()
        {
            _profileService = new ProfileService();
            _installationService = new InstallationService();
        }

        public async Task StartAsync()
        {
            Console.Clear();
            Console.WriteLine("=== Arch Linux Installer Flow ===");
            Console.WriteLine("Select your user profile to get started with recommended applications.\n");

            var profiles = _profileService.GetProfiles();
            var selectedProfile = SelectProfile(profiles);

            if (selectedProfile != null)
            {
                await HandleProfileSelection(selectedProfile);
            }
        }

        private UserProfile? SelectProfile(List<UserProfile> profiles)
        {
            while (true)
            {
                Console.WriteLine("Available Profiles:");
                for (int i = 0; i < profiles.Count; i++)
                {
                    Console.WriteLine($"{i + 1}. {profiles[i].Name}");
                    Console.WriteLine($"   {profiles[i].Description}");
                    Console.WriteLine();
                }

                Console.Write("Enter the number of your profile (or 'q' to quit): ");
                var input = Console.ReadLine();

                if (input?.ToLower() == "q")
                {
                    return null;
                }

                if (int.TryParse(input, out int selection) && selection > 0 && selection <= profiles.Count)
                {
                    return profiles[selection - 1];
                }

                Console.WriteLine("Invalid selection. Please try again.\n");
            }
        }

        private async Task HandleProfileSelection(UserProfile profile)
        {
            Console.Clear();
            Console.WriteLine($"=== {profile.Name} Profile ===");
            Console.WriteLine($"{profile.Description}\n");

            Console.WriteLine("Recommended Applications:");
            DisplayAppSelection(profile.RecommendedApps);

            if (ConfirmSelection(profile))
            {
                await ShowOverviewAndInstall(profile);
            }
            else
            {
                Console.WriteLine("Installation cancelled.");
            }
        }

        private void DisplayAppSelection(List<AppPackage> apps)
        {
            for (int i = 0; i < apps.Count; i++)
            {
                var status = apps[i].IsSelected ? "[✓]" : "[ ]";
                Console.WriteLine($"{i + 1}. {status} {apps[i].Name}");
                Console.WriteLine($"     {apps[i].Description}");
                Console.WriteLine();
            }
        }

        private void HandleAppSelection(List<AppPackage> apps)
        {
            while (true)
            {
                Console.WriteLine("\nApp Selection Options:");
                Console.WriteLine("1. Toggle app selection (enter app number)");
                Console.WriteLine("2. Select all apps");
                Console.WriteLine("3. Deselect all apps");
                Console.WriteLine("4. Continue to installation");
                Console.Write("Choose an option: ");

                var input = Console.ReadLine();

                switch (input)
                {
                    case "1":
                        ToggleAppSelection(apps);
                        break;
                    case "2":
                        apps.ForEach(app => app.IsSelected = true);
                        Console.Clear();
                        Console.WriteLine("All apps selected.\n");
                        DisplayAppSelection(apps);
                        break;
                    case "3":
                        apps.ForEach(app => app.IsSelected = false);
                        Console.Clear();
                        Console.WriteLine("All apps deselected.\n");
                        DisplayAppSelection(apps);
                        break;
                    case "4":
                        return;
                    default:
                        Console.WriteLine("Invalid option. Please try again.");
                        break;
                }
            }
        }

        private void ToggleAppSelection(List<AppPackage> apps)
        {
            Console.Write("Enter app number to toggle: ");
            if (int.TryParse(Console.ReadLine(), out int appNumber) && appNumber > 0 && appNumber <= apps.Count)
            {
                apps[appNumber - 1].IsSelected = !apps[appNumber - 1].IsSelected;
                Console.Clear();
                DisplayAppSelection(apps);
            }
            else
            {
                Console.WriteLine("Invalid app number.");
            }
        }

        private bool ConfirmSelection(UserProfile profile)
        {
            HandleAppSelection(profile.RecommendedApps);

            var selectedCount = profile.RecommendedApps.Count(app => app.IsSelected);
            Console.WriteLine($"\nYou have selected {selectedCount} applications for installation.");
            Console.Write("Proceed with installation? (y/n): ");

            return Console.ReadLine()?.ToLower() == "y";
        }

        private async Task ShowOverviewAndInstall(UserProfile profile)
        {
            Console.Clear();
            Console.WriteLine("=== Installation Overview ===");
            Console.WriteLine($"Profile: {profile.Name}");
            Console.WriteLine($"Total apps to install: {profile.RecommendedApps.Count(app => app.IsSelected)}\n");

            Console.WriteLine("Selected Applications:");
            foreach (var app in profile.RecommendedApps.Where(app => app.IsSelected))
            {
                Console.WriteLine($"• {app.Name} - {app.Description}");
            }

            Console.WriteLine("\nPress Enter to start installation or Ctrl+C to cancel...");
            Console.ReadLine();

            var progress = new Progress<string>(message =>
            {
                Console.WriteLine(message);
            });

            var success = await _installationService.InstallPackagesAsync(profile.RecommendedApps, progress);

            if (success)
            {
                Console.WriteLine("\n🎉 Installation completed successfully!");
                Console.WriteLine("You may need to restart your system or log out/in for some changes to take effect.");
            }
            else
            {
                Console.WriteLine("\n❌ Installation encountered errors. Please check the messages above.");
            }

            Console.WriteLine("\nPress Enter to exit...");
            Console.ReadLine();
        }
    }
}

