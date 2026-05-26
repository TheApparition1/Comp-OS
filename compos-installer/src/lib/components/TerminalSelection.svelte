<script>
  import { Terminal, Settings, Cpu, Monitor, Package, CheckCircle2, Box, Layers, HardDrive, Zap, Info } from 'lucide-svelte';
  import { installerState, updateInstallerSection } from '$lib/stores/installerState.js';

  let selectedTerminalId = $installerState.terminal.selectedTerminal || 'kitty';
  let enableFishShell = $installerState.terminal.enableFishShell !== false;
  
  let terminals = [
    {
      id: 'kitty',
      name: 'Kitty',
      description: 'Kitty is a fast, feature-rich, GPU-based terminal emulator. It offloads rendering to the GPU for lower CPU usage and buttery smooth scrolling. It supports complex features like tiling, startup sessions, and graphics display.',
      icon: 'KT',
      image: '/images/kitty-screenshot.png',
      packages: ['kitty'],
      size: '12 MB',
      ram: '128 MB',
      cpu: 'GPU Required',
      features: ['GPU acceleration', 'Image support', 'Ligatures', 'Multiple layouts', 'Scriptable'],
      dependencies: ['HarfBuzz', 'Zlib', 'Libpng', 'OpenGL'],
      installCommands: ['pacman -S kitty']
    },
    {
      id: 'konsole',
      name: 'Konsole',
      description: 'Konsole is a powerful terminal emulator developed by the KDE community. It is highly integrated with the KDE Plasma desktop and offers advanced profile management, bookmarks, and a highly customizable interface.',
      icon: 'KN',
      image: '/images/konsole-screenshot.png',
      packages: ['konsole'],
      size: '25 MB',
      ram: '256 MB',
      cpu: 'Standard dual-core',
      features: ['Profile management', 'Split views', 'SSH bookmarks', 'Transparency', 'Search'],
      dependencies: ['Qt6', 'KDE Frameworks', 'KParts'],
      installCommands: ['pacman -S konsole']
    },
    {
      id: 'alacritty',
      name: 'Alacritty',
      description: 'Alacritty is a modern terminal emulator that comes with sensible defaults, but allows for extensive configuration. By focusing on simplicity and performance, it achieves speeds unmatched by any other terminal.',
      icon: 'AL',
      image: '/images/alacritty-screenshot.png',
      packages: ['alacritty'],
      size: '8 MB',
      ram: '64 MB',
      cpu: 'OpenGL 3.3+',
      features: ['Extreme speed', 'Vi mode', 'Multi-window', 'Regex search', 'YAML config'],
      dependencies: ['Rust-runtime', 'Fontconfig', 'Freetype'],
      installCommands: ['pacman -S alacritty']
    }
  ];
  
  let shells = [
    {
      id: 'bash',
      name: 'Bash',
      description: 'The Bourne Again Shell is the default shell for most Linux distributions. It is reliable, standard, and compatible with almost every script in existence.',
      icon: 'BA',
      packages: ['bash'],
      size: '2 MB',
      ram: '2 MB',
      cpu: 'Standard',
      features: ['Standard default', 'Scripting support', 'Command history', 'Tab completion'],
      installCommands: ['pacstrap install -S bash']

    },
    {
      id: 'fish',
      name: 'Fish Shell',
      description: 'Fish is a smart and user-friendly command line shell for macOS, Linux, and the rest of the family. It includes features like syntax highlighting and autosuggestions out of the box.',
      icon: 'FI',
      packages: ['fish'],
      size: '5 MB',
      features: ['Smart autocompletion', 'Syntax highlighting', 'Web-based config', 'Command suggestions']
    }
  ];
  
  function selectTerminal(terminalId) {
    selectedTerminalId = terminalId;
    updateInstallerSection('terminal', { selectedTerminal: terminalId });
  }
  
  $: activeTerminal = terminals.find(t => t.id === selectedTerminalId) || terminals[0];

  $: {
    updateInstallerSection('terminal', { 
      selectedTerminal: selectedTerminalId,
      enableFishShell: enableFishShell
    });
  }
</script>

