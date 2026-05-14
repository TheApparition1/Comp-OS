<script>
  import { onMount } from 'svelte';
  import { fade } from 'svelte/transition';
  import { invoke } from '$lib/tauri.js';
  import { installerState, setCurrentStep, setCurrentTab, setInstallerMode, setUserExperienceMode, tabCompletion, stepValidity, getVisibleTabs } from '$lib/stores/installerState.js';
  import { CircleHelp, ShieldAlert, TriangleAlert, ChevronRight, ChevronLeft, UserCircle, Rocket, Battery, BatteryCharging, BatteryLow, BatteryMedium, BatteryFull } from 'lucide-svelte';
  
  // Component imports...
  import DesktopSelection from '$lib/components/DesktopSelection.svelte';
  import BrowserSelection from '$lib/components/BrowserSelection.svelte';
  import Installation from '$lib/components/Installation.svelte';
  import Welcome from '$lib/components/Welcome.svelte';
  import UtilityApps from '$lib/components/UtilityApps.svelte';
  import LanguageSelection from '$lib/components/LanguageSelection.svelte';
  import DateTimeConfig from '$lib/components/DateTimeConfig.svelte';
  import DiskSelection from '$lib/components/DiskSelection.svelte';
  import UserAccount from '$lib/components/UserAccount.svelte';
  import NetworkConfig from '$lib/components/NetworkConfig.svelte';
  import TerminalSelection from '$lib/components/TerminalSelection.svelte';
  import WineConfig from '$lib/components/WineConfig.svelte';
  import AuthScreen from '$lib/components/AuthScreen.svelte';

  let currentStep = 0;
  let currentTab = 0;
  let isAuthorized = false;
  let installSteps = [];
  let hasSudo = false;
  let systemInfo = { hostname: 'Unknown' };
  let systemStats = {};
  let batteryInfo = { percentage: 100, status: 'Full', is_present: false };
  let showDangerModal = false;
  let dangerConfirmation = '';
  const DANGER_PHRASE = 'ENABLE DANGEROUS MODE';
  /** @type {ReturnType<typeof setInterval> | undefined} */
  let statsInterval;
  
  const steps = [
    { name: 'Welcome', component: Welcome },
    { name: 'Language', component: LanguageSelection },
    { name: 'Network', component: NetworkConfig },
    { name: 'Disk', component: DiskSelection },
    { name: 'Desktop', component: DesktopSelection },
    { name: 'Terminal', component: TerminalSelection },
    { name: 'Utilities', component: UtilityApps },
    { name: 'Browser', component: BrowserSelection },
    { name: 'Wine', component: WineConfig },
    { name: 'User', component: UserAccount },
    { name: 'DateTime', component: DateTimeConfig },
    { name: 'Installation', component: Installation }
  ];
  
  $: setCurrentStep(currentStep);
  $: setCurrentTab(currentTab);

  $: currentStepComponent = steps[currentStep]?.component;
  $: currentStepTabs = getVisibleTabs(steps[currentStep]?.name, $installerState.userExperienceMode);
  $: isLastTab = currentTab >= currentStepTabs.length - 1;
  $: isFirstTab = currentTab === 0;

  $: progressPercent = steps.length > 0 ? ((currentStep + (currentStepTabs.length > 0 ? currentTab / currentStepTabs.length : 0)) / steps.length) * 100 : 0;

  $: {
    if (isAuthorized && !statsInterval) {
      updateSystemStats();
      statsInterval = setInterval(updateSystemStats, 5000);
    } else if (!isAuthorized && statsInterval) {
      clearInterval(statsInterval);
      statsInterval = undefined;
    }
  }

  onMount(() => {
    return () => {
      if (statsInterval) {
        clearInterval(statsInterval);
      }
    };
  });

  async function updateSystemStats() {
    try {
      const sudoResult = await invoke('check_sudo_available');
      hasSudo = sudoResult || false;
      const info = await invoke('get_system_info');
      if (info) systemInfo = info;
      const stats = await invoke('get_system_statistics');
      systemStats = stats || {};
      const battery = await invoke('get_battery_info');
      if (battery) batteryInfo = battery;
      const stepsResult = await invoke('get_install_steps');
      installSteps = stepsResult || [];
    } catch (e) {
      console.error('Stats update failed:', e);
    }
  }

  /** 
   * @param {number} percentage 
   * @param {string} status 
   */
  function getBatteryIcon(percentage, status) {
    if (status === 'Charging') return BatteryCharging;
    if (percentage > 80) return BatteryFull;
    if (percentage > 50) return BatteryMedium;
    if (percentage > 20) return BatteryLow;
    return Battery;
  }
  
  function nextStep() {
    /** @type {any} */
    const stepName = steps[currentStep].name;
    if (currentTab < currentStepTabs.length - 1) {
      currentTab++;
    } else if (currentStep < steps.length - 1 && $stepValidity[stepName]) {
      currentStep++;
      currentTab = 0;
    }
  }
  
  function prevStep() {
    if (currentTab > 0) {
      currentTab--;
    } else if (currentStep > 0) {
      currentStep--;
      const prevStepTabs = getVisibleTabs(steps[currentStep].name, $installerState.userExperienceMode);
      currentTab = prevStepTabs.length - 1;
    }
  }
  
  function requestDangerMode() {
    showDangerModal = true;
  }

  function enableDangerMode() {
    if (dangerConfirmation !== DANGER_PHRASE) return;
    setInstallerMode('dangerous');
    showDangerModal = false;
    dangerConfirmation = '';
  }

  function closeDangerModeModal() {
    showDangerModal = false;
    dangerConfirmation = '';
  }

  /**
   * @param {string} credential
   */
  async function authorizeInstaller(credential) {
    if (!credential) return false;

    if (!window?.crypto?.subtle) return false;
    const expectedHash = import.meta.env.VITE_INSTALLER_PASSWORD_HASH || '';
    if (!expectedHash) {
      console.warn('Installer auth is not configured: set VITE_INSTALLER_PASSWORD_HASH.');
      return false;
    }

    const data = new TextEncoder().encode(credential);
    const digest = await window.crypto.subtle.digest('SHA-256', data);
    const hash = Array.from(new Uint8Array(digest))
      .map((byte) => byte.toString(16).padStart(2, '0'))
      .join('');

    console.log('Auth attempt hash:', hash);
    return hash === expectedHash;
  }

  /**
   * @param {number} targetStepIndex
   */
  function canNavigateToStep(targetStepIndex) {
    if (targetStepIndex <= currentStep) return true;

    for (let i = 0; i < targetStepIndex; i++) {
      /** @type {any} */
      const stepName = steps[i]?.name;
      if (stepName && !$stepValidity[stepName]) {
        return false;
      }
    }

    return true;
  }
