# dashboard/resources

Build-time resources for **electron-builder** (`directories.buildResources`).
Nothing here is bundled by Vite.

| File / folder         | Used for                                                     |
|-----------------------|--------------------------------------------------------------|
| `icon.icns`           | macOS app icon (1024×1024 source)                            |
| `icon.ico`            | Windows app icon (multi-size: 16–256 px)                     |
| `icon.png`            | Linux app icon (512×512 or 1024×1024)                        |
| `sidecar/mac/`        | Frozen Python API for macOS   — generated, git-ignored       |
| `sidecar/win/`        | Frozen Python API for Windows — generated, git-ignored       |
| `sidecar/linux/`      | Frozen Python API for Linux   — generated, git-ignored       |

`sidecar/<os>/` is produced by `python scripts/build_sidecar.py` on that OS and
copied into the installer via `extraResources` (see `../electron-builder.yml`).

Icons are not committed yet — add them before running `npm run dist`.
