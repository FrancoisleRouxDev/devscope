"""``GET /health`` — liveness probe.

PURPOSE
    Electron polls this after spawning the sidecar (and periodically after)
    to know the backend is up. It is the only unauthenticated route and it
    reveals nothing sensitive: just a status and version strings, which the
    dashboard's "About" panel also shows.
"""

# =============================================================================
# LOGIC SKETCH (commented out — not implemented yet)
# =============================================================================
#
# from fastapi import APIRouter, Request
#
# from devscope_api import __version__, schemas
#
# router = APIRouter(tags=["health"])
#
#
# @router.get("/health", response_model=schemas.Health)
# async def health(request: Request) -> schemas.Health:
#     # async def is fine here: no blocking work, just reading attributes.
#     return schemas.Health(api_version=__version__,
#                           scanner_version=request.app.state.scanner.version)
