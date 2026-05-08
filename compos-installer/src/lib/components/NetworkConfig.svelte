<script>
  import { Wifi, Cable, Globe, Settings, CheckCircle, AlertTriangle, Lock, LockOpen, SlidersHorizontal, RotateCw, Search } from 'lucide-svelte';
  import { installerState, updateInstallerSection, setCurrentTab, getVisibleTabs } from '$lib/stores/installerState.js';
  import { onMount } from 'svelte';
  import { invoke } from '$lib/tauri.js';
  
  $: activeTab = tabs[$installerState.currentTab] || 'connection';
  $: experienceMode = $installerState.userExperienceMode;
  $: tabs = getVisibleTabs('Network', experienceMode);
  
  let networkType = $installerState.network?.type || 'wired';
  let wifiNetworks = [];
  let isScanning = false;
  let scanError = null;

  async function scanWifi() {
    if (networkType !== 'wireless') return;
    
    isScanning = true;
    scanError = null;
    try {
      const result = await invoke('scan_wifi_networks', { 
        simulation: $installerState.mode === 'simulation' 
      });
      if (result) {
        wifiNetworks = result;
      }
    } catch (e) {
      console.error('WiFi scan failed:', e);
      scanError = e.message || 'Failed to scan for WiFi networks';
      // Fallback to empty if scan fails and not in simulation
      if ($installerState.mode !== 'simulation') {
        wifiNetworks = [];
      }
    } finally {
      isScanning = false;
    }
  }

  // Scan when switching to wireless or opening wifi tab
  $: if (networkType === 'wireless' && activeTab === 'wifi') {
    if (wifiNetworks.length === 0 && !isScanning) {
      scanWifi();
    }
  }

  onMount(() => {
    if (networkType === 'wireless' && activeTab === 'wifi') {
      scanWifi();
    }
  });

  let selectedWifiNetwork = $installerState.network?.wifiNetwork || '';
  let wifiPassword = $installerState.network?.wifiPassword || '';
  let useDHCP = $installerState.network?.useDHCP !== false;
  let staticIP = $installerState.network?.staticIP || '';
  let staticNetmask = $installerState.network?.staticNetmask || '255.255.255.0';
  let staticGateway = $installerState.network?.staticGateway || '';
  let staticDNS = $installerState.network?.staticDNS || '8.8.8.8,8.8.4.4';
  let networkEnabled = $installerState.network?.enabled !== false;
  
  const tabLabels = {
    connection: 'Connection',
    wifi: 'WiFi',
    advanced: 'Advanced',
    proxy: 'Proxy'
  };
  const tabIcons = {
    connection: Cable,
    wifi: Wifi,
    advanced: Settings,
    proxy: Globe
  };

  function selectWifiNetwork(ssid) {
    selectedWifiNetwork = ssid;
  }

  $: selectedWifiInfo = wifiNetworks.find(n => n.ssid === selectedWifiNetwork);

  $: {
    updateInstallerSection('network', {
      type: networkType,
      wifiNetwork: selectedWifiNetwork,
      wifiPassword: wifiPassword,
      enabled: networkEnabled,
      useDHCP: useDHCP,
      staticIP: staticIP,
      staticNetmask: staticNetmask,
      staticGateway: staticGateway,
      staticDNS: staticDNS
    });
  }
</script>

