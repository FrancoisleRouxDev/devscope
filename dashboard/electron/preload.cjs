// =============================================================================
// electron/preload.cjs — the bridge between main and renderer
// =============================================================================
// Runs in an isolated context before the Svelte app loads. It exposes a tiny,
// explicit API on `window.devscope` via `contextBridge`. The renderer's types
// for it are declared in src/vite-env.d.ts — keep the two in sync.
//
// Why `.cjs`? Sandboxed preload scripts must be CommonJS, while the rest of
// ./electron is ESM ("type": "module" in package.json).
//
// Sandboxed preloads also can't `require()` local files, so IPC channel names
// are written out here and in main.js. They're namespaced `devscope:*`.
// =============================================================================

// =============================================================================
// LOGIC SKETCH (commented out — not implemented yet)
// =============================================================================
//
// const { contextBridge, ipcRenderer } = require('electron');
//
// contextBridge.exposeInMainWorld('devscope', {
//   /** Opens the native folder picker. Resolves to an absolute path or null. */
//   pickFolder: () => ipcRenderer.invoke('devscope:pick-folder'),
//
//   /** { baseUrl: "http://127.0.0.1:53127", token: "..." } once the API is up. */
//   getApiConnection: () => ipcRenderer.invoke('devscope:api-connection'),
//
//   /** "win32" | "darwin" | "linux" — for small platform-specific UI tweaks. */
//   platform: process.platform,
// });
//
// // NOT like this (exposes every IPC channel, including future privileged ones):
// //   contextBridge.exposeInMainWorld('ipc', { invoke: ipcRenderer.invoke });
