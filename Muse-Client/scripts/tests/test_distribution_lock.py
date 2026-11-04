from __future__ import annotations

import json
import tempfile
import unittest
from pathlib import Path

from scripts.distribution_lock import (
    canonical,
    DistributionError,
    notices,
    resolve,
    sbom,
    scan_closure,
    sha256_bytes,
)


ROOT = Path(__file__).resolve().parents[2]
MANIFESTS = sorted(
    path for path in (ROOT / "schemas/fixtures/plugin/v2").glob("*.json")
    if path.name != "manifest.json"
)


def read(path: Path):
    return json.loads(path.read_text(encoding="utf-8"))


class DistributionLockTests(unittest.TestCase):
    def test_desktop_resolution_is_deterministic_and_keeps_current_closure(self):
        capability = read(ROOT / "distribution/targets/macos-aarch64.json")
        first = resolve(MANIFESTS, capability)
        second = resolve(list(reversed(MANIFESTS)), capability)
        self.assertEqual(first, second)
        self.assertEqual(
            {item["id"] for item in first["plugins"]},
            {
                "com.openmuse.helix",
                "com.openmuse.dsh-agent",
                "com.openmuse.native-text-gate",
                "com.openmuse.open-file-viewer",
            },
        )
        self.assertEqual(sbom(first, 0), sbom(second, 0))
        self.assertEqual(notices(first), notices(second))
        self.assertEqual(
            len(sbom(first, 0)["packages"]),
            sum(len(plugin["artifacts"]) for plugin in first["plugins"]),
        )

    def test_mobile_lock_excludes_desktop_plugins_and_scanner_rejects_node(self):
        capability = read(ROOT / "distribution/targets/android-aarch64.json")
        lock = resolve(MANIFESTS, capability)
        self.assertEqual(lock["plugins"], [])
        with tempfile.TemporaryDirectory() as directory:
            root = Path(directory)
            (root / "assets").mkdir()
            (root / "assets/app.bin").write_bytes(b"mobile")
            scan_closure(lock, root, {}, capability)
            node = root / "openmuse/dsh/node/node"
            node.parent.mkdir(parents=True)
            node.write_bytes(b"desktop runtime")
            with self.assertRaisesRegex(DistributionError, "desktop runtime"):
                scan_closure(lock, root, {}, capability)

            wrong_capability = read(ROOT / "distribution/targets/macos-aarch64.json")
            with self.assertRaisesRegex(DistributionError, "target does not match"):
                scan_closure(lock, root, {}, wrong_capability)

    def test_missing_target_and_invalid_declared_digest_fail_closed(self):
        capability = read(ROOT / "distribution/targets/macos-aarch64.json")
        with tempfile.TemporaryDirectory() as directory:
            manifest = read(MANIFESTS[0])
            manifest["compatibility"]["targets"] = [
                item for item in manifest["compatibility"]["targets"]
                if item["target"]["os"] != "macos"
            ]
            path = Path(directory) / "manifest.json"
            path.write_text(json.dumps(manifest), encoding="utf-8")
            capability["allowedPlugins"] = [manifest["id"]]
            capability["requiredPlugins"] = []
            with self.assertRaisesRegex(DistributionError, "target"):
                resolve([path], capability)

            manifest = read(MANIFESTS[0])
            manifest["artifacts"][0]["digest"]["value"] = "not-a-digest"
            path.write_text(json.dumps(manifest), encoding="utf-8")
            with self.assertRaisesRegex(DistributionError, "invalid digest"):
                resolve([path], capability)

    def test_actual_artifact_digest_and_exact_catalog_are_enforced(self):
        with tempfile.TemporaryDirectory() as directory:
            root = Path(directory)
            artifact = root / "worker.bin"
            artifact.write_bytes(b"worker-v1")
            target = {"os": "linux", "arch": "x86_64", "libc": "gnu"}
            manifest = {
                "manifest_version": 2,
                "id": "com.openmuse.office-worker",
                "version": "1.0.0",
                "compatibility": {"targets": [{"target": target, "status": "supported"}]},
                "artifacts": [{
                    "id": "office.worker",
                    "kind": "sandbox-worker",
                    "target": target,
                    "digest": {"algorithm": "sha256", "value": sha256_bytes(b"worker-v1")},
                    "signature": {
                        "algorithm": "ed25519",
                        "key_id": "fixture.release",
                        "value": "fixture-signature",
                    },
                    "license": "AGPL-3.0-only",
                    "abi": "openmuse.sandbox-worker/v1",
                }],
            }
            manifest_path = root / "manifest.json"
            manifest_path.write_text(json.dumps(manifest), encoding="utf-8")
            capability = {
                "profile": "sandbox",
                "target": target,
                "allowedArtifactKinds": ["sandbox-worker"],
                "allowedPlugins": [manifest["id"]],
                "requiredPlugins": [manifest["id"]],
            }
            lock = resolve([manifest_path], capability)
            scan_closure(lock, root, {"office.worker": "worker.bin"})

            unsigned = read(manifest_path)
            del unsigned["artifacts"][0]["signature"]
            manifest_path.write_text(json.dumps(unsigned), encoding="utf-8")
            with self.assertRaisesRegex(DistributionError, "unsigned"):
                resolve([manifest_path], capability)

            artifact.write_bytes(b"tampered")
            with self.assertRaisesRegex(DistributionError, "digest mismatch"):
                scan_closure(lock, root, {"office.worker": "worker.bin"})
            with self.assertRaisesRegex(DistributionError, "exactly match"):
                scan_closure(lock, root, {})

            artifact.write_bytes(b"worker-v1")
            lock["plugins"][0]["version"] = "tampered"
            with self.assertRaisesRegex(DistributionError, "lock digest mismatch"):
                scan_closure(lock, root, {"office.worker": "worker.bin"})

    def test_sandbox_lock_rejects_non_worker_artifacts(self):
        with tempfile.TemporaryDirectory() as directory:
            root = Path(directory)
            artifact = root / "host.bin"
            artifact.write_bytes(b"host")
            lock = {
                "schemaVersion": 1,
                "profile": "sandbox",
                "target": {"os": "linux", "arch": "x86_64", "libc": "gnu"},
                "capabilityDigest": sha256_bytes(b"fixture"),
                "plugins": [{
                    "id": "bad",
                    "version": "1",
                    "artifacts": [{
                        "id": "bad.host",
                        "kind": "host-bundle",
                        "target": {"os": "linux", "arch": "x86_64", "libc": "gnu"},
                        "sha256": sha256_bytes(b"host"),
                        "license": "AGPL-3.0-only",
                        "abi": "openmuse.plugin-ui/v2",
                    }],
                }],
            }
            lock["closureDigest"] = sha256_bytes(canonical(lock))
            with self.assertRaisesRegex(DistributionError, "non-worker"):
                scan_closure(lock, root, {"bad.host": "host.bin"})


if __name__ == "__main__":
    unittest.main()
