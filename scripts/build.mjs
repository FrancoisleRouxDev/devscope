// build.mjs - production build pipeline. Run from anywhere: node scripts/build.mjs
//
//   1. Python venv with the build tools (api/.venv-build)
//   2. cargo compiles the Rust scanner and it is installed into that venv
//   3. PyInstaller freezes the API (+ scanner) into api/dist/devscope-api/
//   4. vite builds the Svelte UI
//   5. electron-builder packages Electron + UI + the frozen API into an installer
//
// The installer is written to dashboard/release/ for the OS you run this on.
// Rust and PyInstaller output is native, so build each OS on its own machine
// (the GitHub Actions workflow in .github/workflows/release.yml does that).
//
// Options: --skip-api   reuse an existing api/dist (skips steps 1-3)
//          --dir        only produce an unpacked app folder (faster, for testing)

import { spawnSync } from 'node:child_process';
import { copyFileSync, existsSync, readdirSync, statSync } from 'node:fs';
import path from 'node:path';
import { fileURLToPath } from 'node:url';

const root = path.resolve(path.dirname(fileURLToPath(import.meta.url)), '..');
const api = path.join(root, 'api');
const scanner = path.join(root, 'scanner');
const dashboard = path.join(root, 'dashboard');
const win = process.platform === 'win32';

const flags = new Set(process.argv.slice(2));

const sleep = (ms) => Atomics.wait(new Int32Array(new SharedArrayBuffer(4)), 0, 0, ms);

// Run a command, streaming its output, and stop the whole build if it fails.
// `retries` re-runs a failed command: on some Windows setups antivirus or an indexer briefly
// locks freshly written .exe files ("Access is denied"), which a second attempt gets past.
// `acceptFailure`, if given, is asked after the last attempt whether the failure can be ignored.
function run(cmd, args, cwd, env = {}, retries = 0, acceptFailure = null) {
  console.log(`\n> ${cmd} ${args.join(' ')}   (in ${path.relative(root, cwd) || '.'})`);
  const options = { cwd, stdio: 'inherit', env: { ...process.env, ...env } };
  for (let attempt = 0; ; attempt++) {
    // npm and npx are .cmd files on Windows and need a shell. All arguments here are fixed
    // strings from this script, so passing one command line is safe.
    const r = win && /^(npm|npx)$/.test(cmd)
      ? spawnSync(`${cmd} ${args.join(' ')}`, { ...options, shell: true })
      : spawnSync(cmd, args, options);
    if (r.status === 0) return;
    if (attempt < retries) {
      // The lock can last tens of seconds after a file is written, so wait before trying again.
      console.warn(`\n"${cmd}" failed (exit ${r.status}); retrying in 30s (${attempt + 1}/${retries})...`);
      sleep(30000);
      continue;
    }
    if (acceptFailure?.()) {
      console.warn(`\nWarning: "${cmd}" reported an error, but its output is complete and up to date; continuing.`);
      return;
    }
    console.error(`\nBuild failed: "${cmd} ${args.join(' ')}" exited with ${r.status ?? r.error}`);
    process.exit(r.status || 1);
  }
}

// File names of the compiled Rust library, and what Python needs it to be called.
const [builtLibName, installedLibName] =
  process.platform === 'win32'
    ? ['devscope_scanner.dll', 'devscope_scanner.pyd']
    : process.platform === 'darwin'
      ? ['libdevscope_scanner.dylib', 'devscope_scanner.so']
      : ['libdevscope_scanner.so', 'devscope_scanner.so'];

// Cargo can report "failed to link or copy ... Access is denied" AFTER rustc has finished the
// library, when antivirus locks the file for cargo's final copy into target/release. The library
// in target/release/deps is then complete. This accepts that case only if the library is newer
// than every Rust source file and Cargo manifest, so a real compile error can never be ignored.
function libraryIsUpToDate() {
  const lib = path.join(buildTarget, 'release', 'deps', builtLibName);
  if (!existsSync(lib)) return false;
  const builtAt = statSync(lib).mtimeMs;
  const sources = (dir) =>
    readdirSync(dir, { withFileTypes: true }).flatMap((e) =>
      e.isDirectory() ? sources(path.join(dir, e.name)) : [path.join(dir, e.name)],
    );
  const inputs = [path.join(scanner, 'Cargo.toml'), path.join(scanner, 'Cargo.lock'), ...sources(path.join(scanner, 'src'))];
  return inputs.every((f) => statSync(f).mtimeMs <= builtAt);
}

