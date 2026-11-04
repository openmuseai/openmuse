#!/usr/bin/env python3
"""Resolve and verify deterministic OpenMuse distribution closures."""

from __future__ import annotations

import argparse
import hashlib
import json
import re
from datetime import datetime, timezone
from pathlib import Path
from typing import Any


class DistributionError(RuntimeError):
    pass


def canonical(value: Any) -> bytes:
    return (json.dumps(value, ensure_ascii=False, sort_keys=True, separators=(",", ":")) + "\n").encode()


def sha256_bytes(value: bytes) -> str:
    return hashlib.sha256(value).hexdigest()


def load_json(path: Path) -> Any:
    try:
        return json.loads(path.read_text(encoding="utf-8"))
    except (OSError, json.JSONDecodeError) as error:
        raise DistributionError(f"cannot read JSON {path}: {error}") from error


def target_key(target: dict[str, str]) -> tuple[str, str, str]:
    try:
        return target["os"], target["arch"], target["libc"]
    except (KeyError, TypeError) as error:
        raise DistributionError(f"invalid target: {target}") from error


def validate_capability(capability: dict[str, Any]) -> None:
    if not isinstance(capability, dict):
        raise DistributionError("capability snapshot must be an object")
    required = {"profile", "target", "allowedArtifactKinds", "allowedPlugins", "requiredPlugins"}
    if set(capability) != required:
        raise DistributionError("capability snapshot fields are incomplete or unknown")
    profile = capability.get("profile")
    if profile not in {"desktop", "android", "ios", "sandbox"}:
        raise DistributionError(f"unknown distribution profile: {profile}")
    target = capability.get("target")
    os_name, _, _ = target_key(target)
    valid_os = {
        "desktop": {"macos", "windows", "linux"},
        "android": {"android"},
        "ios": {"ios"},
        "sandbox": {"linux"},
    }
    if os_name not in valid_os[profile]:
        raise DistributionError("capability profile and target are inconsistent")
    for field in ("allowedArtifactKinds", "allowedPlugins", "requiredPlugins"):
        values = capability.get(field)
        if not isinstance(values, list) or not all(isinstance(value, str) and value for value in values):
            raise DistributionError(f"capability snapshot {field} must be a string list")
        if len(values) != len(set(values)):
            raise DistributionError(f"capability snapshot {field} contains duplicates")
    if not set(capability["requiredPlugins"]).issubset(capability["allowedPlugins"]):
        raise DistributionError("required plugins must be included in the allowlist")


def validate_manifest(manifest: dict[str, Any]) -> None:
    if not isinstance(manifest, dict):
        raise DistributionError("plugin manifest must be an object")
    if manifest.get("manifest_version") != 2:
        raise DistributionError("only plugin manifest v2 can enter a distribution")
    if not isinstance(manifest.get("id"), str) or not isinstance(manifest.get("version"), str):
        raise DistributionError("plugin id/version missing")
    decisions: dict[tuple[str, str, str], dict[str, Any]] = {}
    for decision in manifest.get("compatibility", {}).get("targets", []):
        key = target_key(decision.get("target"))
        if key in decisions:
            raise DistributionError(f"{manifest['id']}: duplicate target {key}")
        if decision.get("status") not in {"supported", "unsupported"}:
            raise DistributionError(f"{manifest['id']}: invalid target status")
        decisions[key] = decision
    if not decisions:
        raise DistributionError(f"{manifest['id']}: no target decisions")
    artifact_ids: set[str] = set()
    for artifact in manifest.get("artifacts", []):
        artifact_id = artifact.get("id")
        if not isinstance(artifact_id, str) or artifact_id in artifact_ids:
            raise DistributionError(f"{manifest['id']}: invalid or duplicate artifact id")
        artifact_ids.add(artifact_id)
        digest = artifact.get("digest", {})
        if digest.get("algorithm") != "sha256" or not re.fullmatch(r"[0-9a-f]{64}", digest.get("value", "")):
            raise DistributionError(f"{manifest['id']}:{artifact_id}: invalid digest")
        if artifact.get("kind") == "sandbox-worker":
            signature = artifact.get("signature", {})
            if signature.get("algorithm") != "ed25519" or not all(
                isinstance(signature.get(field), str) and signature[field] for field in ("key_id", "value")
            ):
                raise DistributionError(f"{manifest['id']}:{artifact_id}: sandbox worker is unsigned")
        decision = decisions.get(target_key(artifact.get("target")))
        if decision is None or decision.get("status") != "supported":
            raise DistributionError(f"{manifest['id']}:{artifact_id}: artifact target is not supported")


