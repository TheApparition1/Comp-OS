using Installer_Flows.UI;

namespace Installer_Flows
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