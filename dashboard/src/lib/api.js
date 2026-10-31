// api.js - client for the local FastAPI service (fetch wrappers for
// /health, /folder, /duplicates, /workspace, /scan and /secrets). Mirrors the
// contract in api/devscope_api/schemas.py.

let baseUrl = null;

// Ask Electron (through the preload bridge) where the API is listening.
// Outside Electron (plain browser dev) there is no bridge, so we get null.
export async function connect() {
  baseUrl = (await window.devscope?.getApiBaseUrl?.()) ?? null;
  return baseUrl !== null;
}

// --- Native features, available only inside Electron ---
export const pickFolder = () => window.devscope?.pickFolder?.() ?? Promise.resolve(null);
export const getKnownFolders = () => window.devscope?.getKnownFolders?.() ?? Promise.resolve({});
export const reveal = (path) => window.devscope?.reveal?.(path); // show a file in the file manager

// POST a JSON body to the API and return the parsed JSON response.
async function post(endpoint, body) {
  if (!baseUrl) throw new Error('API is offline');
  const res = await fetch(`${baseUrl}${endpoint}`, {
    method: 'POST',
    headers: { 'Content-Type': 'application/json' },
    body: JSON.stringify(body),
  });
  if (!res.ok) {
    const detail = (await res.json().catch(() => null))?.detail;
    throw new Error(detail ?? `Request failed (${res.status})`);
  }
  return res.json();
}

export const folder = (path) => post('/folder', { path });             // any folder: categories, sizes, ages
export const duplicates = (path) => post('/duplicates', { path });     // identical files (slower)
export const workspace = (root) => post('/workspace', { root });       // code projects under root
export const scan = (path) => post('/scan', { path });                 // line/language stats
export const secrets = (path) => post('/secrets', { path });           // possible leaked secrets
