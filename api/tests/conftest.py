"""Shared pytest fixtures.

PURPOSE
    Build a fresh app per test with test settings and a FAKE scanner, so the
    API suite runs in milliseconds and needs no Rust toolchain. The real Rust
    extension gets its own small smoke test (``test_scanner_bridge.py``,
    skipped if ``devscope_scanner`` isn't installed).
"""

# =============================================================================
# LOGIC SKETCH (commented out — not implemented yet)
# =============================================================================
#
# import pytest
# from fastapi.testclient import TestClient
#
# from devscope_api import schemas
# from devscope_api.app import create_app
# from devscope_api.config import Settings
#
# TEST_TOKEN = "test-token"
#
#
# class FakeScanner:
#     version = "fake-0.0.0"
#     def scan(self, request: schemas.ScanRequest) -> schemas.ScanReport:
#         return schemas.ScanReport(root=str(request.path), total_files=1, total_lines=10, ...)
#
#
# @pytest.fixture
# def client() -> TestClient:
#     app = create_app(Settings(token=TEST_TOKEN), scanner=FakeScanner())
#     with TestClient(app) as c:             # `with` runs lifespan startup/shutdown
#         c.headers["Authorization"] = f"Bearer {TEST_TOKEN}"
#         yield c
