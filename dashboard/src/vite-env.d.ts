// =============================================================================
// src/vite-env.d.ts — ambient type declarations for the renderer
// =============================================================================
// * Pulls in Vite's client types (import.meta.env, asset imports).
// * Declares `window.devscope` — the API exposed by electron/preload.cjs —
//   so TypeScript checks every call the UI makes into Electron.
//   Keep this in sync with preload.cjs.
// =============================================================================

/// <reference types="svelte" />
/// <reference types="vite/client" />

// =============================================================================
// PLANNED DECLARATIONS (commented out — not implemented yet)
// =============================================================================
//
// interface ApiConnection {
//   baseUrl: string;   // e.g. "http://127.0.0.1:53127"
//   token: string;
// }
//
// interface DevscopeBridge {
//   pickFolder(): Promise<string | null>;
//   getApiConnection(): Promise<ApiConnection | null>;
//   readonly platform: 'win32' | 'darwin' | 'linux';
// }
//
// interface Window {
//   // Optional: undefined when the UI runs in a plain browser via `npm run dev:web`.
//   readonly devscope?: DevscopeBridge;
// }
