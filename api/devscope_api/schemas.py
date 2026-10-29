"""Pydantic models — the HTTP contract between the API and the dashboard.

PURPOSE
    Every request body and response is a typed model. FastAPI uses them to:
      * validate input (bad JSON → 422 with a precise error, before our code runs),
      * serialise output (``response_model=`` strips anything not declared),
      * generate OpenAPI docs at ``/docs`` — and, optionally, TypeScript types
        for ``dashboard/src/lib/api/types.ts`` via ``openapi-typescript``.

    These mirror the Rust output types in ``scanner/src/stats.rs``. The Rust
    objects are converted into these models in ``scanner_bridge.py`` so the
    HTTP layer never depends on the native extension's classes.

NAMING
    Python uses snake_case; the JSON the dashboard receives is camelCase via an
    alias generator, so TypeScript code stays idiomatic on its side too.
"""

# =============================================================================
# LOGIC SKETCH (commented out — not implemented yet)
# =============================================================================
#
# from datetime import datetime
# from enum import StrEnum
# from pathlib import Path
# from uuid import UUID
#
# from pydantic import BaseModel, ConfigDict, Field
# from pydantic.alias_generators import to_camel
#
#
# class ApiModel(BaseModel):
#     """Base for all models: camelCase JSON, immutable, no unknown fields."""
#     model_config = ConfigDict(alias_generator=to_camel, populate_by_name=True,
#                               frozen=True, extra="forbid")
#
#
# # ---- requests ---------------------------------------------------------------
#
# class ScanRequest(ApiModel):
#     path: Path
#     respect_gitignore: bool = True
#     include_hidden: bool = False
#     max_file_size: int = Field(default=5_000_000, gt=0)
#     top_n_files: int = Field(default=20, ge=1, le=500)
#
#
# # ---- responses --------------------------------------------------------------
#
# class LineCounts(ApiModel):
#     code: int
#     comments: int
#     blanks: int
#
# class LanguageStats(ApiModel):
#     language: str
#     files: int
#     lines: LineCounts
#     bytes: int
#
# class FileStats(ApiModel):
#     path: str                       # relative, "/"-separated (normalised in Rust)
#     language: str
#     lines: LineCounts
#     bytes: int
#
# class ScanReport(ApiModel):
#     root: str
#     total_files: int
#     total_lines: int
#     languages: list[LanguageStats]
#     largest_files: list[FileStats]
#     skipped: int
#     duration_ms: int
#
# class JobStatus(StrEnum):
#     QUEUED = "queued"
#     RUNNING = "running"
#     DONE = "done"
#     FAILED = "failed"
#
# class ScanJob(ApiModel):
#     id: UUID
#     status: JobStatus
#     request: ScanRequest
#     created_at: datetime
#     report: ScanReport | None = None
#     error: str | None = None
#
# class Health(ApiModel):
#     status: str = "ok"
#     api_version: str
#     scanner_version: str

# =============================================================================
# JUNIOR vs SENIOR — request/response data
# =============================================================================
#
# ── JUNIOR ──────────────────────────────────────────────────────────────────
#
#     @app.post("/scan")
#     async def scan(body: dict):
#         path = body["path"]                       # KeyError → 500, not 422
#         top = int(body.get("top", 20))            # "abc" → 500; -5 → accepted
#         ...
#         return {"ok": True, "data": {...}}        # shape documented nowhere
#
# ── SENIOR ──────────────────────────────────────────────────────────────────
#
#   * `body: ScanRequest` — FastAPI validates and documents it; bounds like
#     `ge=1, le=500` reject nonsense before any code runs.
#   * `response_model=ScanJob` — the response shape is a contract, visible at
#     /docs, and accidental extra fields are never leaked.
#   * `extra="forbid"` catches client typos (`topNFile`) instead of silently
#     ignoring them.
#   * `frozen=True` — models are values; nobody mutates a response half-way.
#   * No `{"ok": True, "data": ...}` envelopes: HTTP status codes already say
#     whether it worked.
