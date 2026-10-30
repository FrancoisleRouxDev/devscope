// stores.js - Svelte stores: selected project, scan result, loading/error state.
// A store is a shared reactive value; any component can read it with `$name`
// and the UI re-renders when it changes.

import { writable } from 'svelte/store';

export const apiOnline = writable(false); // is the Python API reachable?
export const project = writable(null);    // chosen folder path, or null
export const result = writable(null);     // latest ScanResult from the API
export const loading = writable(false);   // true while a scan runs
export const error = writable(null);      // error message to show, or null
