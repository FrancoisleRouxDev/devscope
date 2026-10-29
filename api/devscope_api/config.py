"""Typed runtime configuration, loaded from ``DEVSCOPE_*`` environment variables.

PURPOSE
    One ``Settings`` object instead of ``os.environ.get(...)`` scattered across
    the codebase. Values are validated at startup, so a bad config fails
    immediately with a clear message rather than deep inside a request.

VARIABLES (planned)
    DEVSCOPE_TOKEN             per-launch bearer token, set by Electron (required)
    DEVSCOPE_LOG_LEVEL         "info" by default
    DEVSCOPE_ALLOWED_ORIGINS   JSON list, e.g. '["http://localhost:5173"]'
    DEVSCOPE_MAX_CACHED_SCANS  how many finished scan results to keep in memory
    DEVSCOPE_SCAN_TIMEOUT_S    abort scans that run longer than this
"""

# =============================================================================
# LOGIC SKETCH (commented out — not implemented yet)
# =============================================================================
#
# from pydantic import Field, SecretStr
# from pydantic_settings import BaseSettings, SettingsConfigDict
#
#
# class Settings(BaseSettings):
#     model_config = SettingsConfigDict(env_prefix="DEVSCOPE_", frozen=True)
#
#     token: SecretStr                                   # SecretStr → never printed in logs/reprs
#     log_level: str = "info"
#     allowed_origins: list[str] = Field(default_factory=lambda: ["http://localhost:5173"])
#     max_cached_scans: int = Field(default=10, ge=1, le=100)
#     scan_timeout_s: float = Field(default=300.0, gt=0)

# =============================================================================
# JUNIOR vs SENIOR — configuration
# =============================================================================
#
# ── JUNIOR ──────────────────────────────────────────────────────────────────
#
#     import os
#     PORT = int(os.environ.get("PORT", 8000))          # crashes on PORT="abc" at import
#     TOKEN = os.getenv("TOKEN")                        # None → auth silently disabled?
#     print("starting with token", TOKEN)               # secret now in the log file
#
# ── SENIOR ──────────────────────────────────────────────────────────────────
#
#   * A single frozen, typed `Settings` model; constraints (`ge`, `gt`) encode
#     the valid ranges.
#   * Required secrets have NO default — missing token = refuse to start.
#   * `SecretStr` keeps secrets out of logs, tracebacks and `repr()`.
#   * A unique prefix (`DEVSCOPE_`) so we never pick up some other tool's `PORT`.
#   * Injected into `create_app()`, so tests pass `Settings(token="test")`
#     directly instead of monkey-patching `os.environ`.
