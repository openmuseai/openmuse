#!/usr/bin/env python3
"""Build the pinned product-neutral DeepSeek Harness npm closure.

The lockfile pins registry integrity and the current client/API contracts are
checked before the closure can enter a desktop package.
"""

from __future__ import annotations

import argparse
import json
import os
import shutil
import subprocess
import tarfile
from pathlib import Path


VENDORED = Path(__file__).resolve().parent.parent / "third_party/dsh"
REMOVED_PRODUCT_IDENTIFIER = "app" + "flow" + "y"
CREATED_FILE_SINGLE_PANE_MARKER = "data-openmuse-created-single-pane"


def run(*command: str, cwd: Path) -> None:
    environment = os.environ.copy()
    resolved = list(command)
    # Windows CreateProcess does not apply PATHEXT, so `npm` must be `npm.cmd`.
    if os.name == "nt" and not os.path.splitext(resolved[0])[1]:
        found = shutil.which(resolved[0])
        if found is None:
            raise RuntimeError(f"required executable is not on PATH: {resolved[0]}")
        resolved[0] = found
    subprocess.run(resolved, cwd=cwd, env=environment, check=True)


def package_name(tarball: Path) -> str:
    with tarfile.open(tarball, "r:gz") as archive:
        member = next(
            item for item in archive.getmembers()
            if item.name == "package/package.json"
        )
        source = archive.extractfile(member)
        if source is None:
            raise RuntimeError(f"missing package.json in {tarball}")
        value = json.load(source)
    return value["name"]


def verify_vendored_inputs() -> None:
    manifest = VENDORED / "package.json"
    lockfile = VENDORED / "package-lock.json"
    if not manifest.is_file() or not lockfile.is_file():
        raise RuntimeError("repository is missing pinned DSH manifest/lockfile")
    dependencies = json.loads(manifest.read_text(encoding="utf-8"))["dependencies"]
    lock = json.loads(lockfile.read_text(encoding="utf-8"))
    if "@deepseek-ai/dsh" not in dependencies:
        raise RuntimeError("repository is missing the DSH CLI package")
    for name, spec in dependencies.items():
        if REMOVED_PRODUCT_IDENTIFIER in name.lower() or name.startswith("@muse/"):
            raise RuntimeError(f"old product package cannot enter closure: {name}")
        if not isinstance(spec, str):
            raise RuntimeError(f"invalid DSH dependency: {name}")
        if spec.startswith("file:tarballs/"):
            tarball = (VENDORED / spec.removeprefix("file:")).resolve(strict=True)
            if not tarball.is_relative_to(VENDORED.resolve()):
                raise RuntimeError(f"DSH tarball escapes repository: {tarball}")
            if package_name(tarball) != name:
                raise RuntimeError(f"DSH tarball package mismatch: {tarball}")
        elif name != "@deepseek-ai/dsh" or spec != "0.1.7-rc.1":
            raise RuntimeError(f"DSH registry dependency must be exact and reviewed: {name}@{spec}")
        locked = lock.get("packages", {}).get(f"node_modules/{name}", {})
        if locked.get("version") != spec and not spec.startswith("file:"):
            raise RuntimeError(f"DSH lockfile version mismatch: {name}")
        if not locked.get("integrity"):
            raise RuntimeError(f"DSH lockfile integrity missing: {name}")


def scrub_build_paths(output: Path) -> int:
    """Remove the build machine path from generated CSS region comments only."""
    count = 0
    for script in (output / "node_modules/@deepseek-ai").rglob("*.js"):
        source = script.read_text(encoding="utf-8")
        if "\\0dsh-css:/" not in source:
            continue
        lines = source.splitlines(keepends=True)
        for index, line in enumerate(lines):
            marker = "\\0dsh-css:"
            if marker not in line or "/packages/" not in line:
                continue
            if not line.lstrip().startswith("//#region \\0dsh-css:"):
                raise RuntimeError(f"refusing to rewrite non-comment source path: {script}")
            before, path = line.split(marker, 1)
            if not path.startswith("/"):
                continue
            _, package_path = path.split("/packages/", 1)
            lines[index] = f"{before}{marker}dsh-source/packages/{package_path}"
            count += 1
        script.write_text("".join(lines), encoding="utf-8")
    return count


