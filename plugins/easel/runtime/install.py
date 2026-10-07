"""Easel-owned dependency installation, invoked by the generic Host install hook."""

from __future__ import annotations

import json
import os
import shutil
import subprocess
import sys
import venv
from pathlib import Path


def main() -> int:
    workspace = Path(os.environ["OPENMUSE_PLUGIN_WORKSPACE"]).resolve()
    source = Path(__file__).resolve().parent
    runtime = workspace / "runtime"
    python = runtime / "venv" / ("Scripts/python.exe" if os.name == "nt" else "bin/python")
    browsers = runtime / "browsers"
    runtime.mkdir(parents=True, exist_ok=True)
    if python.exists() and subprocess.run([str(python), "-c", "import encodings"], capture_output=True).returncode != 0:
        shutil.rmtree(python.parent.parent)
    if not python.exists():
        print("Easel install: creating isolated Python environment", flush=True)
        venv.EnvBuilder(with_pip=True, symlinks=True).create(python.parent.parent)
    print("Easel install: installing declared Python dependencies", flush=True)
    subprocess.run([str(python), "-m", "pip", "install", "--disable-pip-version-check", "-r", str(source / "requirements.txt")], check=True)
    environment = os.environ.copy()
    environment["PLAYWRIGHT_BROWSERS_PATH"] = str(browsers)
    print("Easel install: installing Chromium", flush=True)
    subprocess.run([str(python), "-m", "playwright", "install", "chromium"], check=True, env=environment)
    if shutil.which("ffmpeg") is None or shutil.which("ffprobe") is None:
        raise RuntimeError("Easel video processing requires ffmpeg and ffprobe on PATH")
    subprocess.run([str(python), "-c", "import PIL, playwright"], check=True)
    status = {"python": str(python), "browsers": str(browsers), "ffmpeg": shutil.which("ffmpeg"), "ready": True}
    (runtime / "install-status.json").write_text(json.dumps(status, ensure_ascii=False, indent=2), encoding="utf-8")
    print("Easel install: ready", flush=True)
    return 0


if __name__ == "__main__":
    sys.exit(main())