</script>

{#if !isAuthorized}
  <AuthScreen
    authorize={authorizeInstaller}
    onAuthorized={async () => {
      isAuthorized = true;
      await updateSystemStats();
    }}
  />
{:else}
  <div class="min-h-screen bg-[#ffffff] text-slate-900 font-sans selection:bg-blue-100" in:fade>
    <!-- Subtle Marble-like background effect -->
    <div
      class="fixed inset-0 pointer-events-none opacity-[0.03] z-0"
      style="background-image:
        radial-gradient(circle at 20% 20%, rgba(15, 23, 42, 0.18) 0, transparent 35%),
        radial-gradient(circle at 80% 30%, rgba(15, 23, 42, 0.12) 0, transparent 30%),
        radial-gradient(circle at 40% 75%, rgba(15, 23, 42, 0.14) 0, transparent 32%),
        linear-gradient(135deg, rgba(15, 23, 42, 0.08) 25%, transparent 25%, transparent 50%, rgba(15, 23, 42, 0.08) 50%, rgba(15, 23, 42, 0.08) 75%, transparent 75%, transparent);
        background-size: 320px 320px, 280px 280px, 360px 360px, 24px 24px;
        background-position: 0 0, 120px 40px, 60px 160px, 0 0;"
    ></div>

    <div class="relative z-10 max-w-6xl mx-auto px-6 py-8">
    <header class="mb-10 flex items-end justify-between border-b border-slate-200 pb-6">
      <div>
        <div class="flex items-center gap-3 mb-1">
          <div class="w-10 h-10 bg-blue-600 rounded-lg flex items-center justify-center text-white shadow-lg shadow-blue-200">
            <Rocket class="w-6 h-6" />
          </div>
          <h1 class="text-3xl font-bold tracking-tight text-slate-900">CompOS</h1>
        </div>
        <p class="text-slate-500 font-medium ml-1">System Installation Utility</p>
      </div>

      <div class="flex items-center gap-6">
        <div class="text-right hidden md:block">
          <p class="text-[10px] uppercase tracking-wider font-bold text-slate-400 mb-0.5">Target Host</p>
          <p class="text-sm font-semibold text-slate-700">{systemInfo?.hostname || 'Unknown'}</p>
        </div>

        {#if batteryInfo.is_present}
          <div class="flex items-center gap-2 px-3 py-1.5 bg-slate-100 rounded-lg border border-slate-200">
            <svelte:component this={getBatteryIcon(batteryInfo.percentage, batteryInfo.status)} class="w-4 h-4 {batteryInfo.percentage < 20 && batteryInfo.status !== 'Charging' ? 'text-rose-500' : 'text-slate-600'}" />
            <span class="text-sm font-bold text-slate-700">{batteryInfo.percentage}%</span>
          </div>
        {/if}
        
        <div class="flex flex-col items-end gap-1">
          <p class="text-[10px] uppercase tracking-wider font-bold text-slate-400">Experience Mode</p>
          <div class="flex bg-slate-100 p-1 rounded-lg border border-slate-200">
            <button 
              class="px-3 py-1 text-xs font-semibold rounded-md transition-all { $installerState.userExperienceMode === 'beginner' ? 'bg-white text-blue-600 shadow-sm' : 'text-slate-500 hover:text-slate-700'}"
              on:click={() => setUserExperienceMode('beginner')}
            >
              Beginner
            </button>
            <button 
              class="px-3 py-1 text-xs font-semibold rounded-md transition-all { $installerState.userExperienceMode === 'experienced' ? 'bg-white text-blue-600 shadow-sm' : 'text-slate-500 hover:text-slate-700'}"
              on:click={() => setUserExperienceMode('experienced')}
            >
              Experienced
            </button>
          </div>
        </div>
      </div>
    </header>

    <!-- Progress Stepper -->
    <div class="mb-12">
      <div class="flex justify-between items-center mb-4">
        <div class="flex gap-1.5">
          {#each steps as step, index}
            <div 
              class="h-1.5 rounded-full transition-all duration-500 {index === currentStep ? 'w-12 bg-blue-600' : index < currentStep ? 'w-6 bg-slate-300' : 'w-6 bg-slate-100'}"
              title={step.name}
            ></div>
          {/each}
        </div>
        <div class="text-xs font-bold text-slate-400 uppercase tracking-widest">
          Step {currentStep + 1} of {steps.length}: {steps[currentStep].name}
        </div>
      </div>
      
      <div class="grid grid-cols-4 md:grid-cols-6 lg:grid-cols-12 gap-3">
        {#each steps as step, index}
          <button 
            class="text-left group transition-all"
            on:click={() => { 
              if (canNavigateToStep(index)) {
                currentStep = index;
                currentTab = 0;
              }
            }}
            disabled={!canNavigateToStep(index)}
          >
            <div class="text-[10px] font-bold uppercase tracking-tighter mb-1 transition-colors {index === currentStep ? 'text-blue-600' : 'text-slate-400 group-hover:text-slate-600'}">
              {step.name}
            </div>
            <div class="h-1 w-full rounded-full transition-colors {index === currentStep ? 'bg-blue-600' : index < currentStep ? 'bg-slate-300' : 'bg-slate-100'}"></div>
          </button>
        {/each}
      </div>
    </div>
  
    {#if !hasSudo}
      <div class="bg-amber-50 border border-amber-200 text-amber-800 p-4 rounded-xl mb-8 flex items-center gap-4 shadow-sm">
        <div class="w-10 h-10 bg-amber-100 rounded-full flex items-center justify-center text-amber-600 flex-shrink-0">
          <TriangleAlert class="w-5 h-5" />
        </div>
        <div>
          <p class="font-bold text-sm">Privileged Access Required</p>
          <p class="text-xs opacity-90">Please restart the installer with <code class="bg-amber-100 px-1.5 py-0.5 rounded font-mono">sudo</code> to apply system changes.</p>
        </div>
      </div>
    {/if}

    <main class="min-h-[500px] mb-24">
      {#key currentStep}
        <div in:fade={{ duration: 300, delay: 300 }} out:fade={{ duration: 300 }} class="animate-in fade-in slide-in-from-bottom-4 duration-700">
          {#if currentStep === 0}
            <Welcome onModeSelect={nextStep} />
          {:else}
            <svelte:component this={currentStepComponent} />
          {/if}
        </div>
      {/key}
    </main>

    <!-- Professional Footer Navigation -->
    <footer class="fixed bottom-0 left-0 right-0 bg-white/80 backdrop-blur-md border-t border-slate-200 z-30">
      <div class="max-w-6xl mx-auto px-6 py-5 flex items-center justify-between">
        <div class="flex items-center gap-8">
          <div class="hidden lg:flex items-center gap-3">
            <div class="flex -space-x-2">
              <div class="w-8 h-8 rounded-full bg-slate-100 border-2 border-white flex items-center justify-center text-slate-400" title="CPU">
                <span class="text-[10px] font-bold">CPU</span>
              </div>
              <div class="w-8 h-8 rounded-full bg-slate-100 border-2 border-white flex items-center justify-center text-slate-400" title="RAM">
                <span class="text-[10px] font-bold">MEM</span>
              </div>
            </div>
            <div class="text-[11px] font-medium text-slate-500">
              System health: <span class="text-emerald-600">Optimal</span>
            </div>
          </div>

          <div class="h-8 w-px bg-slate-200 hidden lg:block"></div>

          <div class="flex items-center gap-2">
            <button 
              class="p-2 text-slate-400 hover:text-slate-600 transition-colors"
              on:click={() => {/* Show logs */}}
              title="System Logs"
            >
              <CircleHelp class="w-5 h-5" />
            </button>
          </div>
        </div>

        <div class="flex items-center gap-4">
          <button 
            class="px-6 py-2.5 text-sm font-bold text-slate-600 hover:text-slate-900 transition-all flex items-center gap-2 disabled:opacity-30" 
            on:click={prevStep} 
            disabled={currentStep === 0 && currentTab === 0}
          >
            <ChevronLeft class="w-4 h-4" />
            Back
          </button>

          <button 
            class="px-8 py-2.5 bg-blue-600 text-white text-sm font-bold rounded-lg shadow-lg shadow-blue-200 hover:bg-blue-700 hover:shadow-blue-300 transition-all flex items-center gap-2 disabled:opacity-50 disabled:shadow-none" 
            on:click={nextStep} 
            disabled={isLastTab
               ? !$stepValidity[/** @type {any} */ (steps[currentStep].name)]
               : !$tabCompletion[/** @type {any} */ (steps[currentStep].name)]?.[
                   /** @type {any} */ (getVisibleTabs(steps[currentStep].name, $installerState.userExperienceMode)[currentTab])
                 ]}
          >
            {#if currentStep === steps.length - 1 && isLastTab}
              Complete Installation
            {:else if isLastTab}
              Next Step: {steps[currentStep + 1]?.name}
              <ChevronRight class="w-4 h-4" />
            {:else}
              Continue
              <ChevronRight class="w-4 h-4" />
            {/if}
          </button>
        </div>
      </div>
    </footer>
  </div>
</div>

{#if showDangerModal}
  <div class="fixed inset-0 z-50 bg-black/80 backdrop-blur-sm flex items-center justify-center p-4">
    <div class="max-w-2xl w-full rounded-xl bg-slate-900 border border-red-500/40 p-6">
      <div class="flex items-start gap-3 mb-3">
        <ShieldAlert class="w-6 h-6 text-red-300 mt-0.5" />
        <div>
          <h3 class="text-xl font-semibold text-red-200">Dangerous Mode Warning</h3>
          <p class="text-sm text-slate-200 mt-1">
            Dangerous mode can erase disks and apply real system changes. Use only when you fully understand the selected operations.
          </p>
        </div>
      </div>
      <p class="text-xs text-slate-400 mb-3">Type <span class="font-mono text-slate-100">{DANGER_PHRASE}</span> to continue.</p>
      <input class="w-full px-3 py-2 rounded-md border border-slate-700 bg-slate-950 text-white mb-4" bind:value={dangerConfirmation} />
      <div class="flex justify-end gap-2">
        <button class="px-3 py-2 rounded-md bg-slate-800 text-slate-200" on:click={closeDangerModeModal}>Cancel</button>
        <button class="px-3 py-2 rounded-md bg-red-600 text-white disabled:opacity-50" on:click={enableDangerMode} disabled={dangerConfirmation !== DANGER_PHRASE}>Enable Dangerous Mode</button>
      </div>
    </div>
  </div>
{/if}

{/if}
