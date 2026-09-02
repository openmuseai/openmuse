#!/usr/bin/env python3
"""Wire packed @muse packages so Node ESM can resolve their runtime deps.

Copied Muse packages have no node_modules. ESM resolves from the file realpath,
so @muse/plugin-kit under dsh/node_modules/@muse still cannot see pnpm-isolated
@deepseek-ai/dsh-tools, and host-bridge cannot see ajv/@noble (those live in the
Muse workspace, not the DSH harness). This script writes relative symlinks or
dereferenced copies next to each packed package.
"""
from __future__ import annotations

import json
import os
import shutil
import subprocess
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
    "@deepseek-ai/dsh-tools": ("packages/core/tools",),
    "@deepseek-ai/cordis": ("vendor/cordis",),
    "@deepseek-ai/dsh-system-prompt": ("packages/core/system-prompt",),
}


def rel_symlink(target: Path, dest: Path) -> None:
    dest.parent.mkdir(parents=True, exist_ok=True)
    if dest.exists() or dest.is_symlink():
        dest.unlink()
    dest.symlink_to(os.path.relpath(target, dest.parent))


def package_specs(pkg_json: Path) -> list[str]:
    data = json.loads(pkg_json.read_text())
    specs = dict(data.get("dependencies") or {})
    specs.update(data.get("peerDependencies") or {})
    return sorted(specs)


def resolve_from_node(node: Path, from_pkg: Path, spec: str) -> Path | None:
    script = (
        "import { createRequire } from 'node:module';"
        "import { dirname } from 'node:path';"
        "import { fileURLToPath } from 'node:url';"
        f"const r = createRequire({json.dumps(from_pkg.as_uri() + '/')});"
        f"try {{ console.log(dirname(r.resolve({json.dumps(spec + '/package.json')}))); }}"
        "catch { process.exit(2); }"
    )
    try:
        out = subprocess.run(
            [str(node), "--input-type=module", "-e", script],
            check=False,
            capture_output=True,
            text=True,
        )
    except OSError:
        return None
    if out.returncode != 0:
        return None
    line = out.stdout.strip()
    return Path(line) if line else None


def copy_from_source(source_pkg_nm: Path, spec: str, dest: Path, hoist_nm: Path) -> bool:
    src = source_pkg_nm.joinpath(*spec.split("/"))
    if not src.exists():
        return False
    real = src.resolve()
    dest.parent.mkdir(parents=True, exist_ok=True)
    if dest.exists() or dest.is_symlink():
        if dest.is_dir() and not dest.is_symlink():
            shutil.rmtree(dest)
        else:
            dest.unlink()
    shutil.copytree(real, dest, symlinks=False, dirs_exist_ok=False)
    # pnpm puts ajv's deps (fast-deep-equal, …) next to ajv, not inside it.
    nm = real.parent
    if spec.startswith("@"):
        if nm.name != spec.split("/")[0]:
            return True
        nm = nm.parent
    if nm.name != "node_modules":
        return True
    for sibling in nm.iterdir():
        if sibling.name.startswith("."):
            continue
        out = hoist_nm / sibling.name
        if out.exists() or out.is_symlink():
            continue
        target = sibling.resolve() if sibling.is_symlink() else sibling
        if target.is_dir():
            shutil.copytree(target, out, symlinks=False)
    return True


def main() -> int:
    if len(sys.argv) < 3:
        print("usage: wire-muse-node-modules.py <harness-dir> <muse-repo-root> [node]", file=sys.stderr)
        return 2
    harness = Path(sys.argv[1]).resolve()
    root = Path(sys.argv[2]).resolve()
    node = Path(sys.argv[3]).resolve() if len(sys.argv) > 3 else Path("node")
    muse_dir = harness / "node_modules" / "@muse"
    if not muse_dir.is_dir():
        print(f"missing {muse_dir}", file=sys.stderr)
        return 1
    tools = harness / "packages" / "core" / "tools" / "package.json"
    packages_root = None
    for candidate in (root / "middlewares" / "dsh", root / "packages"):
        if (candidate / "plugins").is_dir() or (candidate / "core").is_dir():
            packages_root = candidate
            break
    if packages_root is None:
        packages_root = root / "middlewares" / "dsh"
    unresolved: list[str] = []
    for pkg in sorted(p for p in muse_dir.iterdir() if p.is_dir()):
        manifest = pkg / "package.json"
        if not manifest.is_file():
            continue
        source_nm = None
        for manifest_src in packages_root.rglob("package.json"):
            if "node_modules" in manifest_src.parts:
                continue
            src = manifest_src.parent
            name = json.loads(manifest_src.read_text()).get("name")
            packed = json.loads(manifest.read_text()).get("name")
            if name == packed:
                source_nm = src / "node_modules"
                break
        for spec in package_specs(manifest):
            dest = pkg / "node_modules" / spec
            target: Path | None = None
            if spec in MUSE_SHORT:
                candidate = muse_dir / MUSE_SHORT[spec]
                if (candidate / "package.json").is_file():
                    target = candidate
            if target is None:
                for rel in HARNESS_ALIASES.get(spec, ()):
                    candidate = harness / rel
                    if (candidate / "package.json").is_file():
                        target = candidate
                        break
            if target is None and tools.is_file() and node.exists():
                target = resolve_from_node(node, tools, spec)
            if target is not None:
                rel_symlink(target, dest)
                print(f"    link {pkg.name} {spec} -> {target}")
                continue
            if source_nm is not None and copy_from_source(source_nm, spec, dest, pkg / "node_modules"):
                print(f"    copy {pkg.name} {spec}")
                continue
            unresolved.append(f"{pkg.name}:{spec}")
    if unresolved:
        print("unresolved: " + ", ".join(unresolved), file=sys.stderr)
        return 1
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
