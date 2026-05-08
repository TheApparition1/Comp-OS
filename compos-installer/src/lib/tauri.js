// Tauri API wrapper for safe dynamic imports.
// Tauri v2 exposes different globals depending on platform/bundler.
function isTauriRuntime() {
  if (typeof window === 'undefined') return false;
  // Access via bracket notation to avoid type-check errors in JS projects.
  return Boolean(window['__TAURI__'] || window['__TAURI_INTERNALS__']);
}

export async function invoke(command, args = {}) {
  if (!isTauriRuntime()) {
    // Fallback for development in a regular browser.
    return null;
  }

  try {
    const { invoke: tauriInvoke } = await import('@tauri-apps/api/core');
    return await tauriInvoke(command, args);
  } catch (error) {
    // Normalize to a string-ish message so UI can render it safely.
    const message =
      error instanceof Error ? error.message : typeof error === 'string' ? error : JSON.stringify(error);
    throw new Error(message);
  }
}
