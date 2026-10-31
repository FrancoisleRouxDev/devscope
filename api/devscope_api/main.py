# main.py - FastAPI application entry point.
# Creates the app, registers routers from routes/, configures CORS for the
# Electron renderer, and is launched by uvicorn (spawned by Electron's
# main process). Binds to 127.0.0.1 on a port passed in by Electron.

from fastapi import FastAPI
from fastapi.middleware.cors import CORSMiddleware

from . import watchdog
from .config import ALLOWED_ORIGINS
from .routes import health, scan

watchdog.start()  # exit if the Electron process that launched us disappears

app = FastAPI(title="devscope", version="0.1.0")

app.add_middleware(
    CORSMiddleware,
    allow_origins=ALLOWED_ORIGINS,
    allow_methods=["GET", "POST"],
    allow_headers=["Content-Type"],
)

app.include_router(health.router)
app.include_router(scan.router)
