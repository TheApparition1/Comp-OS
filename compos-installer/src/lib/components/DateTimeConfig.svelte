<script>
  import { onMount } from 'svelte';
  import { Clock, Calendar, Globe, Settings, Timer, CheckCircle2, MapPin, Monitor, Zap, ChevronRight } from 'lucide-svelte';
  import { installerState, updateInstallerSection, setCurrentTab, getVisibleTabs } from '$lib/stores/installerState.js';
  
  $: activeTab = tabs[$installerState.currentTab] || 'timezone';
  $: experienceMode = $installerState.userExperienceMode;
  $: tabs = getVisibleTabs('DateTime', experienceMode);
  
  let selectedTimezone = $installerState.datetime.timezone || 'UTC';
  let useNTP = $installerState.datetime.useNTP !== false;
  let manualTime = $installerState.datetime.manualTime || '';
  let manualDate = $installerState.datetime.manualDate || '';
  let timeFormat = $installerState.datetime.timeFormat || '24h';
  let firstDayOfWeek = $installerState.datetime.firstDayOfWeek || 'monday';
  let currentSystemTime = new Date();
  let hardwareClockUTC = $installerState.datetime.hardwareClockUTC !== false;
  
  const ntpServers = ['pool.ntp.org', 'time.google.com', 'time.cloudflare.com', 'time.nist.gov'];
  
  const tabLabels = {
    timezone: 'Timezone',
    time: 'Date & Time',
    format: 'Formats',
    advanced: 'Advanced'
  };
  const tabIcons = {
    timezone: Globe,
    time: Clock,
    format: Calendar,
    advanced: Settings
  };

  let timezones = [
    { code: 'UTC', name: 'UTC (Coordinated Universal Time)', offset: '+00:00', region: 'Global' },
    { code: 'America/New_York', name: 'Eastern Time (US & Canada)', offset: '-05:00', region: 'North America' },
    { code: 'America/Chicago', name: 'Central Time (US & Canada)', offset: '-06:00', region: 'North America' },
    { code: 'America/Denver', name: 'Mountain Time (US & Canada)', offset: '-07:00', region: 'North America' },
    { code: 'America/Los_Angeles', name: 'Pacific Time (US & Canada)', offset: '-08:00', region: 'North America' },
    { code: 'Europe/London', name: 'Greenwich Mean Time (London)', offset: '+00:00', region: 'Europe' },
    { code: 'Europe/Paris', name: 'Central European Time (Paris)', offset: '+01:00', region: 'Europe' },
    { code: 'Europe/Berlin', name: 'Central European Time (Berlin)', offset: '+01:00', region: 'Europe' },
    { code: 'Asia/Shanghai', name: 'China Standard Time (Shanghai)', offset: '+08:00', region: 'Asia' },
    { code: 'Asia/Tokyo', name: 'Japan Standard Time (Tokyo)', offset: '+09:00', region: 'Asia' },
    { code: 'Australia/Sydney', name: 'Australian Eastern Time (Sydney)', offset: '+10:00', region: 'Oceania' }
  ];
  
  function selectTimezone(timezoneCode) {
    selectedTimezone = timezoneCode;
    updateInstallerSection('datetime', { timezone: timezoneCode });
  }
  
  function getCurrentDateTime() {
    const now = new Date();
    return {
      time: now.toLocaleTimeString('en-US', { hour12: timeFormat === '12h' }),
      date: now.toLocaleDateString('en-US', { year: 'numeric', month: 'long', day: 'numeric' })
    };
  }
  
  $: currentTime = getCurrentDateTime();
  $: selectedTimezoneInfo = timezones.find(tz => tz.code === selectedTimezone);

  onMount(() => {
    const intervalId = setInterval(() => {
      currentSystemTime = new Date();
    }, 1000);
    return () => clearInterval(intervalId);
  });

  $: {
    updateInstallerSection('datetime', {
      timezone: selectedTimezone,
      useNTP,
      manualTime,
      manualDate,
      timeFormat,
      firstDayOfWeek,
      hardwareClockUTC
    });
  }
</script>

