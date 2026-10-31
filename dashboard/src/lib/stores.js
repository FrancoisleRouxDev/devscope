// stores.js - Svelte stores: selected folder, analysis results, loading/error state.
// A store is a shared reactive value; any component can read it with `$name`
// and the UI re-renders when it changes.

import { writable } from 'svelte/store';

export const apiOnline = writable(false); // is the Python API reachable?
export const known = writable({});        // OS folders for quick access: downloads, documents, desktop
export const root = writable(null);       // folder being analysed
export const report = writable(null);     // general folder analysis
export const dupes = writable(null);      // duplicate groups (null = not searched yet)
export const dupesBusy = writable(false); // true while searching for duplicates
export const tab = writable('folder');    // 'folder' or 'projects'
export const projects = writable([]);     // code projects found inside the folder
export const selected = writable(null);   // the project the user clicked
export const result = writable(null);     // line/language stats for `selected`
export const findings = writable([]);     // possible secrets in `selected`
export const loading = writable(false);   // true while the API works
export const error = writable(null);      // error message to show, or null
