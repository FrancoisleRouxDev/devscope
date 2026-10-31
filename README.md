# devscope

A cross-platform (Windows, macOS, Linux) desktop app that shows you what is
inside any folder on your computer: what takes the space, what is old, what is
clutter or duplicated, and, for code folders, which projects have unsaved work
or leaked secrets. It runs entirely on your machine and never changes your files.

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
| Packaging (installers) and CI | Not started |

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
│             folder.rs  duplicates.rs  discovery.rs  secrets.rs
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
2. Safe cleanup: move clutter, duplicates and junk folders to the recycle bin, with a preview and confirmation.
3. Scan history and trends.
4. Wider language support and tuning of secret detection.
5. Packaging and CI for all three platforms.
