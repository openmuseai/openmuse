//! Stable `office/openmuse` dispatcher and isolated worker supervisor.

use openmuse_cli_registry::{CliRegistrySnapshot, ResolvedCommand};
use openmuse_plugin_protocol::Permission;
use serde::{Deserialize, Serialize};
use serde_json::{Value, json};
use std::collections::{BTreeMap, BTreeSet};
use std::sync::{Arc, Mutex};

pub const OFFICE_ENTRYPOINT: &str = "/runtime/bin/office";
pub const OPENMUSE_ENTRYPOINT: &str = "/runtime/bin/openmuse";

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Placement {
    Local,
    Cloud,
}

#[derive(Debug, Clone)]
pub struct DispatchRequest {
    pub command_identity: String,
    pub argv: Vec<String>,
    pub cwd: String,
    pub now_ms: u64,
    pub deadline_at_ms: u64,
    pub lease_permissions: BTreeSet<Permission>,
    pub placement: Placement,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct WorkerOutput {
    pub schema: String,
    pub value: Value,
    pub result_handle: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct WorkerContext {
    pub worker_identity: String,
    pub artifact_digest: String,
    pub argv: Vec<String>,
    pub cwd: String,
    pub permissions: BTreeSet<Permission>,
    pub deadline_at_ms: u64,
    pub placement: Placement,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, thiserror::Error)]
pub enum DispatchError {
    #[error("command unavailable")]
    Unavailable,
    #[error("request denied")]
    Denied,
    #[error("invalid arguments")]
    InvalidArguments,
    #[error("deadline exceeded")]
    DeadlineExceeded,
    #[error("worker failed")]
    WorkerFailed,
}

pub type Result<T> = std::result::Result<T, DispatchError>;

pub trait WorkerProcessPort: Send + Sync {
    fn execute(&self, context: &WorkerContext) -> Result<WorkerOutput>;
    fn terminate_process_range(&self, worker_identity: &str) -> u32;
}

pub struct WorkerSupervisor {
    snapshot: CliRegistrySnapshot,
    workers: BTreeMap<String, Arc<dyn WorkerProcessPort>>,
    next_identity: Mutex<u64>,
}

impl WorkerSupervisor {
    pub fn new(snapshot: CliRegistrySnapshot) -> Self {
        Self {
            snapshot,
            workers: BTreeMap::new(),
            next_identity: Mutex::new(1),
        }
    }

    pub fn register_worker(&mut self, artifact_digest: &str, worker: Arc<dyn WorkerProcessPort>) {
        self.workers.insert(artifact_digest.into(), worker);
    }

    pub fn dispatch(&self, request: DispatchRequest) -> Result<WorkerOutput> {
        let command = self
            .snapshot
            .commands
            .iter()
            .find(|command| command.identity == request.command_identity)
            .ok_or(DispatchError::Unavailable)?;
        validate_request(command, &request)?;
        let worker = self
            .workers
            .get(&command.worker_digest)
            .ok_or(DispatchError::Unavailable)?;
        let mut next = self.next_identity.lock().unwrap();
        let worker_identity = format!("worker.{}", *next);
        *next += 1;
        drop(next);
        let context = WorkerContext {
            worker_identity: worker_identity.clone(),
            artifact_digest: command.worker_digest.clone(),
            argv: request.argv,
            cwd: request.cwd,
            permissions: command.command.required_permissions.clone(),
            deadline_at_ms: request.deadline_at_ms,
            placement: request.placement,
        };
        match worker.execute(&context) {
            Err(DispatchError::DeadlineExceeded) => {
                worker.terminate_process_range(&worker_identity);
                Err(DispatchError::DeadlineExceeded)
            }
            result => result,
        }
    }

    pub fn snapshot_digest(&self) -> &str {
        &self.snapshot.digest
    }
}

fn validate_request(command: &ResolvedCommand, request: &DispatchRequest) -> Result<()> {
    if request.deadline_at_ms <= request.now_ms
        || request.cwd != "/workspace"
        || request.argv.is_empty()
        || request.argv.iter().any(|arg| arg.contains('\0'))
    {
        return Err(DispatchError::InvalidArguments);
    }
    if !command
        .command
        .required_permissions
        .is_subset(&request.lease_permissions)
    {
        return Err(DispatchError::Denied);
    }
    if command.command.schema.get("type").and_then(Value::as_str) != Some("object") {
        return Err(DispatchError::InvalidArguments);
    }
    Ok(())
}

#[derive(Default)]
pub struct DocxInspectWorker {
    files: Mutex<BTreeMap<String, Vec<u8>>>,
    timed_out: Mutex<bool>,
    terminated: Mutex<u32>,
    contexts: Mutex<Vec<WorkerContext>>,
}

impl DocxInspectWorker {
    pub fn insert(&self, workspace_path: &str, bytes: Vec<u8>) {
        self.files
            .lock()
            .unwrap()
            .insert(workspace_path.into(), bytes);
    }

    pub fn time_out_next(&self) {
        *self.timed_out.lock().unwrap() = true;
    }

    pub fn terminated(&self) -> u32 {
        *self.terminated.lock().unwrap()
    }

    pub fn contexts(&self) -> Vec<WorkerContext> {
        self.contexts.lock().unwrap().clone()
    }
}

impl WorkerProcessPort for DocxInspectWorker {
    fn execute(&self, context: &WorkerContext) -> Result<WorkerOutput> {
        self.contexts.lock().unwrap().push(context.clone());
        if std::mem::take(&mut *self.timed_out.lock().unwrap()) {
            return Err(DispatchError::DeadlineExceeded);
        }
        if context.argv.len() != 2 || context.argv[0] != "inspect" {
            return Err(DispatchError::InvalidArguments);
        }
        let path = &context.argv[1];
        if !path.starts_with("/workspace/") || path.contains("..") {
            return Err(DispatchError::Denied);
        }
        let files = self.files.lock().unwrap();
        let bytes = files.get(path).ok_or(DispatchError::WorkerFailed)?;
        if !bytes.starts_with(b"PK") {
            return Err(DispatchError::WorkerFailed);
        }
        Ok(WorkerOutput {
            schema: "openmuse.office.inspect@1".into(),
            value: json!({"format":"docx","size":bytes.len(),"validContainer":true}),
            result_handle: None,
        })
    }

    fn terminate_process_range(&self, _worker_identity: &str) -> u32 {
        *self.terminated.lock().unwrap() += 1;
        1
    }
}
