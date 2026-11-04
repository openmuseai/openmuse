use openmuse_workspace_sandbox::{
    CreateLeaseRequest, ExecutionPlacement, FileEffectMode, InMemorySandboxRuntime, LeaseState,
    MountDescriptor, PolicyCeiling, QuiesceStrategy, ReleaseDisposition, SERVICE_ID, SERVICE_MAJOR,
    SandboxErrorCode, WorkspaceSandboxService,
};
use std::sync::Arc;

fn policy(strict_host_isolation: bool) -> PolicyCeiling {
    PolicyCeiling {
        file_effect: FileEffectMode::WorkspaceWrite,
        strict_host_isolation,
        network_egress: false,
        process_spawn: true,
        process_background: true,
        max_processes: 16,
        max_memory_bytes: 512 * 1024 * 1024,
        max_runtime_ms: 60_000,
    }
}

fn request(placement: ExecutionPlacement, strict_host_isolation: bool) -> CreateLeaseRequest {
    CreateLeaseRequest {
        actor_ref: "actor:user-1".into(),
        caller_plugin_ref: "plugin:agent".into(),
        workspace_ref: "workspace:one".into(),
        checkout_handle_ref: "opaque-checkout:one".into(),
        base_revision: "revision:base".into(),
        placement,
        policy_ceiling: policy(strict_host_isolation),
        registry_digest: "registry:sha256:one".into(),
        now_ms: 1_000,
        ttl_ms: 10_000,
    }
}

#[test]
fn contract_identity_and_mounts_are_fixed() {
    assert_eq!(SERVICE_ID, "workspace.sandbox");
    assert_eq!(SERVICE_MAJOR, 1);
    assert_eq!(
        MountDescriptor::default(),
        MountDescriptor {
            workspace: "/workspace".into(),
            runtime: "/runtime".into(),
            agent_home: "/home/agent".into(),
            temp: "/tmp".into(),
            control: "/run/openmuse".into(),
        }
    );
}

#[test]
fn native_mode_fails_closed_when_strict_host_isolation_is_required() {
    let runtime = Arc::new(InMemorySandboxRuntime::default());
    let mut service = WorkspaceSandboxService::new(runtime);
    let error = service
        .create_lease(request(ExecutionPlacement::LocalNative, true))
        .unwrap_err();
    assert_eq!(error.code, SandboxErrorCode::PolicyDenied);
}

#[test]
fn public_lease_contains_only_opaque_runtime_and_logical_mounts() {
    let runtime = Arc::new(InMemorySandboxRuntime::default());
    let mut service = WorkspaceSandboxService::new(runtime);
    let lease = service
        .create_lease(request(ExecutionPlacement::LocalIsolated, true))
        .unwrap();
    let encoded = serde_json::to_string(&lease).unwrap();
    assert!(encoded.contains("/workspace"));
    assert!(!encoded.contains("opaque-checkout:one"));
    assert!(!encoded.contains("/Users/"));
    assert!(!encoded.contains("credential"));
}

#[test]
fn attachments_are_audience_generation_and_expiry_bound() {
    let runtime = Arc::new(InMemorySandboxRuntime::default());
    let mut service = WorkspaceSandboxService::new(runtime);
    let lease = service
        .create_lease(request(ExecutionPlacement::LocalIsolated, true))
        .unwrap();
    let attachment = service
        .attach_consumer(&lease.lease_ref, "dsh", 1, 1, 1_100, 500)
        .unwrap();
    assert_eq!(
        service
            .resolve_attachment(&attachment.attachment_ref, "worker", 1, 1_200)
            .unwrap_err()
            .code,
        SandboxErrorCode::PolicyDenied
    );
    assert_eq!(
        service
            .resolve_attachment(&attachment.attachment_ref, "dsh", 2, 1_200)
            .unwrap_err()
            .code,
        SandboxErrorCode::StaleGeneration
    );
    assert_eq!(
        service
            .resolve_attachment(&attachment.attachment_ref, "dsh", 1, 1_600)
            .unwrap_err()
            .code,
        SandboxErrorCode::LeaseExpired
    );
}

