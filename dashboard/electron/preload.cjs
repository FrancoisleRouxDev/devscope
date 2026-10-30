// preload.cjs - runs in an isolated context before the renderer loads.
// Uses contextBridge to expose a minimal, safe API to Svelte
// (e.g. pickFolder(), getApiBaseUrl()). Node APIs are never exposed directly.
// (.cjs because sandboxed preload scripts must be CommonJS.)

const { contextBridge, ipcRenderer } = require('electron');

// Appears in the renderer as `window.devscope`.
contextBridge.exposeInMainWorld('devscope', {
  pickFolder: () => ipcRenderer.invoke('dialog:pickFolder'),
  getApiBaseUrl: () => ipcRenderer.invoke('api:baseUrl'),
});
