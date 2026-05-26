<script>
  import { User, Shield, Terminal, CircleUserRound, UserCircle2, Settings, Users, ChevronRight, Info, UserCircle as UserCircleRound } from 'lucide-svelte';
  import { installerState, updateInstallerSection, setCurrentTab, getVisibleTabs } from '$lib/stores/installerState.js';
  
  $: experienceMode = $installerState.userExperienceMode;
  $: tabs = getVisibleTabs('User', experienceMode);
  $: activeTab = tabs[$installerState.currentTab] || 'profile';
  
  let username = $installerState.user.username || '';
  let fullName = $installerState.user.fullName || '';
  let hostname = $installerState.user.hostname || 'compos-pc';
  let autoLogin = $installerState.user.autoLogin || false;
  let enableSudo = $installerState.user.enableSudo !== false;
  let avatarStyle = 'minimal';


  const tabLabels = {
    profile: 'Identity',
    security: 'Security', 
    system: 'Host',
    advanced: 'Advanced'
  };
  const tabIcons = {
    profile: User,
    security: Shield,
    system: Terminal,
    advanced: Settings
  };

  function validateUsername() {
    return /^[a-z_][a-z0-9_-]*$/.test(username) && username.length >= 3 && username.length <= 32;
  }
  
  $: usernameValid = username.length > 0 && validateUsername();

  $: {
    updateInstallerSection('user', {
      username,
      fullName,
      hostname,
      autoLogin,
      enableSudo,
      shell: $installerState.user.shell || 'bash',
      groups: $installerState.user.groups || ['audio', 'video', 'input', 'dialout', 'cdrom', 'plugdev']
    });
  }
</script>

<div class="max-w-4xl mx-auto">
  <div class="mb-10">
    <h2 class="text-2xl font-bold text-slate-900 mb-2">Account Administration</h2>
    <p class="text-slate-500">Configure your primary user identity and system authentication standards.</p>
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

  <div class="bg-white border border-slate-200 rounded-2xl p-8 shadow-sm">
    {#if activeTab === 'profile'}
      <div class="space-y-8">
        <div class="flex items-center gap-6">
          <div class="w-20 h-20 rounded-3xl bg-slate-50 border-2 border-slate-100 flex items-center justify-center text-slate-300">
            <UserCircle2 class="w-10 h-10" />
          </div>
          <div>
            <h3 class="text-lg font-bold text-slate-900 mb-1">User Identity</h3>
            <p class="text-xs text-slate-500 uppercase tracking-widest font-bold">Standard Account</p>
          </div>
        </div>

        <div class="grid grid-cols-1 md:grid-cols-2 gap-6">
          <div class="space-y-3">
            <label class="block text-[10px] font-bold text-slate-400 uppercase tracking-widest" for="fname">Full Name</label>
            <input 
              id="fname"
              type="text" 
              class="w-full p-4 bg-slate-50 border border-slate-100 rounded-xl focus:border-blue-600 focus:outline-none font-medium text-slate-700" 
              placeholder="e.g. Samuel Dingle"
              bind:value={fullName}
            />
          </div>
          <div class="space-y-3">
            <label class="block text-[10px] font-bold text-slate-400 uppercase tracking-widest" for="uname">Username</label>
            <input 
              id="uname"
              type="text" 
              class="w-full p-4 bg-slate-50 border border-slate-100 rounded-xl focus:border-blue-600 focus:outline-none font-medium text-slate-700" 
              placeholder="e.g. samuel"
              bind:value={username}
            />
          </div>
        </div>
      </div>

    {:else if activeTab === 'security'}
      <div class="space-y-8">
        <h3 class="text-lg font-bold text-slate-900 mb-4">Authentication</h3>
        <div class="flex items-center justify-between p-6 bg-slate-50 rounded-2xl border border-slate-100">
          <div class="flex items-center gap-4">
            <div class="w-10 h-10 rounded-full bg-white border border-slate-100 flex items-center justify-center text-slate-400">
              <Shield class="w-5 h-5" />
            </div>
            <div>
              <p class="font-bold text-slate-900 text-sm">Administrative Privileges</p>
              <p class="text-xs text-slate-500">Allow user to execute system commands via sudo</p>
            </div>
          </div>
          <button 
            class="w-12 h-6 rounded-full transition-all relative {enableSudo ? 'bg-blue-600' : 'bg-slate-300'}"
            on:click={() => enableSudo = !enableSudo}
          >
            <div class="absolute top-1 left-1 w-4 h-4 bg-white rounded-full transition-all {enableSudo ? 'translate-x-6' : ''}"></div>
          </button>
        </div>
      </div>

    {:else if activeTab === 'system'}
      <div class="space-y-8">
        <h3 class="text-lg font-bold text-slate-900 mb-4">System Identity</h3>
        <div class="space-y-4">
          <label class="block text-[10px] font-bold text-slate-400 uppercase tracking-widest" for="hname">Hostname (Network Name)</label>
          <input 
            id="hname"
            type="text" 
            class="w-full p-4 bg-slate-50 border border-slate-100 rounded-xl focus:border-blue-600 focus:outline-none font-medium text-slate-700" 
            placeholder="compos-pc"
            bind:value={hostname}
          />
          <div class="bg-blue-50 border border-blue-100 rounded-xl p-4 flex items-start gap-4">
            <Info class="w-5 h-5 text-blue-400 flex-shrink-0 mt-0.5" />
            <p class="text-xs text-blue-800 leading-relaxed">
              The hostname is used to identify this machine on your local network. It should be unique and contain only lowercase letters, numbers, and hyphens.
            </p>
          </div>
        </div>
      </div>

    {:else if activeTab === 'advanced'}
      <div class="space-y-8">
        <h3 class="text-lg font-bold text-slate-900 mb-4">Advanced Configuration</h3>
        <div class="space-y-3">
          <label class="block text-[10px] font-bold text-slate-400 uppercase tracking-widest">Default System Shell</label>
          <div class="grid grid-cols-2 md:grid-cols-4 gap-3">
            {#each ['bash', 'zsh', 'fish'] as sh}
              <button 
                class="p-4 border-2 rounded-xl font-bold text-sm transition-all { ($installerState.user.shell || 'bash') === sh ? 'border-blue-600 bg-blue-50/50 text-blue-600' : 'border-slate-100 text-slate-400 hover:border-slate-200'}"
                on:click={() => updateInstallerSection('user', { shell: sh })}
              >
                {sh.toUpperCase()}
              </button>
            {/each}
          </div>
        </div>
      </div>
    {/if}
  </div>

  <div class="mt-8 p-6 bg-blue-600 rounded-2xl text-white flex items-center justify-between shadow-xl shadow-blue-100">
    <div class="flex items-center gap-4">
      <div class="w-12 h-12 rounded-xl bg-white/10 flex items-center justify-center">
        <UserCircleRound class="w-6 h-6" />
      </div>
      <div>
        <p class="text-xs font-bold uppercase tracking-widest opacity-70">Authenticated Identity</p>
        <p class="font-bold text-sm">{fullName || 'New User'} ({username || '...' })</p>
      </div>
    </div>
  </div>
</div>
