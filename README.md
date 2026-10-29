# devscope

A cross-platform desktop app (Windows · macOS · Linux) that scans coding projects
and shows their stats in a dark-mode dashboard: files, lines of code, comments,
blanks, a per-language breakdown, and the largest files.

> **Status: scaffold.** The folder structure, build configuration and contracts
> are in place. Source files are documented placeholders. Each one explains its
> purpose, contains a **commented-out logic sketch**, and (for Rust and Python) a
> **JUNIOR vs SENIOR** comparison. See [docs/JUNIOR_VS_SENIOR.md](docs/JUNIOR_VS_SENIOR.md).

---

## Table of contents

1. [Architecture at a glance](#architecture-at-a-glance)
2. [Why this stack](#why-this-stack)
3. [Repository layout](#repository-layout)
4. [Component deep-dive](#component-deep-dive)
5. [Data contract](#data-contract)
6. [Runtime: how the pieces start and talk](#runtime-how-the-pieces-start-and-talk)
7. [Security model](#security-model)
8. [Cross-platform considerations](#cross-platform-considerations)
9. [Development setup](#development-setup)
10. [Building and packaging](#building-and-packaging)
11. [Testing and quality](#testing-and-quality)
12. [Roadmap](#roadmap)

---

## Architecture at a glance

```text
┌──────────────────────────────────────────────────────────────┐
│ RENDERER · dashboard/src · Svelte 5 + Vite + TypeScript      │
│ dark-mode dashboard UI · sandboxed · no Node.js access       │
└───────┬───────────────────────────────────────────┬──────────┘
        │ IPC: window.devscope.*  (preload.cjs)     │ HTTP + Bearer token
        │ pickFolder(), getApiConnection()          │ 127.0.0.1:<random port>
        ▼                                           │
┌─────────────────────────────────────────┐         │
│ MAIN · dashboard/electron · Node.js     │         │
│ window, lifecycle, native dialogs       │         │
│ spawns + supervises the API sidecar     │         │
└───────┬─────────────────────────────────┘         │
        │ spawn (--port 0, token in env)            │
        ▼                                           ▼
┌──────────────────────────────────────────────────────────────┐
│ API SIDECAR · api/ · Python + FastAPI + uvicorn              │
│ routes/ → services/scan_jobs.py → scanner_bridge.py          │
└───────────────────────────────┬──────────────────────────────┘
                                │ PyO3: same process, GIL released
                                ▼
┌──────────────────────────────────────────────────────────────┐
│ SCANNER · scanner/ · Rust extension module (devscope_scanner)│
│ walker → language → counter → stats   (parallel via rayon)   │
└──────────────────────────────────────────────────────────────┘
```

**One sentence per layer:**

| Layer      | Folder        | Tech                          | Job                                                    |
|------------|---------------|-------------------------------|--------------------------------------------------------|
| Scanner    | `/scanner`    | Rust, `ignore`, `rayon`, PyO3 | Walk a directory and count lines, as fast as the disk allows |
| API        | `/api`        | Python, FastAPI, Pydantic     | Validate requests, run scans as jobs, serve JSON on localhost |
| Shell      | `/dashboard/electron` | Electron                | Native window and dialogs, start/stop the API, security boundary |
| UI         | `/dashboard/src` | Svelte 5 + Vite + TypeScript | Render the dashboard                                  |

---

## Why this stack

- **Rust for scanning.** Counting lines across 100k+ files is CPU- and
  syscall-heavy. Rust gives native speed, safe multithreading (`rayon`), and
  ripgrep's directory walker (`ignore`). PyO3 lets Python call it in-process,
  with no subprocess overhead and no JSON serialisation.
- **Python + FastAPI in the middle.** A typed, self-documenting HTTP API
  (`/docs`) that's quick to extend. Future features like git history, churn or
  dependency analysis can use Python's ecosystem without touching Rust.
- **Electron.** One codebase for three desktop OSes, with mature packaging
  (`electron-builder`), auto-update and native dialogs.
- **Svelte 5 + Vite.** Small bundles, fast HMR, and runes (`$state`,
  `$derived`) that keep dashboard state simple. No SvelteKit, because a
  single-window desktop app needs neither SSR nor file-based routing.

**Trade-off we accept:** three runtimes (Rust, Python, Node) means a more
involved build. The sidecar pattern and `scripts/build_sidecar.py` contain
that cost to packaging time, and users install one normal app.

---

## Repository layout

```text
devscope/
├── README.md                     ← you are here: full architecture
├── LICENSE
├── .gitignore                    Rust · Python · Node · Electron/Svelte · OS/editor
├── .gitattributes                LF line endings everywhere (matters for Windows)
├── .editorconfig                 shared indentation/whitespace rules
│
├── docs/
│   └── JUNIOR_VS_SENIOR.md       how juniors vs seniors write the Rust & Python here
│
├── scanner/                      ── RUST CORE ──────────────────────────────────────
│   ├── Cargo.toml                crate manifest; `python` feature gates PyO3
│   ├── pyproject.toml            maturin config → Python wheel `devscope-scanner`
│   ├── devscope_scanner.pyi      Python type stubs = Rust⇄Python contract
│   ├── README.md
│   ├── src/
│   │   ├── lib.rs                crate root, module map, public re-exports
│   │   ├── error.rs              ScanError enum + Result alias
│   │   ├── language.rs           Language enum, detection, comment syntax
│   │   ├── counter.rs            classify one file's lines: code/comment/blank
│   │   ├── walker.rs             parallel, .gitignore-aware directory walk
│   │   ├── stats.rs              output types + lock-free aggregation
│   │   ├── scan.rs               orchestrator: scan(root, &ScanOptions)
│   │   ├── python.rs             PyO3 bindings (only with --features python)
│   │   └── bin/devscope-scan.rs  debug/benchmark CLI
│   └── tests/
│       ├── scan_integration.rs   end-to-end tests of the public API
│       └── fixtures/             tiny sample projects with known counts
│
├── api/                          ── PYTHON SERVICE ─────────────────────────────────
│   ├── pyproject.toml            deps + ruff/mypy/pytest config
│   ├── README.md
│   ├── devscope_api/
│   │   ├── __init__.py           package overview + layering rule
│   │   ├── __main__.py           entry point; port/token handshake with Electron
│   │   ├── app.py                create_app() factory
│   │   ├── config.py             Settings from DEVSCOPE_* env vars
│   │   ├── security.py           per-launch bearer token check
│   │   ├── schemas.py            Pydantic models = HTTP contract
│   │   ├── scanner_bridge.py     ONLY module importing the Rust extension
│   │   ├── services/
│   │   │   └── scan_jobs.py      background scan jobs + bounded LRU cache
│   │   └── routes/
│   │       ├── health.py         GET /health
│   │       └── scans.py          POST /scans · GET /scans · GET /scans/{id}
│   └── tests/                    pytest + TestClient + FakeScanner
│
├── dashboard/                    ── ELECTRON + SVELTE ──────────────────────────────
│   ├── package.json              scripts: dev · build · check · dist:{win,mac,linux}
│   ├── vite.config.ts            renderer build (base './' for file://)
│   ├── svelte.config.js          vitePreprocess + runes mode
│   ├── tsconfig.json
│   ├── electron-builder.yml      NSIS (Win) · DMG (mac) · AppImage/deb (Linux)
│   ├── index.html                renderer entry + Content-Security-Policy
│   ├── README.md
│   ├── electron/
│   │   ├── main.js               lifecycle, secure BrowserWindow, IPC handlers
│   │   ├── preload.cjs           window.devscope bridge (contextBridge)
│   │   └── sidecar.js            spawn / health-check / stop the Python API
│   ├── src/
│   │   ├── main.ts               mounts App.svelte
│   │   ├── App.svelte            app shell (sidebar + current view)
│   │   ├── app.css               dark-theme design tokens
│   │   ├── vite-env.d.ts         types for window.devscope
│   │   ├── views/                Welcome.svelte · Overview.svelte
│   │   └── lib/
│   │       ├── api/              client.ts (fetch wrapper) · types.ts (TS contract)
│   │       ├── stores/           scan.svelte.ts (runes-based state)
│   │       ├── components/       Sidebar · ProjectPicker · StatCard ·
│   │       │                     LanguageBreakdown · LargestFiles · ScanProgress
│   │       └── utils/format.ts   Intl-based number/byte formatting
│   ├── public/                   static files copied verbatim to dist/
│   └── resources/                icons + frozen sidecar (per OS) for packaging
│
└── scripts/
    └── build_sidecar.py          PyInstaller: freeze the API for packaging
```

---

## Component deep-dive

### 1. `scanner/`: Rust core

**Responsibility:** `scan(root, &ScanOptions) -> Result<ScanReport>`. Nothing more.

Pipeline:

1. **`walker`**: `ignore::WalkBuilder` walks in parallel, honours
   `.gitignore`/`.ignore`, skips hidden entries and always prunes
   `node_modules`, `target`, `dist`, `build`, `__pycache__`, `.venv`. It does
   not follow symlinks, so it can't loop.
2. **`language`**: maps extension or filename to a `Language` enum (case-insensitive)
   and provides each language's comment syntax.
3. **`counter`**: streams each file as **bytes** through a `BufReader` with one
   reused buffer. It skips files over `max_file_size`, files with a NUL byte in
   the first 8 KiB (binary), and unreadable files. Those skips are *counted*,
   never fatal. CRLF and LF produce identical counts.
4. **`stats`**: per-thread `Accumulator`s merged with `rayon` `fold`/`reduce`
   (no locks), plus a bounded Top-N heap for the largest files.

**Two build shapes** from one crate:

| Shape             | Command                         | Used by                         |
|-------------------|---------------------------------|---------------------------------|
| rlib + CLI binary | `cargo build` / `cargo test`    | tests, benchmarks, debugging    |
| cdylib (Python)   | `maturin develop` / `maturin build` | the API (`import devscope_scanner`) |

PyO3 sits behind the `python` Cargo feature, so plain `cargo test` needs no
Python. Using `abi3-py311` means one wheel per OS/arch works on every CPython ≥ 3.11.

### 2. `api/`: Python service

**Responsibility:** a stable, validated HTTP interface over the scanner.

Layering (imports only flow downward, so each layer can be faked in tests):

```text
routes/      HTTP only: parse → call service → return schema
   │
services/    business logic: jobs, cache, lifecycle (no HTTP types)
   │
scanner_bridge.py   adapter: Pydantic ⇄ Rust objects, error mapping
   │
devscope_scanner    the Rust extension (imported ONLY here, lazily)
```

Key decisions:

- **Scans are jobs.** `POST /scans` returns `202` immediately and the UI polls
  `GET /scans/{id}`. No HTTP request stays open for a long scan, and progress
  or cancellation can be added later without changing the model (via SSE or a
  WebSocket).
- **No event-loop blocking.** Scans run in a small `ThreadPoolExecutor`, and
  the Rust code releases the GIL (`py.detach`) while working. `/health` stays
  responsive during a scan.
- **App factory.** `create_app(settings, scanner=...)` gives tests a fresh
  app with a `FakeScanner`, so no Rust toolchain is needed to run them.

### 3. `dashboard/electron/`: desktop shell

**Responsibility:** be the trusted, privileged process.

- `main.js` creates a hardened `BrowserWindow` (`contextIsolation`, `sandbox`,
  no `nodeIntegration`), holds the single-instance lock, and registers a few
  specific IPC handlers (`devscope:pick-folder`, `devscope:api-connection`).
- `preload.cjs` exposes exactly those as `window.devscope.*`.
- `sidecar.js` starts the API, waits for its ready line and a `/health`
  check, and kills the process tree on quit.

### 4. `dashboard/src/`: Svelte UI

**Responsibility:** display only. It does no filesystem access and has no
privileges of its own.

- `lib/api/client.ts` is the **only** place that calls `fetch()`. It adds the
  token, applies timeouts and maps errors to `ApiError`.
- `lib/stores/scan.svelte.ts` is a runes-based store with intent-level actions
  (`pickAndScan()`, `rescan()`) and the polling loop.
- Components are presentational and get their data through props. Styling
  comes from the CSS custom properties in `app.css`, so the dark theme is
  defined in one place.

---

## Data contract

The same shape is defined in four places, each mirroring the one before it:

```text
scanner/src/stats.rs  →  scanner/devscope_scanner.pyi  →  api/devscope_api/schemas.py  →  dashboard/src/lib/api/types.ts
   (Rust structs)           (Python stubs)                  (Pydantic, camelCase JSON)       (TypeScript)
```

`types.ts` can later be **generated** from the API's `/openapi.json` with
`openapi-typescript`, which removes one copy to keep in sync by hand.

### HTTP API (v1)

| Method | Path           | Auth | Body          | Response                         |
|--------|----------------|------|---------------|----------------------------------|
| GET    | `/health`      | no   | —             | `200 { status, apiVersion, scannerVersion }` |
| POST   | `/scans`       | yes  | `ScanRequest` | `202 ScanJob` (status `queued`)  |
| GET    | `/scans`       | yes  | —             | `200 ScanJob[]` (recent first)   |
| GET    | `/scans/{id}`  | yes  | —             | `200 ScanJob` · `404`            |

Example `ScanJob` once finished:

```json
{
  "id": "6f1c…",
  "status": "done",
  "request": { "path": "/Users/me/code/devscope", "respectGitignore": true, "topNFiles": 20 },
  "createdAt": "2026-10-29T12:00:00Z",
  "report": {
    "root": "/Users/me/code/devscope",
    "totalFiles": 412,
    "totalLines": 58210,
    "languages": [
      { "language": "Rust", "files": 38, "lines": { "code": 9120, "comments": 1404, "blanks": 1310 }, "bytes": 412001 }
    ],
    "largestFiles": [
      { "path": "scanner/src/counter.rs", "language": "Rust", "lines": { "code": 610, "comments": 120, "blanks": 88 }, "bytes": 30111 }
    ],
    "skipped": 3,
    "durationMs": 184
  },
  "error": null
}
```

Errors use HTTP status codes (`401`, `404`, `422`) with
`{ "detail": "...", "code": "not_found" }`. They are never `200 { ok: false }`.

---

## Runtime: how the pieces start and talk

```text
 User launches devscope
        │
        ▼
 Electron main ──1. random 256-bit token, spawn sidecar───────────────▶ Python API
   (main.js +      env DEVSCOPE_TOKEN=…                                 (__main__.py)
    sidecar.js)    args --port 0 --parent-pid <pid>                         │
        ▲                                                                   │ 2. OS assigns free port
        └────────────3. stdout: "DEVSCOPE_READY port=53127"─────────────────┘
        │
        │ 4. GET /health ✓
        ▼
 BrowserWindow loads UI  ──5. window.devscope.getApiConnection()──▶ { baseUrl, token }
        │
        ▼
 Svelte UI ──6. POST /scans {path} (Bearer token)──▶ API ──thread──▶ Rust scan (GIL released)
           ◀─7. 202 {id}──────────────────────────
           ──8. GET /scans/{id} … poll …──────────▶
           ◀─9. {status:"done", report}───────────
        │
 User quits ──▶ Electron kills sidecar (SIGTERM / taskkill /T). The sidecar also exits by itself if its parent PID dies.
```

**Dev vs production:**

|                | Development (`npm run dev`)           | Production (installed app)                         |
|----------------|---------------------------------------|----------------------------------------------------|
| UI source      | Vite dev server `127.0.0.1:5173` (HMR) | `dist/index.html` inside `app.asar`               |
| Python         | `api/.venv` interpreter, `-m devscope_api` | Frozen `resources/sidecar/devscope-api[.exe]` |
| Rust extension | Installed into venv by `maturin develop` | Bundled inside the frozen sidecar                |

---

## Security model

The API can read any folder the user can, so it is treated as privileged even
though it only listens locally.

| Threat                                                  | Mitigation                                           |
|---------------------------------------------------------|------------------------------------------------------|
| Other machines on the network                           | Bind **127.0.0.1** only, never `0.0.0.0`             |
| Other local programs / browser tabs calling the API     | Per-launch **bearer token** (env var, never on disk or argv), constant-time compare |
| Port collisions / guessable port                        | `--port 0`: OS-assigned random port                  |
| XSS in the UI (e.g. a malicious filename)               | Svelte escapes by default, no `{@html}` with data, strict **CSP**, renderer **sandbox** + `contextIsolation`, no `nodeIntegration` |
| Renderer escalating to Node.js                          | Preload exposes 2–3 specific functions, no generic `invoke` |
| Orphaned server after a crash                           | `--parent-pid` watchdog, plus tree-kill on quit      |
| Malicious repo content crashing the scanner             | Rust skips bad files instead of panicking, size caps, no symlink following |

---

## Cross-platform considerations

| Concern            | Windows                                  | macOS                                   | Linux                         | Where handled |
|--------------------|------------------------------------------|-----------------------------------------|-------------------------------|---------------|
| Paths              | `\` separators, drive letters, 260-char limit, `\\?\` prefix | case-insensitive APFS by default | arbitrary bytes in names | Rust `Path`/`PathBuf`; output normalised to relative `/` paths (`stats.rs`) |
| Line endings       | CRLF common                              | LF                                      | LF                            | `counter.rs` counts `\n`, trims `\r`; `.gitattributes` |
| venv layout        | `.venv\Scripts\python.exe`               | `.venv/bin/python`                      | `.venv/bin/python`            | `sidecar.js` |
| Killing the sidecar| no POSIX signals → `taskkill /T /F`      | `SIGTERM`                               | `SIGTERM`                     | `sidecar.js` |
| Console window     | `windowsHide: true`                      | —                                       | —                             | `sidecar.js` |
| Window chrome      | default frame                            | `hiddenInset` + traffic-light padding   | default frame                 | `main.js`, `Sidebar.svelte` |
| App lifecycle      | quit on last window closed               | stay alive in the dock                  | quit on last window closed    | `main.js` |
| Native extension   | `.pyd`                                   | `.so` (universal2 or per-arch)          | `.so` (manylinux)             | maturin |
| Installer          | NSIS `.exe`                              | `.dmg` + `.zip`                         | AppImage + `.deb`             | `electron-builder.yml` |
| Code signing       | Authenticode / Azure Trusted Signing     | Developer ID + notarization (sign the sidecar binaries too) | optional | CI secrets |
| Fonts              | Segoe UI                                 | SF Pro (system-ui)                      | Ubuntu/Roboto/…               | `app.css` system font stack |

**Rule of thumb:** every OS-specific branch lives in one obvious place
(`sidecar.js`, `main.js`, `build_sidecar.py`). Everything else is OS-agnostic.

---

## Development setup

### Prerequisites (all OSes)

| Tool    | Version | Install                                                                 |
|---------|---------|-------------------------------------------------------------------------|
| Rust    | ≥ 1.80  | <https://rustup.rs> (Windows also needs the MSVC Build Tools)            |
| Python  | ≥ 3.11  | <https://python.org> / `brew` / your distro (or `uv python install`)     |
| Node.js | ≥ 22 LTS| <https://nodejs.org> or `nvm` / `fnm`                                    |
| maturin | ≥ 1.7   | `pip install maturin` (inside the venv below)                            |

Linux additionally needs the usual Electron runtime libraries (e.g. `libgtk-3-0`,
`libnss3`, `libasound2`) and `python3-venv` on Debian/Ubuntu.

### One-time setup

```bash
# 1. Python virtualenv for the API
python -m venv api/.venv          # Windows may need: py -3.12 -m venv api\.venv

# 2. Activate it
source api/.venv/bin/activate     # macOS / Linux
api\.venv\Scripts\Activate.ps1    # Windows PowerShell
api\.venv\Scripts\activate.bat    # Windows cmd

# 3. Build the Rust extension into the venv, then install the API
pip install maturin
(cd scanner && maturin develop --release)
pip install -e "./api[dev]"

# 4. Dashboard dependencies
cd dashboard && npm install
```

<sub>Prefer `uv`? `cd api && uv sync --extra dev` resolves the local scanner via `[tool.uv.sources]`.</sub>

### Day-to-day

```bash
cd dashboard && npm run dev       # Vite + Electron; Electron spawns the API from api/.venv
```

Working on one layer in isolation:

```bash
cargo test --manifest-path scanner/Cargo.toml          # Rust only
cargo run  --manifest-path scanner/Cargo.toml --release --bin devscope-scan -- .
python -m devscope_api --port 8765                     # API only → http://127.0.0.1:8765/docs
cd dashboard && npm run dev:web                        # UI only, in a browser
```

After changing Rust code, re-run `maturin develop --release` so the API picks it up.

---

## Building and packaging

Installers are built **on each target OS** (locally or in a CI matrix of
`windows-latest`, `macos-latest`, `ubuntu-latest`). Native code (Rust
extension, PyInstaller bundle) can't be cross-compiled by this toolchain.

```bash
# 1. Release build of the Rust extension into the venv
(cd scanner && maturin develop --release)

# 2. Freeze the API (+ extension) into dashboard/resources/sidecar/<os>/
pip install -e "./api[freeze]"
python scripts/build_sidecar.py

# 3. Build the UI and the installer for this OS
cd dashboard && npm run dist      # → dashboard/release/
```

| OS      | Output in `dashboard/release/`        |
|---------|---------------------------------------|
| Windows | `devscope Setup x.y.z.exe` (NSIS)     |
| macOS   | `devscope-x.y.z.dmg`, `…-mac.zip`     |
| Linux   | `devscope-x.y.z.AppImage`, `.deb`     |

---

## Testing and quality

| Layer     | Tests                                     | Lint / types                                        |
|-----------|-------------------------------------------|-----------------------------------------------------|
| scanner   | `cargo test` (unit + `tests/`)            | `cargo clippy --all-targets -- -D warnings`, `cargo fmt --check` |
| api       | `pytest` (fake scanner) + bridge smoke tests (`importorskip`) | `ruff check`, `ruff format --check`, `mypy --strict` |
| dashboard | component tests with Vitest (planned), E2E with Playwright's Electron support (planned) | `npm run check` (svelte-check) |

---

## Roadmap

- [ ] **v0.1:** implement scanner core and CLI, matching `tokei` on fixtures
- [ ] **v0.2:** PyO3 bindings, API job flow, tests green on all 3 OSes
- [ ] **v0.3:** dashboard: Welcome → scan → Overview with language chart and largest files
- [ ] **v0.4:** packaging: sidecar freeze, signed installers, CI matrix
- [ ] Later: live progress (SSE), cancellation, git history and churn stats, saved projects, light theme, auto-update

---

## License

[MIT](LICENSE) © 2026 Francois le Roux
