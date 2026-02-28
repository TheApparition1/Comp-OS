using System;
using System.Diagnostics;
using System.Diagnostics.CodeAnalysis;

internal class AutoComp_File1
{
    public static void Main(string[] args)
    {
        FileOpened();

        void FileOpened()
        {
            string executableType = "exe"; 
            if (executableType == "exe")
            {
                RunWithWine();
            }
            else if (executableType == "msi")
            {
                RunWithWine();
            }
            else
            {  // This is a placeholder for future cancellation.
            }
        }

        static void RunWithWine()
        {
            string file = "kitty.deb";
            string command = "wine " + file;
            Console.WriteLine("Command to run: " + command);
    
            var process = new Process();
            process.StartInfo.FileName = "wine";
            process.StartInfo.Arguments = file;
            process.StartInfo.UseShellExecute = false;
            process.StartInfo.RedirectStandardOutput = true;
            process.Start();

            string output = process.StandardOutput.ReadToEnd();
            process.WaitForExit();

            Console.WriteLine(output);
        }
    }
}
