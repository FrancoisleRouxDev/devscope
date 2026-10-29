// =============================================================================
// src/lib/api/client.ts — typed HTTP client for the local FastAPI sidecar
// =============================================================================
// The ONLY place in the UI that calls fetch(). Components and stores call
// these functions and get typed results (types.ts) or a typed ApiError.
//
//   * Base URL + token come from Electron (window.devscope.getApiConnection()).
//     During `npm run dev:web` (browser only) they fall back to
//     import.meta.env.VITE_API_URL / VITE_API_TOKEN.
//   * Every request carries `Authorization: Bearer <token>`.
//   * Non-2xx responses become `ApiError { status, code, message }` — UI code
//     never parses error bodies itself.
//   * Requests use AbortController timeouts, so a hung backend can't freeze
//     a spinner forever.
// =============================================================================

// =============================================================================
// LOGIC SKETCH (commented out — not implemented yet)
// =============================================================================
//
// import type { ScanJob, ScanRequest } from './types';
//
// export class ApiError extends Error {
//   constructor(readonly status: number, readonly code: string | undefined, message: string) {
//     super(message);
//   }
// }
//
// export function createClient(conn: { baseUrl: string; token: string }) {
//   async function request<T>(method: 'GET' | 'POST', path: string, body?: unknown): Promise<T> {
//     const res = await fetch(`${conn.baseUrl}${path}`, {
//       method,
//       headers: { Authorization: `Bearer ${conn.token}`, 'Content-Type': 'application/json' },
//       body: body === undefined ? undefined : JSON.stringify(body),
//       signal: AbortSignal.timeout(10_000),
//     });
//     if (!res.ok) {
//       const err = await res.json().catch(() => ({}));
//       throw new ApiError(res.status, err.code, err.detail ?? res.statusText);
//     }
//     return res.json() as Promise<T>;
//   }
//
//   return {
//     startScan: (req: ScanRequest) => request<ScanJob>('POST', '/scans', req),
//     getScan:   (id: string)       => request<ScanJob>('GET', `/scans/${encodeURIComponent(id)}`),
//     listScans: ()                 => request<ScanJob[]>('GET', '/scans'),
//   };
// }
//
// export type ApiClient = ReturnType<typeof createClient>;

export {};
