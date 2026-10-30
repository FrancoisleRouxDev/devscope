// vite.config.js - Vite build config. Uses the Svelte plugin, sets
// base './' so built assets load from file:// inside Electron, and defines
// the dev server port that Electron loads during development.

import { defineConfig } from 'vite';
import { svelte } from '@sveltejs/vite-plugin-svelte';

export default defineConfig({
  plugins: [svelte()],
  base: './',
  server: { port: 5173, strictPort: true }, // Electron expects exactly this port
  build: { outDir: 'dist', emptyOutDir: true },
});
