using System;
using System.Diagnostics;

#!/usr/bin/env -S dotnet;

string executableType;
string file = "kitty.deb";
string command;
string consoleName = "kitty.deb";

FileOpened();

void FileOpened()
{
    executableType = "exe"; 
    if (executableType == "exe")
    {
        RunWithWine();
    }
    else if (executableType == "msi")
    {
        RunWithWine();
    }
    else
    {
    }
}

void RunWithWine();
{
    command = "wine " + file;
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
