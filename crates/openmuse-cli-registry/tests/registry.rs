use openmuse_cli_registry::*;
use openmuse_plugin_protocol::*;
use serde_json::json;
use std::collections::BTreeSet;

fn target(os: TargetOs) -> TargetTriple {
    match os {
        TargetOs::Linux => TargetTriple {
            os,
            arch: TargetArch::X86_64,
            libc: TargetLibc::Gnu,
        },
        TargetOs::Macos => TargetTriple {
            os,
            arch: TargetArch::Aarch64,
            libc: TargetLibc::Darwin,
        },
        _ => unreachable!(),
    }
}

fn installed(
    id: &str,
    version: &str,
    worker_target: TargetTriple,
    namespace: &str,
) -> InstalledPlugin {
    let digest = "a".repeat(64);
    let permission = Permission::new("workspace.read");
    InstalledPlugin {
        manifest: PluginManifestV2 {
            manifest_version: 2,
            id: PluginId::new(id),
            name: id.into(),
            version: version.into(),
            protocol: PROTOCOL_VERSION,
            ui_runtime: UiRuntime {
                kind: UiRuntimeKind::None,
                entrypoint: None,
            },
            execution_connector: ExecutionConnector {
                kind: ExecutionConnectorKind::SandboxProvider,
                protocol: Some("worker@1".into()),
            },
            compatibility: Compatibility {
                targets: vec![TargetDecision {
                    target: worker_target,
                    status: TargetStatus::Supported,
                    reason: None,
                }],
            },
            artifacts: vec![PluginArtifact {
                id: "worker".into(),
                kind: ArtifactKind::SandboxWorker,
                target: worker_target,
                digest: ArtifactDigest {
                    algorithm: DigestAlgorithm::Sha256,
                    value: digest.clone(),
                },
                signature: Some(ArtifactSignature {
                    algorithm: SignatureAlgorithm::Ed25519,
                    key_id: "release".into(),
                    value: "signature".into(),
                }),
                license: "Apache-2.0".into(),
                abi: "openmuse-worker@1".into(),
            }],
            activation_events: vec![],
            requested_permissions: BTreeSet::from([permission.clone()]),
            presentation: Presentation::default(),
            contributes: ContributionsV2 {
                agent_cli: vec![AgentCliContribution {
                    group: "office".into(),
                    namespace: namespace.into(),
                    command: "inspect".into(),
                    schema: json!({"type":"object"}),
                    required_permissions: BTreeSet::from([permission.clone()]),
                    effects: BTreeSet::from(["read".into()]),
                }],
                ..Default::default()
            },
            install: None,
        },
        granted_permissions: BTreeSet::from([permission]),
        artifacts: vec![ArtifactEvidence {
            artifact_id: "worker".into(),
            digest,
            signature_valid: true,
            revoked: false,
        }],
    }
}

fn resolver() -> CliRegistryResolver {
    CliRegistryResolver::new(
        target(TargetOs::Linux),
        BTreeSet::from(["openmuse-worker@1".into()]),
        BTreeSet::from(["Apache-2.0".into()]),
    )
}

#[test]
fn mac_only_plugin_is_explicitly_unavailable_in_cloud() {
    let snapshot = resolver()
        .resolve(
            1,
            &[installed("helix", "1", target(TargetOs::Macos), "helix")],
            &BTreeSet::new(),
        )
        .unwrap();
    assert!(snapshot.commands.is_empty());
    assert_eq!(snapshot.unavailable[0].plugin_id, "helix");
}

#[test]
fn existing_lease_snapshot_does_not_silently_upgrade() {
    let first = resolver()
        .resolve(
            1,
            &[installed("office", "1", target(TargetOs::Linux), "docs")],
            &BTreeSet::new(),
        )
        .unwrap();
    let second = resolver()
        .resolve(
            2,
            &[installed("office", "2", target(TargetOs::Linux), "docs")],
            &BTreeSet::new(),
        )
        .unwrap();
    assert_eq!(first.commands[0].plugin_version, "1");
    assert_eq!(first.generation, 1);
    assert_ne!(first.digest, second.digest);
}

#[test]
fn workspace_requirements_cannot_install_or_register_a_forged_plugin() {
    let snapshot = resolver()
        .resolve(1, &[], &BTreeSet::from(["evil.forged".into()]))
        .unwrap();
    assert!(snapshot.commands.is_empty());
    assert_eq!(
        snapshot.unavailable[0].reason,
        "workspace requirement is not an installed Plugin"
    );
}

#[test]
fn discovery_only_returns_granted_verified_current_target_commands() {
    let mut plugin = installed("office", "1", target(TargetOs::Linux), "docs");
    plugin.granted_permissions.clear();
    let denied = resolver().resolve(1, &[plugin], &BTreeSet::new()).unwrap();
    assert!(denied.commands.is_empty());
    let admitted = resolver()
        .resolve(
            1,
            &[installed("office", "1", target(TargetOs::Linux), "docs")],
            &BTreeSet::new(),
        )
        .unwrap();
    assert_eq!(admitted.commands.len(), 1);
}

#[test]
fn namespace_conflicts_fail_deterministically() {
    let plugins = vec![
        installed("a", "1", target(TargetOs::Linux), "docs"),
        installed("b", "1", target(TargetOs::Linux), "docs"),
    ];
    assert_eq!(
        resolver()
            .resolve(1, &plugins, &BTreeSet::new())
            .unwrap_err(),
        RegistryError::NamespaceConflict("office/docs/inspect".into())
    );
}
