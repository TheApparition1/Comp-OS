<script>
  import { Globe, Download, CheckCircle2, Box, Cpu, Layers, HardDrive, Zap, Info, Shield, Eye } from 'lucide-svelte';
  import { installerState, updateInstallerSection } from '$lib/stores/installerState.js';
  import { fade, fly, slide } from 'svelte/transition';

  let selectedBrowserId = $installerState.browser.selectedBrowser || 'firefox';
  
  let browsers = [
    { 
      id: 'firefox', 
      name: 'Firefox', 
      description: 'Mozilla Firefox is a free and open-source web browser. It uses the Gecko layout engine to render web pages, which implements current and anticipated web standards. Firefox is known for its strong privacy protections and extensive extension ecosystem.',
      icon: Globe,
      image: '/images/firefox-logo.png',
      packages: ['firefox'],
      size: '220 MB',
      ram: '512 MB+',
      cpu: 'Standard dual-core',
      features: ['Privacy focused', 'Containers', 'Enhanced Tracking Protection', 'Open source'],
      dependencies: ['GTK3', 'Libdbus', 'NSS']
    },
    { 
      id: 'chromium', 
      name: 'Chromium', 
      description: 'Chromium is an open-source browser project that aims to build a safer, faster, and more stable way for all users to experience the web. It serves as the foundation for Chrome and many other modern browsers.',
      icon: Globe,
      image: '/images/chromium-logo.png',
      packages: ['chromium-browser'],
      size: '180 MB',
      ram: '1 GB+',
      cpu: 'Modern dual-core',
      features: ['Sandboxing', 'V8 Engine', 'Extension support', 'Fast rendering'],
      dependencies: ['GTK3', 'Alsa-lib', 'NSS']
    },
    {
      id: 'vivaldi',
      name: 'Vivaldi',
      description: 'Vivaldi is a feature-rich, highly customizable web browser based on Chromium. Created by the co-founder of Opera, it offers extensive customization options, built-in ad blocking, and powerful productivity features.',
      icon: Globe,
      image: '/images/vivaldi-logo.png',
      packages: ['vivaldi-stable'],
      size: '220 MB',
      ram: '1 GB+',
      cpu: 'Modern dual-core',
      features: ['Tab stacking', 'Notes', 'Screenshot tool', 'Custom themes', 'Built-in ad blocker', 'Built-in VPN', 'Open source and Customisable'],
      dependencies: ['GTK3', 'NSS', 'Libasound2']
    },
    { 
      id: 'brave', 
      name: 'Brave', 
      description: 'Brave is a free and open-source web browser based on the Chromium web browser. It blocks ads and website trackers, and provides a way for users to send cryptocurrency contributions in the form of Basic Attention Tokens.',
      icon: Shield,
      image: '/images/brave-logo.png',
      packages: ['brave-browser'],
      size: '200 MB',
      ram: '1 GB+',
      cpu: 'Modern dual-core',
      features: ['Ad blocking', 'Tor integration', 'Brave Rewards', 'Fast performance'],
      dependencies: ['GTK3', 'NSS', 'Libasound2']
    },
    { 
      id: 'tor', 
      name: 'Tor Browser', 
      description: 'The Tor Browser protects your privacy and anonymity on the web by routing your traffic through the Tor network. It is designed to prevent anyone watching your connection from knowing what sites you visit.',
      icon: Eye,
      image: '/images/tor-logo.png',
      packages: ['tor-browser'],
      size: '250 MB',
      ram: '1.5 GB+',
      cpu: 'Modern quad-core',
      features: ['Onion routing', 'Anti-fingerprinting', 'No tracking', 'Encrypted traffic'],
      dependencies: ['Tor-service', 'NSS', 'Libevent']
    }
  ];
  
  function selectBrowser(browserId) {
    selectedBrowserId = browserId;
    updateInstallerSection('browser', { selectedBrowser: browserId });
  }

  $: activeBrowser = browsers.find(b => b.id === selectedBrowserId) || browsers[0];
</script>

