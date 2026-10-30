// preload.js - runs in an isolated context before the renderer loads.
// Uses contextBridge to expose a minimal, safe API to Svelte
// (e.g. pickFolder(), getApiBaseUrl()). Node APIs are never exposed directly.
