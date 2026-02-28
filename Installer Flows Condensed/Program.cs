using InstallerFlows.UI;

namespace InstallerFlows
{
    class Program
    {
        static async Task Main(string[] args)
        {
            try
            {
                var menuInterface = new MenuInterface();
                await menuInterface.StartAsync();
            }
            catch (Exception ex)
            {
                Console.WriteLine($"An error occurred: {ex.Message}");
                Console.WriteLine("Press Enter to exit...");
                Console.ReadLine();
            }
        }
    }
}

