# api

Local FastAPI service. Wraps the Rust scanner (via PyO3) and serves results
over HTTP on 127.0.0.1 only. Started and stopped by the Electron main process.
