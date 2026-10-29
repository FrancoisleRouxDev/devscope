"""devscope_api — the local HTTP service between the Rust scanner and the dashboard.

Package layout (each module's docstring explains it in detail):

    __main__.py         process entry point (Electron spawns this)
    app.py              create_app() factory
    config.py           Settings from environment variables
    security.py         per-launch auth token dependency
    schemas.py          Pydantic models = the HTTP contract
    scanner_bridge.py   the ONLY place that imports the Rust extension
    services/           business logic (scan jobs, caching)
    routes/             thin HTTP handlers, one module per resource

Layering rule: routes → services → scanner_bridge → devscope_scanner (Rust).
Nothing imports "upwards", so each layer can be tested by faking the one below.
"""

# Planned (uncomment when packaging is wired up):
#
# from importlib.metadata import version
#
# __version__ = version("devscope-api")
