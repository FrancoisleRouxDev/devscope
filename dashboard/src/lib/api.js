// api.js - client for the local FastAPI service (fetch wrappers for
// /health and /scan). Mirrors the contract in api/devscope_api/schemas.py.

let baseUrl = null;

// Ask Electron (through the preload bridge) where the API is listening.
// Outside Electron (plain browser dev) there is no bridge, so we get null.
export async function connect() {
  baseUrl = (await window.devscope?.getApiBaseUrl?.()) ?? null;
  return baseUrl !== null;
}

// Open the native folder dialog. Returns a path or null if cancelled.
export function pickFolder() {
  return window.devscope?.pickFolder?.() ?? Promise.resolve(null);
}

// POST /scan with the folder path; resolves to the ScanResult JSON.
export async function scan(path) {
  if (!baseUrl) throw new Error('API is offline');
  const res = await fetch(`${baseUrl}/scan`, {
    method: 'POST',
    headers: { 'Content-Type': 'application/json' },
    body: JSON.stringify({ path }),
  });
  if (!res.ok) throw new Error(`Scan failed (${res.status})`);
  return res.json();
}
