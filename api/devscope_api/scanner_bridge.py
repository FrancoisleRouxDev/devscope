# scanner_bridge.py - thin wrapper around the compiled devscope_scanner
# Rust module. Keeps PyO3 details out of the routes and runs scans off the
# event loop (thread pool) so the API stays responsive.
# (Routes are plain `def`, which FastAPI already runs in a thread pool; the
# Rust calls also release the GIL, so scans run in parallel.)

import subprocess
from pathlib import Path

import devscope_scanner as scanner

from .config import GIT_TIMEOUT
from .schemas import (
    AgeBucketOut,
    CategoryOut,
    DirOut,
    DuplicateGroupOut,
    FileStat,
    FolderFileOut,
    FolderResponse,
    GitStatus,
    LanguageStat,
    ProjectOut,
    ScanResponse,
    SecretOut,
)


def _require_dir(path: str) -> str:
    """Resolve to an absolute path and ensure it is a directory."""
    p = Path(path).expanduser().resolve()
    if not p.is_dir():
        raise NotADirectoryError(str(p))
    return str(p)


def scan_project(path: str) -> ScanResponse:
    r = scanner.scan_project(_require_dir(path))
    return ScanResponse(
        root=r.root,
        total_files=r.total_files,
        total_lines=r.total_lines,
        code=r.code,
        comments=r.comments,
        blanks=r.blanks,
        duration_ms=r.duration_ms,
        languages=[
            LanguageStat(name=l.name, files=l.files, code=l.code, comments=l.comments, blanks=l.blanks)
            for l in r.languages
        ],
        largest_files=[
            FileStat(path=f.path, language=f.language, code=f.code, comments=f.comments, blanks=f.blanks)
            for f in r.largest_files
        ],
    )


def _folder_file(f) -> FolderFileOut:
    return FolderFileOut(path=f.path, size=f.size, modified=f.modified, category=f.category, reason=f.reason)


def analyze_folder(path: str) -> FolderResponse:
    r = scanner.analyze_folder(_require_dir(path))
    return FolderResponse(
        root=r.root,
        total_files=r.total_files,
        total_bytes=r.total_bytes,
        duration_ms=r.duration_ms,
        project_count=r.project_count,
        categories=[CategoryOut(name=c.name, files=c.files, bytes=c.bytes) for c in r.categories],
        largest_files=[_folder_file(f) for f in r.largest_files],
        largest_dirs=[DirOut(path=d.path, name=d.name, files=d.files, bytes=d.bytes) for d in r.largest_dirs],
        age_buckets=[AgeBucketOut(label=a.label, files=a.files, bytes=a.bytes) for a in r.age_buckets],
        clutter=[_folder_file(f) for f in r.clutter],
    )


def find_duplicates(path: str) -> list[DuplicateGroupOut]:
    return [
        DuplicateGroupOut(size=g.size, paths=list(g.paths), wasted=g.wasted)
        for g in scanner.find_duplicates(_require_dir(path))
    ]


def find_secrets(path: str) -> list[SecretOut]:
    return [SecretOut(path=s.path, line=s.line, kind=s.kind) for s in scanner.find_secrets(_require_dir(path))]


def _git(project: str, *args: str) -> str | None:
    """Run a git command in `project`; None if git is missing or the command fails."""
    try:
        out = subprocess.run(
            ["git", "-C", project, *args],
            capture_output=True,
            text=True,
            timeout=GIT_TIMEOUT,
            check=True,
        )
        return out.stdout
    except (OSError, subprocess.SubprocessError):
        return None


def git_status(project: str) -> GitStatus | None:
    """Branch, uncommitted file count, unpushed commits and remote presence."""
    if not (Path(project) / ".git").exists():
        return None
    porcelain = _git(project, "status", "--porcelain=v1", "--branch")
    if porcelain is None:
        return None

    lines = porcelain.splitlines()
    header = lines[0] if lines else ""  # e.g. "## main...origin/main [ahead 2]"
    branch = header.removeprefix("## ").split("...")[0].split(" ")[0] or None
    ahead = 0
    if "[ahead " in header:
        ahead = int(header.split("[ahead ")[1].split("]")[0].split(",")[0])

    remotes = _git(project, "remote") or ""
    has_remote = bool(remotes.strip())
    if not has_remote:
        # With no remote nothing is ever pushed, so count all commits as unpushed.
        count = _git(project, "rev-list", "--count", "HEAD")
        ahead = int(count) if count and count.strip().isdigit() else 0

    return GitStatus(branch=branch, uncommitted=max(len(lines) - 1, 0), ahead=ahead, has_remote=has_remote)


def discover_workspace(root: str, max_depth: int) -> list[ProjectOut]:
    projects = scanner.discover_projects(_require_dir(root), max_depth)
    return [
        ProjectOut(
            path=p.path,
            name=p.name,
            kinds=list(p.kinds),
            size_bytes=p.size_bytes,
            junk_bytes=p.junk_bytes,
            junk_dirs=list(p.junk_dirs),
            last_modified=p.last_modified,
            git=git_status(p.path),
        )
        for p in projects
    ]