def resolve(manifest_paths: list[Path], capability: dict[str, Any]) -> dict[str, Any]:
    validate_capability(capability)
    target = capability.get("target")
    wanted = target_key(target)
    allowed_kinds = set(capability.get("allowedArtifactKinds", []))
    allowed_plugins = set(capability["allowedPlugins"])
    required_plugins = set(capability.get("requiredPlugins", []))
    plugins = []
    seen_plugins: set[str] = set()
    for path in sorted(manifest_paths, key=lambda item: str(item)):
        manifest = load_json(path)
        validate_manifest(manifest)
        plugin_id = manifest["id"]
        if plugin_id in seen_plugins:
            raise DistributionError(f"duplicate plugin manifest: {plugin_id}")
        seen_plugins.add(plugin_id)
        if plugin_id not in allowed_plugins:
            continue
        matches = [
            item for item in manifest["compatibility"]["targets"]
            if target_key(item["target"]) == wanted
        ]
        if len(matches) != 1:
            raise DistributionError(f"{plugin_id}: target decision missing or ambiguous for {wanted}")
        if matches[0]["status"] == "unsupported":
            continue
        artifacts = []
        for artifact in manifest["artifacts"]:
            if target_key(artifact["target"]) != wanted:
                continue
            if artifact["kind"] not in allowed_kinds:
                raise DistributionError(
                    f"{plugin_id}:{artifact['id']}: kind {artifact['kind']} is forbidden for {capability.get('profile')}"
                )
            artifacts.append({
                "id": artifact["id"],
                "kind": artifact["kind"],
                "target": artifact["target"],
                "sha256": artifact["digest"]["value"],
                "license": artifact["license"],
                "abi": artifact["abi"],
                **({"signature": artifact["signature"]} if "signature" in artifact else {}),
            })
        if not artifacts:
            raise DistributionError(f"{plugin_id}: supported target has no selectable artifact")
        plugins.append({
            "id": plugin_id,
            "version": manifest["version"],
            "artifacts": sorted(artifacts, key=lambda item: item["id"]),
        })
    present = {plugin["id"] for plugin in plugins}
    missing = required_plugins - present
    if missing:
        raise DistributionError(f"required plugins unavailable: {sorted(missing)}")
    content = {
        "schemaVersion": 1,
        "profile": capability["profile"],
        "target": target,
        "capabilityDigest": sha256_bytes(canonical(capability)),
        "plugins": sorted(plugins, key=lambda item: item["id"]),
    }
    content["closureDigest"] = sha256_bytes(canonical(content))
    return content


def lock_artifacts(lock: dict[str, Any]) -> dict[str, dict[str, Any]]:
    result = {}
    for plugin in lock.get("plugins", []):
        for artifact in plugin.get("artifacts", []):
            if artifact["id"] in result:
                raise DistributionError(f"duplicate lock artifact {artifact['id']}")
            result[artifact["id"]] = artifact
    return result


