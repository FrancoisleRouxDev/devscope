# test_scan.py - API tests for /scan and /health using FastAPI's TestClient.

from fastapi.testclient import TestClient

from devscope_api.main import app

client = TestClient(app)


def make_project(root):
    """A tiny node project with a junk folder and a fake leaked key."""
    proj = root / "demo"
    (proj / "node_modules" / "dep").mkdir(parents=True)
    (proj / "package.json").write_text("{}")
    (proj / "app.js").write_text("// hi\nconsole.log(1);\n\n")
    (proj / "node_modules" / "dep" / "x.js").write_text("x" * 50)
    (proj / "config.txt").write_text("key = AKIAABCDEFGHIJKLMNOP\n")
    return proj


def test_health():
    assert client.get("/health").json() == {"status": "ok"}


def test_scan_counts_lines(tmp_path):
    proj = make_project(tmp_path)
    data = client.post("/scan", json={"path": str(proj)}).json()
    js = next(l for l in data["languages"] if l["name"] == "JavaScript")
    assert (js["code"], js["comments"], js["blanks"]) == (1, 1, 1)


def test_scan_rejects_missing_folder(tmp_path):
    r = client.post("/scan", json={"path": str(tmp_path / "nope")})
    assert r.status_code == 400


def test_workspace_finds_project_and_junk(tmp_path):
    make_project(tmp_path)
    projects = client.post("/workspace", json={"root": str(tmp_path)}).json()
    assert [p["name"] for p in projects] == ["demo"]
    assert projects[0]["junk_bytes"] == 50
    assert projects[0]["junk_dirs"] == ["node_modules"]
    assert projects[0]["git"] is None  # not a git repo


def test_folder_analysis(tmp_path):
    (tmp_path / "sub").mkdir()
    (tmp_path / "photo.jpg").write_bytes(b"x" * 300)
    (tmp_path / "setup.exe").write_bytes(b"x" * 200)
    (tmp_path / "report (1).pdf").write_bytes(b"x" * 50)
    (tmp_path / "sub" / "film.mp4").write_bytes(b"x" * 1000)
    data = client.post("/folder", json={"path": str(tmp_path)}).json()
    assert (data["total_files"], data["total_bytes"]) == (4, 1550)
    assert data["categories"][0]["name"] == "Video"
    assert data["largest_dirs"][0]["name"] == "sub"
    assert {c["reason"] for c in data["clutter"]} == {"installer", "copy"}
    assert len(data["clutter"]) == 2
    assert data["project_count"] == 0


def test_folder_detects_projects(tmp_path):
    make_project(tmp_path)
    assert client.post("/folder", json={"path": str(tmp_path)}).json()["project_count"] == 1


def test_duplicates_only_identical(tmp_path):
    (tmp_path / "a.bin").write_bytes(b"A" * 2048)
    (tmp_path / "b.bin").write_bytes(b"A" * 2048)
    (tmp_path / "c.bin").write_bytes(b"B" * 2048)  # same size, different content
    groups = client.post("/duplicates", json={"path": str(tmp_path)}).json()
    assert len(groups) == 1
    assert groups[0]["wasted"] == 2048 and len(groups[0]["paths"]) == 2


def test_secrets_found_without_value(tmp_path):
    proj = make_project(tmp_path)
    found = client.post("/secrets", json={"path": str(proj)}).json()
    assert {"path": str(proj / "config.txt"), "line": 1, "kind": "aws-access-key"} in found
    assert "AKIA" not in str(found)


# --- security -------------------------------------------------------------------------------

def make_git_repo(path):
    """A minimal git repo (needs the git CLI, which the app itself also requires)."""
    import subprocess

    path.mkdir(parents=True, exist_ok=True)
    subprocess.run(["git", "init", "-q", str(path)], check=True)
    (path / "a.txt").write_text("hi")


def test_hostile_git_config_cannot_run_programs(tmp_path):
    """A repo whose config names a program (core.fsmonitor) must not make us execute it."""
    import subprocess

    from devscope_api import scanner_bridge

    repo = tmp_path / "evil"
    make_git_repo(repo)
    marker = tmp_path / "PWNED.txt"
    script = tmp_path / "evil.sh"
    script.write_text(f'#!/bin/sh\necho ran > "{marker.as_posix()}"\nexit 1\n')
    subprocess.run(["git", "-C", str(repo), "config", "core.fsmonitor", script.as_posix()], check=True)

    status = scanner_bridge.git_status(str(repo))
    assert not marker.exists(), "git ran a program named in the repository's own config"
    assert status is not None and status.uncommitted == 1  # still reports normally


def test_repo_with_filter_config_is_not_checked(tmp_path):
    """Clean filters run during `git status`; repos defining them are skipped and flagged."""
    import subprocess

    from devscope_api import scanner_bridge

    repo = tmp_path / "filtered"
    make_git_repo(repo)
    subprocess.run(["git", "-C", str(repo), "config", "filter.x.clean", "cat"], check=True)

    status = scanner_bridge.git_status(str(repo))
    assert status is not None and status.checked is False


def test_new_repo_branch_name(tmp_path):
    from devscope_api import scanner_bridge

    repo = tmp_path / "fresh"
    make_git_repo(repo)  # no commits yet: git prints "No commits yet on <branch>"
    status = scanner_bridge.git_status(str(repo))
    assert status.branch not in (None, "No")


def test_api_requires_token_when_configured(tmp_path, monkeypatch):
    monkeypatch.setenv("DEVSCOPE_TOKEN", "s3cret")
    body = {"path": str(tmp_path)}
    assert client.get("/health").status_code == 200  # open on purpose
    assert client.post("/folder", json=body).status_code == 401
    assert client.post("/folder", json=body, headers={"X-Devscope-Token": "wrong"}).status_code == 401
    assert client.post("/folder", json=body, headers={"X-Devscope-Token": "s3cret"}).status_code == 200
