# health.py - GET /health. Lets Electron poll until the backend is ready.

from fastapi import APIRouter

router = APIRouter()


@router.get("/health")
def health() -> dict[str, str]:
    return {"status": "ok"}
