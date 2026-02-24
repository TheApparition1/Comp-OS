using System.Diagnostics;
using System;
using System.IO;
#!/usr/bin/env -S dotnet run

string executableType;
string file;
string command;
string consoleName = "kitty.deb";
void FileOpened()
{
    executableType = "exe";
    if(Convert.ToBoolean(executableType = "exe"))
    {
        RunWithWine();
    }
    else if (Convert.ToBoolean(executableType = "msi"))
    {
        RunWithWine();
    }
    else
    {}
}

void RunWithWine()
{ 
    command = "wine " + file;
    Console.Write(command);
    Console.Write("exit");
}