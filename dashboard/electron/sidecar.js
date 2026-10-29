// @ts-check
// =============================================================================
// electron/sidecar.js — start, health-check and stop the Python API process
// =============================================================================
// The FastAPI service runs as a child process ("sidecar") of Electron.
//
//   DEV   → spawn the API's virtualenv Python:  <venv-python> -m devscope_api
//   PROD  → spawn the frozen binary shipped in extraResources:
//           <resourcesPath>/sidecar/devscope-api[.exe]   (see electron-builder.yml)
//
// Handshake (mirrors api/devscope_api/__main__.py):
//   1. Generate a random token (crypto.randomBytes) → env DEVSCOPE_TOKEN.
//   2. Pass `--port 0 --parent-pid <our pid>`.
//   3. Read stdout until "DEVSCOPE_READY port=NNNNN"; time out after ~15 s.
//   4. Confirm with GET /health; then expose { baseUrl, token } to the renderer.
//
// CROSS-PLATFORM GOTCHAS handled here
//   * venv layout differs:  Windows  api/.venv/Scripts/python.exe
//                           POSIX    api/.venv/bin/python
//   * Executable suffix:    ".exe" only on Windows.
//   * Killing:  child.kill() sends SIGTERM on POSIX, which uvicorn handles
//     gracefully. On Windows there are no signals — kill() terminates only
//     the direct process, so use `taskkill /pid <pid> /T /F` to take down the
//     whole tree.
//   * Use spawn() with an args ARRAY and `shell: false` — never build a shell
//     command string (quoting rules differ per OS, and it's an injection risk).
//   * `windowsHide: true` so no console window flashes up on Windows.
// =============================================================================

// =============================================================================
// LOGIC SKETCH (commented out — not implemented yet)
// =============================================================================
//
// import { spawn, execFile } from 'node:child_process';
// import { randomBytes } from 'node:crypto';
// import path from 'node:path';
// import readline from 'node:readline';
// import { app } from 'electron';
//
// const IS_WIN = process.platform === 'win32';
//
// /**
//  * @typedef {{ baseUrl: string, token: string }} ApiConnection
//  * @typedef {{ connection: ApiConnection, stop: () => void }} Sidecar
//  */
//
// /** @returns {{ command: string, args: string[] }} */
// function resolveCommand(dev) {
//   if (dev) {
//     const venv = path.resolve(app.getAppPath(), '..', 'api', '.venv');
//     const python = IS_WIN ? path.join(venv, 'Scripts', 'python.exe') : path.join(venv, 'bin', 'python');
//     return { command: python, args: ['-m', 'devscope_api'] };
//   }
//   const bin = path.join(process.resourcesPath, 'sidecar', IS_WIN ? 'devscope-api.exe' : 'devscope-api');
//   return { command: bin, args: [] };
// }
//
// /** @returns {Promise<Sidecar>} */
// export async function startSidecar({ dev }) {
//   const token = randomBytes(32).toString('hex');
//   const { command, args } = resolveCommand(dev);
//   const child = spawn(command, [...args, '--port', '0', '--parent-pid', String(process.pid)], {
//     env: { ...process.env, DEVSCOPE_TOKEN: token },
//     stdio: ['ignore', 'pipe', 'pipe'],
//     windowsHide: true,
//   });
//
//   const port = await waitForReadyLine(child, 15_000);   // rejects on timeout / early exit
//   const baseUrl = `http://127.0.0.1:${port}`;
//   await waitForHealth(baseUrl);
//
//   return {
//     connection: { baseUrl, token },
//     stop() {
//       if (child.exitCode !== null) return;
//       if (IS_WIN) execFile('taskkill', ['/pid', String(child.pid), '/T', '/F']);
//       else child.kill('SIGTERM');
//     },
//   };
// }
//
// // Also forward the child's stderr to Electron's log so Python tracebacks
// // aren't lost, and surface a friendly error window if the API can't start.
