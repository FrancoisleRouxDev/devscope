"""Application factory: ``create_app(settings) -> FastAPI``.

PURPOSE
    Build and wire the FastAPI app: routers, CORS, exception handlers, and the
    lifespan (startup/shutdown) hook. Nothing runs at import time — the app
    only exists once ``create_app()`` is called. Tests call it with test
    settings; ``__main__.py`` calls it with real ones.
"""

# =============================================================================
# LOGIC SKETCH (commented out — not implemented yet)
# =============================================================================
#
# from contextlib import asynccontextmanager
# from collections.abc import AsyncIterator
#
# from fastapi import FastAPI, Request
# from fastapi.middleware.cors import CORSMiddleware
# from fastapi.responses import JSONResponse
#
# from devscope_api.config import Settings
# from devscope_api.routes import health, scans
# from devscope_api.scanner_bridge import ScanFailed, Scanner
# from devscope_api.services.scan_jobs import ScanJobService
#
#
# def create_app(settings: Settings | None = None, scanner: Scanner | None = None) -> FastAPI:
#     settings = settings or Settings()
#     if scanner is None:
#         # Imported lazily: tests pass a FakeScanner and never load the Rust .so/.pyd.
#         from devscope_api.scanner_bridge import RustScanner
#         scanner = RustScanner()
#
#     @asynccontextmanager
#     async def lifespan(app: FastAPI) -> AsyncIterator[None]:
#         app.state.settings = settings
#         app.state.scanner = scanner
#         app.state.jobs = ScanJobService(scanner, max_cached=settings.max_cached_scans)
#         yield
#         app.state.jobs.shutdown()          # cancel running scans, free threads
#
#     app = FastAPI(title="devscope API", version="0.1.0", lifespan=lifespan)
#
#     # Renderer origin: Vite dev server in dev, file:// (origin "null") or a
#     # custom app:// protocol in production. Never "*".
#     app.add_middleware(CORSMiddleware, allow_origins=settings.allowed_origins,
#                        allow_methods=["GET", "POST"],
#                        allow_headers=["Authorization", "Content-Type"])
#
#     @app.exception_handler(ScanFailed)
#     async def _scan_failed(_: Request, exc: ScanFailed) -> JSONResponse:
#         return JSONResponse(status_code=422, content={"detail": str(exc), "code": exc.code})
#
#     app.include_router(health.router)
#     app.include_router(scans.router)
#     return app

# =============================================================================
# JUNIOR vs SENIOR — structuring a FastAPI app
# =============================================================================
#
# ── JUNIOR ──────────────────────────────────────────────────────────────────
#
#     # main.py — everything in one file
#     from fastapi import FastAPI
#     import devscope_scanner
#
#     app = FastAPI()
#     results = {}                                  # global, grows forever
#
#     @app.get("/scan")
#     async def scan(path: str):
#         r = devscope_scanner.scan(path)           # blocks the event loop
#         results[path] = r
#         return {"files": r.total_files, "langs": [l.language for l in r.languages]}
#
#     if __name__ == "__main__":
#         import uvicorn
#         uvicorn.run(app, host="0.0.0.0", port=8000)   # exposed to the whole LAN!
#
#   Problems:
#     * Module-level `app` + global dict → tests share state, can't run with
#       different settings, and import has side effects.
#     * Rust import at the top of the web module → tests can't run without
#       compiling Rust first.
#     * Everything (routing, caching, conversion, config) in one place.
#     * `0.0.0.0` + fixed port + no auth: anyone on the café Wi-Fi can make the
#       app read any folder on your disk. Port 8000 also collides with half the
#       dev servers on a developer's machine.
#
# ── SENIOR ──────────────────────────────────────────────────────────────────
#
#   * `create_app(settings)` factory; state lives on `app.state`, created in
#     `lifespan`, cleaned up on shutdown.
#   * Layers: routes (HTTP only) → services (logic) → bridge (Rust adapter).
#     Each is replaceable in tests via `app.dependency_overrides`.
#   * Bind 127.0.0.1, random port, per-launch token, explicit CORS origins.
#   * Domain exceptions mapped to HTTP status codes in ONE handler, not
#     try/except copy-pasted in every route.
