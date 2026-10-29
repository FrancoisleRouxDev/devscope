"""Tests for the ``/scans`` routes and the job lifecycle."""

# =============================================================================
# PLANNED TESTS (commented out — not implemented yet)
# =============================================================================
#
# def test_requires_token(client):                 ...  # no header → 401
# def test_wrong_token_rejected(client):           ...  # bad header → 401
# def test_post_returns_202_and_job_id(client):    ...
# def test_job_eventually_done(client):            ...  # poll GET /scans/{id}
# def test_missing_folder_marks_job_failed(client, tmp_path): ...
# def test_unknown_job_is_404(client):             ...
# def test_invalid_body_is_422(client):            ...  # e.g. topNFiles = 0
# def test_cache_evicts_oldest(client):            ...  # > max_cached_scans jobs
# def test_response_uses_camel_case(client):       ...  # "totalFiles", not "total_files"
