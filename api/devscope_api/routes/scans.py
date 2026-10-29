"""``/scans`` — start scans and fetch their results.

PURPOSE
    HTTP surface for ``services.scan_jobs.ScanJobService``. Every route here
    requires the bearer token (``security.require_token``).

    POST /scans          body: ScanRequest  → 202 Accepted + ScanJob (status "queued")
    GET  /scans          → list[ScanJob]    (most recent first, reports omitted)
    GET  /scans/{id}     → ScanJob          (404 if unknown / evicted)
"""

# =============================================================================
# LOGIC SKETCH (commented out — not implemented yet)
# =============================================================================
#
# from typing import Annotated
# from uuid import UUID
#
# from fastapi import APIRouter, Depends, HTTPException, Request, status
#
# from devscope_api import schemas
# from devscope_api.security import require_token
# from devscope_api.services.scan_jobs import ScanJobService
#
# router = APIRouter(prefix="/scans", tags=["scans"], dependencies=[Depends(require_token)])
#
#
# def get_jobs(request: Request) -> ScanJobService:
#     """Dependency → tests swap it with app.dependency_overrides[get_jobs]."""
#     return request.app.state.jobs
#
# Jobs = Annotated[ScanJobService, Depends(get_jobs)]
#
#
# @router.post("", status_code=status.HTTP_202_ACCEPTED, response_model=schemas.ScanJob)
# async def start_scan(body: schemas.ScanRequest, jobs: Jobs) -> schemas.ScanJob:
#     return jobs.submit(body)            # returns immediately; work runs in a thread
#
#
# @router.get("", response_model=list[schemas.ScanJob])
# async def list_scans(jobs: Jobs) -> list[schemas.ScanJob]:
#     return jobs.recent()
#
#
# @router.get("/{job_id}", response_model=schemas.ScanJob)
# async def get_scan(job_id: UUID, jobs: Jobs) -> schemas.ScanJob:
#     job = jobs.get(job_id)
#     if job is None:
#         raise HTTPException(status.HTTP_404_NOT_FOUND, "scan not found")
#     return job
