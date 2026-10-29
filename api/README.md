# api — local FastAPI service

Wraps the Rust scanner (`devscope_scanner`, built from [`/scanner`](../scanner))
and serves scan results to the Electron dashboard over HTTP on `127.0.0.1`.

See the root [README](../README.md) for the full architecture.

## Layout

| Path                                  | Purpose                                                      |
|---------------------------------------|--------------------------------------------------------------|
| `pyproject.toml`                      | Package metadata, deps, ruff/mypy/pytest config              |
| `devscope_api/__main__.py`            | Process entry point used by Electron (`python -m devscope_api`) |
| `devscope_api/app.py`                 | `create_app()` factory — wires routers, middleware, lifespan |
| `devscope_api/config.py`              | `Settings` loaded from `DEVSCOPE_*` env vars                 |
| `devscope_api/security.py`            | Per-launch bearer-token check                                |
| `devscope_api/schemas.py`             | Pydantic request/response models (the HTTP contract)         |
| `devscope_api/scanner_bridge.py`      | The only module that imports the Rust extension              |
| `devscope_api/services/scan_jobs.py`  | Background scan jobs + bounded result cache                  |
| `devscope_api/routes/health.py`       | `GET /health`                                                |
| `devscope_api/routes/scans.py`        | `POST /scans`, `GET /scans/{id}`, `GET /scans`               |
| `tests/`                              | pytest suite using FastAPI's `TestClient` and a fake scanner |

## Commands

```bash
python -m devscope_api --port 8765   # run (once implemented)
pytest                               # tests
ruff check . && ruff format --check  # lint / format
mypy devscope_api                    # types
```

Interactive docs (once running): <http://127.0.0.1:8765/docs>
