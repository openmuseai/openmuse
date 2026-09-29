# OpenMuse local desktop

This repository contains the clean-room OpenMuse desktop Host, plugin SDK,
platform broker and first-party plugins. It contains no web product and does
not inherit a legacy workspace format.

The current executable slice includes:

- RPC-friendly protocol envelopes and version negotiation;
- separate Helix, open-file-viewer, DSH and Native View plugin packages;
- capability/permission enforcement;
- deterministic command and service routing;
- lifecycle-bound cleanup;
- a three-pane local Workspace / Editor / DSH workbench.

Run the validation:

```bash
cargo test --workspace
./scripts/test_muse_packages.sh
cd app/openmuse_host
flutter analyze
flutter test
flutter build macos --debug
```

The macOS release build is self-contained with respect to other source
repositories: pinned Helix assets, the DSH registry lockfile plus the local
model-plugin tarball, and official Node archives live under this repository.
Run `./scripts/package_macos.sh` on
macOS; it assembles the DSH npm closure in `target/`, verifies checksums, builds
the Flutter app, and writes `dist/OpenMuse-macos.zip`. npm still downloads
third-party registry dependencies pinned by `third_party/dsh/package-lock.json`
unless they are already cached. No other product checkout is used.

Architecture: [`docs/PLUGIN-HOST-DSH-ARCHITECTURE.zh-CN.md`](docs/PLUGIN-HOST-DSH-ARCHITECTURE.zh-CN.md).
Workbench parity specification: [`docs/WORKBENCH-PARITY-SPEC.zh-CN.md`](docs/WORKBENCH-PARITY-SPEC.zh-CN.md).
Code reuse and provenance: [`docs/CODE-REUSE-PROVENANCE.zh-CN.md`](docs/CODE-REUSE-PROVENANCE.zh-CN.md).
Mobile product design: [`docs/MOBILE-PRODUCT-PRD.zh-CN.md`](docs/MOBILE-PRODUCT-PRD.zh-CN.md).
Mobile architecture and implementation plan: [`docs/MOBILE-ARCHITECTURE-IMPLEMENTATION-PLAN.zh-CN.md`](docs/MOBILE-ARCHITECTURE-IMPLEMENTATION-PLAN.zh-CN.md).
S3 Storage ABI and MinIO/RustFS selection: [`docs/S3-STORAGE-TECHNICAL-SELECTION.zh-CN.md`](docs/S3-STORAGE-TECHNICAL-SELECTION.zh-CN.md).
OpenMuse Workspace Sandbox and DSH execution plane: [`docs/DSH-WORKSPACE-EXECUTION-PLANE-FEASIBILITY.zh-CN.md`](docs/DSH-WORKSPACE-EXECUTION-PLANE-FEASIBILITY.zh-CN.md).
Mobile/Cloud/Sandbox delivery roadmap: [`docs/MOBILE-CLOUD-SANDBOX-DELIVERY-ROADMAP.zh-CN.md`](docs/MOBILE-CLOUD-SANDBOX-DELIVERY-ROADMAP.zh-CN.md).
Cross-domain contract baseline v1: [`docs/CONTRACT-BASELINE-V1.zh-CN.md`](docs/CONTRACT-BASELINE-V1.zh-CN.md).
Plugin Manifest v2: [`docs/PLUGIN-MANIFEST-V2.zh-CN.md`](docs/PLUGIN-MANIFEST-V2.zh-CN.md).
Broker delegation, policy and audit: [`docs/BROKER-DELEGATION-POLICY-AUDIT.zh-CN.md`](docs/BROKER-DELEGATION-POLICY-AUDIT.zh-CN.md).
Workspace / Resource Authority v1: [`docs/WORKSPACE-RESOURCE-AUTHORITY-V1.zh-CN.md`](docs/WORKSPACE-RESOURCE-AUTHORITY-V1.zh-CN.md).
Distribution lock, SBOM and closure gate: [`docs/DISTRIBUTION-LOCK-SBOM-CLOSURE-GATE.zh-CN.md`](docs/DISTRIBUTION-LOCK-SBOM-CLOSURE-GATE.zh-CN.md).
BlobStorePort, S3 profile and provider TCK: [`docs/STORAGE-CONTRACT-S3-PROFILE-TCK.zh-CN.md`](docs/STORAGE-CONTRACT-S3-PROFILE-TCK.zh-CN.md).
S3 adapter, credentials and endpoint security: [`docs/S3-ADAPTER-CREDENTIAL-ENDPOINT-SECURITY.zh-CN.md`](docs/S3-ADAPTER-CREDENTIAL-ENDPOINT-SECURITY.zh-CN.md).
Cloud Workspace metadata, CAS and outbox: [`docs/CLOUD-WORKSPACE-METADATA-CAS-OUTBOX.zh-CN.md`](docs/CLOUD-WORKSPACE-METADATA-CAS-OUTBOX.zh-CN.md).
Workspace Sync Plugin policy, durable orchestration and conflicts: [`docs/WORKSPACE-SYNC-PLUGIN.zh-CN.md`](docs/WORKSPACE-SYNC-PLUGIN.zh-CN.md).
BYOS, provider migration and portable export: [`docs/BYOS-PROVIDER-MIGRATION-PORTABILITY.zh-CN.md`](docs/BYOS-PROVIDER-MIGRATION-PORTABILITY.zh-CN.md).
RustFS qualification and managed rollout gate: [`docs/RUSTFS-QUALIFICATION-MANAGED-ROLLOUT.zh-CN.md`](docs/RUSTFS-QUALIFICATION-MANAGED-ROLLOUT.zh-CN.md).
DSH 0.1.7 provider contract and execution-world TCK: [`docs/DSH-0.1.7-PROVIDER-CONTRACT-TCK.zh-CN.md`](docs/DSH-0.1.7-PROVIDER-CONTRACT-TCK.zh-CN.md).
Host Sandbox Service, lease and local isolated runtime: [`docs/HOST-SANDBOX-SERVICE-LOCAL-RUNTIME.zh-CN.md`](docs/HOST-SANDBOX-SERVICE-LOCAL-RUNTIME.zh-CN.md).
Cloud execution runtime and DSH provider group: [`docs/CLOUD-EXECUTION-RUNTIME-DSH-PROVIDERS.zh-CN.md`](docs/CLOUD-EXECUTION-RUNTIME-DSH-PROVIDERS.zh-CN.md).
Draft transaction, checkpoint, quiescence and recovery: [`docs/DRAFT-CHECKPOINT-QUIESCENCE-RECOVERY.zh-CN.md`](docs/DRAFT-CHECKPOINT-QUIESCENCE-RECOVERY.zh-CN.md).
Plugin CLI registry and artifact resolver: [`docs/PLUGIN-CLI-REGISTRY-ARTIFACT-RESOLVER.zh-CN.md`](docs/PLUGIN-CLI-REGISTRY-ARTIFACT-RESOLVER.zh-CN.md).

OpenMuse source uses the repository's AGPL-3.0 `LICENSE`, as selected by the
project owner. Third-party components retain their own licenses; see
[`third_party/README.md`](third_party/README.md). A distributable release still
requires complete third-party notices and platform-specific verification.
