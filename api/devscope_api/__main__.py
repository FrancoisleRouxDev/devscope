"""Process entry point: ``python -m devscope_api`` (or the frozen sidecar binary).

PURPOSE
    Electron's main process spawns this as a child process ("sidecar"). This
    module parses CLI flags, builds the app, and starts uvicorn.

HANDSHAKE WITH ELECTRON
    1. Electron generates a random token and passes it via the env var
       ``DEVSCOPE_TOKEN`` (env, not argv: argv is visible to every user in
       ``ps`` / Task Manager).
    2. Electron passes ``--port 0`` → the OS picks a free port, so two app
       instances or another local service can never collide.
    3. Once listening, this process prints ONE line to stdout:
           DEVSCOPE_READY port=53127
       Electron waits for that line, then tells the renderer the base URL.
    4. When Electron quits it kills this process. As a safety net we also exit
       if our parent dies (``--parent-pid``), so no orphaned servers linger —
       especially on Windows, where child processes are not killed with the parent.
"""

# =============================================================================
# LOGIC SKETCH (commented out — not implemented yet)
# =============================================================================
#
# import argparse
# import socket
# import sys
#
# import uvicorn
#
# from devscope_api.app import create_app
# from devscope_api.config import Settings
#
#
# def main(argv: list[str] | None = None) -> int:
#     parser = argparse.ArgumentParser(prog="devscope-api")
#     parser.add_argument("--host", default="127.0.0.1")
#     parser.add_argument("--port", type=int, default=0)        # 0 = pick a free port
#     parser.add_argument("--parent-pid", type=int, default=None)
#     args = parser.parse_args(argv)
#
#     settings = Settings()                                     # reads DEVSCOPE_* env vars
#
#     # Bind the socket ourselves so we know the real port BEFORE uvicorn starts,
#     # then hand the open socket to uvicorn.
#     sock = socket.socket(socket.AF_INET, socket.SOCK_STREAM)
#     sock.bind((args.host, args.port))
#     port = sock.getsockname()[1]
#
#     print(f"DEVSCOPE_READY port={port}", flush=True)          # flush! stdout is a pipe
#
#     config = uvicorn.Config(create_app(settings), log_level=settings.log_level)
#     uvicorn.Server(config).run(sockets=[sock])
#     return 0
#
#
# if __name__ == "__main__":
#     sys.exit(main())
