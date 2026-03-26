# CompOS 

A comprehensive system installation and configuration framework with multiple tools for managing Linux distributions, package installation, and system setup.

## Project Structure

This repository contains several interconnected projects designed to make Linux system configuration easier and more streamlined.

---
## Central Dependencies

- **.NET Frameworks (12 recommended)** – Required to compile and run C# scripts
- **Python 3 (system-level)** – Required to run Python scripts (e.g., main.py)
- **Tkinter** – Python GUI/UI framework
- **Prettier** – Code formatting and linting tool
- **PyQt5** – Not yet implemented; planned as a new GUI layer
- **Shadcn** – UI skins and elements
- **Shadcn/UI** – Similar to Shadcn, focused on UI components rather than core framework functionality/Mainline
- **Tailwind CSS** – CSS framework to simplify ideation and UI development
## Core Components

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

### 4. **App Installer** (Next.js/React)
Modern web-based package manager for Arch Linux with a beautiful, responsive interface.

**Location:** `/app-installer/`

**Purpose:**
- Search and install packages from Pacman repositories and AUR
- Modern web interface for package management
- Real-time package status and installation tracking
- Terminal command generation for manual execution

**Key Features:**
-  **Package Search**: Search Pacman and AUR repositories simultaneously
-  **Package Management**: Install/uninstall packages with one click
-  **Modern UI**: Beautiful, responsive interface using Shadcn/UI components
-  **Terminal Integration**: Copy terminal commands for manual execution
-  **Real-time Status**: Track installation progress and package status
-  **Dark Mode**: Full dark/light theme support
-  **Mobile Responsive**: Works seamlessly on all device sizes

**Technology Stack:**
- **Frontend**: Next.js 16, React 19, TypeScript
- **UI Framework**: Shadcn/UI, Radix UI primitives
- **Styling**: Tailwind CSS v4
- **Icons**: Lucide React
- **State Management**: React hooks (useState, useEffect, useCallback)

**Key Files:**
- **src/app/page.tsx** - Main package manager interface
- **src/app/api/packages/route.ts** - API endpoints for package operations
- **components/ui/** - Reusable UI components (shadcn/ui)

**Quick Start:**
```bash
cd app-installer
npm install
npm run dev
```

Visit `http://localhost:3000` to access the package manager.

**API Endpoints:**
- `GET /api/packages?search=<query>` - Search packages
- `POST /api/packages` - Install/uninstall packages

**Dependencies:**
- Node.js 18+ 
- npm or yarn
- Pacman (for package operations)
- Optional: yay/yay-bin (for AUR support)

**Architecture:**
```
app-installer/
├── src/
│   ├── app/
│   │   ├── page.tsx              # Main UI component
│   │   ├── api/
│   │   │   └── packages/
│   │   │       └── route.ts      # API handlers
│   │   ├── layout.tsx            # Root layout
│   │   └── globals.css           # Global styles
│   ├── components/
│   │   └── ui/                   # Shadcn/ui components
│   └── lib/                      # Utility functions
├── public/                       # Static assets
├── package.json                  # Dependencies
└── next.config.ts               # Next.js config
```

---

## 📊 Project Overview

| Component | Language | Type | Status |
|-----------|----------|------|--------|
| Installer Flows Condensed | C# / .NET | CLI Tool |  Active |
| AutoComp | C# / .NET | Utility Library |  Active |
| Installer GUI | Python | GUI Application |  Legacy |
| App Installer | TypeScript/Next.js | Web Application |  Active |

---

## 🔄 How Components Work Together

```
User Input
    ↓
┌─────────────────────────────────────┐
│   Multiple Interface Options:        │
│   ┌─────────────────────────────────┐ │
│   │ Installer GUI (Python/Tkinter)  │ │ ← Visual setup wizard
│   │ Installer Flows CLI (C#)        │ │ ← Command-line interface  
│   │ App Installer (Web)             │ │ ← Modern web interface
│   └─────────────────────────────────┘ │
└─────────────────────────────────────┘
    ↓
┌─────────────────────────────────────┐
│   Services Layer (C#)               │ ← Process installation profiles
│   - InstallationService             │
│   - ProfileService                  │
└─────────────────────────────────────┘
    ↓
┌─────────────────────────────────────┐
│   Package Management:               │
│   ┌─────────────────────────────────┐ │
│   │ Pacman/AUR Operations          │ │ ← System package management
│   │ AutoComp (C#)                  │ │ ← Windows apps via Wine
│   └─────────────────────────────────┘ │
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

### For Web Application (App Installer)
- Node.js 18+ 
- npm or yarn
- Modern web browser

### For System Integration
- Pacman package manager (Arch Linux)
- Optional: yay/yay-bin (for AUR support)

### For Windows Application Support
- Wine (for AutoComp functionality)

---

##  Getting Started

### Option 1: CLI Interface (Recommended)
```bash
cd "Installer Flows Condensed"
dotnet restore
dotnet build
dotnet run
```

### Option 2: Web Interface (Modern)
```bash
cd app-installer
npm install
npm run dev
```
Visit `http://localhost:3000` to access the web-based package manager.

### Option 3: GUI Interface (Legacy)
```bash
cd "Installer GUI"
python3 main.py
```

### Option 4: Build Everything
```bash
# Build Installer Flows
cd "Installer Flows Condensed"
dotnet build

# Build AutoComp
cd ../AutoComp
dotnet build

# Setup Web Interface
cd ../app-installer
npm install

# GUI is ready to run as-is
```

---

## 📚 Documentation

- **CONDENSATION_DETAILS.md** - Detailed analysis of project restructuring
- **INSTALLER_FLOWS_CONDENSATION_SUMMARY.md** - Summary of improvements and metrics
- **Installer Flows Condensed/README.md** - Specific documentation for main installer
- **app-installer/README.md** - Web-based package manager documentation

---

##  Architecture Highlights

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

##  License & Status

**Project Status:** In Development  
**Latest Update:** February 2026

---

##  Contributing

To contribute to this project:

1. Understand the project's philosophy on clean, minimal nesting
2. Follow C# naming conventions (PascalCase for classes/namespaces)
3. Keep related code grouped (avoid unnecessary folder structure)
4. Update relevant README files with your changes
5. Test across CLI and GUI interfaces when applicable

---

##  Support

For issues or questions:
1. Check the individual component READMEs
2. Review the condensation documentation
3. Check existing issues in the repository

---

**CompOS** - Building Better Linux Configuration Tools

