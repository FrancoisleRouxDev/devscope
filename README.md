# devscope

A cross-platform (Windows, macOS, Linux) desktop app that shows you what is
inside any folder on your computer: what takes the space, what is old, what is
clutter or duplicated, and, for code folders, which projects have unsaved work
or leaked secrets. It runs entirely on your machine and never changes your files.

## Download

Get the installer from the **[Releases page](https://github.com/FrancoisleRouxDev/devscope/releases/latest)**.

> **Early preview.** devscope is new and the installers are **not yet code-signed**, so your
> computer will warn you about an unknown publisher. That warning means "this publisher is not
> verified", not "this file is malware". You can check the file really came from this repository
> (see below) before you run it.

- **Windows 10/11 (tested):** download `devscope Setup <version>.exe` and run it. If SmartScreen says
  "Windows protected your PC", click **More info**, then **Run anyway**. It installs for your user
  only and does not need administrator rights.
- **macOS and Linux:** not available yet. The code is cross-platform and installers are configured,
  but they have not been built or tested, so they are not published. They will follow.

**Check your download.** Every installer is built by GitHub Actions from this source and has a signed
build-provenance record. With the [GitHub CLI](https://cli.github.com/):

```
gh attestation verify "devscope Setup 0.1.0.exe" --repo FrancoisleRouxDev/devscope
```

If that prints a success message, the file is the one this repository built. devscope works fully
offline; it only contacts `github.com` to look for a newer version. To report a security problem see
[SECURITY.md](SECURITY.md).

## What it is and what it's for

Point devscope at any folder (Downloads, Documents, Desktop, a whole drive) and
it gives you a **folder overview**:

- **What's taking space**, broken down by file type (video, images, documents, archives, installers, code, ...).
- **How old it is**: how much sits untouched for months or years.
- **Biggest files and subfolders**, with a button to show each one in your file manager.
- **Likely clutter**: installers you have probably already used and repeat downloads like `file (1).pdf`
  (files inside `node_modules`, virtual envs and other tooling folders are ignored).
- **Duplicate files**: identical content, confirmed byte for byte (on demand, since it reads file contents).

If the folder contains code projects, devscope also opens a **developer view**.
Over the years a developer's workspace fills up with dozens of project folders,
and nobody keeps track of them. That causes three real problems devscope tackles:

1. **Lost work.** Uncommitted changes and unpushed commits sit in forgotten
   folders, and a dead laptop or wiped drive takes them with it. devscope shows
   which projects have unsaved work, no remote at all, or unpushed commits.
2. **Wasted disk space.** `node_modules`, Rust `target/`, Python `venv` and
   similar rebuildable folders pile up, often tens of GB. devscope shows how
   much each project could give back.
3. **Leaked secrets.** API keys, private keys and `.env` files get left in old
   projects. devscope flags files and well-known token formats (AWS, GitHub,
   Slack, PEM keys). It reports where they are and never returns the secret.

It also keeps the original per-project view: files and lines of code by language
(code vs. comments vs. blank), and the largest files.

Everything is computed locally by a fast Rust core. Nothing leaves your machine.

## Usage

1. Start the app and click **Downloads**, **Documents** or **Desktop**, or **Choose folder…** for anything else.
2. The **Folder overview** tab shows size by type, file ages, biggest files and
   folders, and likely clutter. Click **Find duplicates** to search for identical files.
3. If the folder holds two or more code projects (folders with `.git`, `package.json`,
   `Cargo.toml`, `pyproject.toml`, ...), the **Code projects** tab opens: size,
   reclaimable space, last activity and git state for each.
4. Click a project for its language breakdown, largest source files and secrets check.

## Progress

| Area | Status |
|------|--------|
| Rust scanner: folder analysis, duplicates, language/line stats, project discovery, junk sizing, secrets check | Working; unit-tested |
| FastAPI service: `/health`, `/folder`, `/duplicates`, `/workspace`, `/scan`, `/secrets`, git status | Working; tested (pytest) and run against real folders |
| Electron shell: window, folder dialog, quick-access folders, show-in-file-manager, API process management, parent-exit watchdog | Working |
| Svelte dashboard: folder overview, duplicates, code projects view, details, secrets panel | Working; checked by running the real app against real folders and screenshotting each view. The native folder dialog itself has not been clicked through by hand |
| Dependency vulnerability check | Not started |
| One-click cleanup of junk folders (with dry run) | Not started |
| Scan history and trends | Not started |
| Packaging: frozen API, Electron bundle, Windows installer | Built and tested on Windows: the installed app starts its own engine and analyses folders. The installer itself was built but not run through its install wizard |
| macOS and Linux installers, GitHub Actions workflows (`ci.yml`, `release.yml`), auto-update, code signing | Written, not run yet (they need GitHub's macOS/Linux machines and your signing certificates) |

**Known limitations**
- The language table covers Rust, Python, JS, TS, Svelte, HTML, CSS, TOML and
  Markdown. Other languages are not counted in line stats yet (file-type totals
  in the folder overview cover all files).
- File categories come from the extension only, and "clutter" is a heuristic
  (installers and `name (1).ext` copies), so review before deleting anything.
  devscope has no delete action yet.
- Secret detection is heuristic (known token formats plus risky file names) and
  can produce false positives or miss custom formats.
- Scanning is read-only; devscope never modifies your projects.

## Architecture

```
┌──────────────────────────── Electron ────────────────────────────┐
│  Main process (dashboard/electron)                               │
│   • creates window, native folder dialog                         │
│   • spawns & supervises the Python API (backend.js)              │
│            │ preload bridge (contextBridge)                      │
│  Renderer: Svelte app (dashboard/src, built by Vite)             │
└──────────────────────────────┬───────────────────────────────────┘
                               │ HTTP, 127.0.0.1 only
                    ┌──────────▼──────────┐
                    │ FastAPI (api/)      │  Python
                    │ /health  /folder    │
                    │ /duplicates         │
                    │ /workspace /scan    │
                    │ /secrets + git info │
                    └──────────┬──────────┘
                               │ PyO3 (in-process call)
                    ┌──────────▼──────────┐
                    │ Scanner (scanner/)  │  Rust
                    │ folder analysis,    │
                    │ duplicates, project │
                    │ discovery, lines,   │
                    │ secrets             │
                    └─────────────────────┘
```

### Layers

| Layer | Dir | Tech | Responsibility |
|-------|-----|------|----------------|
| Scanner | `scanner/` | Rust, PyO3, maturin | Parallel directory walks: project discovery, size and junk measurement, per-language line counts, secrets heuristics. Compiled to the Python module `devscope_scanner`. |
| API | `api/` | Python, FastAPI, uvicorn | Imports the Rust module, adds git status (via the `git` CLI), validates input/output with Pydantic, exits if Electron disappears. |
| Desktop shell | `dashboard/electron/` | Electron | Window lifecycle, native dialogs, starts/stops the API, secure preload bridge. |
| UI | `dashboard/src/` | Svelte 5, Vite | Dark-mode dashboard: workspace overview, project list, stats, secrets panel. |

### API

| Endpoint | Body | Returns |
|----------|------|---------|
| `GET /health` | | `{ "status": "ok" }` |
| `POST /folder` | `{ path }` | Folder overview: `total_files, total_bytes, project_count, categories[], largest_files[], largest_dirs[], age_buckets[], clutter[]` |
| `POST /duplicates` | `{ path }` | Groups of identical files: `size, paths[], wasted` (slower; run on demand) |
| `POST /workspace` | `{ root, max_depth? }` | Projects: `path, name, kinds, size_bytes, junk_bytes, junk_dirs, last_modified, git` |
| `POST /scan` | `{ path }` | `root, total_files, total_lines, code, comments, blanks, duration_ms, languages[], largest_files[]` |
| `POST /secrets` | `{ path }` | Findings: `path, line, kind` (never the secret value) |

`git` is `{ branch, uncommitted, ahead, has_remote }`, or `null` for a folder
that is not a git repository. The same shapes are defined in
`scanner/src/types.rs`, `api/devscope_api/schemas.py` and used by
`dashboard/src/lib/api.js`. Keep them in sync.

### Data flow

1. User picks a folder (Electron native dialog via the preload bridge) or a quick-access folder.
2. Svelte calls `POST /folder`; FastAPI calls `devscope_scanner.analyze_folder` (Rust),
   which walks the folder in parallel and returns the overview, plus a count of code projects inside.
3. If there are two or more projects, Svelte also calls `POST /workspace`
   (`discover_projects` in Rust, plus git status per project from the `git` CLI).
4. Clicking a project calls `POST /scan` and `POST /secrets` in parallel.
5. **Find duplicates** calls `POST /duplicates` (group by size, hash candidates, confirm byte for byte).
6. The UI renders the results.

### Process lifecycle

- Electron main picks a free port and spawns the API on `127.0.0.1:<port>`
  (uvicorn from the venv in development, the bundled `devscope-api` executable in the
  installed app), passing its own process id in `DEVSCOPE_PARENT_PID`.
- It polls `/health` until ready; if the API cannot start, the UI still opens and shows "API offline".
- On quit, Electron terminates the API. If Electron crashes or is force-killed, the
  API notices (watchdog) and exits within a few seconds.
- The API never listens on a public interface, and CORS only allows the app's own origin.

### Cross-platform notes

- Paths use `pathlib` (Python), `std::path` (Rust) and `path` (Node); never string concatenation.
- Python venv location (`Scripts\python.exe` vs `bin/python`) is handled in `backend.js`.
- The Rust extension is built per-OS with maturin; the Python API is frozen with PyInstaller for packaged builds.
- `electron-builder` produces an NSIS installer (Windows), DMG (macOS), AppImage and deb (Linux).
- Developed and tested on Windows so far; macOS and Linux are untested.

## Repository layout

```
devscope/
├── scanner/                 Rust crate (PyO3 extension)
│   ├── Cargo.toml
│   ├── pyproject.toml       maturin config
│   └── src/  lib.rs  walker.rs  counter.rs  languages.rs  types.rs
│             folder.rs  duplicates.rs  discovery.rs  secrets.rs
├── api/                     FastAPI service
│   ├── pyproject.toml  requirements.txt  requirements-build.txt
│   ├── run.py               entry point of the packaged (frozen) API
│   ├── devscope_api/  main.py  config.py  schemas.py  scanner_bridge.py
│   │                  watchdog.py  routes/ (health.py, scan.py)
│   └── tests/
├── dashboard/               Electron + Svelte + Vite
│   ├── package.json  vite.config.js  svelte.config.js  index.html
│   ├── electron/  main.js  preload.cjs  backend.js
│   └── src/  main.js  App.svelte  lib/  components/  styles/
├── scripts/                 build.mjs (installer pipeline), dev.mjs (placeholder)
├── .github/workflows/       ci.yml (tests), release.yml (installers for 3 OSes)
├── .gitignore
└── README.md
```

## Getting started

Prerequisites: Rust (stable), Python 3.10+, Node.js 20+, and `git` on your PATH.

```
# 1. Python environment and the Rust extension
cd api
python -m venv .venv
.venv/Scripts/activate            # macOS/Linux: source .venv/bin/activate
pip install -r requirements.txt pytest httpx maturin
cd ../scanner && maturin develop --release

# 2. Run the app (Electron starts the API for you)
cd ../dashboard
npm install
npm run dev
```

Tests: `cargo test` in `scanner/`, `python -m pytest` in `api/`.

## Security and privacy

**What it does with your data.** devscope reads file names, sizes and dates, and (for the
duplicate and secrets checks) file contents, all on your own computer. It never uploads
anything, has no telemetry and never modifies or deletes your files. It works fully offline;
the only network request the installed app makes is a check for a newer version on
`github.com` at start-up, and that is skipped silently when there is no connection.

**How it is protected**
- The window is sandboxed with context isolation, no Node access and a Content-Security-Policy
  that only allows the app's own script and connections to `127.0.0.1`. It cannot open pop-ups
  or navigate away, and every browser permission (camera, location, ...) is denied. Electron
  features attackers abuse (`runAsNode`, `NODE_OPTIONS`, `--inspect`, loading code from outside
  the app archive) are switched off.
- The engine listens on `127.0.0.1` only, on a random port, and requires a random one-time token
  that exists only in memory, so other programs on the computer cannot use it. It also exits by
  itself if the app dies.
- **Hostile folders.** Analysing a downloaded folder must never run code from it. Git runs programs
  named in a repository's own config, so devscope switches off `core.fsmonitor`, never lets git write
  to a repository, and does not run git at all on repositories whose config defines filters or
  includes other files (they are shown as "git not checked"). This is covered by tests.
- Secret findings contain only the file, line and kind, never the secret itself.
- Dependencies that ship in the app have no known vulnerabilities (`npm audit --omit=dev`).

**Verifying a download.** Installers come only from this repository's GitHub Releases.
Each release is built by GitHub Actions and carries a signed build-provenance attestation, so you can
check that a file really was built from this source:

```
gh attestation verify "devscope Setup 0.1.0.exe" --repo FrancoisleRouxDev/devscope
```

Until the installers are code-signed (see "Signing" below) Windows SmartScreen and macOS Gatekeeper
will still warn about an unknown publisher.

## Building an installer

The installed app is fully self-contained: Python, Rust and Node are **not** needed
on the user's machine. The Python API is frozen into an executable
(`devscope-api`) with PyInstaller, bundled inside the Electron app, and started on
a free local port exactly like in development. Nothing is hosted in the cloud:
devscope reads your local disk, so the backend has to run on your computer.

```
node scripts/build.mjs          # builds the installer for the OS you are on
node scripts/build.mjs --dir    # unpacked app folder only (faster, for testing)
```

The script creates `api/.venv-build`, compiles the Rust scanner into it
(`maturin`), freezes the API (`PyInstaller`, output `api/dist/devscope-api/`),
builds the UI (`vite`) and packages everything (`electron-builder`). The result is in
`dashboard/release/`: an NSIS installer (`.exe`) on Windows, a `.dmg` on macOS, an
`.AppImage` and `.deb` on Linux. Prerequisites are the same as for development.

Because the Rust module and the frozen API are native, **each OS must be built on that
OS**. The GitHub Actions workflow `.github/workflows/release.yml` builds the Windows installer:
push a tag such as `v0.1.0` and it builds it and attaches it to a GitHub Release (macOS and Linux
are configured but switched off until they have been tested; the workflow file explains how to
re-enable them). `.github/workflows/ci.yml` runs the tests on every push.

**Updates.** Installed apps check the project's GitHub Releases on start-up
(`electron-updater`) and offer a newer version when there is one. If the check fails
(offline, no release yet, private repository) it is silently skipped.

**Signing.** Without signing, Windows SmartScreen and macOS Gatekeeper warn users about
the app, and macOS may refuse to open it. To sign:
- Windows: buy a code-signing certificate and add the repository secrets `CSC_LINK`
  (the `.pfx`, base64 or a URL) and `CSC_KEY_PASSWORD`.
- macOS: join the Apple Developer Program, add a "Developer ID Application" certificate
  with the same two secrets, then enable notarization in the `mac` section of
  `dashboard/package.json` and add `APPLE_ID`, `APPLE_APP_SPECIFIC_PASSWORD` and `APPLE_TEAM_ID`.
- Linux needs no signing.

**Troubleshooting the build.**
- The scanner is compiled with plain `cargo` and copied into the build venv, not with
  `maturin develop`. On one Windows machine `maturin` froze while compiling and its
  executable disappeared from the venv; `cargo` was unaffected. The build also uses its
  own folders (`api/.venv-build`, `scanner/target-build`) so a running `npm run dev`
  session, which keeps the scanner file open, can never block it.
- If a build step fails with "Access is denied" on a freshly written file, antivirus or an
  indexer is briefly locking it. Re-run the build; the copy step already retries.

## Roadmap

1. Dependency vulnerability check across all projects (reads `package.json`, `Cargo.lock`, `requirements.txt`).
2. Safe cleanup: move clutter, duplicates and junk folders to the recycle bin, with a preview and confirmation.
3. Scan history and trends.
4. Wider language support and tuning of secret detection.
5. Run and verify the macOS and Linux builds, then set up code signing.
