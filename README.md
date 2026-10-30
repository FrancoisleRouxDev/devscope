# devscope

A cross-platform (Windows, macOS, Linux) desktop app that scans coding projects
and shows their stats (files, lines, languages) in a dark-mode dashboard.

## What it is and what it's for

devscope answers "what's actually in this codebase?" at a glance. Point it at
any project folder and it reports how many files and lines of code it contains,
broken down by language (code vs. comments vs. blank lines), plus the largest
files. It is meant for developers who want a quick, good-looking overview of a
project's size and makeup, for example when joining a new codebase, comparing
projects, or tracking growth. Scanning is done by a fast Rust core and runs
entirely locally; nothing leaves your machine.

## Progress

| Area | Status |
|------|--------|
| Folder structure for all four components | Done |
| Placeholder files documenting each file's purpose | Done |
| Architecture docs and data contract (this README) | Done |
| `.gitignore` for Rust, Python, Node, Electron/Svelte | Done |
| Rust scanner (`scanner/`) | Not started |
| FastAPI service (`api/`) | Not started |
| Electron shell (`dashboard/electron/`) | Not started |
| Svelte dashboard UI (`dashboard/src/`) | Not started |
| Packaging and CI | Not started |

**Current stage:** project skeleton only. Files contain comments describing
their purpose; no logic is implemented yet.

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
                    │  /health  /scan     │
                    └──────────┬──────────┘
                               │ PyO3 (in-process call)
                    ┌──────────▼──────────┐
                    │ Scanner (scanner/)  │  Rust
                    │ walk + count lines  │
                    └─────────────────────┘
```

### Layers

| Layer | Dir | Tech | Responsibility |
|-------|-----|------|----------------|
| Scanner | `scanner/` | Rust, PyO3, maturin | Parallel, gitignore-aware directory walk; per-language file/line/comment/blank counts. Compiled to a Python module `devscope_scanner`. |
| API | `api/` | Python, FastAPI, uvicorn | Imports the Rust module, exposes `/health` and `/scan`, validates input/output with Pydantic, runs scans off the event loop. |
| Desktop shell | `dashboard/electron/` | Electron | Window lifecycle, native dialogs, starts/stops the API, secure preload bridge. |
| UI | `dashboard/src/` | Svelte, Vite | Dark-mode dashboard: project picker, stat cards, language chart, file table. |

### Data flow

1. User picks a folder in the UI (Electron native dialog via preload).
2. Svelte calls `POST /scan` on the local API.
3. FastAPI calls `devscope_scanner.scan_project(path, options)` (Rust).
4. Rust walks the tree in parallel and returns aggregated stats.
5. API serialises the result (Pydantic) and the UI renders it.

### Data contract (planned)

`ScanResult`: `{ root, total_files, total_lines, code, comments, blanks, duration_ms, languages: [{ name, files, code, comments, blanks }], largest_files: [{ path, language, lines }] }`

The same shape is defined in `scanner/src/types.rs`, `api/devscope_api/schemas.py`
and `dashboard/src/lib/api.js`. Keep them in sync.

### Process lifecycle

- Electron main picks a free port and spawns the API on `127.0.0.1:<port>`.
- It polls `/health` until ready, then shows the window with the API URL.
- On quit, Electron terminates the API process.
- The API never listens on a public interface.

### Cross-platform notes

- Paths use `pathlib` (Python), `std::path` (Rust) and `path` (Node); never string concatenation.
- The Rust extension is built per-OS with maturin; the Python API is frozen (e.g. PyInstaller) for packaged builds.
- `electron-builder` produces NSIS/MSI (Windows), DMG (macOS), AppImage/deb (Linux).

## Repository layout

```
devscope/
├── scanner/                 Rust crate (PyO3 extension)
│   ├── Cargo.toml
│   ├── pyproject.toml       maturin config
│   └── src/  lib.rs  walker.rs  counter.rs  languages.rs  types.rs
├── api/                     FastAPI service
│   ├── pyproject.toml  requirements.txt
│   ├── devscope_api/  main.py  config.py  schemas.py  scanner_bridge.py  routes/
│   └── tests/
├── dashboard/               Electron + Svelte + Vite
│   ├── package.json  vite.config.js  svelte.config.js  index.html
│   ├── electron/  main.js  preload.js  backend.js
│   └── src/  main.js  App.svelte  lib/  components/  styles/
├── scripts/                 dev.mjs, build.mjs
├── .gitignore
└── README.md
```

## Prerequisites (planned)

- Rust (stable) and `maturin`
- Python 3.10+
- Node.js 20+

## Development (planned)

```
node scripts/dev.mjs      # build scanner, start Vite + Electron (+ API)
node scripts/build.mjs    # production installers for the current OS
```

## Roadmap

1. Implement the Rust scanner and expose it through PyO3.
2. Implement `/health` and `/scan` in FastAPI.
3. Build the Electron shell and API process management.
4. Build the Svelte dashboard UI and theme.
5. Packaging and CI for all three platforms.
