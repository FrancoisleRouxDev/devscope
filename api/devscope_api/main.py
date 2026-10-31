# main.py - FastAPI application entry point.
# Creates the app, registers routers from routes/, configures CORS for the
# Electron renderer, and is launched by uvicorn (spawned by Electron's
# main process). Binds to 127.0.0.1 on a port passed in by Electron.

import hmac
import os

from fastapi import FastAPI, Request
from fastapi.middleware.cors import CORSMiddleware
from fastapi.responses import JSONResponse

from . import watchdog
from .config import ALLOWED_ORIGINS
from .routes import health, scan

watchdog.start()  # exit if the Electron process that launched us disappears

app = FastAPI(title="devscope", version="0.1.0")


# SECURITY: the API listens on a loopback port that any program on this computer can reach.
# Electron starts it with a random one-time token (DEVSCOPE_TOKEN) and the app sends it with
# every request, so other programs cannot ask it to list folders or look for secrets.
# /health stays open (it only says "ok"). The token is read per request so tests can change it.
# Registered before CORS so that CORS headers are also added to the 401 replies.
@app.middleware("http")
async def require_token(request: Request, call_next):
    token = os.environ.get("DEVSCOPE_TOKEN")
    if token and request.method != "OPTIONS" and request.url.path != "/health":
        sent = request.headers.get("x-devscope-token", "")
        if not hmac.compare_digest(sent.encode(), token.encode()):
            return JSONResponse({"detail": "Unauthorized"}, status_code=401)
    return await call_next(request)


app.add_middleware(
    CORSMiddleware,
    allow_origins=ALLOWED_ORIGINS,
    allow_methods=["GET", "POST"],
    allow_headers=["Content-Type", "X-Devscope-Token"],
)

app.include_router(health.router)
app.include_router(scan.router)
