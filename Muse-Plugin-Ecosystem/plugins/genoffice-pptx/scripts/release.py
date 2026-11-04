"""Build, verify and submit the next immutable GenOffice PPTX MCPB."""

from __future__ import annotations

import argparse
import json
import subprocess
import sys
from pathlib import Path

ROOT = Path(__file__).resolve().parents[1]
VERSION = json.loads((ROOT / "package.json").read_text())["version"]


def run(*args: str, cwd: Path = ROOT) -> None:
    subprocess.run(args, cwd=cwd, check=True)


def main() -> None:
    parser = argparse.ArgumentParser()
    parser.add_argument("--registry", required=True)
    parser.add_argument("--public-origin")
    parser.add_argument("--client-version", default="latest")
    parser.add_argument("--remote")
    parser.add_argument("--build-only", action="store_true")
    args = parser.parse_args()
    run("git", "submodule", "update", "--init", "--recursive", "--", "plugins/genoffice-pptx/vendor/genoffice", cwd=ROOT.parents[1])
    run("npm", "ci")
    run("npm", "test")
    run("npm", "run", "pack:desktop")
    bundle = ROOT / f"dist/genoffice-pptx-{VERSION}.mcpb"
    if args.build_only:
        print(bundle)
        return
    command = [sys.executable, str(ROOT / "scripts/publish.py"), str(bundle),
               "--registry", args.registry,
               "--client-version", args.client_version]
    if args.public_origin:
        command += ["--public-origin", args.public_origin]
    if args.remote:
        command += ["--remote", args.remote]
    run(*command)


if __name__ == "__main__":
    main()
