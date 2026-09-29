use openmuse_sandbox_security::*;
use std::collections::{BTreeMap, BTreeSet};

fn profile() -> SecurityProfile {
    SecurityProfile {
        rootless: true,
        read_only_image: true,
        seccomp_enforced: true,
        capabilities_dropped: true,
        proc_hidepid: true,
        max_cpu_millis: 10_000,
        max_memory_bytes: 512 * 1024 * 1024,
        max_pids: 64,
        max_disk_bytes: 1024 * 1024 * 1024,
        max_runtime_ms: 60_000,
        egress_allowlist: BTreeSet::from(["api.openmuse.example".into()]),
    }
}

fn request() -> ExecutionAdmissionRequest {
    ExecutionAdmissionRequest {
        actor_ref: "actor:user".into(),
        caller_ref: "plugin:dsh".into(),
        target_ref: "worker:office".into(),
        artifact: ArtifactAttestation {
            digest: format!("sha256:{}", "a".repeat(64)),
            signature_valid: true,
            sbom_digest: format!("sha256:{}", "b".repeat(64)),
            revoked: false,
        },
        draft_ref: "draft:1".into(),
        expected_revision: "revision:1".into(),
        requested: RequestedCeiling {
            cpu_millis: 1000,
            memory_bytes: 64 * 1024 * 1024,
            pids: 8,
            disk_bytes: 1024,
            runtime_ms: 1000,
        },
    }
}

#[test]
fn incomplete_profile_unsigned_revoked_and_escalated_requests_fail_closed() {
    let mut incomplete = profile();
    incomplete.seccomp_enforced = false;
    assert!(matches!(
        ProductionSecurityGate::new(incomplete),
        Err(SecurityError::ProfileIncomplete)
    ));
    let mut gate = ProductionSecurityGate::new(profile()).unwrap();
    let mut unsigned = request();
    unsigned.artifact.signature_valid = false;
    assert_eq!(
        gate.admit(unsigned).unwrap_err(),
        SecurityError::ArtifactDenied
    );
    let mut escalated = request();
    escalated.requested.pids = 65;
    assert_eq!(
        gate.admit(escalated).unwrap_err(),
        SecurityError::QuotaExceeded
    );
}

#[test]
fn egress_is_default_deny_and_exact_https_allowlist_only() {
    let gate = ProductionSecurityGate::new(profile()).unwrap();
    assert!(
        gate.authorize_egress("https://api.openmuse.example/v1")
            .is_ok()
    );
    for url in [
        "http://api.openmuse.example",
        "https://evil.example",
        "https://api.openmuse.example.evil.test",
    ] {
        assert_eq!(
            gate.authorize_egress(url).unwrap_err(),
            SecurityError::EgressDenied
        );
    }
}

#[test]
fn path_link_archive_and_output_attacks_are_rejected() {
    assert!(validate_workspace_entry("/workspace/a", EntryKind::File { hard_links: 1 }).is_ok());
    assert_eq!(
        validate_workspace_entry(
            "/workspace/../etc/passwd",
            EntryKind::File { hard_links: 1 }
        )
        .unwrap_err(),
        SecurityError::PathDenied
    );
    assert_eq!(
        validate_workspace_entry("/workspace/link", EntryKind::Symlink).unwrap_err(),
        SecurityError::PathDenied
    );
    assert_eq!(
        validate_workspace_entry("/workspace/hard", EntryKind::File { hard_links: 2 }).unwrap_err(),
        SecurityError::PathDenied
    );
    assert_eq!(
        validate_archive(1, 1000, 1).unwrap_err(),
        SecurityError::ArchiveDenied
    );
    assert_eq!(
        validate_untrusted_output(b"ok\x1b[2J", 100).unwrap_err(),
        SecurityError::OutputDenied
    );
}

#[test]
fn child_environment_contains_no_secret_values() {
    let env = BTreeMap::from([
        ("PATH".into(), "/runtime/bin".into()),
        ("AWS_SECRET_ACCESS_KEY".into(), "secret".into()),
        ("SESSION_TOKEN".into(), "token".into()),
        ("PASSWORD".into(), "password".into()),
    ]);
    assert_eq!(
        sanitize_child_env(&env),
        BTreeMap::from([("PATH".into(), "/runtime/bin".into())])
    );
}

#[test]
fn audit_chain_covers_every_authority_hop_and_is_tamper_evident() {
    let mut gate = ProductionSecurityGate::new(profile()).unwrap();
    let first = gate.admit(request()).unwrap();
    let mut second_request = request();
    second_request.draft_ref = "draft:2".into();
    let second = gate.admit(second_request).unwrap();
    assert_eq!(second.previous_hash, first.receipt_hash);
    assert_eq!(first.actor_ref, "actor:user");
    assert_eq!(first.caller_ref, "plugin:dsh");
    assert_eq!(first.target_ref, "worker:office");
    assert_eq!(first.expected_revision, "revision:1");
}
