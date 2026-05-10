<script>
  import { Wine, Settings, Package, AlertTriangle, CheckCircle, XCircle, CheckCircle2, Box, Cpu, Layers, HardDrive, Zap, Info } from 'lucide-svelte';
  import { installerState, updateInstallerSection } from '$lib/stores/installerState.js';
  
  let wineOptionId = $installerState.wine.option || 'winetricks';
  let enable32BitSupport = true;
  let enableGeckoMono = true;
  
  let wineOptions = [
    {
      id: 'none',
      name: 'No Wine',
      description: 'Skip Windows compatibility layer. Choose this if you only plan to run native Linux applications and want to minimize system overhead and complexity.',
      icon: 'NO',
      image: '/images/wine-logo.png',
      packages: [],
      size: '0 MB',
      ram: '0 MB',
      cpu: 'N/A',
      features: ['No overhead', 'Native only', 'Maximum security', 'Lean system'],
      dependencies: []
    },
    {
      id: 'wine-basic',
      name: 'Wine (CLI)',
      description: 'Basic Wine installation. Provides the core compatibility layer for running Windows executables. Best for advanced users who prefer manual configuration via command line.',
      icon: 'WB',
      image: '/images/wine-logo.png',
      packages: ['wine'],
      size: '250 MB',
      ram: '128 MB+',
      cpu: 'Standard dual-core',
      features: ['Core layer', 'CLI tools', 'Manual control', 'Lightweight'],
      dependencies: ['Libwine', 'Freetype', 'X11/Wayland']
    },
    {
      id: 'winetricks',
      name: 'Wine + Winetricks',
      description: 'The recommended setup for most users. Includes Wine along with Winetricks, a helpful script to download and install various redistributable runtime libraries needed to run some programs.',
      icon: 'WT',
      image: '/images/winetricks-logo.png',
      packages: ['wine', 'winetricks', 'zenity'],
      size: '380 MB',
      ram: '256 MB+',
      cpu: 'Standard dual-core',
      features: ['Helper scripts', 'Easy DLL install', 'GUI config', 'Gaming ready'],
      dependencies: ['Wine', 'Cabextract', 'Unzip', 'Wget']
    }
  ];
  
  /** @param {string} optionId */
  function selectWineOption(optionId) {
    wineOptionId = optionId;
    updateInstallerSection('wine', { option: optionId });
  }
  
  $: activeWine = wineOptions.find(o => o.id === wineOptionId) || wineOptions[0];

  $: {
    updateInstallerSection('wine', { 
      option: wineOptionId,
      enable32Bit: enable32BitSupport,
      enableGecko: enableGeckoMono
    });
  }
</script>

