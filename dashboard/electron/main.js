// main.js - Electron main process.
// Creates the BrowserWindow, loads the Vite dev server (dev) or built
// index.html (prod), starts/stops the Python API via backend.js, and
// handles native dialogs such as "choose project folder".

import { app, BrowserWindow, dialog, ipcMain, Menu, session, shell } from 'electron';
import path from 'node:path';
import { fileURLToPath, pathToFileURL } from 'node:url';
import { getBaseUrl, getToken, startBackend, stopBackend } from './backend.js';

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

  // SECURITY: the window only ever shows our own UI. Refuse pop-ups, and refuse to navigate
  // anywhere else (a link or script must not be able to load a web page into this window,
  // where it would sit next to the bridge to the file system).
  const ownPage = app.isPackaged ? pathToFileURL(path.join(__dirname, '../dist/index.html')).href : DEV_URL;
  win.webContents.setWindowOpenHandler(() => ({ action: 'deny' }));
  win.webContents.on('will-navigate', (event, url) => {
    if (!url.startsWith(ownPage.split('#')[0])) event.preventDefault();
  });

  // Development only: F12 toggles DevTools (the menu that normally offers it is removed below).
  if (!app.isPackaged) {
    win.webContents.on('before-input-event', (_event, input) => {
      if (input.type === 'keyDown' && input.key === 'F12') win.webContents.toggleDevTools();
    });
  }

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

// URL of the Python API (null while it is offline) and the one-time token it requires.
ipcMain.handle('api:baseUrl', () => getBaseUrl());
ipcMain.handle('api:token', () => getToken());

// Installed builds look for a newer release on GitHub (publish settings in package.json),
// download it in the background and offer to install it. Skipped during development, and
// any failure (offline, private repo, no release yet) is ignored so it never blocks the app.
async function checkForUpdates() {
  if (!app.isPackaged) return;
  try {
    const { default: updater } = await import('electron-updater');
    await updater.autoUpdater.checkForUpdatesAndNotify();
  } catch (err) {
    console.warn('Update check skipped:', err.message);
  }
}

app.whenReady().then(() => {
  // Start the engine in the background. The window opens straight away and the UI keeps asking
  // for the engine until it is ready (a first launch can take a while: antivirus scans new files).
  startBackend().then((result) => {
    if (!result.ok) console.warn(`API not available (${result.reason}); UI will show offline.`);
  });

  // The app needs no camera, microphone, location, notifications and so on: deny every request.
  session.defaultSession.setPermissionRequestHandler((_webContents, _permission, callback) => callback(false));

  // No File/Edit/View/Window menu bar on Windows and Linux; the app has its own UI.
  // macOS keeps its system menu, which it needs for Quit and Copy/Paste.
  if (process.platform !== 'darwin') Menu.setApplicationMenu(null);

  createWindow();
  checkForUpdates();

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
