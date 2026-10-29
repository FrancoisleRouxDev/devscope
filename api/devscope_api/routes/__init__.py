"""HTTP route modules — one ``APIRouter`` per resource.

Routes are deliberately thin: parse input (via Pydantic schemas), call a
service, return a schema. No scanning, caching or path logic lives here.

    health.py   GET  /health          liveness + versions (no auth — Electron polls it)
    scans.py    POST /scans           start a scan job          (auth required)
                GET  /scans           list recent jobs          (auth required)
                GET  /scans/{id}      job status + report       (auth required)
"""