<div class="max-w-6xl mx-auto">
  <div class="mb-8">
    <h2 class="text-2xl font-bold text-slate-900 mb-2">Web Browser</h2>
    <p class="text-slate-500">Select the primary gateway to the internet for your system.</p>
  </div>

  <div class="flex flex-col lg:flex-row gap-6 min-h-[600px]">
    <!-- Left Sidebar: Selection -->
    <div class="w-full lg:w-80 flex-shrink-0 space-y-3">
      {#each browsers as browser}
        <button 
          class="w-full flex items-center gap-4 p-4 border-2 rounded-2xl text-left transition-all {selectedBrowserId === browser.id ? 'border-blue-600 bg-blue-50/50' : 'border-slate-100 hover:border-slate-200 bg-white'}"
          on:click={() => selectBrowser(browser.id)}
        >
          <div class="w-10 h-10 rounded-xl bg-white border border-slate-200 flex items-center justify-center {selectedBrowserId === browser.id ? 'text-blue-600' : 'text-slate-400'}">
            <svelte:component this={browser.icon} class="w-5 h-5" />
          </div>
          <div class="flex-1 min-w-0">
            <p class="font-bold text-slate-900 truncate">{browser.name}</p>
            <p class="text-[10px] text-slate-500 uppercase tracking-widest font-bold">Web Standard</p>
          </div>
          {#if selectedBrowserId === browser.id}
            <CheckCircle2 class="w-5 h-5 text-blue-600" />
          {/if}
        </button>
      {/each}
    </div>

    <!-- Right Panel: Browser Details -->
    <div class="flex-1 bg-white border border-slate-200 rounded-3xl overflow-hidden shadow-sm flex flex-col">
      <div class="h-64 w-full bg-slate-50 relative overflow-hidden group flex items-center justify-center p-12">
        {#key activeBrowser.id}
          <div transition:fade={{ duration: 300 }} class="relative w-full h-full">
            <img src={activeBrowser.image} alt={activeBrowser.name} class="w-full h-full {activeBrowser.id === 'chromium' || activeBrowser.id === 'brave' ? 'object-contain' : 'object-cover'} transition-transform duration-700 group-hover:scale-105 {activeBrowser.id === 'tor' ? 'scale-150' : ''} {activeBrowser.id === 'firefox' ? 'scale-125' : ''} {activeBrowser.id === 'chromium' ? 'scale-75' : ''} {activeBrowser.id === 'brave' ? 'scale-150' : ''}" />
            <div class="absolute inset-0 bg-gradient-to-t from-slate-50/80 via-transparent to-transparent"></div>
            <div class="absolute bottom-6 left-8">
              <h3 class="text-3xl font-bold text-slate-900 mb-1">{activeBrowser.name}</h3>
              <p class="text-slate-500 text-sm font-medium">Internet Gateway</p>
            </div>
          </div>
        {/key}
      </div>

      {#key activeBrowser.id}
      <div class="p-8 grid grid-cols-1 md:grid-cols-2 gap-8 overflow-y-auto" transition:slide={{ duration: 400, delay: 100 }}>
        <div class="space-y-6">
          <div transition:fade={{ duration: 300, delay: 200 }}>
            <h4 class="text-[10px] font-bold text-slate-400 uppercase tracking-[0.2em] mb-3">Overview</h4>
            <p class="text-slate-600 text-sm leading-relaxed">{activeBrowser.description}</p>
          </div>

          <div transition:fade={{ duration: 300, delay: 300 }}>
            <h4 class="text-[10px] font-bold text-slate-400 uppercase tracking-[0.2em] mb-3">Hardware Requirements</h4>
            <div class="grid grid-cols-2 gap-3">
              <div class="p-3 bg-slate-50 rounded-xl flex items-center gap-3">
                <Cpu class="w-4 h-4 text-blue-600" />
                <div class="min-w-0">
                  <p class="text-[9px] font-bold text-slate-400 uppercase">Process</p>
                  <p class="text-[11px] font-bold text-slate-900 truncate">{activeBrowser.cpu}</p>
                </div>
              </div>
              <div class="p-3 bg-slate-50 rounded-xl flex items-center gap-3">
                <Layers class="w-4 h-4 text-blue-600" />
                <div class="min-w-0">
                  <p class="text-[9px] font-bold text-slate-400 uppercase">Memory</p>
                  <p class="text-[11px] font-bold text-slate-900 truncate">{activeBrowser.ram}</p>
                </div>
              </div>
              <div class="p-3 bg-slate-50 rounded-xl flex items-center gap-3">
                <HardDrive class="w-4 h-4 text-blue-600" />
                <div class="min-w-0">
                  <p class="text-[9px] font-bold text-slate-400 uppercase">Disk</p>
                  <p class="text-[11px] font-bold text-slate-900 truncate">{activeBrowser.size}</p>
                </div>
              </div>
              <div class="p-3 bg-slate-50 rounded-xl flex items-center gap-3">
                <Zap class="w-4 h-4 text-blue-600" />
                <div class="min-w-0">
                  <p class="text-[9px] font-bold text-slate-400 uppercase">Engine</p>
                  <p class="text-[11px] font-bold text-slate-900 truncate">WebKit/Blink</p>
                </div>
              </div>
            </div>
          </div>
        </div>

        <div class="space-y-6">
          <div transition:fade={{ duration: 300, delay: 400 }}>
            <h4 class="text-[10px] font-bold text-slate-400 uppercase tracking-[0.2em] mb-3">System Libraries</h4>
            <div class="flex flex-wrap gap-2">
              {#each activeBrowser.dependencies as dep}
                <span class="px-2.5 py-1 bg-slate-100 text-slate-600 text-[10px] font-bold rounded-lg border border-slate-200">{dep}</span>
              {/each}
            </div>
          </div>

          <div transition:fade={{ duration: 300, delay: 500 }}>
            <h4 class="text-[10px] font-bold text-slate-400 uppercase tracking-[0.2em] mb-3">Key Features</h4>
            <div class="bg-slate-50 rounded-2xl p-4 border border-slate-100">
              <ul class="space-y-2">
                {#each activeBrowser.features as feature}
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
      {/key}
    </div>
  </div>
</div>
