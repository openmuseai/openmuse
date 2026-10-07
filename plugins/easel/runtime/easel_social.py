"""Easel social entrypoint for OpenMuse. Owns defaults and interaction events."""

from __future__ import annotations

import json
import os
import subprocess
import sys
import time
import uuid
from pathlib import Path


def _option(args: list[str], name: str) -> str | None:
    try:
        return args[args.index(name) + 1]
    except (ValueError, IndexError):
        return None


def _emit_image_challenge(status_file: Path, workspace: Path) -> bool:
    try:
        status = json.loads(status_file.read_text(encoding="utf-8"))
        if status.get("state") != "qr_ready":
            return False
        image = Path(status["qr"]).resolve()
        if not image.is_file() or not image.is_relative_to(workspace):
            return False
        inbox_raw = os.environ.get("OPENMUSE_PLUGIN_INTERACTION_DIR")
        plugin_id = os.environ.get("OPENMUSE_PLUGIN_ID")
        if not inbox_raw or plugin_id != "com.openmuse.easel":
            return False
        inbox = Path(inbox_raw).resolve()
        inbox.mkdir(parents=True, exist_ok=True)
        event = {
            "protocol": "openmuse.plugin-interaction/v1",
            "type": "image.challenge",
            "pluginId": plugin_id,
            "title": status.get("message") or "扫码登录",
            "imagePath": str(image),
            "statusPath": str(status_file),
            "issuedAt": int(time.time()),
        }
        target = inbox / f"{uuid.uuid4().hex}.json"
        temporary = target.with_suffix(".tmp")
        temporary.write_text(json.dumps(event, ensure_ascii=False), encoding="utf-8")
        temporary.replace(target)
        print(json.dumps({"stage": "authorization_required", "interaction": "image.challenge", "statusFile": str(status_file)}, ensure_ascii=False), flush=True)
        return True
    except (OSError, KeyError, ValueError, json.JSONDecodeError):
        return False


def main(argv: list[str]) -> int:
    if len(argv) < 2 or argv[0] not in {"douyin", "zhihu"}:
        raise ValueError("usage: easel_social.py <douyin|zhihu> <command> [options]")
    platform, command, *options = argv
    workspace = Path(os.environ["OPENMUSE_PLUGIN_WORKSPACE"]).resolve()
    os.environ["PLAYWRIGHT_BROWSERS_PATH"] = str(workspace / "runtime" / "browsers")
    script = Path(__file__).resolve().parents[1] / "shared" / "scripts" / (
        "douyin_publish.py" if platform == "douyin" else "web_publisher.py"
    )
    upstream_command = "login-qr" if platform == "zhihu" and command == "login" else command
    translated = [upstream_command]
    if platform == "zhihu" and upstream_command not in {"check", "selftest", "platforms"}:
        translated.extend(["--platform", "zhihu"])
    translated.extend(options)
    if command in {"login", "whoami", "publish", "publish-video"}:
        if _option(translated, "--profile-base") is None:
            translated.extend(["--profile-base", str(workspace / "profiles")])
    status_file = workspace / "state" / f"{platform}-login-{uuid.uuid4().hex}.json"
    if command == "login":
        image_file = workspace / "state" / f"{platform}-login-{uuid.uuid4().hex}.png"
        if _option(translated, "--qr-out") is None:
            translated.extend(["--qr-out", str(image_file)])
        if _option(translated, "--status-file") is None:
            translated.extend(["--status-file", str(status_file)])
        status_file = Path(_option(translated, "--status-file") or status_file).resolve()
    process = subprocess.Popen([sys.executable, str(script), *translated], cwd=workspace)
    emitted = False
    if command == "login":
        while process.poll() is None:
            if not emitted:
                emitted = _emit_image_challenge(status_file, workspace)
            time.sleep(0.25)
        if not emitted:
            _emit_image_challenge(status_file, workspace)
    return process.wait()


if __name__ == "__main__":
    try:
        sys.exit(main(sys.argv[1:]))
    except Exception as error:
        print(json.dumps({"stage": "error", "error": str(error)}, ensure_ascii=False), file=sys.stderr)
        sys.exit(1)
