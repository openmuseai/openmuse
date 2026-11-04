"""Package the standalone GenOffice MCP server as a standard MCP Bundle."""

from __future__ import annotations

import hashlib
import json
import sys
import zipfile
from pathlib import Path

ROOT = Path(__file__).resolve().parents[1]
NAME = "genoffice-pptx"
VERSION = json.loads((ROOT / "package.json").read_text())["version"]
METADATA = json.loads((ROOT / "config/package-metadata.json").read_text())


def build(output: Path) -> Path:
    output.mkdir(parents=True, exist_ok=True)
    worker = (ROOT / "dist/server.mjs").read_bytes()
    manifest = {
        "manifest_version": "0.3",
        "name": NAME,
        "display_name": "OpenMuse Slides",
        "version": VERSION,
        "description": "OpenMuse in-app presentation rendering and editing via standard MCP Apps.",
        "author": {"name": "OpenMuse contributors"},
        "license": "Apache-2.0",
        "repository": {"type": "git", "url": METADATA["repository"]},
        "tools": [{"name": "inspect_pptx", "description": "Inspect a PPTX deck"}],
        "compatibility": {
            "platforms": ["darwin", "win32", "linux"],
            "runtimes": {"node": ">=22.0.0"},
        },
        "server": {
            "type": "node",
            "entry_point": "server/index.mjs",
            "mcp_config": {
                "command": "node",
                "args": ["${__dirname}/server/index.mjs"],
            },
        },
    }
    notices = []
    for metadata in sorted((ROOT / "node_modules").glob("**/package.json")):
        if "node_modules" in metadata.parent.relative_to(ROOT / "node_modules").parts:
            continue
        try:
            info = json.loads(metadata.read_text())
        except (ValueError, OSError):
            continue
        texts = []
        for pattern in ("LICENSE*", "LICENCE*", "COPYING*", "NOTICE*"):
            for license_file in metadata.parent.glob(pattern):
                if license_file.is_file():
                    texts.append(license_file.read_text(errors="replace"))
        if texts:
            notices.append(f"{info.get('name', metadata.parent.name)} {info.get('version', '')}\n" + "\n".join(texts))
    package = output / f"{NAME}-{VERSION}.mcpb"
    entries = {
        "manifest.json": (json.dumps(manifest, separators=(",", ":")) + "\n").encode(),
        "server/index.mjs": worker,
        "LICENSE": (ROOT / "UPSTREAM-LICENSE").read_bytes(),
        "NOTICE": (ROOT / "UPSTREAM-NOTICE").read_bytes(),
        "THIRD-PARTY-NOTICES.txt": "\n\n".join(notices).encode(),
        "licenses/Carlito-OFL.txt": (ROOT / "vendor/genoffice/apps/docs/src/renderer/fonts/LICENSE-OFL.txt").read_bytes(),
        "licenses/Arimo-OFL.txt": (ROOT / "node_modules/@fontsource/arimo/LICENSE").read_bytes(),
    }
    web_root = ROOT / "dist/web"
    if not (web_root / "index.html").is_file():
        raise RuntimeError("Build the original editor UI before packing")
    # The server exposes this exact HTML through resources/read. The offline
    # copy is cached by Mobile without executing Node or contacting localhost.
    entries["server/web/index.html"] = (web_root / "index.html").read_bytes()
    entries["web/index.html"] = (web_root / "index.html").read_bytes()
    entries["web/mcp-offline.json"] = (web_root / "mcp-offline.json").read_bytes()
    for resource in (ROOT / "i18n").glob("*.json"):
        entries[f"i18n/{resource.name}"] = resource.read_bytes()
    entries["config/globalization.json"] = (ROOT / "config/globalization.json").read_bytes()
    with zipfile.ZipFile(package, "w", compression=zipfile.ZIP_DEFLATED) as archive:
        for name, data in sorted(entries.items()):
            info = zipfile.ZipInfo(name, date_time=(2020, 1, 1, 0, 0, 0))
            info.compress_type = zipfile.ZIP_DEFLATED
            info.external_attr = 0o644 << 16
            archive.writestr(info, data)
    digest = hashlib.sha256(package.read_bytes()).hexdigest()
    (output / "mcpb.sha256").write_text(f"{digest}  {package.name}\n")
    return package


if __name__ == "__main__":
    print(build(Path(sys.argv[1]).resolve() if len(sys.argv) > 1 else ROOT / "dist"))
