//! `workspace.sandbox@1` control-plane service and local-runtime contract.
//!
//! Public descriptors contain logical mount paths and opaque handles only.
//! Host checkout paths, processes and substrate credentials remain behind the
//! runtime port.

use serde::{Deserialize, Serialize};
use std::collections::{BTreeSet, HashMap};
use std::sync::{Arc, Mutex};

pub const SERVICE_ID: &str = "workspace.sandbox";
pub const SERVICE_MAJOR: u16 = 1;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ExecutionPlacement {
    LocalNative,
    LocalIsolated,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum FileEffectMode {
    ReadOnly,
    WorkspaceWrite,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct PolicyCeiling {
    pub file_effect: FileEffectMode,
    pub strict_host_isolation: bool,
    pub network_egress: bool,
    pub process_spawn: bool,
    pub process_background: bool,
    pub max_processes: u32,
    pub max_memory_bytes: u64,
    pub max_runtime_ms: u64,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct MountDescriptor {
    pub workspace: String,
    pub runtime: String,
    pub agent_home: String,
    pub temp: String,
    pub control: String,
}

impl Default for MountDescriptor {
    fn default() -> Self {
        Self {
            workspace: "/workspace".to_owned(),
            runtime: "/runtime".to_owned(),
            agent_home: "/home/agent".to_owned(),
            temp: "/tmp".to_owned(),
            control: "/run/openmuse".to_owned(),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum LeaseState {
    Ready,
    Quiescing,
    Quiescent,
    Degraded,
    Released,
    Expired,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct WorkspaceSandboxLease {
    pub lease_ref: String,
    pub runtime_ref: String,
    pub actor_ref: String,
    pub caller_plugin_ref: String,
    pub workspace_ref: String,
    pub base_revision: String,
    pub placement: ExecutionPlacement,
    pub policy_ceiling: PolicyCeiling,
    pub registry_digest: String,
    pub mounts: MountDescriptor,
    pub state: LeaseState,
    pub generation: u64,
    pub expires_at_ms: u64,
    pub event_sequence: u64,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CreateLeaseRequest {
    pub actor_ref: String,
    pub caller_plugin_ref: String,
    pub workspace_ref: String,
    pub checkout_handle_ref: String,
    pub base_revision: String,
    pub placement: ExecutionPlacement,
    pub policy_ceiling: PolicyCeiling,
    pub registry_digest: String,
    pub now_ms: u64,
    pub ttl_ms: u64,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct ConsumerAttachment {
    pub attachment_ref: String,
    pub lease_ref: String,
    pub audience: String,
    pub protocol_major: u16,
    pub generation: u64,
    pub expires_at_ms: u64,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct SandboxStatus {
    pub lease_ref: String,
    pub state: LeaseState,
    pub active_processes: u32,
    pub draft_dirty: bool,
    pub generation: u64,
    pub event_sequence: u64,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum QuiesceStrategy {
    RejectIfActive,
    Terminate,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct QuiesceReceipt {
    pub receipt_ref: String,
    pub lease_ref: String,
    pub terminated_processes: u32,
    pub flushed: bool,
    pub generation: u64,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct DraftReceipt {
    pub draft_ref: String,
    pub lease_ref: String,
    pub workspace_ref: String,
    pub base_revision: String,
    pub manifest_digest: String,
    pub changed_paths: u64,
    pub generation: u64,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ReleaseDisposition {
    PreserveDraft,
    Discard,
    AlreadyCheckpointed,
    Expired,
    PluginShutdown,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct CleanupReceipt {
    pub receipt_ref: String,
    pub lease_ref: String,
    pub disposition: ReleaseDisposition,
    pub processes_terminated: u32,
    pub volume_scrubbed: bool,
    pub succeeded: bool,
    pub failure_code: Option<SandboxErrorCode>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RuntimeAllocationRequest {
    pub lease_ref: String,
    pub checkout_handle_ref: String,
    pub placement: ExecutionPlacement,
    pub policy_ceiling: PolicyCeiling,
    pub mounts: MountDescriptor,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RuntimeAllocation {
    pub runtime_ref: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RuntimeStatus {
    pub active_processes: u32,
    pub draft_dirty: bool,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RuntimeDraft {
    pub manifest_digest: String,
    pub changed_paths: u64,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RuntimeCleanup {
    pub processes_terminated: u32,
    pub volume_scrubbed: bool,
}

pub trait SandboxRuntimePort: Send + Sync {
    fn allocate(&self, request: &RuntimeAllocationRequest) -> SandboxResult<RuntimeAllocation>;
    fn status(&self, runtime_ref: &str) -> SandboxResult<RuntimeStatus>;
    fn attach(&self, runtime_ref: &str, attachment_ref: &str) -> SandboxResult<()>;
    fn quiesce(
        &self,
        runtime_ref: &str,
        strategy: QuiesceStrategy,
        deadline_at_ms: u64,
    ) -> SandboxResult<QuiesceReceipt>;
    fn prepare_draft(&self, runtime_ref: &str) -> SandboxResult<RuntimeDraft>;
    fn release(
        &self,
        runtime_ref: &str,
        disposition: ReleaseDisposition,
    ) -> SandboxResult<RuntimeCleanup>;
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum SandboxErrorCode {
    UnsupportedTarget,
    PolicyDenied,
    SandboxUnavailable,
    ArtifactUnavailable,
    LeaseExpired,
    StaleGeneration,
    QuotaExceeded,
    NotQuiescent,
    DraftConflict,
    ProviderFailed,
    NotFound,
    Conflict,
}

#[derive(Debug, Clone, PartialEq, Eq, thiserror::Error)]
#[error("{code:?}: {message}")]
pub struct SandboxError {
    pub code: SandboxErrorCode,
    pub message: String,
    pub retryable: bool,
}

impl SandboxError {
    pub fn new(code: SandboxErrorCode, message: impl Into<String>, retryable: bool) -> Self {
        Self {
            code,
            message: message.into(),
            retryable,
        }
    }
}

pub type SandboxResult<T> = Result<T, SandboxError>;

pub struct WorkspaceSandboxService {
    runtime: Arc<dyn SandboxRuntimePort>,
    leases: HashMap<String, WorkspaceSandboxLease>,
    attachments: HashMap<String, ConsumerAttachment>,
    released: HashMap<String, CleanupReceipt>,
    next_lease: u64,
    next_attachment: u64,
    next_receipt: u64,
}

impl WorkspaceSandboxService {
    pub fn new(runtime: Arc<dyn SandboxRuntimePort>) -> Self {
        Self {
            runtime,
            leases: HashMap::new(),
            attachments: HashMap::new(),
            released: HashMap::new(),
            next_lease: 1,
            next_attachment: 1,
            next_receipt: 1,
        }
    }

    pub fn create_lease(
        &mut self,
        request: CreateLeaseRequest,
    ) -> SandboxResult<WorkspaceSandboxLease> {
        validate_create(&request)?;
        if request.placement == ExecutionPlacement::LocalNative
            && request.policy_ceiling.strict_host_isolation
        {
            return Err(SandboxError::new(
                SandboxErrorCode::PolicyDenied,
                "local-native cannot guarantee host read isolation; use local-isolated",
                false,
            ));
        }
        let lease_ref = format!("sandbox-lease.{}", self.next_lease);
        self.next_lease += 1;
        let mounts = MountDescriptor::default();
        let allocation = self.runtime.allocate(&RuntimeAllocationRequest {
            lease_ref: lease_ref.clone(),
            checkout_handle_ref: request.checkout_handle_ref,
            placement: request.placement,
            policy_ceiling: request.policy_ceiling.clone(),
            mounts: mounts.clone(),
        })?;
        let lease = WorkspaceSandboxLease {
            lease_ref: lease_ref.clone(),
            runtime_ref: allocation.runtime_ref,
            actor_ref: request.actor_ref,
            caller_plugin_ref: request.caller_plugin_ref,
            workspace_ref: request.workspace_ref,
            base_revision: request.base_revision,
            placement: request.placement,
            policy_ceiling: request.policy_ceiling,
            registry_digest: request.registry_digest,
            mounts,
            state: LeaseState::Ready,
            generation: 1,
            expires_at_ms: request.now_ms.saturating_add(request.ttl_ms),
            event_sequence: 1,
        };
        self.leases.insert(lease_ref, lease.clone());
        Ok(lease)
    }

    pub fn attach_consumer(
        &mut self,
        lease_ref: &str,
        audience: &str,
        protocol_major: u16,
        expected_generation: u64,
        now_ms: u64,
        ttl_ms: u64,
    ) -> SandboxResult<ConsumerAttachment> {
        let lease = self
            .live_lease(lease_ref, expected_generation, now_ms)?
            .clone();
        if audience.is_empty() || ttl_ms == 0 || protocol_major != SERVICE_MAJOR {
            return Err(SandboxError::new(
                SandboxErrorCode::PolicyDenied,
                "consumer attachment request is invalid",
                false,
            ));
        }
        let attachment_ref = format!("sandbox-attachment.{}", self.next_attachment);
        self.next_attachment += 1;
        let attachment = ConsumerAttachment {
            attachment_ref: attachment_ref.clone(),
            lease_ref: lease_ref.to_owned(),
            audience: audience.to_owned(),
            protocol_major,
            generation: lease.generation,
            expires_at_ms: now_ms.saturating_add(ttl_ms).min(lease.expires_at_ms),
        };
        self.runtime
            .attach(&lease.runtime_ref, &attachment.attachment_ref)?;
        self.attachments.insert(attachment_ref, attachment.clone());
        Ok(attachment)
    }

    pub fn resolve_attachment(
        &self,
        attachment_ref: &str,
        audience: &str,
        generation: u64,
        now_ms: u64,
    ) -> SandboxResult<ConsumerAttachment> {
        let attachment = self.attachments.get(attachment_ref).ok_or_else(not_found)?;
        if attachment.audience != audience {
            return Err(SandboxError::new(
                SandboxErrorCode::PolicyDenied,
                "attachment audience mismatch",
                false,
            ));
        }
        if attachment.generation != generation {
            return Err(stale_generation());
        }
        if now_ms >= attachment.expires_at_ms {
            return Err(expired());
        }
        let lease = self
            .leases
            .get(&attachment.lease_ref)
            .ok_or_else(not_found)?;
        if lease.generation != generation || lease.state == LeaseState::Released {
            return Err(stale_generation());
        }
        Ok(attachment.clone())
    }

    pub fn status(
        &mut self,
        lease_ref: &str,
        expected_generation: u64,
        now_ms: u64,
    ) -> SandboxResult<SandboxStatus> {
        if self.leases.get(lease_ref).is_some_and(|lease| {
            now_ms >= lease.expires_at_ms && lease.state != LeaseState::Released
        }) {
            self.expire(lease_ref)?;
            return Err(expired());
        }
        let lease = self.live_lease(lease_ref, expected_generation, now_ms)?;
        let runtime = self.runtime.status(&lease.runtime_ref)?;
        Ok(SandboxStatus {
            lease_ref: lease_ref.to_owned(),
            state: lease.state,
            active_processes: runtime.active_processes,
            draft_dirty: runtime.draft_dirty,
            generation: lease.generation,
            event_sequence: lease.event_sequence,
        })
    }

    pub fn quiesce(
        &mut self,
        lease_ref: &str,
        expected_generation: u64,
        now_ms: u64,
        deadline_at_ms: u64,
        strategy: QuiesceStrategy,
    ) -> SandboxResult<QuiesceReceipt> {
        let lease = self
            .live_lease(lease_ref, expected_generation, now_ms)?
            .clone();
        if deadline_at_ms <= now_ms {
            return Err(SandboxError::new(
                SandboxErrorCode::NotQuiescent,
                "quiesce deadline already elapsed",
                false,
            ));
        }
        self.set_state(lease_ref, LeaseState::Quiescing);
        match self
            .runtime
            .quiesce(&lease.runtime_ref, strategy, deadline_at_ms)
        {
            Ok(mut receipt) => {
                self.set_state(lease_ref, LeaseState::Quiescent);
                receipt.lease_ref = lease_ref.to_owned();
                receipt.generation = expected_generation;
                Ok(receipt)
            }
            Err(error) => {
                self.set_state(lease_ref, LeaseState::Ready);
                Err(error)
            }
        }
    }

    pub fn prepare_draft(
        &mut self,
        lease_ref: &str,
        expected_generation: u64,
        now_ms: u64,
    ) -> SandboxResult<DraftReceipt> {
        let lease = self
            .live_lease(lease_ref, expected_generation, now_ms)?
            .clone();
        if lease.state != LeaseState::Quiescent {
            return Err(SandboxError::new(
                SandboxErrorCode::NotQuiescent,
                "draft preparation requires a quiescent lease",
                false,
            ));
        }
        let draft = self.runtime.prepare_draft(&lease.runtime_ref)?;
        let receipt = DraftReceipt {
            draft_ref: format!("sandbox-draft.{}", self.next_receipt),
            lease_ref: lease_ref.to_owned(),
            workspace_ref: lease.workspace_ref,
            base_revision: lease.base_revision,
            manifest_digest: draft.manifest_digest,
            changed_paths: draft.changed_paths,
            generation: lease.generation,
        };
        self.next_receipt += 1;
        Ok(receipt)
    }

    pub fn refresh_capabilities(
        &mut self,
        lease_ref: &str,
        expected_registry_digest: &str,
        new_registry_digest: &str,
        now_ms: u64,
    ) -> SandboxResult<WorkspaceSandboxLease> {
        let current_generation = self.leases.get(lease_ref).ok_or_else(not_found)?.generation;
        let lease = self.live_lease(lease_ref, current_generation, now_ms)?;
        if lease.registry_digest != expected_registry_digest || new_registry_digest.is_empty() {
            return Err(SandboxError::new(
                SandboxErrorCode::Conflict,
                "registry digest changed or replacement is invalid",
                false,
            ));
        }
        let lease = self.leases.get_mut(lease_ref).ok_or_else(not_found)?;
        lease.registry_digest = new_registry_digest.to_owned();
        lease.generation += 1;
        lease.event_sequence += 1;
        Ok(lease.clone())
    }

    pub fn release(
        &mut self,
        lease_ref: &str,
        disposition: ReleaseDisposition,
    ) -> SandboxResult<CleanupReceipt> {
        if let Some(receipt) = self.released.get(lease_ref) {
            return Ok(receipt.clone());
        }
        let lease = self.leases.get(lease_ref).ok_or_else(not_found)?.clone();
        let cleanup = self.runtime.release(&lease.runtime_ref, disposition)?;
        let receipt = self.cleanup_receipt(lease_ref, disposition, cleanup, true, None);
        if let Some(lease) = self.leases.get_mut(lease_ref) {
            lease.state = if disposition == ReleaseDisposition::Expired {
                LeaseState::Expired
            } else {
                LeaseState::Released
            };
            lease.generation += 1;
            lease.event_sequence += 1;
        }
        self.attachments
            .retain(|_, attachment| attachment.lease_ref != lease_ref);
        self.released.insert(lease_ref.to_owned(), receipt.clone());
        Ok(receipt)
    }

    pub fn shutdown_plugin(&mut self) -> Vec<CleanupReceipt> {
        let leases: Vec<String> = self
            .leases
            .iter()
            .filter(|(_, lease)| !matches!(lease.state, LeaseState::Released | LeaseState::Expired))
            .map(|(lease_ref, _)| lease_ref.clone())
            .collect();
        leases
            .into_iter()
            .map(
                |lease_ref| match self.release(&lease_ref, ReleaseDisposition::PluginShutdown) {
                    Ok(receipt) => receipt,
                    Err(error) => {
                        self.set_state(&lease_ref, LeaseState::Degraded);
                        self.cleanup_receipt(
                            &lease_ref,
                            ReleaseDisposition::PluginShutdown,
                            RuntimeCleanup {
                                processes_terminated: 0,
                                volume_scrubbed: false,
                            },
                            false,
                            Some(error.code),
                        )
                    }
                },
            )
            .collect()
    }

    fn expire(&mut self, lease_ref: &str) -> SandboxResult<()> {
        self.release(lease_ref, ReleaseDisposition::Expired)
            .map(|_| ())
    }

    fn live_lease(
        &self,
        lease_ref: &str,
        expected_generation: u64,
        now_ms: u64,
    ) -> SandboxResult<&WorkspaceSandboxLease> {
        let lease = self.leases.get(lease_ref).ok_or_else(not_found)?;
        if lease.generation != expected_generation {
            return Err(stale_generation());
        }
        if now_ms >= lease.expires_at_ms || lease.state == LeaseState::Expired {
            return Err(expired());
        }
        if matches!(lease.state, LeaseState::Released | LeaseState::Degraded) {
            return Err(SandboxError::new(
                SandboxErrorCode::ProviderFailed,
                "lease is not available",
                false,
            ));
        }
        Ok(lease)
    }

    fn set_state(&mut self, lease_ref: &str, state: LeaseState) {
        if let Some(lease) = self.leases.get_mut(lease_ref) {
            lease.state = state;
            lease.event_sequence += 1;
        }
    }

    fn cleanup_receipt(
        &mut self,
        lease_ref: &str,
        disposition: ReleaseDisposition,
        cleanup: RuntimeCleanup,
        succeeded: bool,
        failure_code: Option<SandboxErrorCode>,
    ) -> CleanupReceipt {
        let receipt = CleanupReceipt {
            receipt_ref: format!("sandbox-cleanup.{}", self.next_receipt),
            lease_ref: lease_ref.to_owned(),
            disposition,
            processes_terminated: cleanup.processes_terminated,
            volume_scrubbed: cleanup.volume_scrubbed,
            succeeded,
            failure_code,
        };
        self.next_receipt += 1;
        receipt
    }
}

fn validate_create(request: &CreateLeaseRequest) -> SandboxResult<()> {
    if [
        request.actor_ref.as_str(),
        request.caller_plugin_ref.as_str(),
        request.workspace_ref.as_str(),
        request.checkout_handle_ref.as_str(),
        request.base_revision.as_str(),
        request.registry_digest.as_str(),
    ]
    .iter()
    .any(|value| value.is_empty())
        || request.ttl_ms == 0
        || request.policy_ceiling.max_processes == 0
        || request.policy_ceiling.max_memory_bytes == 0
        || request.policy_ceiling.max_runtime_ms == 0
    {
        return Err(SandboxError::new(
            SandboxErrorCode::PolicyDenied,
            "lease request is incomplete or has invalid quotas",
            false,
        ));
    }
    Ok(())
}

fn stale_generation() -> SandboxError {
    SandboxError::new(
        SandboxErrorCode::StaleGeneration,
        "lease generation is stale",
        false,
    )
}

fn expired() -> SandboxError {
    SandboxError::new(SandboxErrorCode::LeaseExpired, "lease expired", false)
}

fn not_found() -> SandboxError {
    SandboxError::new(
        SandboxErrorCode::NotFound,
        "sandbox object not found",
        false,
    )
}

#[derive(Default)]
pub struct InMemorySandboxRuntime {
    runtimes: Mutex<HashMap<String, InMemoryRuntimeState>>,
    fail_release: Mutex<BTreeSet<String>>,
}

#[derive(Debug, Clone)]
struct InMemoryRuntimeState {
    active_processes: u32,
    draft_dirty: bool,
    released: bool,
    attachments: BTreeSet<String>,
}

impl InMemorySandboxRuntime {
    pub fn set_active_processes(&self, runtime_ref: &str, count: u32) {
        if let Some(runtime) = self.runtimes.lock().unwrap().get_mut(runtime_ref) {
            runtime.active_processes = count;
        }
    }

    pub fn fail_release_for(&self, runtime_ref: &str) {
        self.fail_release
            .lock()
            .unwrap()
            .insert(runtime_ref.to_owned());
    }
}

impl SandboxRuntimePort for InMemorySandboxRuntime {
    fn allocate(&self, request: &RuntimeAllocationRequest) -> SandboxResult<RuntimeAllocation> {
        let runtime_ref = format!("local-runtime.{}", request.lease_ref);
        self.runtimes.lock().unwrap().insert(
            runtime_ref.clone(),
            InMemoryRuntimeState {
                active_processes: 0,
                draft_dirty: false,
                released: false,
                attachments: BTreeSet::new(),
            },
        );
        Ok(RuntimeAllocation { runtime_ref })
    }

    fn status(&self, runtime_ref: &str) -> SandboxResult<RuntimeStatus> {
        let runtimes = self.runtimes.lock().unwrap();
        let runtime = runtimes.get(runtime_ref).ok_or_else(not_found)?;
        if runtime.released {
            return Err(SandboxError::new(
                SandboxErrorCode::ProviderFailed,
                "runtime was released",
                false,
            ));
        }
        Ok(RuntimeStatus {
            active_processes: runtime.active_processes,
            draft_dirty: runtime.draft_dirty,
        })
    }

    fn attach(&self, runtime_ref: &str, attachment_ref: &str) -> SandboxResult<()> {
        self.runtimes
            .lock()
            .unwrap()
            .get_mut(runtime_ref)
            .ok_or_else(not_found)?
            .attachments
            .insert(attachment_ref.to_owned());
        Ok(())
    }

    fn quiesce(
        &self,
        runtime_ref: &str,
        strategy: QuiesceStrategy,
        _deadline_at_ms: u64,
    ) -> SandboxResult<QuiesceReceipt> {
        let mut runtimes = self.runtimes.lock().unwrap();
        let runtime = runtimes.get_mut(runtime_ref).ok_or_else(not_found)?;
        if runtime.active_processes > 0 && strategy == QuiesceStrategy::RejectIfActive {
            return Err(SandboxError::new(
                SandboxErrorCode::NotQuiescent,
                "runtime still has active processes",
                true,
            ));
        }
        let terminated_processes = runtime.active_processes;
        runtime.active_processes = 0;
        Ok(QuiesceReceipt {
            receipt_ref: format!("runtime-quiesce.{runtime_ref}"),
            lease_ref: String::new(),
            terminated_processes,
            flushed: true,
            generation: 0,
        })
    }

    fn prepare_draft(&self, runtime_ref: &str) -> SandboxResult<RuntimeDraft> {
        let runtimes = self.runtimes.lock().unwrap();
        let runtime = runtimes.get(runtime_ref).ok_or_else(not_found)?;
        if runtime.active_processes != 0 {
            return Err(SandboxError::new(
                SandboxErrorCode::NotQuiescent,
                "runtime still has active processes",
                true,
            ));
        }
        Ok(RuntimeDraft {
            manifest_digest: "a".repeat(64),
            changed_paths: u64::from(runtime.draft_dirty),
        })
    }

    fn release(
        &self,
        runtime_ref: &str,
        _disposition: ReleaseDisposition,
    ) -> SandboxResult<RuntimeCleanup> {
        if self.fail_release.lock().unwrap().contains(runtime_ref) {
            return Err(SandboxError::new(
                SandboxErrorCode::ProviderFailed,
                "runtime cleanup failed",
                true,
            ));
        }
        let mut runtimes = self.runtimes.lock().unwrap();
        let runtime = runtimes.get_mut(runtime_ref).ok_or_else(not_found)?;
        let processes_terminated = runtime.active_processes;
        runtime.active_processes = 0;
        runtime.released = true;
        Ok(RuntimeCleanup {
            processes_terminated,
            volume_scrubbed: true,
        })
    }
}