#[test]
fn registry_refresh_is_explicit_and_invalidates_old_attachments() {
    let runtime = Arc::new(InMemorySandboxRuntime::default());
    let mut service = WorkspaceSandboxService::new(runtime);
    let lease = service
        .create_lease(request(ExecutionPlacement::LocalIsolated, true))
        .unwrap();
    let attachment = service
        .attach_consumer(&lease.lease_ref, "dsh", 1, 1, 1_100, 500)
        .unwrap();
    let refreshed = service
        .refresh_capabilities(
            &lease.lease_ref,
            "registry:sha256:one",
            "registry:sha256:two",
            1_200,
        )
        .unwrap();
    assert_eq!(refreshed.generation, 2);
    assert_eq!(
        service
            .resolve_attachment(&attachment.attachment_ref, "dsh", 1, 1_300)
            .unwrap_err()
            .code,
        SandboxErrorCode::StaleGeneration
    );
}

#[test]
fn quiescence_rejects_or_terminates_the_full_process_range() {
    let runtime = Arc::new(InMemorySandboxRuntime::default());
    let mut service = WorkspaceSandboxService::new(runtime.clone());
    let lease = service
        .create_lease(request(ExecutionPlacement::LocalIsolated, true))
        .unwrap();
    runtime.set_active_processes(&lease.runtime_ref, 3);
    assert_eq!(
        service
            .quiesce(
                &lease.lease_ref,
                1,
                1_100,
                2_000,
                QuiesceStrategy::RejectIfActive
            )
            .unwrap_err()
            .code,
        SandboxErrorCode::NotQuiescent
    );
    assert_eq!(
        service.status(&lease.lease_ref, 1, 1_200).unwrap().state,
        LeaseState::Ready
    );

    let receipt = service
        .quiesce(
            &lease.lease_ref,
            1,
            1_300,
            2_000,
            QuiesceStrategy::Terminate,
        )
        .unwrap();
    assert_eq!(receipt.lease_ref, lease.lease_ref);
    assert_eq!(receipt.terminated_processes, 3);
    assert_eq!(
        service
            .status(&lease.lease_ref, 1, 1_400)
            .unwrap()
            .active_processes,
        0
    );
    assert_eq!(
        service
            .prepare_draft(&lease.lease_ref, 1, 1_500)
            .unwrap()
            .changed_paths,
        0
    );
}

#[test]
fn prepare_draft_requires_quiescence() {
    let runtime = Arc::new(InMemorySandboxRuntime::default());
    let mut service = WorkspaceSandboxService::new(runtime);
    let lease = service
        .create_lease(request(ExecutionPlacement::LocalIsolated, true))
        .unwrap();
    assert_eq!(
        service
            .prepare_draft(&lease.lease_ref, 1, 1_100)
            .unwrap_err()
            .code,
        SandboxErrorCode::NotQuiescent
    );
}

#[test]
fn expiry_and_revoke_terminate_processes_and_return_idempotent_receipts() {
    let runtime = Arc::new(InMemorySandboxRuntime::default());
    let mut service = WorkspaceSandboxService::new(runtime.clone());
    let mut short = request(ExecutionPlacement::LocalIsolated, true);
    short.ttl_ms = 100;
    let lease = service.create_lease(short).unwrap();
    runtime.set_active_processes(&lease.runtime_ref, 4);
    assert_eq!(
        service.status(&lease.lease_ref, 1, 1_100).unwrap_err().code,
        SandboxErrorCode::LeaseExpired
    );
    let first = service
        .release(&lease.lease_ref, ReleaseDisposition::Discard)
        .unwrap();
    let second = service
        .release(&lease.lease_ref, ReleaseDisposition::PreserveDraft)
        .unwrap();
    assert_eq!(first, second);
    assert_eq!(first.disposition, ReleaseDisposition::Expired);
    assert_eq!(first.processes_terminated, 4);
    assert!(first.volume_scrubbed);
}

#[test]
fn plugin_shutdown_contains_cleanup_failure_without_panicking() {
    let runtime = Arc::new(InMemorySandboxRuntime::default());
    let mut service = WorkspaceSandboxService::new(runtime.clone());
    let lease = service
        .create_lease(request(ExecutionPlacement::LocalIsolated, true))
        .unwrap();
    runtime.fail_release_for(&lease.runtime_ref);
    let receipts = service.shutdown_plugin();
    assert_eq!(receipts.len(), 1);
    assert!(!receipts[0].succeeded);
    assert_eq!(
        receipts[0].failure_code,
        Some(SandboxErrorCode::ProviderFailed)
    );
    assert_eq!(
        service.status(&lease.lease_ref, 1, 1_100).unwrap_err().code,
        SandboxErrorCode::ProviderFailed
    );
}
