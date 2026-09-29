use openmuse_plugin_protocol::{
    ArtifactDigest, ArtifactKind, ArtifactSignature, ArtifactSignatureVerifier, Compatibility,
    DigestAlgorithm, ManifestV2Error, PluginArtifact, PluginManifest, PluginManifestV2,
    SignatureAlgorithm, TargetArch, TargetDecision, TargetLibc, TargetOs, TargetStatus,
    TargetTriple, migrate_manifest_v1,
};
use serde_json::Value;

const HELIX_V1: &str = include_str!("../../../plugins/helix/openmuse.plugin.json");

const V2_FIXTURES: [(&str, &[u8]); 4] = [
    (
        "helix",
        include_bytes!("../../../schemas/fixtures/plugin/v2/helix.json"),
    ),
    (
        "dsh-agent",
        include_bytes!("../../../schemas/fixtures/plugin/v2/dsh-agent.json"),
    ),
    (
        "native-text-gate",
        include_bytes!("../../../schemas/fixtures/plugin/v2/native-text-gate.json"),
    ),
    (
        "open-file-viewer",
        include_bytes!("../../../schemas/fixtures/plugin/v2/open-file-viewer.json"),
    ),
];

fn android() -> TargetTriple {
    TargetTriple {
        os: TargetOs::Android,
        arch: TargetArch::Aarch64,
        libc: TargetLibc::Bionic,
    }
}

fn ios() -> TargetTriple {
    TargetTriple {
        os: TargetOs::Ios,
        arch: TargetArch::Aarch64,
        libc: TargetLibc::Darwin,
    }
}

fn macos_arm64() -> TargetTriple {
    TargetTriple {
        os: TargetOs::Macos,
        arch: TargetArch::Aarch64,
        libc: TargetLibc::Darwin,
    }
}

#[test]
fn shipped_v2_fixtures_are_strict_and_have_explicit_platform_decisions() {
    for (name, raw) in V2_FIXTURES {
        let manifest = PluginManifestV2::from_json_slice(raw)
            .unwrap_or_else(|error| panic!("{name}: {error}"));
        assert_eq!(manifest.compatibility.targets.len(), 6, "{name}");
        for target in [android(), ios()] {
            let decision = manifest
                .compatibility
                .targets
                .iter()
                .find(|decision| decision.target == target)
                .expect("mobile target decision");
            assert_eq!(decision.status, TargetStatus::Unsupported, "{name}");
            assert!(
                decision
                    .reason
                    .as_ref()
                    .is_some_and(|reason| !reason.is_empty())
            );
            assert!(matches!(
                manifest.resolve_artifacts(target),
                Err(ManifestV2Error::UnsupportedTarget(actual)) if actual == target
            ));
        }
    }
}

#[test]
fn unknown_fields_and_targets_fail_closed() {
    let mut value: Value = serde_json::from_slice(V2_FIXTURES[0].1).unwrap();
    value
        .as_object_mut()
        .unwrap()
        .insert("future_grant".into(), Value::Bool(true));
    assert!(matches!(
        PluginManifestV2::from_json_slice(&serde_json::to_vec(&value).unwrap()),
        Err(ManifestV2Error::InvalidJson(_))
    ));

    let mut value: Value = serde_json::from_slice(V2_FIXTURES[0].1).unwrap();
    value["contributes"]["commands"][0]["ambient_authority"] = Value::Bool(true);
    assert!(matches!(
        PluginManifestV2::from_json_slice(&serde_json::to_vec(&value).unwrap()),
        Err(ManifestV2Error::InvalidJson(_))
    ));

    let mut value: Value = serde_json::from_slice(V2_FIXTURES[0].1).unwrap();
    value["compatibility"]["targets"][0]["target"]["os"] = Value::String("plan9".into());
    assert!(matches!(
        PluginManifestV2::from_json_slice(&serde_json::to_vec(&value).unwrap()),
        Err(ManifestV2Error::InvalidJson(_))
    ));
}

#[test]
fn v1_migration_is_pure_deterministic_and_conservative() {
    let v1: PluginManifest = serde_json::from_str(HELIX_V1).unwrap();
    let first = migrate_manifest_v1(&v1);
    let second = migrate_manifest_v1(&v1);
    assert_eq!(first, second);
    assert_eq!(
        serde_json::to_vec(&first).unwrap(),
        serde_json::to_vec(&second).unwrap()
    );
    first.validate().unwrap();
    assert!(
        first
            .compatibility
            .targets
            .iter()
            .all(|decision| decision.status == TargetStatus::Unsupported)
    );
    assert!(first.artifacts.is_empty());
    assert!(first.contributes.agent_cli.is_empty());
}

struct SignatureDecision(bool);

impl ArtifactSignatureVerifier for SignatureDecision {
    fn verify(
        &self,
        _artifact: &PluginArtifact,
        _bytes: &[u8],
        _signature: &ArtifactSignature,
    ) -> bool {
        self.0
    }
}

fn sandbox_manifest(signature: Option<ArtifactSignature>) -> PluginManifestV2 {
    let mut manifest = PluginManifestV2::from_json_slice(V2_FIXTURES[3].1).expect("viewer fixture");
    manifest.compatibility = Compatibility {
        targets: vec![TargetDecision {
            target: macos_arm64(),
            status: TargetStatus::Supported,
            reason: None,
        }],
    };
    manifest.artifacts = vec![PluginArtifact {
        id: "office.worker.fixture".into(),
        kind: ArtifactKind::SandboxWorker,
        target: macos_arm64(),
        digest: ArtifactDigest {
            algorithm: DigestAlgorithm::Sha256,
            value: "87eba76e7f3164534045ba922e7770fb58bbd14ad732bbf5ba6f11cc56989e6e".into(),
        },
        signature,
        license: "AGPL-3.0-only".into(),
        abi: "openmuse.sandbox-worker/v1".into(),
    }];
    manifest
}

fn fixture_signature() -> ArtifactSignature {
    ArtifactSignature {
        algorithm: SignatureAlgorithm::Ed25519,
        key_id: "fixture.release".into(),
        value: "fixture-signature".into(),
    }
}

#[test]
fn sandbox_worker_requires_matching_digest_and_accepted_signature() {
    let unsigned = sandbox_manifest(None);
    assert!(matches!(
        unsigned.verify_sandbox_worker("office.worker.fixture", b"worker", &SignatureDecision(true)),
        Err(ManifestV2Error::SignatureRequired(id)) if id == "office.worker.fixture"
    ));

    let signed = sandbox_manifest(Some(fixture_signature()));
    assert!(matches!(
        signed.verify_sandbox_worker("office.worker.fixture", b"tampered", &SignatureDecision(true)),
        Err(ManifestV2Error::DigestMismatch(id)) if id == "office.worker.fixture"
    ));
    assert!(matches!(
        signed.verify_sandbox_worker("office.worker.fixture", b"worker", &SignatureDecision(false)),
        Err(ManifestV2Error::SignatureRejected(id)) if id == "office.worker.fixture"
    ));

    let verified = signed
        .verify_sandbox_worker("office.worker.fixture", b"worker", &SignatureDecision(true))
        .unwrap();
    assert_eq!(verified.target, macos_arm64());
    assert_eq!(verified.sha256, signed.artifacts[0].digest.value);
}
