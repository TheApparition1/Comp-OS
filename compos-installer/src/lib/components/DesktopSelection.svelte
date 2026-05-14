<script>
  import { Monitor, Package, Cpu, Zap, Settings, Palette, ChevronRight, CheckCircle2, Box, Layers, HardDrive } from 'lucide-svelte';
  import { installerState, updateInstallerSection, setCurrentTab } from '$lib/stores/installerState.js';
  
  $: activeTab = tabs[$installerState.currentTab] || 'desktop';
  let selectedDesktopId = $installerState.desktop.selectedDesktop || 'gnome';
  
  const tabs = ['desktop', 'display', 'extras', 'themes'];
  const tabLabels = {
    desktop: 'Environment',
    display: 'Display',
    extras: 'Extras',
    themes: 'Themes'
  };
  const tabIcons = {
    desktop: Monitor,
    display: Zap,
    extras: Package,
    themes: Palette
  };

  let desktops = [
    { 
      id: 'hyprland',
      name: 'Hyprland', 
      description: 'Dynamic Tiling Wayland Compositor. Hyprland is a tiling Wayland compositor that doesn\'t sacrifice on looks. It provides a highly customizable and fluid experience with amazing animations.',
      icon: 'HY',
      image: '/images/hyprland-logo.png',
      packages: ['hyprland', 'hyprpaper', 'waybar', 'wofi', 'kitty'],
      ram: '512MB+',
      cpu: '1.0 GHz dual-core',
      disk: '1.2 GB',
      category: 'tiling',
      dependencies: ['Wayland', 'Mesa', 'libinput', 'wl-roots']
    },
    { 
      id: 'gnome', 
      name: 'GNOME', 
      description: 'Simplistic and modern desktop environment designed to be easy and fast to use. It focuses on distraction-free computing and elegant design.',
      icon: 'GN',
      image: '/images/gnome-logo.jpg',
      packages: ['gnome-shell', 'nautilus', 'gnome-control-center', 'mutter'],
      ram: '2GB+',
      cpu: '2.0 GHz dual-core',
      disk: '3.5 GB',
      category: 'traditional',
      dependencies: ['GTK4', 'Mutter', 'GJS', 'Wayland/X11']
    },
    { 
      id: 'kde', 
      name: 'KDE Plasma', 
      description: 'Well known and highly customizable desktop environment. It offers a powerful, yet familiar experience that can be tuned to any workflow.',
      icon: 'KD',
      image: '/images/kde-logo.png',
      packages: ['plasma-desktop', 'dolphin', 'konsole', 'kwin'],
      ram: '1.5GB+',
      cpu: '1.5 GHz dual-core',
      disk: '4.0 GB',
      category: 'traditional',
      dependencies: ['Qt6', 'KDE Frameworks', 'KWin', 'X11/Wayland']
    },
    { 
      id: 'cosmic', 
      name: 'COSMIC', 
      description: 'New and modern desktop environment built using Rust for extreme speed and security. Designed by System76 for modern high-performance workflows.',
      icon: 'CS',
      image: '/images/Cosmic-Preview.png',
      packages: ['cosmic-session', 'cosmic-files', 'cosmic-terminal', 'cosmic-settings'],
      ram: '4GB+',
      cpu: '2.5 GHz quad-core',
      disk: '5.0 GB',
      category: 'modern',
      dependencies: ['Rust-runtime', 'Iced', 'Wgpu', 'Wayland']
    }
  ];

  $: selectedDesktop = desktops.find(d => d.id === selectedDesktopId) || desktops[1];
  
  /** @param {string} desktopId */
  function selectDesktop(desktopId) {
    selectedDesktopId = desktopId;
    updateInstallerSection('desktop', { selectedDesktop: desktopId });
  }
</script>

