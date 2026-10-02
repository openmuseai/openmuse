#!/usr/bin/env python3
"""Exercise the pinned DSH web runtime and default model plugin without keys."""

from __future__ import annotations

import argparse
import http.cookiejar
import json
import os
import re
import select
import subprocess
import tempfile
import time
import urllib.error
import urllib.request
from pathlib import Path


def main() -> None:
    parser = argparse.ArgumentParser()
    parser.add_argument("--node", required=True, type=Path)
    parser.add_argument("--closure", required=True, type=Path)
    args = parser.parse_args()
    root = args.closure.resolve()
    cli = root / "node_modules/@deepseek-ai/dsh/lib/bin.js"
    patch = root / "node_modules/dsh-model-capabilities/openmuse.patch.yml"
    if not cli.is_file() or not patch.is_file():
        raise RuntimeError("pinned DSH CLI/model plugin is incomplete")

    with tempfile.TemporaryDirectory(prefix="openmuse-dsh-smoke-") as home:
        bridge_token = "openmuse-smoke-bridge-token-0123456789abcdef"
        environment = os.environ.copy()
        for name in list(environment):
            if name.endswith("_API_KEY") or name.endswith("_TOKEN"):
                environment.pop(name)
        environment["DSH_HOME"] = home
        environment["OPENMUSE_DSH_BRIDGE_TOKEN"] = bridge_token
        environment["OPENMUSE_DSH_NATIVE_EXPERIMENTAL"] = "1"
        command = [
            str(args.node.resolve()), str(cli), "web", "--patch", str(patch),
            "--host", "127.0.0.1", "--port", "0", "--no-open",
        ]
        process = subprocess.Popen(
            command, stdout=subprocess.PIPE, stderr=subprocess.STDOUT,
            text=True, bufsize=1, env=environment,
        )
        try:
            deadline = time.monotonic() + 45
            endpoint = None
            diagnostics: list[str] = []
            while time.monotonic() < deadline and endpoint is None:
                if process.poll() is not None:
                    break
                assert process.stdout is not None
                if not select.select([process.stdout], [], [], 1)[0]:
                    continue
                line = process.stdout.readline().strip()
                diagnostics.append(re.sub(r"\?[^\s]+", "?<redacted>", line))
                match = re.search(r"http://127\.0\.0\.1:\d+/\?[^\s]+", line)
                if match:
                    endpoint = match.group(0)
            if endpoint is None:
                raise RuntimeError(f"DSH did not start: {diagnostics[-6:]}")
            client = urllib.request.build_opener(
                urllib.request.HTTPCookieProcessor(http.cookiejar.CookieJar())
            )
            with client.open(endpoint, timeout=10) as response:
                if response.status != 200:
                    raise RuntimeError(f"DSH HTTP {response.status}")
                html = response.read().decode("utf-8", errors="replace")
            if "dsh-model-capabilities" not in html:
                raise RuntimeError("default model plugin missing from DSH client graph")
            if "openmuse-dsh-bridge" not in html:
                raise RuntimeError("OpenMuse DSH Host bridge missing from client graph")
            base = endpoint.split("/?", 1)[0]
            bridge_url = base + "/openmuse-bridge/workspaces"
            payload = json.dumps({"mounts": [home]}).encode()
            unauthorized = urllib.request.Request(
                bridge_url, data=payload, headers={"Content-Type": "application/json"}, method="POST",
            )
            try:
                client.open(unauthorized, timeout=10)
            except urllib.error.HTTPError as error:
                if error.code != 403:
                    raise RuntimeError("DSH bridge did not reject missing token") from error
            else:
                raise RuntimeError("DSH bridge accepted missing token")
            authorized = urllib.request.Request(
                bridge_url, data=payload,
                headers={"Content-Type": "application/json", "X-OpenMuse-Bridge-Token": bridge_token},
                method="POST",
            )
            with client.open(authorized, timeout=10) as response:
                if response.status != 200 or home not in response.read().decode():
                    raise RuntimeError("DSH bridge failed to register local workspace")
            native_hello = urllib.request.Request(
                base + "/openmuse-native/v1/hello",
                headers={"X-OpenMuse-Bridge-Token": bridge_token},
            )
            with client.open(native_hello, timeout=10) as response:
                hello = json.load(response)
                if hello.get("protocolVersion") != 1 or hello.get("dshVersion") != "0.1.7-rc.1":
                    raise RuntimeError("DSH Native Gateway handshake mismatch")
            native_negotiate = urllib.request.Request(
                base + "/openmuse-native/v1/negotiate",
                data=json.dumps({
                    "nativeUiApi": 1,
                    "components": ["toolCard@1", "keyValue@1"],
                    "slots": ["tool.call.toolview"],
                }).encode(),
                headers={
                    "Content-Type": "application/json",
                    "X-OpenMuse-Bridge-Token": bridge_token,
                },
                method="POST",
            )
            with client.open(native_negotiate, timeout=10) as response:
                negotiation = json.load(response)
                if negotiation.get("schemaVersion") != 1 or negotiation.get("mode") != "native":
                    raise RuntimeError(
                        f"DSH Native UI negotiation failed: {negotiation}; "
                        f"active={hello.get('compatibility', {}).get('activePlugins')}"
                    )
            try:
                client.open(base + "/model-capabilities?provider=missing", timeout=10)
            except urllib.error.HTTPError as error:
                if error.code != 404 or b"provider-not-found" not in error.read():
                    raise RuntimeError("model plugin settings route failed") from error
            else:
                raise RuntimeError("model plugin accepted an unknown provider")
            print("DSH 0.1.7-rc.1 no-key web + default model plugin: passed")
        finally:
            process.terminate()
            try:
                process.wait(timeout=5)
            except subprocess.TimeoutExpired:
                process.kill()
                process.wait()


if __name__ == "__main__":
    main()