def validate_lock(lock: dict[str, Any], capability: dict[str, Any] | None = None) -> None:
    if not isinstance(lock, dict):
        raise DistributionError("distribution lock must be an object")
    required = {"schemaVersion", "profile", "target", "capabilityDigest", "plugins", "closureDigest"}
    if set(lock) != required or lock.get("schemaVersion") != 1:
        raise DistributionError("distribution lock schema is invalid")
    digest = lock.get("closureDigest")
    if not isinstance(digest, str) or not re.fullmatch(r"[0-9a-f]{64}", digest):
        raise DistributionError("distribution lock has no valid closure digest")
    if not re.fullmatch(r"[0-9a-f]{64}", lock.get("capabilityDigest", "")):
        raise DistributionError("distribution lock has no valid capability digest")
    target_key(lock.get("target"))
    plugins = lock.get("plugins")
    if not isinstance(plugins, list):
        raise DistributionError("distribution lock plugins must be a list")
    plugin_ids: set[str] = set()
    artifact_ids: set[str] = set()
    for plugin in plugins:
        if not isinstance(plugin, dict) or set(plugin) != {"id", "version", "artifacts"}:
            raise DistributionError("distribution lock plugin entry is invalid")
        plugin_id = plugin.get("id")
        if not isinstance(plugin_id, str) or not plugin_id or plugin_id in plugin_ids:
            raise DistributionError("distribution lock plugin id is invalid or duplicate")
        plugin_ids.add(plugin_id)
        if not isinstance(plugin.get("version"), str) or not plugin["version"]:
            raise DistributionError(f"{plugin_id}: distribution lock version is invalid")
        if not isinstance(plugin.get("artifacts"), list) or not plugin["artifacts"]:
            raise DistributionError(f"{plugin_id}: distribution lock artifacts are invalid")
        for artifact in plugin["artifacts"]:
            fields = {"id", "kind", "target", "sha256", "license", "abi"}
            if not isinstance(artifact, dict) or frozenset(artifact) not in {
                frozenset(fields),
                frozenset(fields | {"signature"}),
            }:
                raise DistributionError(f"{plugin_id}: distribution lock artifact entry is invalid")
            artifact_id = artifact.get("id")
            if not isinstance(artifact_id, str) or not artifact_id or artifact_id in artifact_ids:
                raise DistributionError("distribution lock artifact id is invalid or duplicate")
            artifact_ids.add(artifact_id)
            if artifact.get("target") != lock.get("target"):
                raise DistributionError(f"{artifact_id}: artifact target differs from lock target")
            if not re.fullmatch(r"[0-9a-f]{64}", artifact.get("sha256", "")):
                raise DistributionError(f"{artifact_id}: artifact digest is invalid")
            for field in ("kind", "license", "abi"):
                if not isinstance(artifact.get(field), str) or not artifact[field]:
                    raise DistributionError(f"{artifact_id}: artifact {field} is invalid")
            signature = artifact.get("signature")
            if signature is not None and (
                not isinstance(signature, dict)
                or signature.get("algorithm") != "ed25519"
                or not all(
                    isinstance(signature.get(field), str) and signature[field]
                    for field in ("key_id", "value")
                )
            ):
                raise DistributionError(f"{artifact_id}: artifact signature is invalid")
            if artifact["kind"] == "sandbox-worker" and signature is None:
                raise DistributionError(f"{artifact_id}: sandbox worker is unsigned")
    content = dict(lock)
    del content["closureDigest"]
    if sha256_bytes(canonical(content)) != digest:
        raise DistributionError("distribution lock digest mismatch")
    if capability is not None:
        validate_capability(capability)
        if lock.get("profile") != capability.get("profile") or lock.get("target") != capability.get("target"):
            raise DistributionError("distribution lock target does not match capability snapshot")
        if lock.get("capabilityDigest") != sha256_bytes(canonical(capability)):
            raise DistributionError("distribution lock capability digest mismatch")
        locked_plugins = {plugin["id"] for plugin in plugins}
        if not locked_plugins.issubset(capability["allowedPlugins"]):
            raise DistributionError("distribution lock contains a plugin outside the allowlist")
        if not set(capability["requiredPlugins"]).issubset(locked_plugins):
            raise DistributionError("distribution lock is missing a required plugin")
        allowed_kinds = set(capability["allowedArtifactKinds"])
        if any(artifact["kind"] not in allowed_kinds for artifact in lock_artifacts(lock).values()):
            raise DistributionError("distribution lock contains a forbidden artifact kind")


def verify_files(
    lock: dict[str, Any],
    catalog: dict[str, str],
    root: Path,
    capability: dict[str, Any] | None = None,
) -> None:
    validate_lock(lock, capability)
    if not isinstance(catalog, dict) or not all(
        isinstance(artifact_id, str) and isinstance(relative, str) and relative
        for artifact_id, relative in catalog.items()
    ):
        raise DistributionError("artifact catalog must map artifact ids to relative paths")
    if not root.is_dir():
        raise DistributionError(f"closure root is not a directory: {root}")
    expected = lock_artifacts(lock)
    if set(catalog) != set(expected):
        raise DistributionError("artifact catalog must exactly match the distribution lock")
    for artifact_id, relative in catalog.items():
        path = (root / relative).resolve()
        try:
            path.relative_to(root.resolve())
        except ValueError as error:
            raise DistributionError(f"artifact escapes closure root: {relative}") from error
        if not path.is_file() or path.is_symlink():
            raise DistributionError(f"artifact missing or symlinked: {relative}")
        digest = sha256_bytes(path.read_bytes())
        if digest != expected[artifact_id]["sha256"]:
            raise DistributionError(f"artifact digest mismatch: {artifact_id}")


