# CompOS Installer

A modern, beautiful Linux installer built with Tauri + Svelte that provides both educational and automated installation modes.

## Features

- 🎨 **Modern UI**: Beautiful web-based interface with Tailwind CSS
- 📚 **Educational Mode**: Learn Linux commands step-by-step
- 🚀 **Automated Mode**: Quick, hands-off installation
- 🛠️ **Real System Commands**: Actual OS installation with sudo privileges
- 📊 **Progress Tracking**: Visual feedback during installation
- 🎯 **Desktop Selection**: GNOME, KDE, XFCE, Cinnamon
- 🌐 **Browser Selection**: Firefox, Chrome, Brave, or none

## Technology Stack

### Frontend
- **Svelte 4.0**: Modern reactive UI framework
- **Tailwind CSS**: Utility-first styling with custom CompOS theme
- **Lucide Svelte**: Beautiful icons
- **Vite**: Fast development and building

### Backend
- **Tauri 2.0**: Cross-platform desktop app framework
- **Rust**: System programming language for command execution
- **Tokio**: Async runtime for non-blocking operations

### System Integration
- **Linux Commands**: apt, systemctl, useradd, etc.
- **Sudo Support**: Privilege escalation for system operations
- **Real Installation**: Actual OS configuration, not just simulation

## Installation Modes

### Educational Mode
- Shows each command before execution
- Provides explanations of what each command does
- User controls when to execute each step
- Perfect for learning Linux administration

### Automated Mode
- Executes all commands automatically
- Handles errors gracefully
- Fast installation for experienced users
- Continues on success, stops on failure

## Project Structure

```
compos-installer/
├── src-tauri/           # Rust backend
│   ├── src/
│   │   └── main.rs    # Main application logic
│   ├── Cargo.toml         # Rust dependencies
│   └── tauri.conf.json   # Tauri configuration
├── src/                  # Svelte frontend
│   ├── lib/
│   │   └── components/     # UI components
│   │       ├── Welcome.svelte
│   │       ├── DesktopSelection.svelte
│   │       ├── BrowserSelection.svelte
│   │       └── Installation.svelte
│   └── App.svelte         # Main application
├── package.json           # Node.js dependencies
├── vite.config.js         # Vite configuration
├── tailwind.config.js     # Tailwind configuration
└── README.md             # This file
```

## Getting Started

### Prerequisites
- Node.js 18+ and npm
- Rust 1.70+ with cargo
- Linux system with sudo access
- Tauri dependencies (WebKitGTK, etc.)

### Installation
```bash
# Clone the repository
git clone <repository-url>
cd compos-installer

# Install frontend dependencies
npm install

# Install Rust dependencies
cd src-tauri
cargo build

# Run in development mode
npm run tauri:dev

# Build for production
npm run tauri:build
```

### Running the Installer
```bash
# Development
npm run tauri:dev

# Production (requires sudo for system installation)
sudo ./src-tauri/target/release/bundle/deb/compos-installer
```

## Usage

1. **Start the installer**: Launch with sudo for full functionality
2. **Choose mode**: Educational or Automated
3. **Select desktop**: Choose your preferred desktop environment
4. **Select browser**: Pick your web browser (or none)
5. **Begin installation**: Watch as commands execute
6. **Complete**: System is ready to use

## Security

- Requires sudo privileges for system operations
- Commands are validated before execution
- Error handling prevents system damage
- No network access required for basic installation

## Contributing

1. Fork the repository
2. Create feature branch
3. Make your changes
4. Test thoroughly
5. Submit pull request

## License

MIT License - see LICENSE file for details
