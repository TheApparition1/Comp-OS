<script>
  import { Terminal, Play, Pause, XCircle, BookOpenText, Rocket, ListTree, RotateCcw, Power, ChevronRight, CheckCircle2 } from 'lucide-svelte';
  import { invoke } from '$lib/tauri.js';
  import {onMount} from "svelte";
  import { installerState, updateInstallerSection } from '$lib/stores/installerState.js';
  
  /** @type {any[]} */
  let installSteps = [];
  let currentStep = 0;
  let isInstalling = false;
  let isPaused = false;
  let output = '';
  /** @type {'automated' | 'educational'} */
  let mode = 'automated';
  let showRestartModal = false;
  let installationStarted = false;

  onMount(async () => {
    try {
      const steps = await invoke('get_install_steps', { state: $installerState });
      installSteps = steps || [];
    } catch (e) {
      installSteps = [];
      console.error('Failed to load install steps:', e);
    }
  });

  async function startInstallation() {
    installationStarted = true;
    isInstalling = true;
    isPaused = mode === 'educational'; // Start paused in educational mode
    output = '';
    
    // Refresh steps just before starting to ensure latest selections are used
    try {
      const steps = await invoke('get_install_steps', { state: $installerState });
      installSteps = steps || [];
    } catch (e) {
      console.error('Final step refresh failed:', e);
    }
    
    for (let i = 0; i < installSteps.length; i++) {
      if (!isInstalling) break;
      currentStep = i;
      const step = installSteps[i];
      
      if (mode === 'educational') {
        isPaused = true;
        // Wait for user to click "Execute Step"
        while (isPaused && isInstalling) {
          await new Promise(resolve => setTimeout(resolve, 100));
        }
        if (!isInstalling) break;
      }

      output += `\n[${new Date().toLocaleTimeString()}] Executing: ${step.command}\n`;
      
      try {
        const result = await invoke('execute_command', { 
          command: step.command, 
          requiresSudo: step.requires_sudo 
        });
        
        if (result.success) {
          output += `✓ ${step.description} completed\n`;
          if (result.output) output += result.output + '\n';
        } else {
          throw new Error(result.error || 'Unknown error');
        }
      } catch (e) {
        const error = e instanceof Error ? e : new Error(String(e));
        output += `❌ Error: ${error.message}\n`;
        
        // In educational mode, always pause on error
        isPaused = true;
        while (isPaused && isInstalling) {
          await new Promise(resolve => setTimeout(resolve, 100));
        }
        if (!isInstalling) break;
      }
      await new Promise(resolve => setTimeout(resolve, 300));
    }
    isInstalling = false;
    if (currentStep === installSteps.length - 1) showRestartModal = true;
  }
  
  function pauseInstallation() {
    isPaused = !isPaused;
  }
  function stopInstallation() {
    isInstalling = false;
    isPaused = false;
  }
  
  /** @param {'automated' | 'educational'} selectedMode */
  function setMode(selectedMode) {
    mode = selectedMode;
    updateInstallerSection('installation', { mode: selectedMode });
  }

  function closeRestartModal() {
    showRestartModal = false;
  }

  async function restartNow() {
    try {
      await invoke('execute_command', { command: 'reboot', requiresSudo: true });
    } catch (e) {
      output += `\nRestart command failed: ${e}\n`;
    } finally {
      showRestartModal = false;
    }
  }
  
  $: progress = currentStep + 1;
  $: totalSteps = installSteps.length;
  $: progressPercentage = totalSteps > 0 ? (progress / totalSteps) * 100 : 0;

  $: summary = {
    Desktop: $installerState.desktop?.selectedDesktop || 'None',
    Disk: $installerState.disk?.selectedDisk || 'None',
    User: $installerState.user?.username || 'None',
    Timezone: $installerState.datetime?.timezone || 'UTC',
    Browser: $installerState.browser?.selectedBrowser || 'None',
    Terminal: $installerState.terminal?.selectedTerminal || 'bash',
    Apps: $installerState.utilities?.selectedApps?.length || 0
  };
</script>

