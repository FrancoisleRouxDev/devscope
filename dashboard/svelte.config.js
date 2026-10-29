// =============================================================================
// dashboard/svelte.config.js — Svelte compiler options
// =============================================================================
// Read by @sveltejs/vite-plugin-svelte (build) and svelte-check (type-check).
// `vitePreprocess` enables <script lang="ts"> and modern CSS in .svelte files.
// This is plain Svelte + Vite (no SvelteKit): Electron is a single-window
// desktop app and doesn't need server-side rendering or file-based routing.
// =============================================================================

import { vitePreprocess } from '@sveltejs/vite-plugin-svelte';

export default {
  preprocess: vitePreprocess(),
  compilerOptions: {
    // Svelte 5 runes mode everywhere ($state, $derived, $props, $effect).
    runes: true,
  },
};
