# CompOS - An Attempt at Being Better Than Arch Linux

A comprehensive system installation and configuration framework with multiple tools for managing Linux distributions, package installation, and system setup.

## 📁 Project Structure

This repository contains several interconnected projects designed to make Linux system configuration easier and more streamlined.

---

## 🎯 Core Components

### 1. **Installer Flows Condensed** (C# .NET)
The main CLI-based installer and configuration tool with a condensed, optimized structure.

**Location:** `/Installer Flows Condensed/`

**Purpose:**
- System installer with interactive menu interface
- Manages installation profiles for different user types
- Handles package installation workflows
- Profile-based configuration system

**Key Files:**
- **Program.cs** - Application entry point
- **MenuInterface.cs** - User interaction and CLI menus
- **Models.cs** - Data models (UserProfile, AppPackage)
- **Services.cs** - Installation and profile management services

**Architecture:**
```
Installer Flows Condensed/
├── InstallerFlows.csproj
├── Program.cs              (entry point)
├── MenuInterface.cs        (UI/CLI logic)
├── Models.cs              (data models)
├── Services.cs            (business logic)
└── README.md
```

**Quick Start:**
```bash
cd "Installer Flows Condensed"
dotnet build
dotnet run
```

**Features:**
✓ Interactive CLI menu system
✓ Multiple installation profiles
✓ Package management
✓ User profile configuration
✓ Clean, flat project structure

---

### 2. **AutoComp** (C# .NET)
Compatibility and execution layer for running Windows applications on Linux.

**Location:** `/AutoComp/`

**Purpose:**
- Execute .exe and .msi files using Wine compatibility layer
- Handle different executable types
- Process management for Windows applications on Linux

**Key Files:**
- **AutoComp-File1/Program.cs** - Wine execution utilities
- **AutoComp-File2/Program.cs** - Additional compatibility tools

**Use Case:**
Running Windows-based software on Linux systems through Wine emulation.

**Quick Start:**
```bash
cd AutoComp
dotnet build -c Release
```

---

### 3. **Installer GUI** (Python/Tkinter)
Legacy graphical user interface for system configuration using Python.

**Location:** `/Installer GUI/`

**Purpose:**
- First-boot setup wizard
- Visual interface for system configuration
- User-friendly alternative to CLI

**Key Features:**
- Welcome screen
- Username configuration
- Timezone selection
- Desktop environment choice (GNOME, KDE, etc.)
- Wine installation option
- Development tools installation option
- Finish/summary screen

**Quick Start:**
```bash
cd "Installer GUI"
python3 main.py
```

**Dependencies:**
- Python 3.x
- tkinter (usually included with Python)

---

## 📊 Project Overview

| Component | Language | Type | Status |
|-----------|----------|------|--------|
| Installer Flows Condensed | C# / .NET | CLI Tool | ✅ Active |
| AutoComp | C# / .NET | Utility Library | ✅ Active |
| Installer GUI | Python | GUI Application | ⚠️ Legacy |

---

## 🔄 How Components Work Together

```
User Input
    ↓
┌─────────────────────────────────────┐
│   Installer GUI (Python/Tkinter)    │ ← Visual setup wizard
│   OR                                 │
│   Installer Flows CLI (C#)          │ ← Command-line interface
└─────────────────────────────────────┘
    ↓
┌─────────────────────────────────────┐
│   Services Layer (C#)               │ ← Process installation profiles
│   - InstallationService             │
│   - ProfileService                  │
└─────────────────────────────────────┘
    ↓
┌─────────────────────────────────────┐
│   AutoComp (C#)                     │ ← Execute Windows apps via Wine
└─────────────────────────────────────┘
    ↓
System Configured & Ready
```

---

## 🏗️ Development Notes

### Project Organization Philosophy
This project emphasizes **clean architecture** and **minimal nesting**:
- Flat directory structures for easy navigation
- Logical code consolidation (e.g., related services in single files)
- Modern C# naming conventions (PascalCase for namespaces)
- Separation of concerns (Models, Services, UI)

### File Condensation Strategy
The "Installer Flows Condensed" folder represents the improved structure of the original project:
- **Before:** Deep nesting with redundant folder levels (4+ levels)
- **After:** Flat structure with logical grouping (1-2 levels)

See `CONDENSATION_DETAILS.md` and `INSTALLER_FLOWS_CONDENSATION_SUMMARY.md` for detailed information about the refactoring.

---

## 📋 Requirements

### For C# Projects (.NET)
- .NET 10.0 (or compatible version as specified in `.csproj`)
- Visual Studio, Rider, or VS Code with C# extension

### For Python Projects
- Python 3.x
- tkinter module

### For Windows Application Support
- Wine (for AutoComp functionality)

---

## 🚀 Getting Started

### Option 1: CLI Interface (Recommended)
```bash
cd "Installer Flows Condensed"
dotnet restore
dotnet build
dotnet run
```

### Option 2: GUI Interface
```bash
cd "Installer GUI"
python3 main.py
```

### Option 3: Build Everything
```bash
# Build Installer Flows
cd "Installer Flows Condensed"
dotnet build

# Build AutoComp
cd ../AutoComp
dotnet build

# GUI is ready to run as-is
```

---

## 📚 Documentation

- **CONDENSATION_DETAILS.md** - Detailed analysis of project restructuring
- **INSTALLER_FLOWS_CONDENSATION_SUMMARY.md** - Summary of improvements and metrics
- **Installer Flows Condensed/README.md** - Specific documentation for main installer

---

## 🔧 Architecture Highlights

### Clean Code Practices
✓ Logical separation of concerns (Models, Services, UI)
✓ Minimal folder nesting (flat structure preferred)
✓ Grouped related functionality in single files
✓ Clear naming conventions

### Extensibility
- Service-based architecture allows easy addition of new installation profiles
- Model system supports various package types
- Menu interface can be extended with new options

### Cross-Platform Support
- C# projects use .NET (cross-platform)
- Python GUI uses tkinter (cross-platform)
- Wine support for running Windows applications on Linux

---

## 📝 License & Status

**Project Status:** In Development  
**Latest Update:** February 2026

---

## 🤝 Contributing

To contribute to this project:

1. Understand the project's philosophy on clean, minimal nesting
2. Follow C# naming conventions (PascalCase for classes/namespaces)
3. Keep related code grouped (avoid unnecessary folder structure)
4. Update relevant README files with your changes
5. Test across CLI and GUI interfaces when applicable

---

## 📞 Support

For issues or questions:
1. Check the individual component READMEs
2. Review the condensation documentation
3. Check existing issues in the repository

---

**CompOS** - Building Better Linux Configuration Tools

