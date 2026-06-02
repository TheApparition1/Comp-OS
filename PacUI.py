import os

service = ""

print("Welcome to PacUI")
print("Your prompt based, guided interface for Arch Linux's Pacman package manager.")
service = input("press Q followed by enter to list installed packages, QD to get details on a specific package, I to install a package, R to delete a package, or U to update all packages: ")
if service == "Q":
    os.system("pacman -Q")
elif service == "QD":
    QueryTarget = input("Type package name here: ")
    os.system("pacman -Q " + QueryTarget)
elif service == "I":
    IMode = input("Press P for arch packages or A for AUR packages: ")
    if IMode == "P":
        package = input("Type the name of your desired package here(if you want multiple packages, seperate with a space): ")
        print("To confirm, you are installing " + package)
        input("Press enter to install " + package)
        os.system("sudo -S pacman -S " + package)
    elif IMode == "S":
        package = input("Type the name of your desired package here(if you want multiple packages, seperate with a space): ")
        print("To confirm, you are installing " + package)
        input("Press enter to install " + package)
        os.system("yay -S " + package)
elif service == "R":
    mode = input("Press A for advanced mode, or S for easy ")
    if mode == "S":
        packageDel = input("Type the name of your desired package here(if you want multiple packages, seperate with a space): ")
        print("To confirm, you are deleting " + packageDel)
        input("Press enter to delete " + packageDel)
        os.system("sudo -S pacman -Rs " + packageDel)
    elif mode ==  "A":
        deleteA = input("Press R to delete a package, D to delete a package and all it's dependencies, F to force delete a package and all it's dependencies: ")
        if deleteA == "R":
            packageDel = input("Type the name of your desired package here(if you want multiple packages, seperate with a space): ")
            print("To confirm, you are deleting " + packageDel)
            input("Press enter to delete " + packageDel)
            os.system("sudo -S pacman -R " + packageDel)
        if deleteA == "D":
            packageDel = input("Type the name of your desired package here(if you want multiple packages, seperate with a space): ")
            print("To confirm, you are deleting " + packageDel)
            input("Press enter to delete " + packageDel)
            os.system("sudo -S pacman -Rs " + packageDel)
        if deleteA == "F":
            print("WARNING, this process will delete a package regardless of use, this is capable of causing major system damage")
            packageDel = input("Type the name of your desired package here(if you want multiple packages, seperate with a space): ")
            print("To confirm, you are deleting " + packageDel)
            input("Press enter to delete " + packageDel)
            os.system("sudo -S pacman -Rns -dd " + packageDel)
    else:
        print("Error: invalid selection")
elif service == "U":
    UDecision = input("Are you sure you would like to update (Y/N)")
    if UDecision == "Y":
        os.system("sudo -S pacman -Syu")
    else:
        print("Update cancelled")
else:
    print("Error: invalid selection")
