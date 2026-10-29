"""Smoke tests against the REAL Rust extension.

Skipped automatically when ``devscope_scanner`` isn't built/installed, so the
rest of the suite still runs on a machine without a Rust toolchain.
"""

# =============================================================================
# PLANNED TESTS (commented out — not implemented yet)
# =============================================================================
#
# import pytest
#
# native = pytest.importorskip("devscope_scanner")
#
# def test_scans_a_tiny_tree(tmp_path): ...           # write 2 files, assert counts
# def test_missing_root_raises_scan_failed(tmp_path): ...
# def test_scan_releases_the_gil(tmp_path): ...       # a 2nd thread keeps running meanwhile