<div class="max-w-4xl mx-auto">
  <div class="mb-10">
    <h2 class="text-2xl font-bold text-slate-900 mb-2">Network Configuration</h2>
    <p class="text-slate-500">Configure how your system connects to the internet and local network.</p>
  </div>

  <!-- Subtle Tab Indicators -->
  <div class="flex gap-1 mb-8 bg-slate-100 p-1 rounded-xl w-fit">
    {#each tabs as tab}
      <button 
        class="px-5 py-2 rounded-lg text-sm font-bold transition-all flex items-center gap-2 {activeTab === tab ? 'bg-white text-blue-600 shadow-sm' : 'text-slate-500 hover:text-slate-700'}"
        on:click={() => setCurrentTab(tabs.indexOf(tab))}
      >
        <svelte:component this={tabIcons[tab]} class="w-4 h-4" />
        {tabLabels[tab]}
      </button>
    {/each}
  </div>

  <div class="bg-white border border-slate-200 rounded-2xl p-8 shadow-sm">
    {#if activeTab === 'connection'}
      <div class="space-y-6">
        <div>
          <h3 class="text-lg font-bold text-slate-900 mb-4">Primary Interface</h3>
          <div class="grid grid-cols-1 md:grid-cols-2 gap-4">
            <button 
              class="flex items-center gap-4 p-5 border-2 rounded-xl text-left transition-all {networkType === 'wired' ? 'border-blue-600 bg-blue-50/50' : 'border-slate-100 hover:border-slate-200'}"
              on:click={() => networkType = 'wired'}
            >
              <div class="w-12 h-12 rounded-lg bg-white border border-slate-200 flex items-center justify-center {networkType === 'wired' ? 'text-blue-600 shadow-sm' : 'text-slate-400'}">
                <Cable class="w-6 h-6" />
              </div>
              <div>
                <p class="font-bold text-slate-900">Ethernet</p>
                <p class="text-xs text-slate-500">Standard wired connection</p>
              </div>
            </button>

            <button 
              class="flex items-center gap-4 p-5 border-2 rounded-xl text-left transition-all {networkType === 'wireless' ? 'border-blue-600 bg-blue-50/50' : 'border-slate-100 hover:border-slate-200'}"
              on:click={() => networkType = 'wireless'}
            >
              <div class="w-12 h-12 rounded-lg bg-white border border-slate-200 flex items-center justify-center {networkType === 'wireless' ? 'text-blue-600 shadow-sm' : 'text-slate-400'}">
                <Wifi class="w-6 h-6" />
              </div>
              <div>
                <p class="font-bold text-slate-900">Wireless</p>
                <p class="text-xs text-slate-500">WiFi and cellular networks</p>
              </div>
            </button>
          </div>
        </div>
      </div>
    
    {:else if activeTab === 'wifi'}
      {#if networkType === 'wireless'}
        <div class="space-y-6">
          <div class="flex items-center justify-between mb-4">
            <h3 class="text-lg font-bold text-slate-900">Available Networks</h3>
            <button 
              class="flex items-center gap-2 px-3 py-1.5 text-xs font-bold text-blue-600 hover:bg-blue-50 rounded-lg transition-all disabled:opacity-50"
              on:click={scanWifi}
              disabled={isScanning}
            >
              <RotateCw class="w-3 h-3 {isScanning ? 'animate-spin' : ''}" />
              {isScanning ? 'Scanning...' : 'Refresh'}
            </button>
          </div>

          {#if scanError}
            <div class="p-4 bg-rose-50 border border-rose-100 rounded-xl flex items-start gap-3 text-rose-800">
              <AlertTriangle class="w-5 h-5 flex-shrink-0 mt-0.5" />
              <div>
                <p class="text-sm font-bold">Scan Error</p>
                <p class="text-xs opacity-90">{scanError}</p>
              </div>
            </div>
          {/if}

          <div class="grid gap-2">
            {#if isScanning && wifiNetworks.length === 0}
              {#each Array(3) as _}
                <div class="h-16 bg-slate-50 rounded-xl animate-pulse"></div>
              {/each}
            {:else if wifiNetworks.length === 0}
              <div class="py-12 text-center border-2 border-dashed border-slate-100 rounded-2xl">
                <Search class="w-10 h-10 text-slate-200 mx-auto mb-3" />
                <p class="text-sm font-bold text-slate-900">No networks found</p>
                <p class="text-xs text-slate-500 mt-1">Try moving closer to your router or refreshing.</p>
              </div>
            {:else}
              {#each wifiNetworks as network}
                <button 
                  class="flex items-center justify-between p-4 border rounded-xl transition-all {selectedWifiNetwork === network.ssid ? 'border-blue-600 bg-blue-50/50' : 'border-slate-100 hover:border-slate-200'}"
                  on:click={() => selectWifiNetwork(network.ssid)}
                >
                  <div class="flex items-center gap-3">
                    <Wifi class="w-5 h-5 {selectedWifiNetwork === network.ssid ? 'text-blue-600' : 'text-slate-400'}" />
                    <div class="text-left">
                      <p class="font-bold text-sm text-slate-900">{network.ssid}</p>
                      <p class="text-[10px] text-slate-500 uppercase tracking-wider font-bold">{network.frequency} • {network.security}</p>
                    </div>
                  </div>
                  <div class="flex items-center gap-3">
                    <span class="text-[10px] font-bold uppercase tracking-widest {
                      network.strength === 'excellent' ? 'text-emerald-600' :
                      network.strength === 'good' ? 'text-blue-600' :
                      network.strength === 'fair' ? 'text-amber-600' : 'text-rose-600'
                    }">
                      {network.strength}
                    </span>
                    {#if network.security !== 'Open' && network.security}
                      <Lock class="w-4 h-4 text-slate-300" />
                    {/if}
                  </div>
                </button>
              {/each}
            {/if}
          </div>

          {#if selectedWifiInfo && selectedWifiInfo.security !== 'Open'}
            <div class="pt-4 border-t border-slate-100">
              <label class="block text-xs font-bold text-slate-400 uppercase tracking-widest mb-2" for="wifi-pass">Network Password</label>
              <input 
                id="wifi-pass"
                type="password"
                class="w-full px-4 py-3 bg-slate-50 border border-slate-200 rounded-xl focus:outline-none focus:ring-2 focus:ring-blue-600/20 focus:border-blue-600 transition-all"
                placeholder="Enter password for {selectedWifiNetwork}"
                bind:value={wifiPassword}
              />
            </div>
          {/if}
        </div>
      {:else}
        <div class="py-12 text-center">
          <div class="w-16 h-16 bg-slate-50 rounded-full flex items-center justify-center text-slate-300 mx-auto mb-4">
            <Wifi class="w-8 h-8" />
          </div>
          <p class="font-bold text-slate-900">Wireless disabled</p>
          <p class="text-sm text-slate-500 max-w-xs mx-auto mt-1">Select Wireless as your primary interface to configure WiFi settings.</p>
        </div>
      {/if}

    {:else if activeTab === 'advanced'}
      <div class="space-y-8">
        <div>
          <h3 class="text-lg font-bold text-slate-900 mb-4">IP Assignment</h3>
          <div class="flex items-center justify-between p-4 bg-slate-50 border border-slate-200 rounded-xl">
            <div>
              <p class="font-bold text-slate-900 text-sm">Automatic Configuration (DHCP)</p>
              <p class="text-xs text-slate-500">Let the network router assign system addresses</p>
            </div>
            <button 
              class="w-12 h-6 rounded-full transition-all relative {useDHCP ? 'bg-blue-600' : 'bg-slate-300'}"
              on:click={() => useDHCP = !useDHCP}
            >
              <div class="absolute top-1 left-1 w-4 h-4 bg-white rounded-full transition-all {useDHCP ? 'translate-x-6' : ''}"></div>
            </button>
          </div>
        </div>

        {#if !useDHCP}
          <div class="grid grid-cols-1 md:grid-cols-2 gap-6 animate-in fade-in duration-500">
            <div class="space-y-2">
              <label class="block text-[10px] font-bold text-slate-400 uppercase tracking-widest">Static IP Address</label>
              <input type="text" class="w-full px-4 py-3 bg-slate-50 border border-slate-200 rounded-xl focus:border-blue-600 focus:outline-none" placeholder="192.168.1.50" bind:value={staticIP} />
            </div>
            <div class="space-y-2">
              <label class="block text-[10px] font-bold text-slate-400 uppercase tracking-widest">Subnet Mask</label>
              <input type="text" class="w-full px-4 py-3 bg-slate-50 border border-slate-200 rounded-xl focus:border-blue-600 focus:outline-none" placeholder="255.255.255.0" bind:value={staticNetmask} />
            </div>
            <div class="space-y-2">
              <label class="block text-[10px] font-bold text-slate-400 uppercase tracking-widest">Gateway</label>
              <input type="text" class="w-full px-4 py-3 bg-slate-50 border border-slate-200 rounded-xl focus:border-blue-600 focus:outline-none" placeholder="192.168.1.1" bind:value={staticGateway} />
            </div>
            <div class="space-y-2">
              <label class="block text-[10px] font-bold text-slate-400 uppercase tracking-widest">DNS Servers</label>
              <input type="text" class="w-full px-4 py-3 bg-slate-50 border border-slate-200 rounded-xl focus:border-blue-600 focus:outline-none" placeholder="1.1.1.1, 8.8.8.8" bind:value={staticDNS} />
            </div>
          </div>
        {/if}
      </div>

    {:else if activeTab === 'proxy'}
      <div class="space-y-6">
        <h3 class="text-lg font-bold text-slate-900 mb-4">Proxy Configuration</h3>
        <div class="space-y-4">
          <div class="space-y-2">
            <label class="block text-[10px] font-bold text-slate-400 uppercase tracking-widest">Proxy Server URL</label>
            <input 
              type="text" 
              class="w-full px-4 py-3 bg-slate-50 border border-slate-200 rounded-xl focus:border-blue-600 focus:outline-none" 
              placeholder="http://proxy.example.com:3128"
              value={$installerState.network.proxy || ''}
              on:input={(e) => updateInstallerSection('network', { proxy: e.currentTarget.value })}
            />
          </div>
          <p class="text-xs text-slate-500 italic">Leave empty if no proxy is required for your network environment.</p>
        </div>
      </div>
    {/if}
  </div>

  <div class="mt-8 flex items-center justify-between p-6 bg-slate-50 rounded-2xl border border-slate-100">
    <div class="flex items-center gap-4">
      <div class="w-10 h-10 rounded-full flex items-center justify-center {networkEnabled ? 'bg-emerald-100 text-emerald-600' : 'bg-slate-200 text-slate-400'}">
        <Globe class="w-5 h-5" />
      </div>
      <div>
        <p class="font-bold text-slate-900 text-sm">Interface Status</p>
        <p class="text-xs text-slate-500">{networkEnabled ? 'Active and online' : 'Interface disabled'}</p>
      </div>
    </div>
    <button 
      class="px-4 py-2 rounded-lg text-xs font-bold transition-all {networkEnabled ? 'bg-emerald-600 text-white shadow-lg shadow-emerald-200' : 'bg-slate-200 text-slate-600'}"
      on:click={() => networkEnabled = !networkEnabled}
    >
      {networkEnabled ? 'ENABLED' : 'DISABLED'}
    </button>
  </div>
</div>
