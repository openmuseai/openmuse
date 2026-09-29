use openmuse_cli_registry::{CliRegistrySnapshot, ResolvedCommand};
use openmuse_plugin_protocol::{
    AgentCliContribution, Permission, TargetArch, TargetLibc, TargetOs, TargetTriple,
};
use openmuse_worker_supervisor::*;
use serde_json::json;
use std::collections::BTreeSet;
use std::sync::Arc;

fn permission() -> Permission {
    Permission::new("workspace.read")
}

fn snapshot() -> CliRegistrySnapshot {
    CliRegistrySnapshot {
        generation: 1,
        digest: "sha256:registry".into(),
        target: TargetTriple {
            os: TargetOs::Linux,
            arch: TargetArch::X86_64,
            libc: TargetLibc::Gnu,
        },
        commands: vec![ResolvedCommand {
            identity: "office/docs/inspect".into(),
            plugin_id: "office".into(),
            plugin_version: "1".into(),
            command: AgentCliContribution {
                group: "office".into(),
                namespace: "docs".into(),
                command: "inspect".into(),
                schema: json!({"type":"object"}),
                required_permissions: BTreeSet::from([permission()]),
                effects: BTreeSet::from(["read".into()]),
            },
            worker_artifact_id: "docx-worker".into(),
            worker_digest: "digest:worker".into(),
            abi: "openmuse-worker@1".into(),
        }],
        unavailable: vec![],
    }
}

fn request(placement: Placement) -> DispatchRequest {
    DispatchRequest {
        command_identity: "office/docs/inspect".into(),
        argv: vec!["inspect".into(), "/workspace/report.docx".into()],
        cwd: "/workspace".into(),
        now_ms: 10,
        deadline_at_ms: 100,
        lease_permissions: BTreeSet::from([permission()]),
        placement,
    }
}

fn setup() -> (WorkerSupervisor, Arc<DocxInspectWorker>) {
    let worker = Arc::new(DocxInspectWorker::default());
    worker.insert("/workspace/report.docx", b"PK fake docx".to_vec());
    let mut supervisor = WorkerSupervisor::new(snapshot());
    supervisor.register_worker("digest:worker", worker.clone());
    (supervisor, worker)
}

#[test]
fn agent_can_only_address_registry_command_not_worker_bundle() {
    let (supervisor, _) = setup();
    let mut direct = request(Placement::Local);
    direct.command_identity = "digest:worker".into();
    assert_eq!(
        supervisor.dispatch(direct).unwrap_err(),
        DispatchError::Unavailable
    );
}

#[test]
fn argv_is_passed_as_a_vector_without_shell_interpolation() {
    let (supervisor, worker) = setup();
    let mut injected = request(Placement::Local);
    injected.argv[1] = "/workspace/report.docx; touch /workspace/pwned".into();
    assert_eq!(
        supervisor.dispatch(injected).unwrap_err(),
        DispatchError::WorkerFailed
    );
    assert_eq!(
        worker.contexts()[0].argv[1],
        "/workspace/report.docx; touch /workspace/pwned"
    );
}

#[test]
fn timeout_terminates_complete_worker_process_range() {
    let (supervisor, worker) = setup();
    worker.time_out_next();
    assert_eq!(
        supervisor.dispatch(request(Placement::Cloud)).unwrap_err(),
        DispatchError::DeadlineExceeded
    );
    assert_eq!(worker.terminated(), 1);
}

#[test]
fn shared_core_produces_same_output_for_local_and_cloud() {
    let (supervisor, _) = setup();
    assert_eq!(
        supervisor.dispatch(request(Placement::Local)).unwrap(),
        supervisor.dispatch(request(Placement::Cloud)).unwrap()
    );
}

#[test]
fn ui_permissions_do_not_leak_and_missing_lease_grant_is_denied() {
    let (supervisor, worker) = setup();
    let output = supervisor.dispatch(request(Placement::Local)).unwrap();
    assert_eq!(output.schema, "openmuse.office.inspect@1");
    assert_eq!(
        worker.contexts()[0].permissions,
        BTreeSet::from([permission()])
    );
    let mut denied = request(Placement::Local);
    denied.lease_permissions.clear();
    assert_eq!(
        supervisor.dispatch(denied).unwrap_err(),
        DispatchError::Denied
    );
}
