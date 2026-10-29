"""Freeze the Python API into a standalone "sidecar" executable for packaging.

PURPOSE
    End users won't have Python installed. Before ``npm run dist``, this script
    uses PyInstaller to bundle the interpreter, FastAPI, uvicorn and the
    compiled Rust extension into one folder:

        dashboard/resources/sidecar/<os>/devscope-api[.exe]   (+ support files)

    where <os> is ``mac`` | ``win`` | ``linux`` — the names electron-builder's
    ``${os}`` macro expands to (see dashboard/electron-builder.yml).

    Run it ON EACH TARGET OS (natively or in a CI matrix). PyInstaller cannot
    cross-compile, and neither can the Rust extension it bundles.

USAGE (planned)
    # with the api/.venv active and `pip install "./api[freeze]"` done:
    python scripts/build_sidecar.py

WHY A PYTHON SCRIPT (not bash / PowerShell)?
    The same file runs unchanged on Windows, macOS and Linux.

WHY ``--onedir`` (not ``--onefile``)?
    --onefile unpacks itself to a temp dir on EVERY launch (slow start, and
    antivirus tools on Windows often flag it). --onedir starts instantly.
"""

# =============================================================================
# LOGIC SKETCH (commented out — not implemented yet)
# =============================================================================
#
# import platform
# import shutil
# import subprocess
# import sys
# from pathlib import Path
#
# ROOT = Path(__file__).resolve().parent.parent
# OS_NAME = {"Darwin": "mac", "Windows": "win", "Linux": "linux"}[platform.system()]
# OUT_DIR = ROOT / "dashboard" / "resources" / "sidecar" / OS_NAME
#
#
# def main() -> int:
#     work = ROOT / "build" / "pyinstaller"
#     subprocess.run(
#         [
#             sys.executable, "-m", "PyInstaller",
#             "--noconfirm", "--clean", "--onedir",
#             "--name", "devscope-api",
#             "--distpath", str(work / "dist"),
#             "--workpath", str(work / "build"),
#             "--collect-submodules", "devscope_api",
#             # Native module: PyInstaller can't see this import statically.
#             "--hidden-import", "devscope_scanner",
#             str(ROOT / "api" / "devscope_api" / "__main__.py"),
#         ],
#         check=True,                          # fail loudly; never ship a half-built sidecar
#     )
#     shutil.rmtree(OUT_DIR, ignore_errors=True)
#     shutil.copytree(work / "dist" / "devscope-api", OUT_DIR)
#     print(f"sidecar ready: {OUT_DIR}")
#     return 0
#
#
# if __name__ == "__main__":
#     sys.exit(main())
