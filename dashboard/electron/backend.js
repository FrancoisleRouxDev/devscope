// backend.js - manages the Python API child process.
// Finds a free port, spawns uvicorn (dev: venv Python; prod: bundled
// executable), waits for GET /health, and kills the process on app quit.
// Handles platform differences (Windows/macOS/Linux paths and signals).

import { spawn } from 'node:child_process';
import { existsSync } from 'node:fs';
import net from 'node:net';
import path from 'node:path';
import { fileURLToPath } from 'node:url';

const HOST = '127.0.0.1';
const API_DIR = path.resolve(path.dirname(fileURLToPath(import.meta.url)), '../../api');

let child = null;
let baseUrl = null;

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
async function waitForHealth(url, timeoutMs = 15000) {
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

// Start the API. Never throws: the UI should still open (showing "API offline")
// even if Python is missing or the API is not written yet.
export async function startBackend() {
  if (process.env.DEVSCOPE_SKIP_API) return { ok: false, baseUrl: null, reason: 'skipped' };

  const port = await getFreePort();
  const url = `http://${HOST}:${port}`;

  child = spawn(
    findPython(),
    ['-m', 'uvicorn', 'devscope_api.main:app', '--host', HOST, '--port', String(port)],
    {
      cwd: API_DIR,
      stdio: ['ignore', 'pipe', 'pipe'],
      windowsHide: true,
      // The API exits on its own if this process dies without cleaning up (see api/devscope_api/watchdog.py).
      env: { ...process.env, DEVSCOPE_PARENT_PID: String(process.pid) },
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

// Stop the API. On Windows, kill() terminates the process; elsewhere SIGTERM.
export function stopBackend() {
  if (child) {
    child.kill();
    child = null;
  }
  baseUrl = null;
}
