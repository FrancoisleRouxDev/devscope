# schemas.py - Pydantic models defining the JSON contract between the API
# and the dashboard (ScanRequest, ScanResponse, LanguageStat, ...).
# Must stay in sync with the Rust types and dashboard/src/lib/api.js.

from pydantic import BaseModel, Field

from .config import DEFAULT_MAX_DEPTH


class PathRequest(BaseModel):
    path: str = Field(min_length=1)


class WorkspaceRequest(BaseModel):
    root: str = Field(min_length=1)
    max_depth: int = Field(default=DEFAULT_MAX_DEPTH, ge=1, le=10)


class LanguageStat(BaseModel):
    name: str
    files: int
    code: int
    comments: int
    blanks: int


class FileStat(BaseModel):
    path: str
    language: str
    code: int
    comments: int
    blanks: int


class ScanResponse(BaseModel):
    root: str
    total_files: int
    total_lines: int
    code: int
    comments: int
    blanks: int
    duration_ms: int
    languages: list[LanguageStat]
    largest_files: list[FileStat]


class GitStatus(BaseModel):
    branch: str | None
    uncommitted: int  # changed or untracked files
    ahead: int  # commits not yet pushed
    has_remote: bool


class ProjectOut(BaseModel):
    path: str
    name: str
    kinds: list[str]
    size_bytes: int
    junk_bytes: int
    junk_dirs: list[str]
    last_modified: int  # unix seconds, 0 if unknown
    git: GitStatus | None  # None when the project is not a git repository


class CategoryOut(BaseModel):
    name: str
    files: int
    bytes: int


class FolderFileOut(BaseModel):
    path: str
    size: int
    modified: int  # unix seconds, 0 if unknown
    category: str
    reason: str  # why it is listed as clutter; empty otherwise


class DirOut(BaseModel):
    path: str
    name: str
    files: int
    bytes: int


class AgeBucketOut(BaseModel):
    label: str
    files: int
    bytes: int


class FolderResponse(BaseModel):
    root: str
    total_files: int
    total_bytes: int
    duration_ms: int
    project_count: int  # code projects found inside the folder
    categories: list[CategoryOut]
    largest_files: list[FolderFileOut]
    largest_dirs: list[DirOut]
    age_buckets: list[AgeBucketOut]
    clutter: list[FolderFileOut]


class DuplicateGroupOut(BaseModel):
    size: int
    paths: list[str]
    wasted: int  # bytes freed by keeping one copy


class SecretOut(BaseModel):
    path: str
    line: int  # 0 = whole-file finding
    kind: str
