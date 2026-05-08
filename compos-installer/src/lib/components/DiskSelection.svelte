<script>
  import { onMount } from 'svelte';
  import { HardDrive, AlertTriangle, CheckCircle, Info, Shield, Plus, Trash2, Settings, Lock, ChevronRight } from 'lucide-svelte';
  import { invoke } from '$lib/tauri.js';
  import { installerState, updateInstallerSection, setCurrentTab, getVisibleTabs } from '$lib/stores/installerState.js';
  
  $: activeTab = tabs[$installerState.currentTab] || 'disks';
  $: experienceMode = $installerState.userExperienceMode;
  $: tabs = getVisibleTabs('Disk', experienceMode);

  $: if (experienceMode === 'beginner' && $installerState.disk.flow !== 'guided') {
    updateInstallerSection('disk', { flow: 'guided' });
  }

  let disks = [
    {
      id: 'sda',
      name: 'Samsung SSD 870 EVO 500GB',
      size: '500GB',
      type: 'SSD',
      model: 'Samsung SSD 870 EVO',
      interface: 'SATA',
      currentPartitions: [
        { name: 'Windows 11', size: '250GB', type: 'NTFS', system: true },
        { name: 'Data', size: '200GB', type: 'NTFS', system: false },
        { name: 'Recovery', size: '50GB', type: 'NTFS', system: false }
      ],
      availableSpace: '0GB',
      canInstall: false,
      warning: 'No free space available. You may need to resize or delete existing partitions.'
    },
    {
      id: 'sdb',
      name: 'WD Blue 1TB',
      size: '1TB',
      type: 'HDD',
      model: 'WD Blue WD10EZEX',
      interface: 'SATA',
      currentPartitions: [
        { name: 'Data', size: '800GB', type: 'NTFS', system: false },
        { name: 'Backup', size: '200GB', type: 'NTFS', system: false }
      ],
      availableSpace: '0GB',
      canInstall: false,
      warning: 'No free space available. Consider using this drive for dual boot or data storage.'
    },
    {
      id: 'sdc',
      name: 'Kingston NV2 250GB',
      size: '250GB',
      type: 'SSD',
      model: 'Kingston NV2 NVMe',
      interface: 'NVMe',
      currentPartitions: [],
      availableSpace: '250GB',
      canInstall: true,
      warning: null
    },
    {
      id: 'sdd',
      name: 'SanDisk Ultra Fit 64GB',
      size: '64GB',
      type: 'USB',
      model: 'SanDisk Ultra Fit',
      interface: 'USB 3.0',
      currentPartitions: [],
      availableSpace: '64GB',
      canInstall: true,
      warning: 'USB drive detected. Installation may be slower and less reliable.'
    }
  ];

  function selectDisk(id) {
    updateInstallerSection('disk', { selectedDisk: id });
  }

  function setFlow(flow) {
    updateInstallerSection('disk', { flow });
  }

  function setGuidedAction(guidedAction) {
    updateInstallerSection('disk', { guidedAction });
  }

  function toDiskType(name = '', model = '') {
    const text = `${name} ${model}`.toLowerCase();
    if (text.includes('nvme')) return 'SSD';
    if (text.includes('usb')) return 'USB';
    if (text.includes('sd') || text.includes('sata') || text.includes('hdd')) return 'HDD';
    return 'SSD';
  }

  async function loadRealDisks() {
    try {
      const response = await invoke('list_block_devices', { simulation: false });
      const devices = response?.devices || [];
      const normalized = devices
        .filter((d) => d.type === 'disk' || !d.type)
        .map((d) => ({
          id: d.name,
          name: `${d.model || 'Disk'} (${d.name})`,
          size: d.size || 'Unknown',
          type: toDiskType(d.name, d.model),
          model: d.model || 'Unknown',
          interface: d.tran || d.model || 'Unknown',
          currentPartitions: (d.children || []).map((c) => ({
            name: c.name || 'partition',
            size: c.size || 'Unknown',
            type: c.fstype || 'unknown',
            system: (c.mountpoint || '').includes('/boot')
          })),
          availableSpace: d.size || 'Unknown',
          canInstall: true,
          warning: null
        }));
      if (normalized.length > 0) disks = normalized;
    } catch (e) {
      console.warn('Real disk discovery failed:', e);
    }
  }

  onMount(async () => {
    await loadRealDisks();
  });

  const tabLabels = {
    disks: 'Disks',
    partitions: 'Partitions',
    advanced: 'Advanced',
    encryption: 'Encryption'
  };
  const tabIcons = {
    disks: HardDrive,
    partitions: Plus,
    advanced: Settings,
    encryption: Lock
  };

  $: selectedDisk = $installerState.disk.selectedDisk;
  $: selectedDiskInfo = disks.find(d => d.id === selectedDisk);
  $: flow = $installerState.disk.flow;
  $: partitionPlan = $installerState.disk.partitionPlan || [];
  $: previewSummary =
    flow === 'guided'
      ? `${$installerState.disk.guidedAction === 'erase_install' ? 'Erase and install' : 'Install alongside'} on ${selectedDiskInfo?.name || 'selected disk'}`
      : `${partitionPlan.length} manual partition(s) prepared`;
