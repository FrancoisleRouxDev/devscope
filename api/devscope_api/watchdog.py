# watchdog.py - makes the API exit when the Electron app that started it is gone.
# Electron normally stops the API on quit, but a crash or force-kill skips that
# and would leave the server running forever. Electron passes its process id in
# DEVSCOPE_PARENT_PID; a background thread polls it and exits when it vanishes.

import os
import sys
import threading
import time


def _alive(pid: int) -> bool:
    if sys.platform == "win32":
        # os.kill(pid, 0) would TERMINATE the process on Windows, so ask the OS instead.
        import ctypes

        PROCESS_QUERY_LIMITED_INFORMATION = 0x1000
        STILL_ACTIVE = 259
        handle = ctypes.windll.kernel32.OpenProcess(PROCESS_QUERY_LIMITED_INFORMATION, False, pid)
        if not handle:
            return False
        code = ctypes.c_ulong()
        ok = ctypes.windll.kernel32.GetExitCodeProcess(handle, ctypes.byref(code))
        ctypes.windll.kernel32.CloseHandle(handle)
        return bool(ok) and code.value == STILL_ACTIVE
    try:
        os.kill(pid, 0)  # signal 0 only checks existence on macOS/Linux
        return True
    except ProcessLookupError:
        return False
    except PermissionError:
        return True  # exists, owned by someone else


def start() -> None:
    """Start the watchdog if a parent pid was provided."""
    raw = os.environ.get("DEVSCOPE_PARENT_PID")
    if not raw or not raw.isdigit():
        return
    pid = int(raw)

    def watch() -> None:
        while True:
            time.sleep(2)
            if not _alive(pid):
                os._exit(0)  # the parent is gone; nothing left to serve

    threading.Thread(target=watch, daemon=True, name="parent-watchdog").start()
