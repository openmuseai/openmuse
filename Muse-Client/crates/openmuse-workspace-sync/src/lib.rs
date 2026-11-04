//! Provider-neutral Workspace Sync orchestration.
//!
//! Local Workspace, Cloud Resource Service and storage remain separate
//! providers. This crate owns only explicit policy, durable work/cursors,
//! idempotent orchestration, conflict records and user-visible receipts.

use async_trait::async_trait;
use openmuse_storage_contract::{BlobDigest, BlobReadStream};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::{Arc, Mutex};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum SyncPolicy {
    LocalOnly,
    Snapshot,
    Mirror,
    Migrate,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum WorkspaceAuthorityPlacement {
    Local,
    Cloud,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum MigrationPhase {
    NotApplicable,
    Transferring,
    Verifying,
    Complete,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct PolicyCapabilities {
    pub uploads_local_changes: bool,
    pub downloads_cloud_changes: bool,
    pub cloud_writable: bool,
    pub mobile_available_when_desktop_offline: bool,
}

impl SyncPolicy {
    pub fn capabilities(self, migration_phase: MigrationPhase) -> PolicyCapabilities {
        match self {
            Self::LocalOnly => PolicyCapabilities {
                uploads_local_changes: false,
                downloads_cloud_changes: false,
                cloud_writable: false,
                mobile_available_when_desktop_offline: false,
            },
            Self::Snapshot => PolicyCapabilities {
                uploads_local_changes: true,
                downloads_cloud_changes: false,
                cloud_writable: false,
                mobile_available_when_desktop_offline: true,
            },
            Self::Mirror => PolicyCapabilities {
                uploads_local_changes: true,
                downloads_cloud_changes: true,
                cloud_writable: true,
                mobile_available_when_desktop_offline: true,
            },
            Self::Migrate => PolicyCapabilities {
                uploads_local_changes: true,
                downloads_cloud_changes: false,
                cloud_writable: migration_phase == MigrationPhase::Complete,
                mobile_available_when_desktop_offline: migration_phase == MigrationPhase::Complete,
            },
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum SyncErrorCode {
    PolicyNotConfirmed,
    Denied,
    Conflict,
    Unavailable,
    IntegrityFailed,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, thiserror::Error)]
#[error("{code:?}: {message}")]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct SyncError {
    pub code: SyncErrorCode,
    pub message: String,
    pub retryable: bool,
}

impl SyncError {
    pub fn new(code: SyncErrorCode, message: impl Into<String>, retryable: bool) -> Self {
        Self {
            code,
            message: message.into(),
            retryable,
        }
    }
}

pub type SyncResult<T> = Result<T, SyncError>;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct LocalChange {
    pub change_ref: String,
    pub cursor: String,
    pub workspace_ref: String,
    pub resource_ref: String,
    pub local_revision: String,
    pub expected_cloud_revision: Option<String>,
    pub digest: BlobDigest,
    pub size: u64,
    pub media_type: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LocalChangePage {
    pub changes: Vec<LocalChange>,
    pub caught_up: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct CloudChange {
    pub change_ref: String,
    pub cursor: String,
    pub workspace_ref: String,
    pub resource_ref: String,
    pub cloud_revision: String,
    pub base_local_revision: String,
    pub digest: BlobDigest,
    pub size: u64,
    pub media_type: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CloudChangePage {
    pub changes: Vec<CloudChange>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct StagedCloudContent {
    pub stage_ref: String,
    pub resource_ref: String,
    pub digest: BlobDigest,
    pub size: u64,
    pub provider_receipt_ref: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct CloudCommitReceipt {
    pub receipt_ref: String,
    pub resource_ref: String,
    pub cloud_revision: String,
    pub provider_receipt_ref: String,
    pub replayed: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum WorkStatus {
    Planned,
    Staged,
    Committed,
    Conflict,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct SyncWork {
    pub work_ref: String,
    pub change: LocalChange,
    pub status: WorkStatus,
    pub stage: Option<StagedCloudContent>,
    pub receipt: Option<CloudCommitReceipt>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ConflictDomain {
    Text,
    Office,
    Binary,
}

impl ConflictDomain {
    pub fn for_media_type(media_type: &str) -> Self {
        if media_type.starts_with("text/") || media_type == "application/json" {
            Self::Text
        } else if media_type.contains("officedocument")
            || media_type.contains("msword")
            || media_type.contains("spreadsheet")
            || media_type.contains("presentation")
        {
            Self::Office
        } else {
            Self::Binary
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct SyncConflict {
    pub conflict_ref: String,
    pub resource_ref: String,
    pub local_revision: String,
    pub cloud_revision: String,
    pub domain: ConflictDomain,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct SyncReceipt {
    pub receipt_ref: String,
    pub workspace_ref: String,
    pub resource_ref: String,
    pub local_revision: String,
    pub cloud_revision: String,
    pub local_cursor: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct WorkspaceSyncState {
    pub workspace_ref: String,
    pub policy: Option<SyncPolicy>,
    pub migration_phase: MigrationPhase,
    pub authority: WorkspaceAuthorityPlacement,
    pub local_cursor: Option<String>,
    pub cloud_cursor: Option<String>,
    pub work: Vec<SyncWork>,
    pub conflicts: Vec<SyncConflict>,
    pub receipts: Vec<SyncReceipt>,
    pub sequence: u64,
}

impl WorkspaceSyncState {
    pub fn unconfigured(workspace_ref: impl Into<String>) -> Self {
        Self {
            workspace_ref: workspace_ref.into(),
            policy: None,
            migration_phase: MigrationPhase::NotApplicable,
            authority: WorkspaceAuthorityPlacement::Local,
            local_cursor: None,
            cloud_cursor: None,
            work: Vec::new(),
            conflicts: Vec::new(),
            receipts: Vec::new(),
            sequence: 0,
        }
    }
}

#[async_trait]
pub trait SyncStateStorePort: Send + Sync {
    async fn load(&self, workspace_ref: &str) -> SyncResult<WorkspaceSyncState>;
    async fn save(&self, state: WorkspaceSyncState) -> SyncResult<()>;
}

#[async_trait]
pub trait LocalWorkspaceSyncPort: Send + Sync {
    async fn changes_after(
        &self,
        workspace_ref: &str,
        cursor: Option<&str>,
        limit: usize,
    ) -> SyncResult<LocalChangePage>;
    async fn materialize(&self, change: &LocalChange) -> SyncResult<Box<dyn BlobReadStream>>;
    async fn current_revision(&self, workspace_ref: &str, resource_ref: &str)
    -> SyncResult<String>;
    async fn apply_cloud_change(&self, change: &CloudChange) -> SyncResult<()>;
}

#[async_trait]
pub trait CloudResourceSyncPort: Send + Sync {
    async fn upload(
        &self,
        change: &LocalChange,
        body: &mut dyn BlobReadStream,
        idempotency_key: &str,
    ) -> SyncResult<StagedCloudContent>;
    async fn commit(
        &self,
        change: &LocalChange,
        stage: &StagedCloudContent,
        idempotency_key: &str,
        cloud_writable: bool,
    ) -> SyncResult<CloudCommitReceipt>;
    async fn changes_after(
        &self,
        workspace_ref: &str,
        cursor: Option<&str>,
        limit: usize,
    ) -> SyncResult<CloudChangePage>;
    async fn verify_migration(&self, workspace_ref: &str) -> SyncResult<bool>;
    async fn activate_cloud_authority(&self, workspace_ref: &str) -> SyncResult<()>;
}

pub struct WorkspaceSyncEngine {
    state: Arc<dyn SyncStateStorePort>,
    local: Arc<dyn LocalWorkspaceSyncPort>,
    cloud: Arc<dyn CloudResourceSyncPort>,
}

impl WorkspaceSyncEngine {
    pub fn new(
        state: Arc<dyn SyncStateStorePort>,
        local: Arc<dyn LocalWorkspaceSyncPort>,
        cloud: Arc<dyn CloudResourceSyncPort>,
    ) -> Self {
        Self {
            state,
            local,
            cloud,
        }
    }

    pub async fn configure_policy(
        &self,
        workspace_ref: &str,
        policy: SyncPolicy,
        confirmed_by_user: bool,
    ) -> SyncResult<()> {
        if !confirmed_by_user {
            return Err(SyncError::new(
                SyncErrorCode::PolicyNotConfirmed,
                "sync policy requires explicit user confirmation",
                false,
            ));
        }
        let mut state = self.state.load(workspace_ref).await?;
        state.policy = Some(policy);
        state.migration_phase = if policy == SyncPolicy::Migrate {
            MigrationPhase::Transferring
        } else {
            MigrationPhase::NotApplicable
        };
        state.authority = WorkspaceAuthorityPlacement::Local;
        self.state.save(state).await
    }

    pub async fn run_local_cycle(&self, workspace_ref: &str, limit: usize) -> SyncResult<usize> {
        let mut state = self.state.load(workspace_ref).await?;
        let policy = state.policy.ok_or_else(|| {
            SyncError::new(
                SyncErrorCode::PolicyNotConfirmed,
                "sync policy has not been selected",
                false,
            )
        })?;
        let page = self
            .local
            .changes_after(workspace_ref, state.local_cursor.as_deref(), limit)
            .await?;
        if policy == SyncPolicy::LocalOnly {
            if let Some(last) = page.changes.last() {
                state.local_cursor = Some(last.cursor.clone());
            }
            self.state.save(state).await?;
            return Ok(0);
        }
        for change in page.changes {
            if !state
                .work
                .iter()
                .any(|item| item.change.change_ref == change.change_ref)
            {
                state.sequence += 1;
                state.work.push(SyncWork {
                    work_ref: format!("sync-work.{}", state.sequence),
                    change,
                    status: WorkStatus::Planned,
                    stage: None,
                    receipt: None,
                });
            }
        }
        self.state.save(state.clone()).await?;

        let mut completed = 0;
        for index in 0..state.work.len() {
            if matches!(
                state.work[index].status,
                WorkStatus::Committed | WorkStatus::Conflict
            ) {
                continue;
            }
            let change = state.work[index].change.clone();
            let work_ref = state.work[index].work_ref.clone();
            if state.work[index].stage.is_none() {
                let mut body = self.local.materialize(&change).await?;
                let stage = self
                    .cloud
                    .upload(&change, body.as_mut(), &format!("{work_ref}:upload"))
                    .await?;
                state.work[index].stage = Some(stage);
                state.work[index].status = WorkStatus::Staged;
                self.state.save(state.clone()).await?;
            }
            let stage = state.work[index].stage.clone().expect("stage persisted");
            match self
                .cloud
                .commit(
                    &change,
                    &stage,
                    &format!("{work_ref}:commit"),
                    policy.capabilities(state.migration_phase).cloud_writable,
                )
                .await
            {
                Ok(receipt) => {
                    state.work[index].status = WorkStatus::Committed;
                    state.work[index].receipt = Some(receipt.clone());
                    state.local_cursor = Some(change.cursor.clone());
                    state.sequence += 1;
                    let receipt_ref = format!("sync-receipt.{}", state.sequence);
                    state.receipts.push(SyncReceipt {
                        receipt_ref,
                        workspace_ref: workspace_ref.to_owned(),
                        resource_ref: change.resource_ref.clone(),
                        local_revision: change.local_revision,
                        cloud_revision: receipt.cloud_revision,
                        local_cursor: change.cursor,
                    });
                    completed += 1;
                    self.state.save(state.clone()).await?;
                }
                Err(error) if error.code == SyncErrorCode::Conflict => {
                    state.work[index].status = WorkStatus::Conflict;
                    state.sequence += 1;
                    state.conflicts.push(SyncConflict {
                        conflict_ref: format!("sync-conflict.{}", state.sequence),
                        resource_ref: change.resource_ref,
                        local_revision: change.local_revision,
                        cloud_revision: change
                            .expected_cloud_revision
                            .unwrap_or_else(|| "revision.unknown".to_owned()),
                        domain: ConflictDomain::for_media_type(&change.media_type),
                    });
                    state.local_cursor = Some(change.cursor);
                    self.state.save(state.clone()).await?;
                }
                Err(error) => return Err(error),
            }
        }

        if policy == SyncPolicy::Migrate
            && state.migration_phase != MigrationPhase::Complete
            && page.caught_up
            && state
                .work
                .iter()
                .all(|item| item.status == WorkStatus::Committed)
        {
            state.migration_phase = MigrationPhase::Verifying;
            self.state.save(state.clone()).await?;
            if self.cloud.verify_migration(workspace_ref).await? {
                self.cloud.activate_cloud_authority(workspace_ref).await?;
                state.migration_phase = MigrationPhase::Complete;
                state.authority = WorkspaceAuthorityPlacement::Cloud;
                self.state.save(state).await?;
            }
        }
        Ok(completed)
    }

    pub async fn run_cloud_cycle(&self, workspace_ref: &str, limit: usize) -> SyncResult<usize> {
        let mut state = self.state.load(workspace_ref).await?;
        if state.policy != Some(SyncPolicy::Mirror) {
            return Err(SyncError::new(
                SyncErrorCode::Denied,
                "cloud-to-local sync is available only for mirror policy",
                false,
            ));
        }
        let page = self
            .cloud
            .changes_after(workspace_ref, state.cloud_cursor.as_deref(), limit)
            .await?;
        let mut applied = 0;
        for change in page.changes {
            let local_revision = self
                .local
                .current_revision(workspace_ref, &change.resource_ref)
                .await?;
            if local_revision != change.base_local_revision {
                state.sequence += 1;
                state.conflicts.push(SyncConflict {
                    conflict_ref: format!("sync-conflict.{}", state.sequence),
                    resource_ref: change.resource_ref.clone(),
                    local_revision,
                    cloud_revision: change.cloud_revision.clone(),
                    domain: ConflictDomain::for_media_type(&change.media_type),
                });
            } else {
                self.local.apply_cloud_change(&change).await?;
                applied += 1;
            }
            state.cloud_cursor = Some(change.cursor);
            self.state.save(state.clone()).await?;
        }
        Ok(applied)
    }
}

#[derive(Clone, Default)]
pub struct InMemorySyncStateStore {
    states: Arc<Mutex<HashMap<String, WorkspaceSyncState>>>,
}

impl InMemorySyncStateStore {
    pub fn insert(&self, state: WorkspaceSyncState) -> SyncResult<()> {
        self.lock()?.insert(state.workspace_ref.clone(), state);
        Ok(())
    }

    fn lock(&self) -> SyncResult<std::sync::MutexGuard<'_, HashMap<String, WorkspaceSyncState>>> {
        self.states.lock().map_err(|_| {
            SyncError::new(
                SyncErrorCode::Unavailable,
                "sync state store is poisoned",
                true,
            )
        })
    }
}

#[async_trait]
impl SyncStateStorePort for InMemorySyncStateStore {
    async fn load(&self, workspace_ref: &str) -> SyncResult<WorkspaceSyncState> {
        self.lock()?.get(workspace_ref).cloned().ok_or_else(|| {
            SyncError::new(
                SyncErrorCode::Unavailable,
                "workspace sync state is missing",
                false,
            )
        })
    }

    async fn save(&self, state: WorkspaceSyncState) -> SyncResult<()> {
        self.lock()?.insert(state.workspace_ref.clone(), state);
        Ok(())
    }
}
