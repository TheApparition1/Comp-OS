<script>
  import { Shield, Key, ChevronRight, AlertCircle, Rocket } from 'lucide-svelte';
  import { fade, fly } from 'svelte/transition';

  export let onAuthorized = () => {};
  export let authorize = async () => {
    throw new Error('Authorization must be implemented by a trusted native/backend layer');
  };

  let password = '';
  let error = '';
  let isLoading = false;

  async function handleSubmit() {
    if (isLoading) return;
    
    isLoading = true;
    error = '';

    try {
      // Keep the brief delay for the existing UX while delegating verification
      await new Promise((resolve) => setTimeout(resolve, 600));

      const isAuthorized = await authorize(password);

      if (isAuthorized) {
        await onAuthorized();
      } else {
        error = 'Invalid authentication credentials';
        password = '';
      }
    } catch {
      error = 'Authentication service unavailable';
      password = '';
    } finally {
      isLoading = false;
    }
  }

  /** @param {KeyboardEvent} e */
  function handleKeydown(e) {
    if (e.key === 'Enter') {
      handleSubmit();
    }
  }
</script>

<div class="fixed inset-0 flex items-center justify-center p-4 bg-slate-50" in:fade>
  <div class="w-full max-w-md" in:fly={{ y: 20, duration: 600 }}>
    <div class="text-center mb-12">
      <div class="w-20 h-20 bg-blue-600 rounded-2xl flex items-center justify-center text-white shadow-xl shadow-blue-200 mx-auto mb-6">
        <Rocket class="w-10 h-10" />
      </div>
      <h1 class="text-3xl font-bold text-slate-900 mb-2">CompOS System</h1>
      <p class="text-slate-500">Authentication required to access installer</p>
    </div>

    <div class="bg-white border border-slate-200 rounded-3xl p-8 shadow-sm">
      <div class="space-y-6">
        <div class="space-y-3">
          <label class="block text-[10px] font-bold text-slate-400 uppercase tracking-widest" for="password">Administrator Password</label>
          <div class="relative">
            <div class="absolute inset-y-0 left-0 pl-4 flex items-center pointer-events-none text-slate-400">
              <Key class="w-5 h-5" />
            </div>
            <input 
              id="password"
              type="password" 
              class="w-full pl-12 pr-4 py-4 bg-slate-50 border border-slate-100 rounded-xl focus:border-blue-600 focus:outline-none font-medium text-slate-700 transition-all {error ? 'border-red-300 bg-red-50' : ''}" 
              placeholder="Enter password"
              bind:value={password}
              on:keydown={handleKeydown}
              disabled={isLoading}
            />
          </div>
          {#if error}
            <p class="text-xs font-bold text-red-500 flex items-center gap-1.5" in:fade>
              <AlertCircle class="w-3.5 h-3.5" />
              {error}
            </p>
          {/if}
        </div>

        <button 
          class="w-full py-4 bg-blue-600 hover:bg-blue-700 disabled:bg-blue-400 text-white rounded-xl font-bold transition-all shadow-lg shadow-blue-200 flex items-center justify-center gap-2 group"
          on:click={handleSubmit}
          disabled={isLoading || !password}
        >
          {#if isLoading}
            <div class="w-5 h-5 border-2 border-white/30 border-t-white rounded-full animate-spin"></div>
            Authenticating...
          {:else}
            Access Installer
            <ChevronRight class="w-5 h-5 group-hover:translate-x-0.5 transition-transform" />
          {/if}
        </button>
      </div>

      <div class="mt-8 pt-8 border-t border-slate-100 flex items-center gap-4">
        <div class="w-10 h-10 rounded-full bg-slate-50 flex items-center justify-center text-slate-400">
          <Shield class="w-5 h-5" />
        </div>
        <p class="text-[11px] font-medium text-slate-500 leading-relaxed">
          This Password gate is simply a protection mechanism to prevent unauthorised viewing of the installer, which is not open source. This does not affect the existing system.
        </p>
      </div>
    </div>
  </div>
</div>
