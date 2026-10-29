"""The ONLY module that imports the Rust extension ``devscope_scanner``.

PURPOSE
    An adapter ("anti-corruption layer") between the native module and the
    rest of the Python code:

      * converts ``ScanRequest`` (Pydantic)  → keyword args for Rust,
      * converts Rust ``ScanReport`` objects → Pydantic ``schemas.ScanReport``,
      * converts ``devscope_scanner.ScanError`` → our own ``ScanFailed``,
      * validates/normalises the path BEFORE handing it to Rust.

    Because nothing else imports ``devscope_scanner``, the rest of the API can
    be tested with a ``FakeScanner`` and no Rust toolchain at all, and a future
    change to the Rust API touches exactly one Python file.

THREADING
    ``scan()`` here is a normal *blocking* function. The Rust side releases the
    GIL while working, and callers run it in a worker thread (see
    ``services/scan_jobs.py``), so the asyncio event loop is never blocked.
"""

# =============================================================================
# LOGIC SKETCH (commented out — not implemented yet)
# =============================================================================
#
# from pathlib import Path
# from typing import Protocol
#
# from devscope_api import schemas
#
# # NOTE: `devscope_scanner` is imported inside RustScanner.__init__, not here,
# # so `ScanFailed` / `Scanner` can be imported (by app.py, by tests) on a
# # machine where the Rust extension hasn't been built.
#
#
# class ScanFailed(Exception):
#     """Domain error: the scan could not run (bad path, permission, ...)."""
#     def __init__(self, message: str, code: str) -> None:
#         super().__init__(message)
#         self.code = code                 # machine-readable: "not_found", "not_a_directory"
#
#
# class Scanner(Protocol):
#     """What the services layer depends on. Real + fake implementations satisfy it."""
#     def scan(self, request: schemas.ScanRequest) -> schemas.ScanReport: ...
#     @property
#     def version(self) -> str: ...
#
#
# class RustScanner:
#     def __init__(self) -> None:
#         import devscope_scanner          # fails fast here with a clear ImportError
#         self._native = devscope_scanner
#
#     @property
#     def version(self) -> str:
#         return self._native.__version__
#
#     def scan(self, request: schemas.ScanRequest) -> schemas.ScanReport:
#         root = request.path.expanduser().resolve()
#         if not root.exists():
#             raise ScanFailed(f"Folder not found: {root}", code="not_found")
#         if not root.is_dir():
#             raise ScanFailed(f"Not a folder: {root}", code="not_a_directory")
#         try:
#             raw = self._native.scan(str(root),
#                                      respect_gitignore=request.respect_gitignore,
#                                      include_hidden=request.include_hidden,
#                                      max_file_size=request.max_file_size,
#                                      top_n_files=request.top_n_files)
#         except self._native.ScanError as exc:
#             raise ScanFailed(str(exc), code="scan_error") from exc
#         return _to_schema(raw)
#
#
# def _to_schema(raw: "devscope_scanner.ScanReport") -> schemas.ScanReport:
#     """Explicit field-by-field conversion: a renamed Rust field fails loudly here."""
#     ...

# =============================================================================
# JUNIOR vs SENIOR — calling native/blocking code from FastAPI
# =============================================================================
#
# ── JUNIOR ──────────────────────────────────────────────────────────────────
#
#     @app.post("/scan")
#     async def scan(req: ScanRequest):
#         try:
#             result = devscope_scanner.scan(req.path)      # blocking call in async def
#             return result.__dict__                        # pyclass has no __dict__ → crash
#         except:                                           # bare except: also eats Ctrl+C
#             print("error!!")
#             return {"error": "something went wrong"}      # 200 OK with an error body
#
#   Problems:
#     * A blocking call inside `async def` freezes the event loop: while one
#       scan runs, /health times out and the UI looks hung.
#     * `os.path.join(a, "/" + b)` style path building and passing str paths
#       around unvalidated — fails differently on Windows ("C:\\…") vs POSIX.
#     * Bare `except:` hides bugs AND swallows KeyboardInterrupt/SystemExit.
#     * `print()` instead of `logging` → no levels, lost when frozen as a GUI app.
#     * Returns 200 for failures, so the frontend has to string-match errors.
#
# ── SENIOR ──────────────────────────────────────────────────────────────────
#
#   * Blocking work runs in a thread: `await anyio.to_thread.run_sync(...)`
#     (or simply declare the route `def`, which FastAPI runs in its threadpool).
#   * `pathlib.Path` end-to-end; `.expanduser().resolve()` once at the boundary.
#   * Catch the SPECIFIC exception (`devscope_scanner.ScanError`), re-raise as a domain
#     error with `from exc` so the traceback chain is preserved.
#   * Errors become proper HTTP statuses via one exception handler (app.py).
#   * Depend on a `Protocol`, not the concrete class → trivial to fake in tests.
#   * `logging.getLogger(__name__)` for diagnostics.
