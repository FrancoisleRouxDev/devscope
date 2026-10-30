// backend.js - manages the Python API child process.
// Finds a free port, spawns uvicorn (dev: venv Python; prod: bundled
// executable), waits for GET /health, and kills the process on app quit.
// Handles platform differences (Windows/macOS/Linux paths and signals).
