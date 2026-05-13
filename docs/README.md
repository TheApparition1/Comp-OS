# CompOS

CompOS is a Linux Distribution designed to make using Linux approachable for anyone, regardless of skill level. It also includes a Tauri based installer in both Rust for the actual install, and Svelte as the UI for the installer to make it both pretty and streamlined. Svelte is a UI framework for JS, and is a commonly used web framework.

## Installation

Install UI Installer with npm:

```bash
  git clone https://github.com/TheApparition1/Comp-OS.git
  cd "Installer Design UI/compos-installer"
  npm install
```

## Tech Stack

**Frontend/UI:** Svelte 5, SvelteKit 2, TailwindCSS 3, Lucide Svelte (icons)

**Desktop Application:** Tauri 2 (Rust backend)

**Development Tools:** Vite 5, TypeScript 5, Node.js

**Build Target:** Cross-platform desktop installer with static web assets

## Requirements

- Node.js 18+
- Rust (for Tauri)
- npm or yarn

## Development

```bash
# Start development server
npm run tauri:dev

# Build for production
npm run tauri:build

# Type checking
npm run check

# Preview build
npm run preview
```

## Project Structure

```
compos-installer/
├── src/
│   ├── lib/components/    # Svelte components
│   │   ├── BrowserSelection.svelte
│   │   ├── NetworkConfig.svelte
│   │   ├── Installation.svelte
│   │   ├── DateTimeConfig.svelte
│   │   └── UserAccount.svelte
│   ├── routes/           # SvelteKit routes
│   │   ├── +page.svelte
│   │   └── +layout.svelte
│   └── static/           # Static assets
├── package.json
├── svelte.config.js
├── tailwind.config.js
└── vite.config.js
```

## Color Reference

| Color             | Hex                                                                |
| ----------------- | ------------------------------------------------------------------ |
| Compos Primary | ![#667eea](https://dummyimage.com/10/667eea/white?text=+) #667eea |
| Compos Secondary | ![#764ba2](https://dummyimage.com/10/764ba2/white?text=+) #764ba2 |
| Compos Accent | ![#48bb78](https://dummyimage.com/10/48bb78/white?text=+) #48bb78 |
| Compos Danger | ![#f56565](https://dummyimage.com/10/f56565/white?text=+) #f56565 |
| Blue 600 | ![#2563eb](https://dummyimage.com/10/2563eb/white?text=+) #2563eb |
| Slate 900 | ![#0f172a](https://dummyimage.com/10/0f172a/white?text=+) #0f172a |
| Slate 100 | ![#f1f5f9](https://dummyimage.com/10/f1f5f9/black?text=+) #f1f5f9 |
| Emerald 600 | ![#059669](https://dummyimage.com/10/059669/white?text=+) #059669 |
| Amber 600 | ![#d97706](https://dummyimage.com/10/d97706/white?text=+) #d97706 |

## Features

- **User-Friendly Interface**: Modern, responsive UI built with Svelte and TailwindCSS
- **Cross-Platform Support**: Runs on Windows, macOS, and Linux via Tauri
- **Step-by-Step Installation**: Guided installation process with validation
- **System Configuration**: Browser selection, network setup, user account creation, and more
- **Real-time System Info**: Displays system specifications and battery status
- **Experience Modes**: Beginner and experienced user modes
- **Modern Design**: Gradient backgrounds, smooth transitions, and professional styling
- **Validation System**: Ensures all required information is provided before proceeding

## Authors

- [Samuel Jordan](https://github.com/Me-Myself-But-Not-I)
- [Samuel Dingle](https://www.github.com/TheApparition1)



## License

This project is not open source. All rights reserved.

## Contributing

This is a private project. Please contact the authors for collaboration opportunities.

## Support

For support, email TheApparition1@outlook.com or make a Github PR and we will respond as soon as possible. Do note that this project is not open source.
