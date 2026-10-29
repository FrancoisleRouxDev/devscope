"""Tests for ``GET /health``."""

# =============================================================================
# PLANNED TESTS (commented out — not implemented yet)
# =============================================================================
#
# def test_health_is_ok_without_token(client):
#     client.headers.pop("Authorization")
#     r = client.get("/health")
#     assert r.status_code == 200
#     assert r.json()["status"] == "ok"
#
# def test_health_reports_scanner_version(client):
#     assert client.get("/health").json()["scannerVersion"] == "fake-0.0.0"