<div class="max-w-6xl mx-auto">
  <div class="mb-8">
    <h2 class="text-2xl font-bold text-slate-900 mb-2">Graphical Environment</h2>
    <p class="text-slate-500">Choose the desktop environment that defines your CompOS experience.</p>
  </div>

  <div class="flex gap-1 mb-8 bg-slate-100 p-1 rounded-xl w-fit">
    {#each tabs as tab}
      <button 
        class="px-5 py-2 rounded-lg text-sm font-bold transition-all flex items-center gap-2 {activeTab === tab ? 'bg-white text-blue-600 shadow-sm' : 'text-slate-500 hover:text-slate-700'}"
        on:click={() => setCurrentTab(tabs.indexOf(tab))}
      >
        <svelte:component this={tabIcons[/** @type {keyof typeof tabIcons} */ (tab)]} class="w-4 h-4" />
        {tabLabels[/** @type {keyof typeof tabLabels} */ (tab)]}
      </button>
    {/each}
  </div>

  {#if activeTab === 'desktop'}
    <div class="flex flex-col lg:flex-row gap-6">
      <!-- Left Sidebar: Selection -->
      <div class="w-full lg:w-80 flex-shrink-0 space-y-3">
        {#each desktops as desktop}
          <button 
            class="w-full flex items-center gap-4 p-4 border-2 rounded-2xl text-left transition-all {selectedDesktopId === desktop.id ? 'border-blue-600 bg-blue-50/50 shadow-md shadow-blue-100' : 'border-slate-100 hover:border-slate-200 bg-white'}"
            on:click={() => selectDesktop(desktop.id)}
          >
            <div class="w-10 h-10 rounded-xl bg-white border border-slate-200 flex items-center justify-center font-bold {selectedDesktopId === desktop.id ? 'text-blue-600' : 'text-slate-400'}">
              {desktop.icon}
            </div>
            <div class="flex-1 min-w-0">
              <p class="font-bold text-slate-900 truncate">{desktop.name}</p>
              <p class="text-[10px] text-slate-500 uppercase tracking-widest font-bold">{desktop.category}</p>
            </div>
            {#if selectedDesktopId === desktop.id}
              <CheckCircle2 class="w-5 h-5 text-blue-600" />
            {/if}
          </button>
        {/each}
      </div>

      <!-- Right Panel: Details -->
      <div class="flex-1 bg-white border border-slate-200 rounded-3xl overflow-hidden shadow-sm flex flex-col">
        <div class="h-64 w-full bg-slate-100 relative overflow-hidden group">
          <img src={selectedDesktop.image} alt={selectedDesktop.name} class="w-full h-full object-cover transition-transform duration-700 group-hover:scale-105" />
          <div class="absolute inset-0 bg-gradient-to-t from-black/60 via-transparent to-transparent"></div>
          <div class="absolute bottom-6 left-8">
            <h3 class="text-3xl font-bold text-white mb-1">{selectedDesktop.name}</h3>
            <p class="text-white/80 text-sm font-medium">Primary Desktop Environment</p>
          </div>
        </div>

        <div class="p-8 grid grid-cols-1 md:grid-cols-2 gap-8">
          <div class="space-y-6">
            <div>
              <h4 class="text-[10px] font-bold text-slate-400 uppercase tracking-[0.2em] mb-3">About</h4>
              <p class="text-slate-600 text-sm leading-relaxed">{selectedDesktop.description}</p>
            </div>

            <div>
              <h4 class="text-[10px] font-bold text-slate-400 uppercase tracking-[0.2em] mb-3">System Requirements</h4>
              <div class="grid grid-cols-2 gap-3">
                <div class="p-3 bg-slate-50 rounded-xl flex items-center gap-3">
                  <Cpu class="w-4 h-4 text-blue-600" />
                  <div class="min-w-0">
                    <p class="text-[9px] font-bold text-slate-400 uppercase">CPU</p>
                    <p class="text-[11px] font-bold text-slate-900 truncate">{selectedDesktop.cpu}</p>
                  </div>
                </div>
                <div class="p-3 bg-slate-50 rounded-xl flex items-center gap-3">
                  <Layers class="w-4 h-4 text-blue-600" />
                  <div class="min-w-0">
                    <p class="text-[9px] font-bold text-slate-400 uppercase">RAM</p>
                    <p class="text-[11px] font-bold text-slate-900 truncate">{selectedDesktop.ram}</p>
                  </div>
                </div>
                <div class="p-3 bg-slate-50 rounded-xl flex items-center gap-3">
                  <HardDrive class="w-4 h-4 text-blue-600" />
                  <div class="min-w-0">
                    <p class="text-[9px] font-bold text-slate-400 uppercase">Storage</p>
                    <p class="text-[11px] font-bold text-slate-900 truncate">{selectedDesktop.disk}</p>
                  </div>
                </div>
                <div class="p-3 bg-slate-50 rounded-xl flex items-center gap-3">
                  <Zap class="w-4 h-4 text-blue-600" />
                  <div class="min-w-0">
                    <p class="text-[9px] font-bold text-slate-400 uppercase">GPU</p>
                    <p class="text-[11px] font-bold text-slate-900 truncate">OpenGL 3.3+</p>
                  </div>
                </div>
              </div>
            </div>
          </div>

          <div class="space-y-6">
            <div>
              <h4 class="text-[10px] font-bold text-slate-400 uppercase tracking-[0.2em] mb-3">Core Dependencies</h4>
              <div class="flex flex-wrap gap-2">
                {#each selectedDesktop.dependencies as dep}
                  <span class="px-2.5 py-1 bg-blue-50 text-blue-600 text-[10px] font-bold rounded-lg border border-blue-100">{dep}</span>
                {/each}
              </div>
            </div>

            <div>
              <h4 class="text-[10px] font-bold text-slate-400 uppercase tracking-[0.2em] mb-3">Included Packages</h4>
              <div class="bg-slate-50 rounded-2xl p-4 border border-slate-100">
                <ul class="space-y-2">
                  {#each selectedDesktop.packages as pkg}
                    <li class="flex items-center gap-2 text-[11px] font-medium text-slate-600">
                      <Box class="w-3 h-3 text-slate-400" />
                      {pkg}
                    </li>
                  {/each}
                </ul>
              </div>
            </div>
          </div>
        </div>
      </div>
    </div>

  {:else if activeTab === 'display'}
    <div class="bg-white border border-slate-200 rounded-3xl p-8 shadow-sm">
      <div class="space-y-6">
        <h3 class="text-lg font-bold text-slate-900 mb-4">Display Manager (Login Screen)</h3>
        <div class="grid gap-3">
          {#each ['GDM', 'SDDM', 'LightDM'] as dm}
            <button 
              class="p-4 border rounded-xl font-bold text-sm transition-all { ($installerState.desktop.displayManager || 'GDM') === dm ? 'border-blue-600 bg-blue-50/50 text-blue-600' : 'border-slate-100 text-slate-400 hover:border-slate-200'}"
              on:click={() => updateInstallerSection('desktop', { displayManager: dm })}
            >
              {dm}
            </button>
          {/each}
        </div>
      </div>
    </div>

  {:else if activeTab === 'extras'}
    <div class="bg-white border border-slate-200 rounded-3xl p-8 shadow-sm">
      <div class="space-y-6">
        <h3 class="text-lg font-bold text-slate-900 mb-4">Desktop Extras</h3>
        <div class="space-y-4">
          <div class="flex items-center justify-between p-4 bg-slate-50 rounded-xl border border-slate-100">
            <div>
              <p class="font-bold text-slate-900 text-sm">Install Recommended Fonts</p>
              <p class="text-xs text-slate-500">Nerd Fonts and system emoji sets</p>
            </div>
            <button class="w-12 h-6 rounded-full bg-blue-600 relative">
              <div class="absolute top-1 left-7 w-4 h-4 bg-white rounded-full"></div>
            </button>
          </div>
        </div>
      </div>
    </div>

  {:else if activeTab === 'themes'}
    <div class="bg-white border border-slate-200 rounded-3xl p-8 shadow-sm">
      <div class="py-12 text-center">
        <Palette class="w-12 h-12 text-slate-200 mx-auto mb-4" />
        <p class="font-bold text-slate-900">Visual Themes</p>
        <p class="text-sm text-slate-500 mt-1">Theme customization is available after environment selection.</p>
      </div>
    </div>
  {/if}
</div>