<div class="max-w-4xl mx-auto">
  <div class="mb-10">
    <h2 class="text-2xl font-bold text-slate-900 mb-2">Finalize Installation</h2>
    <p class="text-slate-500">Review your settings and choose your preferred installation method.</p>
  </div>

  {#if !installationStarted}
    <div class="grid md:grid-cols-2 gap-8 mb-10">
      <button 
        class="group p-8 bg-white border-2 rounded-3xl text-left transition-all hover:shadow-xl hover:shadow-slate-200/50 { mode === 'automated' ? 'border-blue-600 ring-4 ring-blue-50' : 'border-slate-100 hover:border-slate-200'}"
        on:click={() => setMode('automated')}
      >
        <div class="w-14 h-14 rounded-2xl bg-slate-50 flex items-center justify-center text-slate-400 group-hover:text-blue-600 transition-colors mb-6 { mode === 'automated' ? 'text-blue-600 bg-blue-50' : ''}">
          <Rocket class="w-7 h-7" />
        </div>
        <h3 class="text-xl font-bold text-slate-900 mb-2">Automated</h3>
        <p class="text-sm text-slate-500 leading-relaxed">
          Standard installation with minimal interaction. Perfect for quick setup.
        </p>
      </button>

      <button 
        class="group p-8 bg-white border-2 rounded-3xl text-left transition-all hover:shadow-xl hover:shadow-slate-200/50 { mode === 'educational' ? 'border-blue-600 ring-4 ring-blue-50' : 'border-slate-100 hover:border-slate-200'}"
        on:click={() => setMode('educational')}
      >
        <div class="w-14 h-14 rounded-2xl bg-slate-50 flex items-center justify-center text-slate-400 group-hover:text-blue-600 transition-colors mb-6 { mode === 'educational' ? 'text-blue-600 bg-blue-50' : ''}">
          <BookOpenText class="w-7 h-7" />
        </div>
        <h3 class="text-xl font-bold text-slate-900 mb-2">Learn to Install</h3>
        <p class="text-sm text-slate-500 leading-relaxed">
          View each command and understand the process step-by-step.
        </p>
      </button>
    </div>

  <!-- Configuration Summary Section -->
    <div class="bg-white border border-slate-200 rounded-3xl p-8 mb-10 shadow-sm">
      <div class="flex items-center gap-3 mb-6">
        <div class="w-10 h-10 rounded-xl bg-slate-50 flex items-center justify-center text-slate-400">
          <ListTree class="w-5 h-5" />
        </div>
        <div>
          <h3 class="text-lg font-bold text-slate-900">Deployment Summary</h3>
          <p class="text-xs text-slate-500 uppercase tracking-widest font-bold">Review your choices</p>
        </div>
      </div>

      <div class="grid grid-cols-2 md:grid-cols-4 gap-6">
        {#each Object.entries(summary) as [key, value]}
          <div class="space-y-1">
            <p class="text-[10px] font-bold text-slate-400 uppercase tracking-[0.2em]">{key}</p>
            <p class="text-sm font-bold text-slate-700 capitalize">{value}</p>
          </div>
        {/each}
      </div>
    </div>

    {#if mode === 'educational'}
      <div class="bg-blue-50 border border-blue-200 rounded-3xl p-8 mb-10">
        <div class="flex items-center gap-4 mb-6">
          <div class="w-12 h-12 rounded-2xl bg-blue-600 flex items-center justify-center text-white shadow-lg shadow-blue-200">
            <BookOpenText class="w-6 h-6" />
          </div>
          <div>
            <h3 class="text-xl font-bold text-slate-900">Learn to Install</h3>
            <p class="text-sm text-slate-500 font-medium">Step-by-step breakdown of the CompOS installation sequence.</p>
          </div>
        </div>

        <div class="space-y-4">
          {#each installSteps as step, i}
            <div class="flex gap-4 p-5 bg-white border border-slate-100 rounded-2xl transition-all {currentStep === i ? 'ring-2 ring-blue-600 shadow-md' : 'opacity-70'}">
              <div class="flex-shrink-0 w-8 h-8 rounded-full bg-slate-100 flex items-center justify-center text-xs font-bold text-slate-500">
                {i + 1}
              </div>
              <div class="flex-1">
                <div class="flex items-center justify-between mb-1">
                  <p class="font-bold text-slate-900">{step.description}</p>
                  <span class="text-[10px] font-bold px-2 py-0.5 bg-slate-100 text-slate-400 rounded uppercase tracking-widest">{step.category}</span>
                </div>
                <p class="text-xs text-slate-500 leading-relaxed mb-3">{step.explanation}</p>
                <div class="bg-slate-900 rounded-lg p-3 font-mono text-[10px] text-emerald-400 border border-slate-800">
                  <span class="text-slate-500 mr-2">$</span> {step.command}
                </div>
              </div>
            </div>
          {/each}
        </div>
      </div>
    {/if}

    <div class="bg-blue-600 rounded-2xl p-8 text-white shadow-xl shadow-blue-200 flex items-center justify-between">
      <div>
        <h3 class="text-xl font-bold mb-1">Ready to deploy CompOS?</h3>
        <p class="text-blue-100 text-sm">All configurations are saved and validated.</p>
      </div>
      <button 
        class="px-8 py-3 bg-white text-blue-600 font-bold rounded-xl shadow-lg hover:bg-blue-50 transition-all flex items-center gap-2"
        on:click={startInstallation}
      >
        Start Installation
        <ChevronRight class="w-4 h-4" />
      </button>
    </div>
  {:else}
    <div class="space-y-6">
      <div class="bg-white border border-slate-200 rounded-2xl p-8 shadow-sm">
        <div class="flex items-center justify-between mb-6">
          <div class="flex items-center gap-3">
            <div class="w-10 h-10 rounded-lg bg-slate-50 flex items-center justify-center text-slate-400">
              <Terminal class="w-5 h-5" />
            </div>
            <div>
              <p class="text-xs font-bold text-slate-400 uppercase tracking-widest">Process Output</p>
              <p class="text-sm font-bold text-slate-900">{isInstalling ? 'Installing system components...' : 'Installation paused'}</p>
            </div>
          </div>
          
          <div class="flex items-center gap-3">
            {#if isInstalling && mode === 'educational'}
              <button 
                class="px-4 py-2 bg-blue-600 text-white text-xs font-bold rounded-lg shadow-lg shadow-blue-200 hover:bg-blue-700 transition-all flex items-center gap-2"
                on:click={() => isPaused = false}
                disabled={!isPaused}
              >
                <Play class="w-4 h-4" />
                Execute Step
              </button>
            {/if}
            <button 
              class="px-4 py-2 bg-slate-100 text-slate-600 text-xs font-bold rounded-lg hover:bg-slate-200 transition-all flex items-center gap-2"
              on:click={stopInstallation}
            >
              <XCircle class="w-4 h-4" />
              Cancel
            </button>
          </div>
        </div>

        <div class="bg-slate-900 rounded-xl p-6 font-mono text-xs leading-relaxed overflow-hidden">
          <div class="h-80 overflow-y-auto scrollbar-hide text-slate-300">
            {#each output.split('\n') as line}
              <div class="mb-1 {line.includes('✓') ? 'text-emerald-400' : line.includes('❌') ? 'text-rose-400' : line.includes('>>>') ? 'text-blue-400 font-bold' : ''}">
                {line}
              </div>
            {/each}
          </div>
        </div>
      </div>

      <div class="bg-white border border-slate-200 rounded-2xl p-6">
        <div class="flex justify-between items-center mb-3">
          <span class="text-xs font-bold text-slate-400 uppercase tracking-widest">Overall Progress</span>
          <span class="text-xs font-bold text-blue-600">{progressPercentage.toFixed(0)}%</span>
        </div>
        <div class="h-2 w-full bg-slate-100 rounded-full overflow-hidden">
          <div class="h-full bg-blue-600 transition-all duration-500" style="width: {progressPercentage}%"></div>
        </div>
      </div>
    </div>
  {/if}
</div>

{#if showRestartModal}
  <div class="fixed inset-0 z-50 flex items-center justify-center p-6 bg-slate-900/40 backdrop-blur-sm">
    <div class="bg-white rounded-3xl shadow-2xl border border-slate-100 p-10 max-w-lg text-center animate-in zoom-in duration-300">
      <div class="w-20 h-20 bg-emerald-100 rounded-full flex items-center justify-center text-emerald-600 mx-auto mb-6">
        <CheckCircle2 class="w-10 h-10" />
      </div>
      <h3 class="text-2xl font-bold text-slate-900 mb-2">Installation Complete</h3>
      <p class="text-slate-500 mb-8 leading-relaxed">
        CompOS has been successfully installed on your system. Please restart to begin your journey.
      </p>
      <div class="flex gap-4">
        <button class="flex-1 px-6 py-3 border-2 border-slate-100 text-slate-600 font-bold rounded-xl hover:bg-slate-50 transition-all" on:click={closeRestartModal}>
          Later
        </button>
        <button class="flex-1 px-6 py-3 bg-blue-600 text-white font-bold rounded-xl shadow-lg shadow-blue-200 hover:bg-blue-700 transition-all" on:click={restartNow}>
          Restart Now
        </button>
      </div>
    </div>
  </div>
{/if}