<div class="max-w-6xl mx-auto">
  <div class="mb-8">
    <h2 class="text-2xl font-bold text-slate-900 mb-2">Windows Compatibility</h2>
    <p class="text-slate-500">Configure Wine to run Windows applications and games seamlessly on CompOS.</p>
  </div>

  <div class="flex flex-col lg:flex-row gap-6 min-h-[600px]">
    <!-- Left Sidebar: Selection -->
    <div class="w-full lg:w-80 flex-shrink-0 space-y-6">
      <div class="space-y-2">
        <h3 class="text-[10px] font-bold text-slate-400 uppercase tracking-widest px-4">Engine Choice</h3>
        {#each wineOptions as option}
          <button 
            class="w-full flex items-center gap-4 p-4 border-2 rounded-2xl text-left transition-all {wineOptionId === option.id ? 'border-blue-600 bg-blue-50/50' : 'border-slate-100 hover:border-slate-200 bg-white'}"
            on:click={() => selectWineOption(option.id)}
          >
            <div class="w-10 h-10 rounded-xl bg-white border border-slate-200 flex items-center justify-center font-bold {wineOptionId === option.id ? 'text-blue-600' : 'text-slate-400'}">
              {option.icon}
            </div>
            <div class="flex-1 min-w-0">
              <p class="font-bold text-slate-900 truncate">{option.name}</p>
              <p class="text-[10px] text-slate-500 uppercase tracking-widest font-bold">Compatibility</p>
            </div>
            {#if wineOptionId === option.id}
              <CheckCircle2 class="w-5 h-5 text-blue-600" />
            {/if}
          </button>
        {/each}
      </div>

      {#if wineOptionId !== 'none'}
        <div class="space-y-4 pt-4 border-t border-slate-100">
          <h3 class="text-[10px] font-bold text-slate-400 uppercase tracking-widest px-4">Runtime Options</h3>
          <div class="px-4 space-y-3">
            <div class="flex items-center justify-between p-4 bg-slate-50 rounded-2xl border border-slate-100">
              <div>
                <p class="font-bold text-slate-900 text-sm">32-bit Support</p>
                <p class="text-xs text-slate-500">For older apps</p>
              </div>
              <button 
                class="w-12 h-6 rounded-full transition-all relative {enable32BitSupport ? 'bg-blue-600' : 'bg-slate-300'}"
                on:click={() => enable32BitSupport = !enable32BitSupport}
              >
                <div class="absolute top-1 left-1 w-4 h-4 bg-white rounded-full transition-all {enable32BitSupport ? 'translate-x-6' : ''}"></div>
              </button>
            </div>

            <div class="flex items-center justify-between p-4 bg-slate-50 rounded-2xl border border-slate-100">
              <div>
                <p class="font-bold text-slate-900 text-sm">Gecko & Mono</p>
                <p class="text-xs text-slate-500">.NET compatibility</p>
              </div>
              <button 
                class="w-12 h-6 rounded-full transition-all relative {enableGeckoMono ? 'bg-blue-600' : 'bg-slate-300'}"
                on:click={() => enableGeckoMono = !enableGeckoMono}
              >
                <div class="absolute top-1 left-1 w-4 h-4 bg-white rounded-full transition-all {enableGeckoMono ? 'translate-x-6' : ''}"></div>
              </button>
            </div>
          </div>
        </div>
      {/if}
    </div>

    <!-- Right Panel: Wine Details -->
    <div class="flex-1 bg-white border border-slate-200 rounded-3xl overflow-hidden shadow-sm flex flex-col">
      <div class="h-64 w-full bg-slate-50 relative overflow-hidden group flex items-center justify-center p-12">
        <img src={activeWine.image} alt={activeWine.name} class="w-full h-full object-cover opacity-80 transition-transform duration-700 group-hover:scale-105" />
        <div class="absolute inset-0 bg-gradient-to-t from-slate-50/80 via-transparent to-transparent"></div>
        <div class="absolute bottom-6 left-8">
          <h3 class="text-3xl font-bold text-slate-900 mb-1">{activeWine.name}</h3>
          <p class="text-slate-500 text-sm font-medium">Windows Translation Layer</p>
        </div>
      </div>

      <div class="p-8 grid grid-cols-1 md:grid-cols-2 gap-8 overflow-y-auto">
        <div class="space-y-6">
          <div>
            <h4 class="text-[10px] font-bold text-slate-400 uppercase tracking-[0.2em] mb-3">Technical Description</h4>
            <p class="text-slate-600 text-sm leading-relaxed">{activeWine.description}</p>
          </div>

          <div>
            <h4 class="text-[10px] font-bold text-slate-400 uppercase tracking-[0.2em] mb-3">Resource Allocation</h4>
            <div class="grid grid-cols-2 gap-3">
              <div class="p-3 bg-slate-50 rounded-xl flex items-center gap-3">
                <Cpu class="w-4 h-4 text-blue-600" />
                <div class="min-w-0">
                  <p class="text-[9px] font-bold text-slate-400 uppercase">Load</p>
                  <p class="text-[11px] font-bold text-slate-900 truncate">{activeWine.cpu}</p>
                </div>
              </div>
              <div class="p-3 bg-slate-50 rounded-xl flex items-center gap-3">
                <Layers class="w-4 h-4 text-blue-600" />
                <div class="min-w-0">
                  <p class="text-[9px] font-bold text-slate-400 uppercase">Active RAM</p>
                  <p class="text-[11px] font-bold text-slate-900 truncate">{activeWine.ram}</p>
                </div>
              </div>
              <div class="p-3 bg-slate-50 rounded-xl flex items-center gap-3">
                <HardDrive class="w-4 h-4 text-blue-600" />
                <div class="min-w-0">
                  <p class="text-[9px] font-bold text-slate-400 uppercase">Binary Size</p>
                  <p class="text-[11px] font-bold text-slate-900 truncate">{activeWine.size}</p>
                </div>
              </div>
              <div class="p-3 bg-slate-50 rounded-xl flex items-center gap-3">
                <Zap class="w-4 h-4 text-blue-600" />
                <div class="min-w-0">
                  <p class="text-[9px] font-bold text-slate-400 uppercase">Translation</p>
                  <p class="text-[11px] font-bold text-slate-900 truncate">POSIX/Win32</p>
                </div>
              </div>
            </div>
          </div>
        </div>

        <div class="space-y-6">
          <div>
            <h4 class="text-[10px] font-bold text-slate-400 uppercase tracking-[0.2em] mb-3">Shared Objects</h4>
            <div class="flex flex-wrap gap-2">
              {#if activeWine.dependencies.length > 0}
                {#each activeWine.dependencies as dep}
                  <span class="px-2.5 py-1 bg-slate-100 text-slate-600 text-[10px] font-bold rounded-lg border border-slate-200">{dep}</span>
                {/each}
              {:else}
                <span class="text-xs text-slate-400 italic">No external dependencies</span>
              {/if}
            </div>
          </div>

          <div>
            <h4 class="text-[10px] font-bold text-slate-400 uppercase tracking-[0.2em] mb-3">Included Tooling</h4>
            <div class="bg-slate-50 rounded-2xl p-4 border border-slate-100">
              <ul class="space-y-2">
                {#each activeWine.features as feature}
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
