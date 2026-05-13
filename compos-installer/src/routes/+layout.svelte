<script>
  import '../app.css';
  import { onMount } from 'svelte';

  const INSTALLER_PASSWORD = 'comp-os-demo';

  let password = '';
  let error = '';
  let unlocked = false;
  let mounted = false;

  onMount(() => {
    unlocked = window.localStorage.getItem('compos-installer-unlocked') === 'true';
    mounted = true;
  });

  function unlockInstaller() {
    if (password === INSTALLER_PASSWORD) {
      unlocked = true;
      error = '';
      window.localStorage.setItem('compos-installer-unlocked', 'true');
      return;
    }

    error = 'Incorrect password. Try again.';
  }
</script>

<svelte:head>
  <title>CompOS Installer</title>
  <meta name="description" content="Modern Linux installation with beautiful UI" />
</svelte:head>

<div class="min-h-screen bg-[#f8fafc] text-slate-900">
  {#if !mounted}
    <div class="min-h-screen bg-[#f8fafc]"></div>
  {:else if !unlocked}
    <div class="min-h-screen relative overflow-hidden bg-[radial-gradient(circle_at_top,_rgba(37,99,235,0.18),_transparent_42%),linear-gradient(135deg,_#f8fafc_0%,_#eef2ff_45%,_#e2e8f0_100%)]">
      <div class="absolute inset-0 pointer-events-none opacity-60" style="background-image: radial-gradient(circle at 1px 1px, rgba(148,163,184,0.16) 1px, transparent 0); background-size: 24px 24px;"></div>
      <div class="relative min-h-screen flex items-center justify-center px-6 py-12">
        <div class="w-full max-w-md rounded-[2rem] border border-white/70 bg-white/85 backdrop-blur-xl shadow-[0_24px_80px_rgba(15,23,42,0.12)] p-8 md:p-10">
          <div class="flex items-center gap-4 mb-8">
            <div class="w-12 h-12 rounded-2xl bg-blue-600 flex items-center justify-center text-white shadow-lg shadow-blue-200">
              <span class="text-lg font-bold">C</span>
            </div>
            <div>
              <p class="text-[10px] uppercase tracking-[0.3em] text-slate-400 font-bold">Private Demo</p>
              <h1 class="text-2xl font-bold text-slate-900">CompOS Installer</h1>
            </div>
          </div>

          <div class="mb-8 space-y-3">
            <h2 class="text-lg font-semibold text-slate-900">Enter access password</h2>
            <p class="text-sm leading-6 text-slate-500">
              This demo hides the installer UI until the password is entered. For production website protection, use server/hosting authentication.
            </p>
          </div>

          <form class="space-y-4" on:submit|preventDefault={unlockInstaller}>
            <div class="space-y-2">
              <label class="block text-[10px] font-bold uppercase tracking-[0.25em] text-slate-400" for="installer-password">
                Password
              </label>
              <input
                id="installer-password"
                type="password"
                class="w-full rounded-2xl border border-slate-200 bg-slate-50 px-4 py-3 text-slate-900 placeholder:text-slate-400 outline-none transition focus:border-blue-500 focus:bg-white focus:ring-4 focus:ring-blue-100"
                placeholder="Enter password"
                bind:value={password}
                autocomplete="current-password"
                aria-describedby={error ? 'installer-password-error' : undefined}
              />
            </div>

            {#if error}
              <div
                id="installer-password-error"
                role="alert"
                aria-live="assertive"
                class="rounded-2xl border border-rose-200 bg-rose-50 px-4 py-3 text-sm text-rose-700"
              >
                {error}
              </div>
            {/if}

            <button
              type="submit"
              class="w-full rounded-2xl bg-blue-600 px-4 py-3 text-sm font-bold text-white shadow-lg shadow-blue-200 transition hover:bg-blue-700 hover:shadow-blue-300"
            >
              Unlock Installer
            </button>
          </form>

        </div>
      </div>
    </div>
  {:else}
    <div class="min-h-screen bg-gradient-to-br from-compos-primary to-compos-secondary">
      <slot />
    </div>
  {/if}
</div>

<style>
  :global(body) {
    margin: 0;
    font-family: Inter, ui-sans-serif, system-ui, -apple-system, BlinkMacSystemFont, 'Segoe UI', sans-serif;
  }
</style>
