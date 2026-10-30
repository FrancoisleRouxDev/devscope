# main.py - FastAPI application entry point.
# Creates the app, registers routers from routes/, configures CORS for the
# Electron renderer, and is launched by uvicorn (spawned by Electron's
# main process). Binds to 127.0.0.1 on a port passed in by Electron.
