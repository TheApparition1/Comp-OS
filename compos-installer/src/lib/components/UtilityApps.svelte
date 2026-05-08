<script>
  import { Search, CheckCircle2, Code2, Box, Cpu, Layers, HardDrive, Zap, Info, ChevronRight } from 'lucide-svelte';
  import { categoryIcons } from '$lib/icons.js';
  import { installerState, updateInstallerSection } from '$lib/stores/installerState.js';

  const utilityApps = [
    { 
      id: 'rhythmbox', 
      name: 'Rhythmbox', 
      description: 'The default music management and playback application for GNOME. Rhythmbox provides a simple interface for organizing and playing your local music collection, podcasts, and online radio stations.', 
      category: 'media', 
      packages: ['rhythmbox', 'rhythmbox-plugins'], 
      size: '45 MB', 
      ram: '256 MB',
      cpu: 'Basic dual-core',
      image: '/images/rhythmbox-screenshot.png',
      features: ['Library', 'Radio', 'Podcasts', 'DAAP Support'],
      dependencies: ['GStreamer', 'GTK3', 'Libsoup']
    },
    { 
      id: 'vlc', 
      name: 'VLC', 
      description: 'A free and open source cross-platform multimedia player and framework that plays most multimedia files as well as DVDs, Audio CDs, VCDs, and various streaming protocols.', 
      category: 'media', 
      packages: ['vlc', 'vlc-plugin-access-extra'], 
      size: '120 MB', 
      ram: '512 MB',
      cpu: 'Modern dual-core',
      image: '/images/vlc-screenshot.png',
      features: ['Formats', 'Streaming', 'Hardware Decode', 'Subtitles'],
      dependencies: ['FFmpeg', 'Qt5', 'Libavcodec']
    },
    { 
      id: 'libreoffice', 
      name: 'LibreOffice', 
      description: 'LibreOffice is a powerful and free office suite, a successor to OpenOffice.org, used by millions of people around the world. It includes several applications that make it the most versatile Free and Open Source office suite.', 
      category: 'productivity', 
      packages: ['libreoffice', 'libreoffice-gtk3'], 
      size: '850 MB', 
      ram: '2 GB+',
      cpu: '2.0 GHz quad-core',
      image: '/images/libreoffice-screenshot.png',
      features: ['Writer', 'Calc', 'Impress', 'Draw'],
      dependencies: ['Java-runtime', 'GTK3', 'Libxml2']
    },
    { 
      id: 'vscode', 
      name: 'VS Code', 
      description: 'A code editor redefined and optimized for building and debugging modern web and cloud applications. Developed by Microsoft, it has become the standard for professional development.', 
      category: 'development', 
      packages: ['code'], 
      size: '350 MB', 
      ram: '4 GB+',
      cpu: 'Modern quad-core',
      image: '/images/vscode-screenshot.png',
      features: ['Extensions', 'Debugger', 'Git', 'IntelliSense'],
      dependencies: ['Electron', 'Node.js', 'Libsecret']
    },
    { 
      id: 'docker', 
      name: 'Docker', 
      description: 'An open source platform that enables developers to build, deploy, run, update and manage containers—standardized, executable components that combine application source code with the operating system libraries.', 
      category: 'virtualization', 
      packages: ['docker.io', 'docker-compose', 'containerd'], 
      size: '420 MB', 
      ram: '4 GB+',
      cpu: '64-bit quad-core',
      image: '/images/docker-screenshot.png',
      features: ['Containers', 'Compose', 'Volumes', 'Images'],
      dependencies: ['Containerd', 'Runc', 'Iptables']
    },
    { 
      id: 'gimp', 
      name: 'GIMP', 
      description: 'GIMP is a cross-platform image editor available for GNU/Linux, OS X, Windows and more operating systems. It is free software, you can change its source code and distribute your changes.', 
      category: 'graphics', 
      packages: ['gimp', 'gimp-data-extras'], 
      size: '280 MB', 
      ram: '2 GB+',
      cpu: '2.5 GHz quad-core',
      image: '/images/gimp-screenshot.png',
      features: ['Layers', 'Filters', 'Photo retouch', 'Painting'],
      dependencies: ['GEGL', 'Babl', 'GTK2']
    }
  ];

  const categoryOrder = ['all', 'media', 'productivity', 'communication', 'development', 'virtualization', 'gaming', 'graphics', 'security'];

  function toggleApp(appId) {
    const current = $installerState.utilities.selectedApps;
    const selectedApps = current.includes(appId) ? current.filter((id) => id !== appId) : [...current, appId];
    updateInstallerSection('utilities', { selectedApps });
  }

  function setSearch(search) {
    updateInstallerSection('utilities', { search });
  }

  function setCategory(category) {
    updateInstallerSection('utilities', { category });
  }

  $: search = $installerState.utilities.search || '';
  $: activeCategory = $installerState.utilities.category || 'all';
  $: selectedApps = $installerState.utilities.selectedApps || [];
  $: filteredApps = utilityApps.filter((app) => {
    const matchesCategory = activeCategory === 'all' || app.category === activeCategory;
    const query = search.toLowerCase().trim();
    const matchesSearch = !query || `${app.name} ${app.description}`.toLowerCase().includes(query);
    return matchesCategory && matchesSearch;
  });

  let hoveredAppId = null;
  $: activeApp = utilityApps.find(a => a.id === hoveredAppId) || filteredApps[0] || utilityApps[0];