</script>

<div class="max-w-4xl mx-auto">
  <div class="mb-10">
    <h2 class="text-2xl font-bold text-slate-900 mb-2">Storage Configuration</h2>
    <p class="text-slate-500">Select an installation target and configure partitioning strategy.</p>
  </div>

  <div class="flex gap-1 mb-8 bg-slate-100 p-1 rounded-xl w-fit">
    {#each tabs as tab}
      <button 
        class="px-5 py-2 rounded-lg text-sm font-bold transition-all flex items-center gap-2 {activeTab === tab ? 'bg-white text-blue-600 shadow-sm' : 'text-slate-500 hover:text-slate-700'}"
        on:click={() => setCurrentTab(tabs.indexOf(tab))}
        disabled={tab === 'partitions' && !selectedDisk}
      >
        <svelte:component this={tabIcons[tab]} class="w-4 h-4" />
        {tabLabels[tab]}
      </button>
    {/each}
  </div>

  <div class="bg-white border border-slate-200 rounded-2xl p-8 shadow-sm">
    {#if activeTab === 'disks'}
      <div class="space-y-6">
        <div class="bg-amber-50 border border-amber-200 rounded-xl p-4 flex items-start gap-4">
          <AlertTriangle class="w-5 h-5 text-amber-600 flex-shrink-0 mt-0.5" />
          <div class="text-xs text-amber-800 leading-relaxed">
            <p class="font-bold mb-1 uppercase tracking-wider">Data Loss Warning</p>
            Installing CompOS will format the selected partitions. Ensure all critical data is backed up before proceeding.
          </div>
        </div>

        <div class="space-y-3 max-h-[400px] overflow-y-auto pr-2 scrollbar-hide">
          {#each disks as disk}
            <button 
              class="w-full flex items-center justify-between p-5 border-2 rounded-2xl transition-all text-left {selectedDisk === disk.id ? 'border-blue-600 bg-blue-50/50' : 'border-slate-100 hover:border-slate-200'}"
              on:click={() => selectDisk(disk.id)}
            >
              <div class="flex items-center gap-4">
                <div class="w-12 h-12 rounded-xl bg-white border border-slate-100 flex items-center justify-center text-slate-400 {selectedDisk === disk.id ? 'text-blue-600' : ''}">
                  <HardDrive class="w-6 h-6" />
                </div>
                <div>
                  <div class="flex items-center gap-2 mb-1">
                    <p class="font-bold text-slate-900">{disk.name}</p>
                    <span class="text-[9px] font-bold px-1.5 py-0.5 bg-slate-100 text-slate-500 rounded uppercase">{disk.type}</span>
                  </div>
                  <p class="text-[10px] font-bold text-slate-400 uppercase tracking-widest">{disk.size} • {disk.interface} • {disk.availableSpace} AVAILABLE</p>
                </div>
              </div>
              {#if selectedDisk === disk.id}
                <div class="w-6 h-6 rounded-full bg-blue-600 flex items-center justify-center text-white">
                  <span class="text-[10px] font-bold">✓</span>
                </div>
              {/if}
            </button>
          {/each}
        </div>
      </div>

    {:else if activeTab === 'partitions'}
      <div class="space-y-8">
        <h3 class="text-lg font-bold text-slate-900 mb-4">Partitioning Strategy</h3>
        
        {#if experienceMode === 'beginner'}
          <div class="bg-blue-50 border border-blue-100 rounded-2xl p-6 mb-6">
            <p class="text-sm text-blue-800 font-medium mb-4">CompOS will automatically organize your storage for the best performance and reliability.</p>
            
            <div class="space-y-3">
              <button 
                class="w-full flex items-center gap-4 p-5 border-2 rounded-xl text-left transition-all { $installerState.disk.guidedAction === 'erase_install' ? 'border-blue-600 bg-white shadow-sm' : 'border-slate-100 bg-white/50 hover:border-slate-200'}"
                on:click={() => setGuidedAction('erase_install')}
              >
                <div class="w-10 h-10 rounded-lg bg-slate-50 flex items-center justify-center text-slate-400 { $installerState.disk.guidedAction === 'erase_install' ? 'text-blue-600' : ''}">
                  <Trash2 class="w-5 h-5" />
                </div>
                <div>
                  <p class="font-bold text-slate-900">Erase Disk and Install</p>
                  <p class="text-xs text-slate-500">Wipe the entire drive and create a fresh installation.</p>
                </div>
              </button>

              <button 
                class="w-full flex items-center gap-4 p-5 border-2 rounded-xl text-left transition-all { $installerState.disk.guidedAction === 'alongside' ? 'border-blue-600 bg-white shadow-sm' : 'border-slate-100 bg-white/50 hover:border-slate-200'}"
                on:click={() => setGuidedAction('alongside')}
              >
                <div class="w-10 h-10 rounded-lg bg-slate-50 flex items-center justify-center text-slate-400 { $installerState.disk.guidedAction === 'alongside' ? 'text-blue-600' : ''}">
                  <Plus class="w-5 h-5" />
                </div>
                <div>
                  <p class="font-bold text-slate-900">Install Alongside</p>
                  <p class="text-xs text-slate-500">Shrink existing partitions to make room for CompOS.</p>
                </div>
              </button>
            </div>
          </div>
        {:else}
          <div class="grid md:grid-cols-2 gap-4">
            <button 
              class="group p-6 bg-white border-2 rounded-2xl text-left transition-all {flow === 'guided' ? 'border-blue-600 bg-blue-50/50' : 'border-slate-100 hover:border-slate-200'}"
              on:click={() => setFlow('guided')}
            >
              <div class="w-10 h-10 rounded-xl bg-slate-50 flex items-center justify-center text-slate-400 mb-4 group-hover:text-blue-600 {flow === 'guided' ? 'text-blue-600 bg-blue-100' : ''}">
                <CheckCircle class="w-5 h-5" />
              </div>
              <h4 class="font-bold text-slate-900 mb-1">Guided</h4>
              <p class="text-xs text-slate-500">Automated partition layout with sensible defaults for performance.</p>
            </button>

            <button 
              class="group p-6 bg-white border-2 rounded-2xl text-left transition-all {flow === 'manual' ? 'border-blue-600 bg-blue-50/50' : 'border-slate-100 hover:border-slate-200'}"
              on:click={() => setFlow('manual')}
            >
              <div class="w-10 h-10 rounded-xl bg-slate-50 flex items-center justify-center text-slate-400 mb-4 group-hover:text-blue-600 {flow === 'manual' ? 'text-blue-600 bg-blue-100' : ''}">
                <Plus class="w-5 h-5" />
              </div>
              <h4 class="font-bold text-slate-900 mb-1">Manual</h4>
              <p class="text-xs text-slate-500">Full control over mount points, file systems, and partition sizes.</p>
            </button>
          </div>
        {/if}
      </div>

    {:else if activeTab === 'advanced'}
      <div class="space-y-8">
        <h3 class="text-lg font-bold text-slate-900 mb-4">File System Standards</h3>
        <div class="grid grid-cols-1 md:grid-cols-2 gap-8">
          <div class="space-y-3">
            <label class="block text-[10px] font-bold text-slate-400 uppercase tracking-widest">Default File System</label>
            <select class="w-full p-4 bg-slate-50 border border-slate-100 rounded-xl focus:border-blue-600 focus:outline-none appearance-none font-medium text-slate-700" value={$installerState.disk.filesystem || 'ext4'} on:change={(e) => updateInstallerSection('disk', { filesystem: e.currentTarget.value })}>
              <option value="ext4">EXT4 (Journaling)</option>
              <option value="btrfs">BTRFS (Snapshots)</option>
              <option value="xfs">XFS (Enterprise)</option>
            </select>
          </div>
          <div class="space-y-3">
            <label class="block text-[10px] font-bold text-slate-400 uppercase tracking-widest">Swap Space (GB)</label>
            <input type="number" min="0" max="32" class="w-full p-4 bg-slate-50 border border-slate-100 rounded-xl focus:border-blue-600 focus:outline-none font-medium text-slate-700" value={$installerState.disk.swapSizeGb || 4} on:input={(e) => updateInstallerSection('disk', { swapSizeGb: Number(e.currentTarget.value) })} />
          </div>
        </div>
      </div>

    {:else if activeTab === 'encryption'}
      <div class="space-y-8">
        <h3 class="text-lg font-bold text-slate-900 mb-4">Security Standards</h3>
        <div class="flex items-center justify-between p-6 bg-slate-50 rounded-2xl border border-slate-100">
          <div class="flex items-center gap-4">
            <div class="w-12 h-12 rounded-xl bg-white border border-slate-100 flex items-center justify-center text-slate-400">
              <Lock class="w-6 h-6" />
            </div>
            <div>
              <p class="font-bold text-slate-900">Full Disk Encryption</p>
              <p class="text-xs text-slate-500">LUKS-based hardware-level security</p>
            </div>
          </div>
          <button 
            class="w-14 h-7 rounded-full transition-all relative { $installerState.disk.encryptionEnabled ? 'bg-blue-600' : 'bg-slate-300'}"
            on:click={() => updateInstallerSection('disk', { encryptionEnabled: !$installerState.disk.encryptionEnabled })}
          >
            <div class="absolute top-1 left-1 w-5 h-5 bg-white rounded-full transition-all { $installerState.disk.encryptionEnabled ? 'translate-x-7' : ''}"></div>
          </button>
        </div>
      </div>
    {/if}
  </div>

  <div class="mt-8 p-6 bg-blue-600 rounded-2xl text-white flex items-center justify-between shadow-xl shadow-blue-100">
    <div class="flex items-center gap-4">
      <div class="w-12 h-12 rounded-xl bg-white/10 flex items-center justify-center">
        <Shield class="w-6 h-6" />
      </div>
      <div>
        <p class="text-xs font-bold uppercase tracking-widest opacity-70">Selected Plan</p>
        <p class="font-bold text-sm">{previewSummary}</p>
      </div>
    </div>
  </div>
</div>