def apply_openmuse_product_patches(output: Path) -> None:
    """Apply small, fail-closed UI policy overlays to the pinned DSH build.

    The upstream review tab starts every file in split mode.  A newly-created
    file has no meaningful left side, so OpenMuse presents it as one unified
    pane on Desktop.  Exact-source guards deliberately fail the build when the
    pinned DSH implementation changes instead of silently shipping a stale
    patch.
    """
    deliverables = (
        output
        / "node_modules/@deepseek-ai/dsh-client-ui-deliverables/lib/client.js"
    )
    source = deliverables.read_text(encoding="utf-8")
    replacements = (
        (
            "\t\t\tconst split = state?.split === true;\n"
            "\t\t\tconst wrap = state?.wrap === true;",
            "\t\t\tconst created = typeof diffState === \"object\" && "
            "diffState.kind === \"text\" && diffState.before === false && "
            "diffState.after === true;\n"
            "\t\t\tconst split = state?.split === true && !created;\n"
            "\t\t\tconst wrap = state?.wrap === true;",
        ),
        (
            "\"data-review-tool\": \"split\",\n",
            "\"data-review-tool\": \"split\",\n"
            f"\t\t\t\t\t\t\t\"{CREATED_FILE_SINGLE_PANE_MARKER}\": "
            "created || void 0,\n"
            "\t\t\t\t\t\t\thidden: created || void 0,\n",
        ),
    )
    for old, new in replacements:
        if source.count(old) != 1:
            raise RuntimeError(
                "pinned DSH deliverables contract changed; cannot apply "
                "created-file single-pane overlay"
            )
        source = source.replace(old, new)
    deliverables.write_text(source, encoding="utf-8")


