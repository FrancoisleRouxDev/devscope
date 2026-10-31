// stores.js - Svelte stores: selected project, scan result, loading/error state.
// A store is a shared reactive value; any component can read it with `$name`
// and the UI re-renders when it changes.

import { writable } from 'svelte/store';

export const apiOnline = writable(false); // is the Python API reachable?
export const root = writable(null);       // workspace folder being analysed
export const projects = writable([]);     // projects found under the workspace
export const selected = writable(null);   // the project the user clicked
export const result = writable(null);     // line/language stats for `selected`
export const findings = writable([]);     // possible secrets in `selected`
export const loading = writable(false);   // true while the API works
export const error = writable(null);      // error message to show, or null
