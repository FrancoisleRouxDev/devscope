# dashboard — Electron shell + Svelte UI

The desktop app. Electron provides the native window, menus and folder
picker, and supervises the Python API sidecar; Svelte 5 (built by Vite)
renders the dark-mode dashboard.

See the root [README](../README.md) for the full architecture.

## Layout

| Path                              | Process     | Purpose                                                   |
|-----------------------------------|-------------|-----------------------------------------------------------|
| `package.json`                    | —           | Deps + scripts (`dev`, `build`, `check`, `dist:*`)        |
| `vite.config.ts`                  | build       | Vite config for the renderer                              |
| `svelte.config.js`                | build       | Svelte preprocess + runes mode                            |
| `tsconfig.json`                   | build       | TypeScript settings for `src/`                            |
| `electron-builder.yml`            | build       | Installers for Windows (NSIS), macOS (DMG), Linux (AppImage/deb) |
| `index.html`                      | renderer    | HTML entry + Content-Security-Policy                      |
| `electron/main.js`                | main        | App lifecycle, window, IPC handlers                       |
| `electron/preload.cjs`            | preload     | `window.devscope` bridge via `contextBridge`              |
| `electron/sidecar.js`             | main        | Spawn / health-check / stop the Python API                |
| `src/main.ts`                     | renderer    | Mounts `App.svelte`                                       |
| `src/App.svelte`                  | renderer    | App shell: sidebar + current view                         |
| `src/app.css`                     | renderer    | Global styles + dark-theme design tokens                  |
| `src/vite-env.d.ts`               | renderer    | Types for `window.devscope` and Vite                      |
| `src/lib/api/`                    | renderer    | Typed HTTP client + TS mirror of the API schemas          |
| `src/lib/stores/scan.svelte.ts`   | renderer    | Scan state (Svelte 5 runes) + actions                     |
| `src/lib/components/`             | renderer    | Presentational components (cards, charts, tables)         |
| `src/lib/utils/format.ts`         | renderer    | Locale-aware number/byte/duration formatting              |
| `src/views/`                      | renderer    | Full-page views: `Welcome`, `Overview`                    |
| `public/`                         | renderer    | Static files copied verbatim into `dist/`                 |
| `resources/`                      | build       | Icons + frozen sidecar for electron-builder               |

## Commands

```bash
npm install
npm run dev        # Vite (HMR) + Electron together
npm run dev:web    # UI only, in a browser (no Electron bridge)
npm run check      # svelte-check (types + a11y)
npm run build      # renderer → dist/
npm run dist       # installer for the current OS → release/
```
