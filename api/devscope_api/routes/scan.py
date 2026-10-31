# scan.py - POST /scan (scan a directory) and related endpoints.
# Validates input, calls scanner_bridge, returns schemas.
#   POST /scan       line/language stats for one project
#   POST /workspace  discover every project under a root folder
#   POST /secrets    possible leaked secrets in one project

from fastapi import APIRouter, HTTPException

from .. import scanner_bridge as bridge
from ..schemas import PathRequest, ProjectOut, ScanResponse, SecretOut, WorkspaceRequest

router = APIRouter()


def _bad_path(err: NotADirectoryError) -> HTTPException:
    return HTTPException(status_code=400, detail=f"Not a folder: {err}")


@router.post("/scan", response_model=ScanResponse)
def scan(req: PathRequest):
    try:
        return bridge.scan_project(req.path)
    except NotADirectoryError as e:
        raise _bad_path(e)


@router.post("/workspace", response_model=list[ProjectOut])
def workspace(req: WorkspaceRequest):
    try:
        return bridge.discover_workspace(req.root, req.max_depth)
    except NotADirectoryError as e:
        raise _bad_path(e)


@router.post("/secrets", response_model=list[SecretOut])
def secrets(req: PathRequest):
    try:
        return bridge.find_secrets(req.path)
    except NotADirectoryError as e:
        raise _bad_path(e)
