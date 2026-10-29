// =============================================================================
// dashboard/vite.config.ts — Vite build config for the Svelte renderer
// =============================================================================
// Vite only builds the RENDERER (the UI inside the Electron window). The
// Electron main/preload scripts in ./electron are plain JS loaded directly by
// Electron and are not bundled.
//
//   dev:   `npm run dev`   → Vite dev server on 127.0.0.1:5173 with HMR;
//                            Electron loads that URL.
//   prod:  `npm run build` → static files in ./dist; Electron loads
//                            dist/index.html from disk (file://).
// =============================================================================

import { svelte } from '@sveltejs/vite-plugin-svelte';
import { defineConfig } from 'vite';

export default defineConfig({
  plugins: [svelte()],

  // Relative asset URLs, so dist/index.html works when opened via file://
  // inside the packaged app (absolute "/assets/..." would resolve to the
  // filesystem root).
  base: './',

  server: {
    host: '127.0.0.1',
    port: 5173,
    // Fail instead of silently moving to 5174 — package.json's `dev` script
    // and electron/main.js expect exactly this port.
    strictPort: true,
  },

  build: {
    outDir: 'dist',
    emptyOutDir: true,
    // Electron ships a known, modern Chromium — no need to transpile for old browsers.
    target: 'esnext',
  },
});
