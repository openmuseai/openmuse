#!/usr/bin/env python3
"""Link harness packages into packed @muse/*/node_modules.

Copied Muse packages have no node_modules. Node ESM resolves from the file
realpath, so @muse/plugin-kit cannot see pnpm-isolated @deepseek-ai/dsh-tools
unless each packed package gets a relative symlink. This is the Docker subset
of scripts/lib/wire-muse-node-modules.py (harness aliases + sibling @muse +
npm deps installed into the harness, e.g. canonicalize / ajv).
"""
from __future__ import annotations

import json
import os
import sys
from pathlib import Path

MUSE_SHORT = {
    "@muse/dsh-appflowy": "dsh-appflowy",
    "@muse/host-bridge": "host-bridge",
    "@muse/plugin-kit": "plugin-kit",
    "@muse/plugin-facets": "plugin-facets",
    "@muse/plugin-graph": "plugin-graph",
    "@muse/context-broker": "context-broker",
    "@muse/contract-document": "contract-document",
    "@muse/plugin-appflowy-markdown": "plugin-appflowy-markdown",
    "@muse/plugin-appflowy-workspace": "plugin-appflowy-workspace",
    "@muse/plugin-appflowy-view-reference": "plugin-appflowy-view-reference",
    "@muse/plugin-appflowy-view-rename": "plugin-appflowy-view-rename",
    "@muse/dsh-mobile-surface": "dsh-mobile-surface",
    "@muse/dsh-mobile-input": "dsh-mobile-input",
}

HARNESS_ALIASES = {
    "@deepseek-ai/dsh-tools": "packages/core/tools",
    "@deepseek-ai/cordis": "vendor/cordis",
    "@deepseek-ai/dsh-system-prompt": "packages/core/system-prompt",
}


def rel_symlink(target: Path, dest: Path) -> None:
    dest.parent.mkdir(parents=True, exist_ok=True)
    if dest.exists() or dest.is_symlink():
        dest.unlink()
    dest.symlink_to(os.path.relpath(target, dest.parent))


def harness_node_module(harness: Path, spec: str) -> Path | None:
    path = harness / "node_modules" / spec
    if (path / "package.json").is_file():
        return path
    return None


def package_dep_specs(pkg: Path) -> list[str]:
    manifest = pkg / "package.json"
    if not manifest.is_file():
        return []
    data = json.loads(manifest.read_text(encoding="utf-8"))
    specs = dict(data.get("dependencies") or {})
    specs.update(data.get("peerDependencies") or {})
    return sorted(specs)


def main() -> int:
    harness = Path(sys.argv[1] if len(sys.argv) > 1 else "/muse/dsh").resolve()
    muse_dir = harness / "node_modules" / "@muse"
    if not muse_dir.is_dir():
        print(f"missing {muse_dir}", file=sys.stderr)
        return 1
    unresolved: list[str] = []
    for pkg in sorted(p for p in muse_dir.iterdir() if p.is_dir()):
        for spec, rel in HARNESS_ALIASES.items():
            target = harness / rel
            if not (target / "package.json").is_file():
                continue
            rel_symlink(target, pkg / "node_modules" / spec)
        for spec, short in MUSE_SHORT.items():
            sibling = muse_dir / short
            if sibling == pkg or not (sibling / "package.json").is_file():
                continue
            rel_symlink(sibling, pkg / "node_modules" / spec)
        for spec in package_dep_specs(pkg):
            if spec in HARNESS_ALIASES or spec in MUSE_SHORT:
                continue
            dest = pkg / "node_modules" / spec
            if dest.exists() or dest.is_symlink():
                continue
            target = harness_node_module(harness, spec)
            if target is None:
                unresolved.append(f"{pkg.name}:{spec}")
                continue
            rel_symlink(target, dest)
    if unresolved:
        print("unresolved: " + ", ".join(unresolved), file=sys.stderr)
        return 1
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
