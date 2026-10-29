"""Background scan jobs + a bounded in-memory cache of finished results.

PURPOSE
    Scanning a large monorepo can take seconds. Holding an HTTP request open
    that long is fragile (timeouts, no progress, can't cancel), so scans are
    *jobs*:

        POST /scans          → creates a job, returns {id, status: "queued"} at once
        GET  /scans/{id}     → dashboard polls until status is "done" / "failed"
        GET  /scans          → recent jobs (for a "recent projects" sidebar)

    Later upgrade path: push progress with Server-Sent Events or a WebSocket
    instead of polling — the job model stays the same.

CONCURRENCY MODEL
    * A small ``ThreadPoolExecutor`` (e.g. 2 workers) runs scans. Rust already
      parallelises *inside* a scan, so more Python workers would just fight
      over the same cores.
    * Job state is guarded by a ``threading.Lock`` because worker threads
      update it while the event loop reads it.
    * Finished jobs live in an ``OrderedDict`` used as an LRU, capped at
      ``Settings.max_cached_scans`` — memory never grows without bound.
"""

# =============================================================================
# LOGIC SKETCH (commented out — not implemented yet)
# =============================================================================
#
# import logging
# import threading
# from collections import OrderedDict
# from concurrent.futures import ThreadPoolExecutor
# from datetime import UTC, datetime
# from uuid import UUID, uuid4
#
# from devscope_api import schemas
# from devscope_api.scanner_bridge import ScanFailed, Scanner
#
# log = logging.getLogger(__name__)
#
#
# class ScanJobService:
#     def __init__(self, scanner: Scanner, max_cached: int = 10, workers: int = 2) -> None:
#         self._scanner = scanner
#         self._max_cached = max_cached
#         self._pool = ThreadPoolExecutor(max_workers=workers, thread_name_prefix="scan")
#         self._jobs: OrderedDict[UUID, schemas.ScanJob] = OrderedDict()
#         self._lock = threading.Lock()
#
#     def submit(self, request: schemas.ScanRequest) -> schemas.ScanJob:
#         job = schemas.ScanJob(id=uuid4(), status=schemas.JobStatus.QUEUED,
#                               request=request, created_at=datetime.now(UTC))
#         with self._lock:
#             self._jobs[job.id] = job
#             self._evict_locked()
#         self._pool.submit(self._run, job.id)
#         return job
#
#     def get(self, job_id: UUID) -> schemas.ScanJob | None: ...
#     def recent(self, limit: int = 20) -> list[schemas.ScanJob]: ...
#
#     def _run(self, job_id: UUID) -> None:
#         self._update(job_id, status=schemas.JobStatus.RUNNING)
#         try:
#             report = self._scanner.scan(self._jobs[job_id].request)
#         except ScanFailed as exc:
#             self._update(job_id, status=schemas.JobStatus.FAILED, error=str(exc))
#         except Exception:
#             log.exception("unexpected scan failure")            # full traceback in logs
#             self._update(job_id, status=schemas.JobStatus.FAILED, error="internal error")
#         else:
#             self._update(job_id, status=schemas.JobStatus.DONE, report=report)
#
#     def _update(self, job_id: UUID, **changes: object) -> None:
#         with self._lock:                                         # models are frozen →
#             self._jobs[job_id] = self._jobs[job_id].model_copy(update=changes)  # replace
#
#     def _evict_locked(self) -> None:
#         while len(self._jobs) > self._max_cached:
#             self._jobs.popitem(last=False)                       # drop the oldest
#
#     def shutdown(self) -> None:
#         self._pool.shutdown(wait=False, cancel_futures=True)

# =============================================================================
# JUNIOR vs SENIOR — caching and shared state
# =============================================================================
#
# ── JUNIOR ──────────────────────────────────────────────────────────────────
#
#     cache = {}
#
#     def get_scan(path):
#         if path in cache:                   # stale forever: files changed since?
#             return cache[path]
#         cache[path] = devscope_scanner.scan(path)   # unbounded growth
#         return cache[path]
#
#     # Called from several threads → two identical scans race and both run.
#
# ── SENIOR ──────────────────────────────────────────────────────────────────
#
#   * Cache keyed by job id (and later by path + options + a cheap "changed?"
#     check such as the root's mtime / git HEAD), never by path alone.
#   * Bounded size with LRU eviction.
#   * One lock around every read-modify-write of shared state.
#   * Immutable job records replaced atomically (`model_copy(update=...)`), so
#     a reader never sees a half-updated job.
#   * Lifecycle owned by the app's lifespan: created on startup, executor shut
#     down on exit — no stray threads keeping the process alive.
