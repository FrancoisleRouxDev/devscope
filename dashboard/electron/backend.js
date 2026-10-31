// backend.js - manages the Python API child process.
// Finds a free port, spawns the API (dev: uvicorn from the venv; installed app:
// the bundled devscope-api executable), waits for GET /health, and kills the
// process on app quit.
// Handles platform differences (Windows/macOS/Linux paths and signals).

import { app } from 'electron';
import { spawn } from 'node:child_process';
import { randomBytes } from 'node:crypto';
import { existsSync } from 'node:fs';
import net from 'node:net';
import path from 'node:path';
import { fileURLToPath } from 'node:url';

const HOST = '127.0.0.1';
const API_DIR = path.resolve(path.dirname(fileURLToPath(import.meta.url)), '../../api');

let child = null;
let baseUrl = null;

// One-time secret for this launch. The API refuses requests without it, so other programs on
// this computer cannot use the local API. It is handed to the renderer over IPC (never written to disk).
const token = randomBytes(32).toString('hex');

// Ask the OS for a free port by binding to port 0, then release it.
function getFreePort() {
  return new Promise((resolve, reject) => {
    const server = net.createServer();
    server.once('error', reject);
    server.listen(0, HOST, () => {
      const { port } = server.address();
      server.close(() => resolve(port));
    });
  });
}

// Pick the Python interpreter: explicit override > api/.venv > system python.
// The venv layout differs per OS (Scripts\python.exe vs bin/python).
function findPython() {
  if (process.env.DEVSCOPE_PYTHON) return process.env.DEVSCOPE_PYTHON;
  const venvPython =
    process.platform === 'win32'
      ? path.join(API_DIR, '.venv', 'Scripts', 'python.exe')
      : path.join(API_DIR, '.venv', 'bin', 'python');
  if (existsSync(venvPython)) return venvPython;
  return process.platform === 'win32' ? 'python' : 'python3';
}

// Poll GET /health until the API answers, or give up after `timeoutMs`.
async function waitForHealth(url, timeoutMs = 60000) {
  const deadline = Date.now() + timeoutMs;
  while (Date.now() < deadline) {
    if (!child) return false; // process died while we were waiting
    try {
      const res = await fetch(`${url}/health`);
      if (res.ok) return true;
    } catch {
      // not listening yet
    }
    await new Promise((r) => setTimeout(r, 250));
  }
  return false;
}

// What to run and where. Packaged app: the frozen executable shipped inside the
// installer (resources/api, built by scripts/build.mjs). Development: uvicorn from
// the Python virtual environment in /api.
function apiCommand(port) {
  if (app.isPackaged) {
    const dir = path.join(process.resourcesPath, 'api');
    const exe = process.platform === 'win32' ? 'devscope-api.exe' : 'devscope-api';
    return { cmd: path.join(dir, exe), args: ['--port', String(port)], cwd: dir };
  }
  return {
    cmd: findPython(),
    args: ['-m', 'uvicorn', 'devscope_api.main:app', '--host', HOST, '--port', String(port)],
    cwd: API_DIR,
  };
}

// Start the API. Never throws: the UI should still open (showing "API offline")
// even if Python is missing or the API cannot start.
export async function startBackend() {
  if (process.env.DEVSCOPE_SKIP_API) return { ok: false, baseUrl: null, reason: 'skipped' };

  const port = await getFreePort();
  const url = `http://${HOST}:${port}`;

  const { cmd, args, cwd } = apiCommand(port);
  child = spawn(
    cmd,
    args,
    {
      cwd,
      stdio: ['ignore', 'pipe', 'pipe'],
      windowsHide: true,
      // The API exits on its own if this process dies without cleaning up (see api/devscope_api/watchdog.py).
      env: { ...process.env, DEVSCOPE_PARENT_PID: String(process.pid), DEVSCOPE_TOKEN: token },
    },
  );
  child.stdout.on('data', (d) => console.log(`[api] ${d}`.trimEnd()));
  child.stderr.on('data', (d) => console.error(`[api] ${d}`.trimEnd()));
  child.on('error', (err) => {
    console.error('[api] failed to start:', err.message);
    child = null;
  });
  child.on('exit', (code) => {
    console.log(`[api] exited with code ${code}`);
    child = null;
  });

  const ok = await waitForHealth(url);
  baseUrl = ok ? url : null;
  return { ok, baseUrl, reason: ok ? null : 'health check failed' };
}

export function getBaseUrl() {
  return baseUrl;
}

export function getToken() {
  return token;
}

// Stop the API. On Windows, kill() terminates the process; elsewhere SIGTERM.
export function stopBackend() {
  if (child) {
    child.kill();
    child = null;
  }
  baseUrl = null;
}