</script>

<div class="max-w-6xl mx-auto">
  <div class="mb-8">
    <h2 class="text-2xl font-bold text-slate-900 mb-2">Utility Applications</h2>
    <p class="text-slate-500">Expand your system capabilities with curated professional software.</p>
  </div>

  <div class="flex flex-col md:flex-row gap-4 mb-8">
    <div class="relative flex-1">
      <Search class="w-4 h-4 absolute left-4 top-1/2 -translate-y-1/2 text-slate-400" />
      <input 
        class="w-full bg-slate-50 border border-slate-200 rounded-xl pl-10 pr-4 py-3 text-sm focus:border-blue-600 focus:outline-none transition-all" 
        placeholder="Search for applications..." 
        value={search} 
        on:input={(e) => setSearch(e.currentTarget.value)} 
      />
    </div>
    <div class="flex gap-2 overflow-x-auto pb-1 scrollbar-hide">
      {#each categoryOrder as category}
        <button 
          class="px-4 py-2 text-xs font-bold rounded-xl border-2 transition-all whitespace-nowrap {activeCategory === category ? 'border-blue-600 bg-blue-50 text-blue-600' : 'border-slate-100 bg-white text-slate-500 hover:border-slate-200'}" 
          on:click={() => setCategory(category)}
        >
          {category.toUpperCase()}
        </button>
      {/each}
    </div>
  </div>

  <div class="flex flex-col lg:flex-row gap-6 min-h-[600px]">
    <!-- Left Sidebar: App List -->
    <div class="w-full lg:w-80 flex-shrink-0 space-y-2 max-h-[700px] overflow-y-auto pr-2 scrollbar-hide">
      {#each filteredApps as app}
        {@const CategoryIcon = categoryIcons[app.category]}
        <button 
          class="w-full flex items-center gap-4 p-4 border-2 rounded-2xl text-left transition-all {hoveredAppId === app.id ? 'border-blue-600/50 bg-slate-50' : selectedApps.includes(app.id) ? 'border-blue-600 bg-blue-50/30' : 'border-slate-100 hover:border-slate-200 bg-white'}"
          on:mouseenter={() => hoveredAppId = app.id}
          on:click={() => toggleApp(app.id)}
        >
          <div class="w-10 h-10 rounded-xl bg-white border border-slate-200 flex items-center justify-center font-bold {selectedApps.includes(app.id) ? 'text-blue-600' : 'text-slate-400'}">
            {#if CategoryIcon}<CategoryIcon class="w-5 h-5" />{/if}
          </div>
          <div class="flex-1 min-w-0">
            <p class="font-bold text-slate-900 truncate">{app.name}</p>
            <p class="text-[10px] text-slate-500 uppercase tracking-widest font-bold">{app.category}</p>
          </div>
          {#if selectedApps.includes(app.id)}
            <CheckCircle2 class="w-5 h-5 text-blue-600" />
          {/if}
        </button>
      {/each}
    </div>

    <!-- Right Panel: App Details -->
    <div class="flex-1 bg-white border border-slate-200 rounded-3xl overflow-hidden shadow-sm flex flex-col">
      {#if activeApp}
        <div class="h-64 w-full bg-slate-100 relative overflow-hidden group">
          <img src={activeApp.image} alt={activeApp.name} class="w-full h-full object-cover" />
          <div class="absolute inset-0 bg-gradient-to-t from-black/60 via-transparent to-transparent"></div>
          <div class="absolute bottom-6 left-8 right-8 flex items-end justify-between">
            <div>
              <h3 class="text-3xl font-bold text-white mb-1">{activeApp.name}</h3>
              <p class="text-white/80 text-sm font-medium uppercase tracking-widest">{activeApp.category}</p>
            </div>
            <button 
              class="px-6 py-2.5 rounded-xl font-bold text-sm transition-all {selectedApps.includes(activeApp.id) ? 'bg-rose-500 text-white shadow-lg shadow-rose-200' : 'bg-blue-600 text-white shadow-lg shadow-blue-200'}"
              on:click={() => toggleApp(activeApp.id)}
            >
              {selectedApps.includes(activeApp.id) ? 'Remove Application' : 'Add to Installation'}
            </button>
          </div>
        </div>

        <div class="p-8 grid grid-cols-1 md:grid-cols-2 gap-8 overflow-y-auto">
          <div class="space-y-6">
            <div>
              <h4 class="text-[10px] font-bold text-slate-400 uppercase tracking-[0.2em] mb-3">Software Overview</h4>
              <p class="text-slate-600 text-sm leading-relaxed">{activeApp.description}</p>
            </div>

            <div>
              <h4 class="text-[10px] font-bold text-slate-400 uppercase tracking-[0.2em] mb-3">Resource Impact</h4>
              <div class="grid grid-cols-2 gap-3">
                <div class="p-3 bg-slate-50 rounded-xl flex items-center gap-3">
                  <Cpu class="w-4 h-4 text-blue-600" />
                  <div class="min-w-0">
                    <p class="text-[9px] font-bold text-slate-400 uppercase">CPU</p>
                    <p class="text-[11px] font-bold text-slate-900 truncate">{activeApp.cpu}</p>
                  </div>
                </div>
                <div class="p-3 bg-slate-50 rounded-xl flex items-center gap-3">
                  <Layers class="w-4 h-4 text-blue-600" />
                  <div class="min-w-0">
                    <p class="text-[9px] font-bold text-slate-400 uppercase">RAM</p>
                    <p class="text-[11px] font-bold text-slate-900 truncate">{activeApp.ram}</p>
                  </div>
                </div>
                <div class="p-3 bg-slate-50 rounded-xl flex items-center gap-3">
                  <HardDrive class="w-4 h-4 text-blue-600" />
                  <div class="min-w-0">
                    <p class="text-[9px] font-bold text-slate-400 uppercase">Storage</p>
                    <p class="text-[11px] font-bold text-slate-900 truncate">{activeApp.size}</p>
                  </div>
                </div>
                <div class="p-3 bg-slate-50 rounded-xl flex items-center gap-3">
                  <Zap class="w-4 h-4 text-blue-600" />
                  <div class="min-w-0">
                    <p class="text-[9px] font-bold text-slate-400 uppercase">Packages</p>
                    <p class="text-[11px] font-bold text-slate-900 truncate">{activeApp.packages.length} Units</p>
                  </div>
                </div>
              </div>
            </div>
          </div>

          <div class="space-y-6">
            <div>
              <h4 class="text-[10px] font-bold text-slate-400 uppercase tracking-[0.2em] mb-3">System Dependencies</h4>
              <div class="flex flex-wrap gap-2">
                {#each activeApp.dependencies as dep}
                  <span class="px-2.5 py-1 bg-slate-100 text-slate-600 text-[10px] font-bold rounded-lg border border-slate-200">{dep}</span>
                {/each}
              </div>
            </div>

            <div>
              <h4 class="text-[10px] font-bold text-slate-400 uppercase tracking-[0.2em] mb-3">Key Features</h4>
              <div class="bg-slate-50 rounded-2xl p-4 border border-slate-100">
                <ul class="space-y-2">
                  {#each activeApp.features as feature}
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
      {:else}
        <div class="flex-1 flex flex-col items-center justify-center text-slate-300 p-12">
          <Box class="w-16 h-16 mb-4 opacity-20" />
          <p class="font-bold text-slate-400">Select an application to view details</p>
        </div>
      {/if}
    </div>
  </div>
</div>
