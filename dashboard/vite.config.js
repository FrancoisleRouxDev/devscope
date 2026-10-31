// vite.config.js - Vite build config. Uses the Svelte plugin, sets
// base './' so built assets load from file:// inside Electron, and defines
// the dev server port that Electron loads during development.

import { defineConfig } from 'vite';
import { svelte } from '@sveltejs/vite-plugin-svelte';

// SECURITY: Content-Security-Policy for the built app. The page may only run its own bundled
// script, load its own files, and talk to the local API on 127.0.0.1. Everything else (remote
// scripts, frames, forms, plugins) is refused, which limits the damage if a bug ever lets
// untrusted text, such as a file name from a downloaded folder, reach the page as markup.
// Only added to production builds: the dev server needs inline scripts and websockets for hot reload.
const csp = [
  "default-src 'none'",
  "script-src 'self'",
  "style-src 'self' 'unsafe-inline'", // bar widths and colours are set with style attributes
  "img-src 'self' data:",
  "connect-src http://127.0.0.1:*",
  "base-uri 'none'",
  "form-action 'none'",
].join('; ');

const addCsp = {
  name: 'devscope-csp',
  apply: 'build',
  transformIndexHtml: (html) =>
    html.replace('<head>', `<head>\n    <meta http-equiv="Content-Security-Policy" content="${csp}" />`),
};

export default defineConfig({
  plugins: [svelte(), addCsp],
  base: './',
  server: { port: 5173, strictPort: true }, // Electron expects exactly this port
  build: { outDir: 'dist', emptyOutDir: true },
});