def validate(output: Path) -> None:
    entry = output / "node_modules/@deepseek-ai/dsh/lib/bin.js"
    if not entry.is_file():
        raise RuntimeError(f"DSH entry missing: {entry}")
    chat = output / "node_modules/@deepseek-ai/dsh-client-ui-chat/lib/client.js"
    if not chat.is_file() or "ctx.sidebarRight.openResource(url)" not in chat.read_text(encoding="utf-8"):
        raise RuntimeError("DSH conversation file-open contract changed")
    controller = output / "node_modules/@deepseek-ai/dsh-api-workspace-controller/lib/index.js"
    if not controller.is_file() or "workspaceRegistry.create(request.path)" not in controller.read_text(encoding="utf-8"):
        raise RuntimeError("DSH Workspace controller contract changed")
    models = output / "node_modules/@deepseek-ai/dsh-client-ui-settings-models/lib/client.js"
    if not models.is_file() or "settings.models.provider-card" not in models.read_text(encoding="utf-8"):
        raise RuntimeError("DSH model capability slot missing")
    model_plugin = output / "node_modules/dsh-model-capabilities"
    model_patch = model_plugin / "openmuse.patch.yml"
    if not model_patch.is_file() or not (model_plugin / "LICENSE").is_file():
        raise RuntimeError("default model capability plugin or license missing")
    if "settings.describe()" not in (model_plugin / "lib/index.js").read_text(encoding="utf-8"):
        raise RuntimeError("model capability plugin has not been adapted to profile settings")
    bridge = output / "node_modules/openmuse-dsh-bridge"
    if not (bridge / "lib/index.js").is_file() or not (bridge / "lib/client.js").is_file():
        raise RuntimeError("OpenMuse DSH Host bridge is missing")
    bridge_source = (bridge / "lib/index.js").read_text(encoding="utf-8")
    bridge_manifest = json.loads((bridge / "package.json").read_text(encoding="utf-8"))
    native_contract_markers = (
        "/openmuse-native/v1/negotiate",
        "/openmuse-native/v1/workspaces",
        "/openmuse-native/v1/session/create",
        "controller.follow",
    )
    if any(marker not in bridge_source for marker in native_contract_markers):
        raise RuntimeError("OpenMuse DSH Native Gateway is missing")
    deliverables = output / "node_modules/@deepseek-ai/dsh-client-ui-deliverables/lib/client.js"
    if CREATED_FILE_SINGLE_PANE_MARKER not in deliverables.read_text(encoding="utf-8"):
        raise RuntimeError("OpenMuse created-file single-pane overlay is missing")
    native_manifest = bridge_manifest.get("openmuse", {}).get("nativeConversation", {})
    if native_manifest.get("schemaVersion") != 1:
        raise RuntimeError("OpenMuse native conversation manifest is missing")
    remote_runtime = output / "node_modules/@openmuse/dsh-workspace-runtime"
    remote_patch = remote_runtime / "cordis.patch.yml"
    if not (remote_runtime / "lib/index.js").is_file() or not remote_patch.is_file():
        raise RuntimeError("OpenMuse remote Workspace Runtime Provider group is missing")
    remote_patch_source = remote_patch.read_text(encoding="utf-8")
    for provider in ("subprocess", "sandbox", "fs-sandbox"):
        if f"id: {provider}" not in remote_patch_source:
            raise RuntimeError(f"remote Workspace Runtime patch is missing {provider} replacement")
    if "agent-loop" in remote_patch_source:
        raise RuntimeError("remote Workspace Runtime must not patch the DSH Agent Loop")
    if "openmuse-host-bridge" not in model_patch.read_text(encoding="utf-8"):
        raise RuntimeError("OpenMuse DSH Host bridge is not mounted")
    market = output / "node_modules/dshmarket"
    market_patch = market / "openmuse.patch.yml"
    if not (market / "lib/index.js").is_file() or not (market / "client/client.js").is_file():
        raise RuntimeError("default dsh-market plugin is missing")
    if "id: dsh-market" not in market_patch.read_text(encoding="utf-8"):
        raise RuntimeError("dsh-market is not mounted by its default patch")
    if not (market / "LICENSE").is_file():
        raise RuntimeError("dsh-market license is missing")
    for script in (output / "node_modules/@deepseek-ai").rglob("*.js"):
        source = script.read_text(encoding="utf-8")
        if REMOVED_PRODUCT_IDENTIFIER in source.lower() or "\\0dsh-css:/Users/" in source:
            raise RuntimeError(f"removed product or build path found in DSH runtime: {script}")
    run("node", str(entry), "--version", cwd=output)
    root = (output / "node_modules").resolve()
    links = [item for item in root.rglob("*") if item.is_symlink()]
    escaping = []
    for link in links:
        try:
            link.resolve(strict=True).relative_to(root)
        except (FileNotFoundError, ValueError):
            escaping.append(link)
    if escaping:
        raise RuntimeError(f"closure has broken or escaping symlinks: {escaping[:3]}")
    print(f"DSH closure ready: {output} ({len(links)} internal links)")


def main() -> None:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--out", type=Path, required=True)
    parser.add_argument("--validate-only", action="store_true")
    args = parser.parse_args()
    output = args.out.resolve()
    if args.validate_only:
        validate(output)
        return
    verify_vendored_inputs()
    if output.exists() and any(output.iterdir()):
        raise RuntimeError(f"output must be empty: {output}")
    output.mkdir(parents=True, exist_ok=True)
    shutil.copy2(VENDORED / "package.json", output / "package.json")
    shutil.copy2(VENDORED / "package-lock.json", output / "package-lock.json")
    if any(spec.startswith("file:tarballs/") for spec in
           json.loads((VENDORED / "package.json").read_text())["dependencies"].values()):
        shutil.copytree(VENDORED / "tarballs", output / "tarballs")
    run("npm", "ci", "--omit=dev", "--no-audit", "--no-fund", cwd=output)
    apply_openmuse_product_patches(output)
    print(f"sanitized {scrub_build_paths(output)} generated path comments")
    validate(output)


if __name__ == "__main__":
    main()
