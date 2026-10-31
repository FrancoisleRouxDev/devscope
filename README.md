# devscope

A cross-platform (Windows, macOS, Linux) desktop app that gives you an overview
of every coding project in your workspace: how big each one is, how much of that
is rebuildable junk, whether work is at risk of being lost, and whether secrets
have been left in the code. It runs entirely on your machine.

## What it is and what it's for

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

1. Start the app and click **Choose workspace…**, then pick the folder that holds your projects.
2. devscope finds every project inside it (a folder with `.git`, `package.json`,
   `Cargo.toml`, `pyproject.toml`, ...) and lists size, reclaimable space, last
   activity and git state.
3. Click a project for its language breakdown, largest files and secrets check.

## Progress

| Area | Status |
|------|--------|
| Rust scanner: language/line stats, project discovery, junk sizing, secrets check | Working; unit-tested |
| FastAPI service: `/health`, `/scan`, `/workspace`, `/secrets`, git status | Working; tested (pytest) and run against real folders |
| Electron shell: window, folder dialog, API process management, parent-exit watchdog | Working |
| Svelte dashboard: workspace overview, project list, details, secrets panel | Written and builds; the folder-dialog flow has not been clicked through by hand yet |
| Dependency vulnerability check | Not started |
| One-click cleanup of junk folders (with dry run) | Not started |
| Scan history and trends | Not started |
| Packaging (installers) and CI | Not started |

**Known limitations**
- The language table covers Rust, Python, JS, TS, Svelte, HTML, CSS, TOML and
  Markdown. Other languages are not counted in line stats yet.
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
                    │ /health /workspace  │
                    │ /scan   /secrets    │
                    │ + git status        │
                    └──────────┬──────────┘
                               │ PyO3 (in-process call)
                    ┌──────────▼──────────┐
                    │ Scanner (scanner/)  │  Rust
                    │ discover, measure,  │
                    │ count lines, secrets│
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
| `POST /workspace` | `{ root, max_depth? }` | Projects: `path, name, kinds, size_bytes, junk_bytes, junk_dirs, last_modified, git` |
| `POST /scan` | `{ path }` | `root, total_files, total_lines, code, comments, blanks, duration_ms, languages[], largest_files[]` |
| `POST /secrets` | `{ path }` | Findings: `path, line, kind` (never the secret value) |

`git` is `{ branch, uncommitted, ahead, has_remote }`, or `null` for a folder
that is not a git repository. The same shapes are defined in
`scanner/src/types.rs`, `api/devscope_api/schemas.py` and used by
`dashboard/src/lib/api.js`. Keep them in sync.

### Data flow

1. User picks a workspace folder (Electron native dialog via the preload bridge).
2. Svelte calls `POST /workspace`; FastAPI calls `devscope_scanner.discover_projects`
   (Rust) and adds git status per project.
3. Clicking a project calls `POST /scan` and `POST /secrets` in parallel.
4. The UI renders the results.

### Process lifecycle

- Electron main picks a free port and spawns the API on `127.0.0.1:<port>`,
  passing its own process id in `DEVSCOPE_PARENT_PID`.
- It polls `/health` until ready; if the API cannot start, the UI still opens and shows "API offline".
- On quit, Electron terminates the API. If Electron crashes or is force-killed, the
  API notices (watchdog) and exits within a few seconds.
- The API never listens on a public interface, and CORS only allows the app's own origin.

### Cross-platform notes

- Paths use `pathlib` (Python), `std::path` (Rust) and `path` (Node); never string concatenation.
- Python venv location (`Scripts\python.exe` vs `bin/python`) is handled in `backend.js`.
- The Rust extension is built per-OS with maturin; the Python API is frozen (e.g. PyInstaller) for packaged builds.
- `electron-builder` is planned to produce NSIS/MSI (Windows), DMG (macOS), AppImage/deb (Linux).
- Developed and tested on Windows so far; macOS and Linux are untested.

## Repository layout

```
devscope/
├── scanner/                 Rust crate (PyO3 extension)
│   ├── Cargo.toml
│   ├── pyproject.toml       maturin config
│   └── src/  lib.rs  walker.rs  counter.rs  languages.rs  types.rs
│             discovery.rs  secrets.rs
├── api/                     FastAPI service
│   ├── pyproject.toml  requirements.txt
│   ├── devscope_api/  main.py  config.py  schemas.py  scanner_bridge.py
│   │                  watchdog.py  routes/ (health.py, scan.py)
│   └── tests/
├── dashboard/               Electron + Svelte + Vite
│   ├── package.json  vite.config.js  svelte.config.js  index.html
│   ├── electron/  main.js  preload.cjs  backend.js
│   └── src/  main.js  App.svelte  lib/  components/  styles/
├── scripts/                 dev.mjs, build.mjs (placeholders)
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

## Roadmap

1. Dependency vulnerability check across all projects (reads `package.json`, `Cargo.lock`, `requirements.txt`).
2. One-click cleanup of junk folders, with a dry-run preview and confirmation.
3. Scan history and trends.
4. Wider language support and tuning of secret detection.
5. Packaging and CI for all three platforms.
