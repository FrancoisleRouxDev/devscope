// =============================================================================
// src/lib/stores/scan.svelte.ts — app state for the current scan (Svelte 5 runes)
// =============================================================================
// The `.svelte.ts` extension lets this plain TS module use runes ($state,
// $derived) so components re-render automatically when state changes.
//
// Holds: connection status, the current job, its report, recent projects,
// and the last error. Exposes intent-level actions (`pickAndScan()`,
// `rescan()`), so components never orchestrate API calls themselves.
//
// Polling: after POST /scans, poll GET /scans/{id} with a short backoff
// (250 ms → 1 s) until status is "done" or "failed". Starting a new scan
// cancels the previous poll loop.
// =============================================================================

// =============================================================================
// LOGIC SKETCH (commented out — not implemented yet)
// =============================================================================
//
// import { createClient, type ApiClient } from '../api/client';
// import type { ScanJob, ScanReport } from '../api/types';
//
// class ScanStore {
//   status = $state<'connecting' | 'idle' | 'scanning' | 'error'>('connecting');
//   job    = $state<ScanJob | null>(null);
//   error  = $state<string | null>(null);
//   report = $derived<ScanReport | null>(this.job?.report ?? null);
//
//   #client: ApiClient | null = null;
//   #pollToken = 0;                         // increments to cancel stale poll loops
//
//   async connect() { /* window.devscope.getApiConnection() → createClient */ }
//   async pickAndScan() { /* window.devscope.pickFolder() → this.scanPath(path) */ }
//   async scanPath(path: string) { /* startScan → poll until done/failed */ }
// }
//
// export const scan = new ScanStore();

export {};
