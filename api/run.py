# run.py - entry point for the packaged API (PyInstaller freezes this file).
# A frozen executable cannot use `python -m uvicorn devscope_api.main:app`, so
# this starts uvicorn directly. Electron runs it as: devscope-api --port <port>
# In development Electron still uses `python -m uvicorn` (see dashboard/electron/backend.js).

import argparse
import os
import sys

import uvicorn

from devscope_api.config import HOST
from devscope_api.main import app


def main() -> None:
    parser = argparse.ArgumentParser(description="devscope local API")
    parser.add_argument("--port", type=int, required=True)
    parser.add_argument("--host", default=HOST)
    args = parser.parse_args()
    # Refuse to run without the access token Electron generates for each launch (see main.py).
    if not os.environ.get("DEVSCOPE_TOKEN"):
        sys.exit("devscope-api: DEVSCOPE_TOKEN is not set; start it through the devscope app.")
    uvicorn.run(app, host=args.host, port=args.port, log_level="info")


if __name__ == "__main__":
    main()
