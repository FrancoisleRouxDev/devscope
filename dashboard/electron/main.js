// main.js - Electron main process.
// Creates the BrowserWindow, loads the Vite dev server (dev) or built
// index.html (prod), starts/stops the Python API via backend.js, and
// handles native dialogs such as "choose project folder".

import { app, BrowserWindow, dialog, ipcMain, shell } from 'electron';
import path from 'node:path';
import { fileURLToPath } from 'node:url';
import { getBaseUrl, startBackend, stopBackend } from './backend.js';

const __dirname = path.dirname(fileURLToPath(import.meta.url));
const DEV_URL = process.env.VITE_DEV_SERVER_URL || 'http://localhost:5173';

function createWindow() {
  const win = new BrowserWindow({
    width: 1200,
    height: 800,
    minWidth: 900,
    minHeight: 600,
    backgroundColor: '#0d1117', // matches the dark theme, avoids a white flash
    show: false,
    webPreferences: {
      preload: path.join(__dirname, 'preload.cjs'),
      // Security: the renderer is a web page, not Node. It only gets what preload exposes.
      contextIsolation: true,
      nodeIntegration: false,
      sandbox: true,
    },
  });

  win.once('ready-to-show', () => win.show());

  if (app.isPackaged) {
    win.loadFile(path.join(__dirname, '../dist/index.html'));
  } else {
    win.loadURL(DEV_URL);
  }
}

// --- IPC: the only ways the renderer can reach native features ---

// Native "choose folder" dialog. Returns the path, or null if cancelled.
ipcMain.handle('dialog:pickFolder', async () => {
  const { canceled, filePaths } = await dialog.showOpenDialog({
    properties: ['openDirectory'],
    title: 'Choose a project to scan',
  });
  return canceled ? null : filePaths[0];
});

// Reveal a file in Explorer/Finder/the file manager. Only absolute string paths are accepted.
ipcMain.handle('shell:reveal', (_event, target) => {
  if (typeof target === 'string' && path.isAbsolute(target)) shell.showItemInFolder(target);
});

// Well-known folders for one-click analysis (paths come from the OS, not the renderer).
ipcMain.handle('folders:known', () => ({
  downloads: app.getPath('downloads'),
  documents: app.getPath('documents'),
  desktop: app.getPath('desktop'),
}));

// URL of the Python API (null while it is offline).
ipcMain.handle('api:baseUrl', () => getBaseUrl());

app.whenReady().then(async () => {
  // Start the API first so the UI can connect as soon as it loads.
  const result = await startBackend();
  if (!result.ok) console.warn(`API not available (${result.reason}); UI will show offline.`);

  createWindow();

  // macOS: re-create the window when the dock icon is clicked and none are open.
  app.on('activate', () => {
    if (BrowserWindow.getAllWindows().length === 0) createWindow();
  });
});

// Quit when all windows close, except on macOS where apps stay alive until Cmd+Q.
app.on('window-all-closed', () => {
  if (process.platform !== 'darwin') app.quit();
});

// Always stop the Python process when the app exits.
app.on('before-quit', stopBackend);