<div class="max-w-6xl mx-auto">
  <div class="mb-8">
    <h2 class="text-2xl font-bold text-slate-900 mb-2">Temporal Configuration</h2>
    <p class="text-slate-500">Synchronize your system clock and define regional time standards.</p>
  </div>

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

  <div class="bg-white border border-slate-200 rounded-3xl overflow-hidden shadow-sm flex flex-col lg:flex-row min-h-[500px]">
    <!-- Left Sidebar: Context & Quick Info -->
    <div class="w-full lg:w-80 bg-slate-50/50 border-r border-slate-100 p-8 space-y-8">
      <div>
        <h3 class="text-[10px] font-bold text-slate-400 uppercase tracking-widest mb-4">Current System Time</h3>
        <div class="space-y-1">
          <p class="text-3xl font-bold text-slate-900 tabular-nums">{currentTime.time}</p>
          <p class="text-sm font-medium text-slate-500">{currentTime.date}</p>
        </div>
      </div>

      <div class="space-y-4">
        <h3 class="text-[10px] font-bold text-slate-400 uppercase tracking-widest">Active Standard</h3>
        <div class="p-4 bg-white border border-slate-200 rounded-2xl shadow-sm">
          <div class="flex items-center gap-3 mb-3">
            <div class="w-8 h-8 rounded-lg bg-blue-50 text-blue-600 flex items-center justify-center">
              <Globe class="w-4 h-4" />
            </div>
            <p class="font-bold text-sm text-slate-900 truncate">{selectedTimezoneInfo?.code || 'UTC'}</p>
          </div>
          <p class="text-[10px] text-slate-500 leading-relaxed">
            {selectedTimezoneInfo?.name || 'Coordinated Universal Time'}
          </p>
        </div>
      </div>
    </div>

    <!-- Right Panel: Controls -->
    <div class="flex-1 p-8 overflow-y-auto max-h-[600px] scrollbar-hide">
      {#if activeTab === 'timezone'}
        <div class="space-y-6">
          <h3 class="text-lg font-bold text-slate-900 mb-4">Geographic Location</h3>
          <div class="grid grid-cols-1 md:grid-cols-2 gap-3">
            {#each timezones as tz}
              <button 
                class="flex items-center justify-between p-4 border-2 rounded-2xl transition-all text-left {selectedTimezone === tz.code ? 'border-blue-600 bg-blue-50/50 shadow-md shadow-blue-100' : 'border-slate-100 hover:border-slate-200 bg-white'}"
                on:click={() => selectTimezone(tz.code)}
              >
                <div>
                  <p class="font-bold text-sm text-slate-900">{tz.name}</p>
                  <p class="text-[10px] text-slate-500 uppercase tracking-widest font-bold">UTC {tz.offset}</p>
                </div>
                {#if selectedTimezone === tz.code}
                  <CheckCircle2 class="w-5 h-5 text-blue-600" />
                {/if}
              </button>
            {/each}
          </div>
        </div>

      {:else if activeTab === 'time'}
        <div class="space-y-8">
          <h3 class="text-lg font-bold text-slate-900 mb-4">Synchronization</h3>
          
          <div class="flex items-center justify-between p-6 bg-slate-50 rounded-2xl border border-slate-100">
            <div class="flex items-center gap-4">
              <div class="w-12 h-12 rounded-xl bg-white border border-slate-100 flex items-center justify-center text-slate-400">
                <Timer class="w-6 h-6" />
              </div>
              <div>
                <p class="font-bold text-slate-900">Network Time Protocol</p>
                <p class="text-xs text-slate-500">Sync with internet time servers</p>
              </div>
            </div>
            <button 
              class="w-12 h-6 rounded-full transition-all relative {useNTP ? 'bg-blue-600' : 'bg-slate-300'}"
              on:click={() => useNTP = !useNTP}
            >
              <div class="absolute top-1 left-1 w-4 h-4 bg-white rounded-full transition-all {useNTP ? 'translate-x-6' : ''}"></div>
            </button>
          </div>

          {#if !useNTP}
            <div class="grid grid-cols-1 md:grid-cols-2 gap-6 animate-in fade-in duration-500">
              <div class="space-y-3">
                <label class="block text-[10px] font-bold text-slate-400 uppercase tracking-widest" for="m-time">Manual Time</label>
                <input id="m-time" type="time" class="w-full p-4 bg-slate-50 border border-slate-100 rounded-xl focus:border-blue-600 focus:outline-none font-medium text-slate-700" bind:value={manualTime} />
              </div>
              <div class="space-y-3">
                <label class="block text-[10px] font-bold text-slate-400 uppercase tracking-widest" for="m-date">Manual Date</label>
                <input id="m-date" type="date" class="w-full p-4 bg-slate-50 border border-slate-100 rounded-xl focus:border-blue-600 focus:outline-none font-medium text-slate-700" bind:value={manualDate} />
              </div>
            </div>
          {:else}
            <div class="space-y-3">
              <label class="block text-[10px] font-bold text-slate-400 uppercase tracking-widest">NTP Server Pool</label>
              <select class="w-full p-4 bg-slate-50 border border-slate-100 rounded-xl focus:border-blue-600 focus:outline-none appearance-none font-medium text-slate-700" value={$installerState.datetime.ntpServer || 'pool.ntp.org'} on:change={(e) => updateInstallerSection('datetime', { ntpServer: e.currentTarget.value })}>
                {#each ntpServers as server}
                  <option value={server}>{server}</option>
                {/each}
              </select>
            </div>
          {/if}
        </div>

      {:else if activeTab === 'format'}
        <div class="space-y-8">
          <h3 class="text-lg font-bold text-slate-900 mb-4">Regional Standards</h3>
          <div class="grid grid-cols-1 md:grid-cols-2 gap-8">
            <div class="space-y-4">
              <label class="block text-[10px] font-bold text-slate-400 uppercase tracking-widest">Time Display</label>
              <div class="grid grid-cols-2 gap-3">
                <button 
                  class="p-4 border-2 rounded-xl font-bold text-sm transition-all {timeFormat === '24h' ? 'border-blue-600 bg-blue-50/50 text-blue-600' : 'border-slate-100 text-slate-400 hover:border-slate-200'}"
                  on:click={() => timeFormat = '24h'}
                >
                  24H
                </button>
                <button 
                  class="p-4 border-2 rounded-xl font-bold text-sm transition-all {timeFormat === '12h' ? 'border-blue-600 bg-blue-50/50 text-blue-600' : 'border-slate-100 text-slate-400 hover:border-slate-200'}"
                  on:click={() => timeFormat = '12h'}
                >
                  12H
                </button>
              </div>
            </div>

            <div class="space-y-4">
              <label class="block text-[10px] font-bold text-slate-400 uppercase tracking-widest">Week Start</label>
              <div class="grid grid-cols-2 gap-3">
                <button 
                  class="p-4 border-2 rounded-xl font-bold text-sm transition-all {firstDayOfWeek === 'monday' ? 'border-blue-600 bg-blue-50/50 text-blue-600' : 'border-slate-100 text-slate-400 hover:border-slate-200'}"
                  on:click={() => firstDayOfWeek = 'monday'}
                >
                  MONDAY
                </button>
                <button 
                  class="p-4 border-2 rounded-xl font-bold text-sm transition-all {firstDayOfWeek === 'sunday' ? 'border-blue-600 bg-blue-50/50 text-blue-600' : 'border-slate-100 text-slate-400 hover:border-slate-200'}"
                  on:click={() => firstDayOfWeek = 'sunday'}
                >
                  SUNDAY
                </button>
              </div>
            </div>
          </div>
        </div>

      {:else if activeTab === 'advanced'}
        <div class="space-y-8">
          <h3 class="text-lg font-bold text-slate-900 mb-4">Low-level Controls</h3>
          <div class="space-y-4">
            <div class="flex items-center justify-between p-6 bg-slate-50 rounded-2xl border border-slate-100">
              <div class="flex items-center gap-4">
                <div class="w-12 h-12 rounded-xl bg-white border border-slate-100 flex items-center justify-center text-slate-400">
                  <Monitor class="w-6 h-6" />
                </div>
                <div>
                  <p class="font-bold text-slate-900">Hardware Clock UTC</p>
                  <p class="text-xs text-slate-500">Standard for Linux systems</p>
                </div>
              </div>
              <button 
                class="w-12 h-6 rounded-full transition-all relative {hardwareClockUTC ? 'bg-blue-600' : 'bg-slate-300'}"
                on:click={() => hardwareClockUTC = !hardwareClockUTC}
              >
                <div class="absolute top-1 left-1 w-4 h-4 bg-white rounded-full transition-all {hardwareClockUTC ? 'translate-x-6' : ''}"></div>
              </button>
            </div>
          </div>
        </div>
      {/if}
    </div>
  </div>

  <div class="mt-8 p-6 bg-blue-600 rounded-2xl text-white flex items-center justify-between shadow-xl shadow-blue-100">
    <div class="flex items-center gap-4">
      <div class="w-12 h-12 rounded-xl bg-white/10 flex items-center justify-center">
        <Zap class="w-6 h-6" />
      </div>
      <div>
        <p class="text-xs font-bold uppercase tracking-widest opacity-70">Temporal Sync</p>
        <p class="font-bold text-sm">{useNTP ? 'Automatic via NTP' : 'Manual override active'}</p>
      </div>
    </div>
  </div>
</div>

