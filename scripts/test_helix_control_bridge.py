#!/usr/bin/env python3
"""Exercise the packaged hx over a real PTY and authenticated Host socket."""

import json
import os
import pty
import socket
import subprocess
import sys
import tempfile
import threading
from pathlib import Path


def main() -> None:
    repo = Path(__file__).resolve().parent.parent
    engine = repo / "plugins/helix/assets/engines/helix"
    with tempfile.TemporaryDirectory(prefix="openmuse-hx-bridge-") as tmp:
        root = Path(tmp)
        source = root / "hello.rs"
        source.write_text("fn main() {}\n", encoding="utf-8")
        config = root / "config.toml"
        config.write_text(
            '[editor]\ninput-profile = "standard-nonmodal"\n',
            encoding="utf-8",
        )
        listener = socket.socket(socket.AF_INET, socket.SOCK_STREAM)
        listener.bind(("127.0.0.1", 0))
        listener.listen(1)
        listener.settimeout(10)
        master, slave = pty.openpty()
        token = os.urandom(32).hex()
        env = os.environ.copy()
        env.update(
            TERM="xterm-256color",
            HELIX_RUNTIME=str(engine / "runtime"),
            OPENMUSE_HELIX_CONTROL_ADDR=f"127.0.0.1:{listener.getsockname()[1]}",
            OPENMUSE_HELIX_CONTROL_TOKEN=token,
            XDG_CONFIG_HOME=str(root / "xdg"),
        )
        process = subprocess.Popen(
            [str(engine / "hx"), "--config", str(config), str(source)],
            stdin=slave,
            stdout=slave,
            stderr=slave,
            cwd=root,
            env=env,
            start_new_session=True,
        )
        os.close(slave)
        def drain_terminal() -> None:
            while True:
                try:
                    chunk = os.read(master, 65536)
                except OSError:
                    return
                if not chunk:
                    return
                # PTY rendering is deliberately separate from the JSON socket.

        reader = threading.Thread(target=drain_terminal, daemon=True)
        reader.start()
        try:
            peer, _ = listener.accept()
            with peer:
                peer.settimeout(10)
                lines = peer.makefile("r", encoding="utf-8")
                hello = json.loads(lines.readline())
                state = json.loads(lines.readline())
                assert hello == {
                    "version": 1,
                    "type": "hello",
                    "token": token,
                    "pid": process.pid,
                }, hello
                assert state["version"] == 1 and state["type"] == "state", state
                assert Path(state["path"]).resolve() == source.resolve(), state
                assert state["dirty"] is False, state
                os.write(master, b"x")
                while True:
                    dirty = json.loads(lines.readline())
                    if dirty["type"] == "state" and dirty["dirty"]:
                        break

                def request(request_id: int, command: str, revision: int) -> tuple[dict, list]:
                    peer.sendall(
                        (json.dumps({
                            "version": 1,
                            "type": "command",
                            "id": request_id,
                            "command": command,
                            "path": dirty["path"],
                            "revision": revision,
                        }) + "\n").encode("utf-8")
                    )
                    events = []
                    while True:
                        event = json.loads(lines.readline())
                        if event.get("type") == "result" and event.get("id") == request_id:
                            return event, events
                        events.append(event)

                stale, _ = request(1, "save", state["revision"])
                assert not stale["ok"] and stale["error"] == "stale_revision", stale
                saved_result, events = request(2, "save", dirty["revision"])
                assert saved_result["ok"] and not saved_result["dirty"], saved_result
                assert any(event["type"] == "saved" for event in events), events
                assert source.read_text(encoding="utf-8").startswith("x"), saved_result

                undone, _ = request(3, "undo", saved_result["revision"])
                assert undone["ok"] and undone["dirty"], undone
                redone, _ = request(4, "redo", undone["revision"])
                assert redone["ok"], redone
                selected, _ = request(5, "select_all", redone["revision"])
                assert selected["ok"], selected
                searched, _ = request(6, "find", selected["revision"])
                assert searched["ok"], searched
                flushed, flush_events = request(7, "flush", searched["revision"])
                assert flushed["ok"] and not flushed["dirty"], flushed
                assert any(event["type"] == "saved" for event in flush_events), flush_events
                assert source.read_text(encoding="utf-8").startswith("x"), flushed
                print("Helix PTY/control bridge: authenticated commands, durable flush and undo/redo OK")
        finally:
            process.terminate()
            try:
                process.wait(timeout=2)
            except subprocess.TimeoutExpired:
                process.kill()
                process.wait(timeout=2)
            os.close(master)
            listener.close()


if __name__ == "__main__":
    if sys.platform != "darwin":
        print("Helix PTY/control bridge: macOS fixture skipped")
    else:
        main()
