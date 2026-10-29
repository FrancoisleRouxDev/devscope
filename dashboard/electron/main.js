// @ts-check
// =============================================================================
// electron/main.js — Electron MAIN process (Node.js side of the app)
// =============================================================================
// Responsibilities:
//   1. App lifecycle: single-instance lock, create the window, quit cleanly.
//   2. Start the Python API sidecar (./sidecar.js) and stop it on quit.
//   3. Own all privileged operations and expose them to the renderer ONLY
//      through narrow IPC handlers (e.g. the native "Choose folder" dialog).
//   4. Load the UI: the Vite dev server in dev, dist/index.html in production.
//
// The renderer (Svelte) never gets Node.js access. It talks to:
//   * this process via `window.devscope.*` (defined in preload.cjs), and
//   * the Python API directly over HTTP on 127.0.0.1, using the port + token
//     this process hands it.
// =============================================================================

// =============================================================================
// LOGIC SKETCH (commented out — not implemented yet)
// =============================================================================
//
// import { app, BrowserWindow, dialog, ipcMain, shell } from 'electron';
// import path from 'node:path';
// import { fileURLToPath } from 'node:url';
// import { startSidecar } from './sidecar.js';
//
// const __dirname = path.dirname(fileURLToPath(import.meta.url));
// const DEV_URL = process.env.VITE_DEV_SERVER_URL;          // set by `npm run dev`
//
// /** @type {import('./sidecar.js').Sidecar | null} */
// let sidecar = null;
//
// // --- 1. Only one devscope at a time; a second launch focuses the first. ---
// if (!app.requestSingleInstanceLock()) app.quit();
//
// function createWindow() {
//   const win = new BrowserWindow({
//     width: 1280, height: 820, minWidth: 900, minHeight: 600,
//     backgroundColor: '#0b0d12',            // matches app.css → no white flash on start
//     show: false,                           // show on 'ready-to-show' to avoid flicker
//     titleBarStyle: process.platform === 'darwin' ? 'hiddenInset' : 'default',
//     webPreferences: {
//       preload: path.join(__dirname, 'preload.cjs'),
//       contextIsolation: true,              // renderer JS can't touch preload internals
//       nodeIntegration: false,              // no require('fs') in the UI
//       sandbox: true,                       // OS-level sandbox for the renderer
//     },
//   });
//
//   win.once('ready-to-show', () => win.show());
//
//   // Open external links in the user's browser, never inside the app.
//   win.webContents.setWindowOpenHandler(({ url }) => {
//     if (url.startsWith('https://')) shell.openExternal(url);
//     return { action: 'deny' };
//   });
//
//   if (DEV_URL) win.loadURL(DEV_URL);
//   else win.loadFile(path.join(__dirname, '..', 'dist', 'index.html'));
// }
//
// // --- 3. IPC: the ONLY privileged operations the renderer can request. ------
// ipcMain.handle('devscope:pick-folder', async () => {
//   const { canceled, filePaths } = await dialog.showOpenDialog({ properties: ['openDirectory'] });
//   return canceled ? null : filePaths[0];
// });
// ipcMain.handle('devscope:api-connection', () => sidecar?.connection ?? null);
//
// app.whenReady().then(async () => {
//   sidecar = await startSidecar({ dev: Boolean(DEV_URL) });   // 2. start Python
//   createWindow();
//   // macOS: re-create a window when the dock icon is clicked and none are open.
//   app.on('activate', () => { if (BrowserWindow.getAllWindows().length === 0) createWindow(); });
// });
//
// // Windows/Linux quit when the last window closes; macOS apps conventionally stay alive.
// app.on('window-all-closed', () => { if (process.platform !== 'darwin') app.quit(); });
// app.on('will-quit', () => sidecar?.stop());

// =============================================================================
// JUNIOR vs SENIOR — Electron window security
// =============================================================================
//
// ── JUNIOR ──────────────────────────────────────────────────────────────────
//
//   new BrowserWindow({ webPreferences: { nodeIntegration: true, contextIsolation: false } })
//   // ...then in Svelte: const fs = require('fs'); fs.readdirSync(userInput)
//
//   Any XSS — e.g. a malicious filename rendered with {@html} — now has full
//   Node.js access: it can read ~/.ssh, run shell commands, anything.
//
// ── SENIOR ──────────────────────────────────────────────────────────────────
//
//   contextIsolation + sandbox + no nodeIntegration, a strict CSP in
//   index.html, and a preload exposing a handful of specific functions
//   (`pickFolder()`), never generic ones (`invoke(channel, ...args)`,
//   `readFile(path)`). Privilege lives in main; the UI only asks.
