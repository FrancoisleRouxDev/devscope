// svelte.config.js - Svelte compiler options and preprocessors.

import { vitePreprocess } from '@sveltejs/vite-plugin-svelte';

export default {
  preprocess: vitePreprocess(), // lets <style>/<script> use Vite's pipeline
};
