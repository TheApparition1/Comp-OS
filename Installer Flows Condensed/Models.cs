namespace InstallerFlows.Models
{
    public class UserProfile
    {
        public string Name { get; set; } = string.Empty;
        public string Description { get; set; } = string.Empty;
        public List<AppPackage> RecommendedApps { get; set; } = new();
    }

    public class AppPackage
    {
        public string Name { get; set; } = string.Empty;
        public string Description { get; set; } = string.Empty;
        public string InstallCommand { get; set; } = string.Empty;
        public bool IsSelected { get; set; } = false;
    }
}

