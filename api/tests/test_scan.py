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


def test_secrets_found_without_value(tmp_path):
    proj = make_project(tmp_path)
    found = client.post("/secrets", json={"path": str(proj)}).json()
    assert {"path": str(proj / "config.txt"), "line": 1, "kind": "aws-access-key"} in found
    assert "AKIA" not in str(found)
