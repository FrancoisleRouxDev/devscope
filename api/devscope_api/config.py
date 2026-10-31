# config.py - runtime settings (host, port, allowed origins, default
# ignore patterns). Values come from CLI args / environment variables.

import os

# The API only ever listens on loopback; Electron passes the port to uvicorn.
HOST = "127.0.0.1"

# Origins allowed to call the API from a browser context:
#   - the Vite dev server (development)
#   - "null", the origin of a page loaded from file:// (packaged Electron app)
ALLOWED_ORIGINS = os.environ.get(
    "DEVSCOPE_ORIGINS", "http://localhost:5173,null"
).split(",")

# How many folder levels to search below a workspace root for projects.
DEFAULT_MAX_DEPTH = 4

# Seconds to wait for a single `git` command before giving up.
GIT_TIMEOUT = 10