def scan_closure(
    lock: dict[str, Any],
    root: Path,
    catalog: dict[str, str],
    capability: dict[str, Any] | None = None,
) -> None:
    verify_files(lock, catalog, root, capability)
    profile = lock.get("profile")
    forbidden_mobile = re.compile(
        r"(^|/)(node(?:\.exe)?|node_modules|hx(?:\.exe)?|helix)(/|$)|"
        r"(^|/)openmuse/dsh(/|$)|\.exe$|\.dylib$",
        re.IGNORECASE,
    )
    for path in root.rglob("*"):
        if path.is_symlink():
            raise DistributionError(f"symlink cannot enter closure: {path}")
        if not path.is_file():
            continue
        relative = path.relative_to(root).as_posix()
        if profile in {"android", "ios"} and forbidden_mobile.search(relative):
            raise DistributionError(f"desktop runtime found in mobile closure: {relative}")
    if profile == "sandbox":
        forbidden = [
            item for item in lock_artifacts(lock).values()
            if item["kind"] != "sandbox-worker"
        ]
        if forbidden:
            raise DistributionError("sandbox closure contains a non-worker artifact")


def sbom(lock: dict[str, Any], source_date_epoch: int) -> dict[str, Any]:
    created = datetime.fromtimestamp(source_date_epoch, timezone.utc).strftime("%Y-%m-%dT%H:%M:%SZ")
    packages = []
    for plugin in lock["plugins"]:
        for artifact in plugin["artifacts"]:
            packages.append({
                "SPDXID": f"SPDXRef-Artifact-{safe_id(artifact['id'])}",
                "name": f"{plugin['id']}/{artifact['id']}",
                "versionInfo": plugin["version"],
                "downloadLocation": "NOASSERTION",
                "filesAnalyzed": False,
                "licenseConcluded": artifact["license"],
                "licenseDeclared": artifact["license"],
                "checksums": [{"algorithm": "SHA256", "checksumValue": artifact["sha256"]}],
            })
    relationships = [
        {
            "spdxElementId": "SPDXRef-DOCUMENT",
            "relationshipType": "DESCRIBES",
            "relatedSpdxElement": package["SPDXID"],
        }
        for package in packages
    ]
    return {
        "spdxVersion": "SPDX-2.3",
        "dataLicense": "CC0-1.0",
        "SPDXID": "SPDXRef-DOCUMENT",
        "name": f"OpenMuse-{lock['profile']}-{lock['closureDigest'][:12]}",
        "documentNamespace": f"https://openmuse.local/sbom/{lock['closureDigest']}",
        "creationInfo": {"created": created, "creators": ["Tool: openmuse-distribution-lock/1"]},
        "packages": packages,
        "relationships": relationships,
    }


def notices(lock: dict[str, Any]) -> str:
    lines = ["# OpenMuse Distribution Notices", "", f"Closure: `{lock['closureDigest']}`", ""]
    for plugin in lock["plugins"]:
        lines.extend([f"## {plugin['id']} {plugin['version']}", ""])
        for artifact in plugin["artifacts"]:
            lines.append(f"- `{artifact['id']}` — {artifact['license']} — sha256:{artifact['sha256']}")
        lines.append("")
    return "\n".join(lines)


def safe_id(value: str) -> str:
    return re.sub(r"[^A-Za-z0-9.-]", "-", value)


def write(path: Path, value: bytes) -> None:
    path.parent.mkdir(parents=True, exist_ok=True)
    path.write_bytes(value)


def main() -> None:
    parser = argparse.ArgumentParser(description=__doc__)
    sub = parser.add_subparsers(dest="command", required=True)
    resolve_parser = sub.add_parser("resolve")
    resolve_parser.add_argument("--capability", type=Path, required=True)
    resolve_parser.add_argument("--manifest", type=Path, action="append", required=True)
    resolve_parser.add_argument("--lock", type=Path, required=True)
    resolve_parser.add_argument("--sbom", type=Path, required=True)
    resolve_parser.add_argument("--notices", type=Path, required=True)
    resolve_parser.add_argument("--source-date-epoch", type=int, default=0)
    verify_parser = sub.add_parser("verify")
    verify_parser.add_argument("--lock", type=Path, required=True)
    verify_parser.add_argument("--capability", type=Path, required=True)
    verify_parser.add_argument("--catalog", type=Path, required=True)
    verify_parser.add_argument("--root", type=Path, required=True)
    args = parser.parse_args()
    try:
        if args.command == "resolve":
            value = resolve(args.manifest, load_json(args.capability))
            write(args.lock, canonical(value))
            write(args.sbom, canonical(sbom(value, args.source_date_epoch)))
            write(args.notices, notices(value).encode())
        else:
            value = load_json(args.lock)
            scan_closure(value, args.root, load_json(args.catalog), load_json(args.capability))
    except DistributionError as error:
        parser.exit(1, f"distribution gate failed: {error}\n")


if __name__ == "__main__":
    main()