// A separate venv from the development one (api/.venv), so a running dev session
// that has the scanner module loaded can never block (or be broken by) a build.
const venv = path.join(api, '.venv-build');
// Rust output folder used only by this script. Override with DEVSCOPE_CARGO_TARGET, e.g. to
// reuse an existing compile when antivirus keeps locking freshly written .exe files.
const buildTarget = process.env.DEVSCOPE_CARGO_TARGET ?? path.join(scanner, 'target-build');
const bin = path.join(venv, win ? 'Scripts' : 'bin');
const python = path.join(bin, win ? 'python.exe' : 'python');
const tool = (name) => path.join(bin, win ? `${name}.exe` : name);

// Copy the compiled Rust library into the venv as an importable Python extension module.
function installScanner() {
  const [built, installed] = [builtLibName, installedLibName];

  const site = spawnSync(python, ['-c', 'import sysconfig; print(sysconfig.get_paths()["purelib"])'], {
    encoding: 'utf8',
  }).stdout.trim();

  // Take the file from deps/: cargo's own copy one level up can be blocked on Windows
  // by antivirus or indexers that briefly lock freshly written files.
  const source = path.join(buildTarget, 'release', 'deps', built);
  const target = path.join(site, installed);
  for (let attempt = 1; ; attempt++) {
    try {
      copyFileSync(source, target);
      console.log(`\nInstalled ${path.relative(root, source)} -> ${path.relative(root, target)}`);
      return;
    } catch (err) {
      if (attempt >= 10) {
        console.error(`\nBuild failed: could not copy the scanner library (${err.message})`);
        process.exit(1);
      }
      Atomics.wait(new Int32Array(new SharedArrayBuffer(4)), 0, 0, 2000); // wait 2s, then retry
    }
  }
}

if (!flags.has('--skip-api')) {
  // 1. venv + build tools
  if (!existsSync(python)) run(win ? 'python' : 'python3', ['-m', 'venv', venv], api);
  run(python, ['-m', 'pip', 'install', '--quiet', '-r', 'requirements-build.txt'], api);

  // 2. compile the Rust scanner and install it into the venv.
  // This is what `maturin develop` does, done by hand: build the cdylib with the
  // extension-module feature, then copy it into site-packages under the name Python imports.
  // (Done directly because maturin froze on some Windows setups, see README "Troubleshooting".)
  run(
    'cargo',
    ['build', '--release', '--features', 'pyo3/extension-module'],
    scanner,
    {
      PYO3_PYTHON: python,
      CARGO_TARGET_DIR: buildTarget, // not scanner/target, so a running dev session cannot lock our files
    },
    2,
    libraryIsUpToDate,
  );
  installScanner();

  // 3. freeze the API (scanner module is picked up from the venv)
  run(
    python,
    [
      '-m', 'PyInstaller', 'run.py',
      '--name', 'devscope-api',
      '--onedir', '--noconfirm', '--clean',
      '--specpath', 'build', '--workpath', 'build/work', '--distpath', 'dist',
      '--collect-submodules', 'uvicorn',
      '--hidden-import', 'devscope_scanner',
    ],
    api,
  );
}

// 4 + 5. UI and installer
run('npm', ['install'], dashboard);
run('npm', ['run', 'build'], dashboard);
run('npx', ['electron-builder', '--publish', 'never', ...(flags.has('--dir') ? ['--dir'] : [])], dashboard, {
  // Signing is off unless you provide certificates (see README, "Signing").
  CSC_IDENTITY_AUTO_DISCOVERY: process.env.CSC_LINK ? 'true' : 'false',
});

console.log('\nDone. Installer(s) are in dashboard/release/');
