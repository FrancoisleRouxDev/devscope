# =============================================================================
# scanner/devscope_scanner.pyi — Python TYPE STUBS for the native Rust module.
# =============================================================================
# Python cannot introspect a compiled extension's signatures, so without this
# file editors and mypy see `devscope_scanner` as `Any`. This stub is the
# single written-down contract between Rust (src/python.rs) and Python
# (api/devscope_api/scanner_bridge.py). Keep all three in sync.
#
# No logic lives here — stubs only describe shapes.
# =============================================================================

# --- Planned stubs (uncomment when src/python.rs exposes them) ---------------
#
# from typing import final
#
# __version__: str
#
# @final
# class LanguageStats:
#     language: str
#     files: int
#     code: int
#     comments: int
#     blanks: int
#     bytes: int
#
# @final
# class FileStats:
#     path: str          # relative to the scan root, always "/"-separated
#     language: str
#     code: int
#     comments: int
#     blanks: int
#     bytes: int
#
# @final
# class ScanReport:
#     root: str
#     total_files: int
#     total_lines: int
#     languages: list[LanguageStats]
#     largest_files: list[FileStats]
#     skipped: int
#     duration_ms: int
#
# class ScanError(Exception): ...
#
# def scan(
#     root: str,
#     *,
#     respect_gitignore: bool = True,
#     include_hidden: bool = False,
#     max_file_size: int = 5_000_000,
#     top_n_files: int = 20,
# ) -> ScanReport:
#     """Scan `root` recursively. Releases the GIL while working.
#
#     Raises:
#         ScanError: if `root` does not exist or is not a directory.
#     """