<div class="max-w-6xl mx-auto">
  <div class="mb-8">
    <h2 class="text-2xl font-bold text-slate-900 mb-2">Terminal and Shell</h2>
    <p class="text-slate-500">Select your command-line environment and primary shell interface.</p>
  </div>

  <div class="flex flex-col lg:flex-row gap-6 min-h-[600px]">
    <!-- Left Sidebar: Selection -->
    <div class="w-full lg:w-80 flex-shrink-0 space-y-6">
      <div class="space-y-2">
        <h3 class="text-[10px] font-bold text-slate-400 uppercase tracking-widest px-4">Terminal Emulator</h3>
        {#each terminals as terminal}
          <button 
            class="w-full flex items-center gap-4 p-4 border-2 rounded-2xl text-left transition-all {selectedTerminalId === terminal.id ? 'border-blue-600 bg-blue-50/50' : 'border-slate-100 hover:border-slate-200 bg-white'}"
            on:click={() => selectTerminal(terminal.id)}
          >
            <div class="w-10 h-10 rounded-xl bg-white border border-slate-200 flex items-center justify-center font-bold {selectedTerminalId === terminal.id ? 'text-blue-600' : 'text-slate-400'}">
              {terminal.icon}
            </div>
            <div class="flex-1 min-w-0">
              <p class="font-bold text-slate-900 truncate">{terminal.name}</p>
              <p class="text-[10px] text-slate-500 uppercase tracking-widest font-bold">Standard</p>
            </div>
            {#if selectedTerminalId === terminal.id}
              <CheckCircle2 class="w-5 h-5 text-blue-600" />
            {/if}
          </button>
        {/each}
      </div>

      <div class="space-y-4 pt-4 border-t border-slate-100">
        <h3 class="text-[10px] font-bold text-slate-400 uppercase tracking-widest px-4">Shell Preference</h3>
        <div class="px-4">
          <div class="flex items-center justify-between p-4 bg-slate-50 rounded-2xl border border-slate-100">
            <div>
              <p class="font-bold text-slate-900 text-sm">Default Fish Shell</p>
              <p class="text-xs text-slate-500">Smart autocompletion</p>
            </div>
            <button 
              class="w-12 h-6 rounded-full transition-all relative {enableFishShell ? 'bg-blue-600' : 'bg-slate-300'}"
              on:click={() => enableFishShell = !enableFishShell}
            >
              <div class="absolute top-1 left-1 w-4 h-4 bg-white rounded-full transition-all {enableFishShell ? 'translate-x-6' : ''}"></div>
            </button>
          </div>
        </div>
      </div>
    </div>

    <!-- Right Panel: Terminal Details -->
    <div class="flex-1 bg-white border border-slate-200 rounded-3xl overflow-hidden shadow-sm flex flex-col">
      <div class="h-64 w-full bg-gradient-to-br from-slate-900 via-slate-800 to-slate-900 relative overflow-hidden group">
        <div class="absolute inset-0 opacity-20">
          <div class="absolute top-0 left-0 w-full h-full" style="background-image: radial-gradient(circle at 20% 30%, rgba(59, 130, 246, 0.2) 0%, transparent 50%), radial-gradient(circle at 80% 70%, rgba(59, 130, 246, 0.2) 0%, transparent 50%);"></div>
        </div>
        <div class="absolute bottom-6 left-8">
          <h3 class="text-3xl font-bold text-white mb-1">{activeTerminal.name}</h3>
          <p class="text-white/60 text-sm font-medium">Terminal Interface</p>
        </div>
      </div>

      <div class="p-8 grid grid-cols-1 md:grid-cols-2 gap-8 overflow-y-auto">
        <div class="space-y-6">
          <div>
            <h4 class="text-[10px] font-bold text-slate-400 uppercase tracking-[0.2em] mb-3">About Terminal</h4>
            <p class="text-slate-600 text-sm leading-relaxed">{activeTerminal.description}</p>
          </div>

          <div>
            <h4 class="text-[10px] font-bold text-slate-400 uppercase tracking-[0.2em] mb-3">Performance Stats</h4>
            <div class="grid grid-cols-2 gap-3">
              <div class="p-3 bg-slate-50 rounded-xl flex items-center gap-3">
                <Cpu class="w-4 h-4 text-blue-600" />
                <div class="min-w-0">
                  <p class="text-[9px] font-bold text-slate-400 uppercase">Engine</p>
                  <p class="text-[11px] font-bold text-slate-900 truncate">{activeTerminal.cpu}</p>
                </div>
              </div>
              <div class="p-3 bg-slate-50 rounded-xl flex items-center gap-3">
                <Layers class="w-4 h-4 text-blue-600" />
                <div class="min-w-0">
                  <p class="text-[9px] font-bold text-slate-400 uppercase">Memory</p>
                  <p class="text-[11px] font-bold text-slate-900 truncate">{activeTerminal.ram}</p>
                </div>
              </div>
              <div class="p-3 bg-slate-50 rounded-xl flex items-center gap-3">
                <HardDrive class="w-4 h-4 text-blue-600" />
                <div class="min-w-0">
                  <p class="text-[9px] font-bold text-slate-400 uppercase">Install Size</p>
                  <p class="text-[11px] font-bold text-slate-900 truncate">{activeTerminal.size}</p>
                </div>
              </div>
              <div class="p-3 bg-slate-50 rounded-xl flex items-center gap-3">
                <Zap class="w-4 h-4 text-blue-600" />
                <div class="min-w-0">
                  <p class="text-[9px] font-bold text-slate-400 uppercase">Acceleration</p>
                  <p class="text-[11px] font-bold text-slate-900 truncate">Vulkan/GL</p>
                </div>
              </div>
            </div>
          </div>
        </div>

        <div class="space-y-6">
          <div>
            <h4 class="text-[10px] font-bold text-slate-400 uppercase tracking-[0.2em] mb-3">Library Dependencies</h4>
            <div class="flex flex-wrap gap-2">
              {#each activeTerminal.dependencies as dep}
                <span class="px-2.5 py-1 bg-slate-100 text-slate-600 text-[10px] font-bold rounded-lg border border-slate-200">{dep}</span>
              {/each}
            </div>
          </div>

          <div>
            <h4 class="text-[10px] font-bold text-slate-400 uppercase tracking-[0.2em] mb-3">Capabilities</h4>
            <div class="bg-slate-50 rounded-2xl p-4 border border-slate-100">
              <ul class="space-y-2">
                {#each activeTerminal.features as feature}
                  <li class="flex items-center gap-2 text-[11px] font-medium text-slate-600">
                    <Box class="w-3 h-3 text-blue-500" />
                    {feature}
                  </li>
                {/each}
              </ul>
            </div>
          </div>
        </div>
      </div>
    </div>
  </div>
</div>
