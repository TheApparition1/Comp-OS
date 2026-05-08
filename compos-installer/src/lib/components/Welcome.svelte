<script>
  import { BookOpenText, Rocket, ShieldCheck, SlidersHorizontal, UserCircle, Star, Terminal, ChevronRight } from 'lucide-svelte';
  import { installerState, updateInstallerSection, setCurrentTab, setUserExperienceMode } from '$lib/stores/installerState.js';
  
  export let onModeSelect = () => {};

  $: activeTab = tabs[$installerState.currentTab] || 'overview';
  
  function handleSelect(mode) {
    setUserExperienceMode(mode);
    onModeSelect();
  }

  let includeRecommendedApps = true;
  let enableDiagnostics = true;
  
  const tabs = ['overview'];
</script>

<div class="max-w-4xl mx-auto text-center">
  <div class="mb-12">
    <div class="w-20 h-20 bg-blue-600 rounded-2xl flex items-center justify-center text-white shadow-xl shadow-blue-200 mx-auto mb-6">
      <Rocket class="w-10 h-10" />
    </div>
    <h2 class="text-4xl font-bold text-slate-900 mb-4 tracking-tight">Welcome to CompOS</h2>
    <p class="text-lg text-slate-500 max-w-2xl mx-auto">
      Experience an extremely customisable Linux OS that is capable of almost anything, while still being easy to use for beginners.
    </p>
  </div>

  <div class="grid md:grid-cols-2 gap-8 mb-12">
    <button 
      class="group p-8 bg-white border-2 rounded-3xl text-left transition-all hover:shadow-xl hover:shadow-slate-200/50 { $installerState.userExperienceMode === 'beginner' ? 'border-blue-600 ring-4 ring-blue-50' : 'border-slate-100 hover:border-slate-200'}"
      on:click={() => handleSelect('beginner')}
    >
      <div class="w-14 h-14 rounded-2xl bg-slate-50 flex items-center justify-center text-slate-400 group-hover:text-blue-600 transition-colors mb-6 { $installerState.userExperienceMode === 'beginner' ? 'text-blue-600 bg-blue-50' : ''}">
        <Star class="w-7 h-7" />
      </div>
      <h3 class="text-xl font-bold text-slate-900 mb-2">Beginner Mode</h3>
      <p class="text-sm text-slate-500 leading-relaxed">
        Simplified installation with privacy focused and simple to use defaults. Ideal for users new to Linux or those who want a quick setup.
      </p>
      <div class="mt-6 flex items-center gap-2 text-xs font-bold text-blue-600 uppercase tracking-widest opacity-0 group-hover:opacity-100 transition-opacity">
        Select this mode <ChevronRight class="w-3 h-3" />
      </div>
    </button>

    <button 
      class="group p-8 bg-white border-2 rounded-3xl text-left transition-all hover:shadow-xl hover:shadow-slate-200/50 { $installerState.userExperienceMode === 'experienced' ? 'border-blue-600 ring-4 ring-blue-50' : 'border-slate-100 hover:border-slate-200'}"
      on:click={() => handleSelect('experienced')}
    >
      <div class="w-14 h-14 rounded-2xl bg-slate-50 flex items-center justify-center text-slate-400 group-hover:text-blue-600 transition-colors mb-6 { $installerState.userExperienceMode === 'experienced' ? 'text-blue-600 bg-blue-50' : ''}">
        <Terminal class="w-7 h-7" />
      </div>
      <h3 class="text-xl font-bold text-slate-900 mb-2">Experienced Mode</h3>
      <p class="text-sm text-slate-500 leading-relaxed">
        Extreme control over everything in the OS. Ideal for users who are comfortable with Linux and the Terminal, or Sys Admins.
      </p>
      <div class="mt-6 flex items-center gap-2 text-xs font-bold text-blue-600 uppercase tracking-widest opacity-0 group-hover:opacity-100 transition-opacity">
        Select this mode <ChevronRight class="w-3 h-3" />
      </div>
    </button>
  </div>

  <div class="p-6 bg-slate-50 rounded-2xl border border-slate-100 inline-flex items-center gap-4">
    <div class="flex -space-x-2">
      <div class="w-8 h-8 rounded-full border-2 border-white bg-emerald-500 flex items-center justify-center text-white text-[10px] font-bold">✓</div>
      <div class="w-8 h-8 rounded-full border-2 border-white bg-blue-500 flex items-center justify-center text-white text-[10px] font-bold">i</div>
    </div>
    <p class="text-xs font-medium text-slate-600">
      You can change your experience mode at any time during the process.
    </p>
  </div>
</div>
