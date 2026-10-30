// main.js - Svelte entry point. Mounts App.svelte and imports global styles.

import { mount } from 'svelte';
import './styles/global.css';
import App from './App.svelte';

// Svelte 5: `mount` attaches the root component to the #app div in index.html.
const app = mount(App, { target: document.getElementById('app') });

export default app;
