//! Provider-neutral BYOS, storage migration and portable export orchestration.
//!
//! Workspace metadata owns the resource set and active provider generation.
//! Blob providers own immutable bytes. This crate coordinates them without
//! depending on S3 SDK types or exporting credentials.

use async_trait::async_trait;
use openmuse_storage_contract::{
    BlobRef, BlobStorePort, ObjectRequest, ProviderCapabilitySnapshot, PutObjectRequest,
    ReadObjectRequest, StorageError, StorageRequestContext,
};
use serde::{Deserialize, Serialize};
use std::collections::{BTreeSet, HashMap};
use std::sync::{Arc, Mutex};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum AccessMediation {
    Server,
    Desktop,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct ProviderConfiguration {
    pub provider_ref: String,
    pub access_generation: u64,
    pub mediation: AccessMediation,
    pub capabilities: ProviderCapabilitySnapshot,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct WorkspaceProviderBinding {
    pub workspace_ref: String,
    pub provider_ref: String,
    pub generation: u64,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct PortableResourceRecord {
    pub resource_ref: String,
    pub revision: String,
    pub media_type: String,
    pub blob_ref: BlobRef,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PortableResourcePage {
    pub resources: Vec<PortableResourceRecord>,
    pub next_cursor: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct ProviderSwitchReceipt {
    pub metadata_receipt_ref: String,
    pub workspace_ref: String,
    pub before_provider_ref: String,
    pub after_provider_ref: String,
    pub before_generation: u64,
    pub after_generation: u64,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MigratedResourceBinding {
    pub resource_ref: String,
    pub expected_revision: String,
    pub blob_ref: BlobRef,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum MigrationStatus {
    Copying,
    Paused,
    ReadyToSwitch,
    Observing,
    Completed,
    RolledBack,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum MigrationObjectStatus {
    Pending,
    Verified,
    SourceDeleted,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct MigrationObject {
    pub resource: PortableResourceRecord,
    pub status: MigrationObjectStatus,
    pub target_blob_ref: Option<BlobRef>,
    pub copy_receipt_ref: Option<String>,
    pub delete_receipt_ref: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct MigrationState {
    pub migration_ref: String,
    pub workspace_ref: String,
    pub source: ProviderConfiguration,
    pub target: ProviderConfiguration,
    pub source_binding_generation: u64,
    pub active_binding_generation: u64,
    pub status: MigrationStatus,
    pub objects: Vec<MigrationObject>,
    pub switch_receipt: Option<ProviderSwitchReceipt>,
    pub rollback_receipt: Option<ProviderSwitchReceipt>,
    pub source_metadata_delete_receipt_ref: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct StartMigrationRequest {
    pub migration_ref: String,
    pub workspace_ref: String,
    pub source_provider_ref: String,
    pub source_access_generation: u64,
    pub target_provider_ref: String,
    pub target_access_generation: u64,
    pub mediation: AccessMediation,
    pub context: StorageRequestContext,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct WorkspaceExportManifest {
    pub schema: String,
    pub workspace_ref: String,
    pub active_provider_ref: String,
    pub provider_generation: u64,
    pub resources: Vec<PortableResourceRecord>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct SourceDeletionReceipt {
    pub migration_ref: String,
    pub metadata_receipt_ref: String,
    pub provider_receipt_refs: Vec<String>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum PortabilityErrorCode {
    Denied,
    NotFound,
    Conflict,
    Unavailable,
    IntegrityFailed,
    InvalidState,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, thiserror::Error)]
#[error("{code:?}: {message}")]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct PortabilityError {
    pub code: PortabilityErrorCode,
    pub message: String,
    pub retryable: bool,
}

impl PortabilityError {
    pub fn new(code: PortabilityErrorCode, message: impl Into<String>, retryable: bool) -> Self {
        Self {
            code,
            message: message.into(),
            retryable,
        }
    }
}

impl From<StorageError> for PortabilityError {
    fn from(error: StorageError) -> Self {
        use openmuse_storage_contract::StorageErrorCode;
        let code = match error.code {
            StorageErrorCode::Denied
            | StorageErrorCode::Expired
            | StorageErrorCode::StaleGeneration => PortabilityErrorCode::Denied,
            StorageErrorCode::NotFound => PortabilityErrorCode::NotFound,
            StorageErrorCode::Conflict => PortabilityErrorCode::Conflict,
            StorageErrorCode::IntegrityFailed => PortabilityErrorCode::IntegrityFailed,
            StorageErrorCode::Unavailable | StorageErrorCode::Transient => {
                PortabilityErrorCode::Unavailable
            }
        };
        Self::new(code, error.message, error.retryable)
    }
}

pub type PortabilityResult<T> = Result<T, PortabilityError>;

#[async_trait]
pub trait ProviderAccessBrokerPort: Send + Sync {
    async fn configuration(
        &self,
        provider_ref: &str,
        expected_access_generation: u64,
    ) -> PortabilityResult<ProviderConfiguration>;
    async fn resolve(
        &self,
        provider_ref: &str,
        expected_access_generation: u64,
    ) -> PortabilityResult<Arc<dyn BlobStorePort>>;
}

#[async_trait]
pub trait PortabilityMetadataPort: Send + Sync {
    async fn binding(&self, workspace_ref: &str) -> PortabilityResult<WorkspaceProviderBinding>;
    async fn list_resources(
        &self,
        workspace_ref: &str,
        cursor: Option<&str>,
        limit: usize,
    ) -> PortabilityResult<PortableResourcePage>;
    async fn switch_provider(
        &self,
        workspace_ref: &str,
        expected_generation: u64,
        target_provider_ref: &str,
        migration_ref: &str,
        resources: &[MigratedResourceBinding],
    ) -> PortabilityResult<ProviderSwitchReceipt>;
    async fn source_objects_safe_to_delete(
        &self,
        workspace_ref: &str,
        migration_ref: &str,
        candidate_object_refs: &[String],
    ) -> PortabilityResult<Vec<String>>;
    async fn record_source_deleted(
        &self,
        workspace_ref: &str,
        migration_ref: &str,
        provider_receipt_refs: &[String],
    ) -> PortabilityResult<String>;
}

#[async_trait]
pub trait MigrationStateStorePort: Send + Sync {
    async fn load(&self, migration_ref: &str) -> PortabilityResult<MigrationState>;
    async fn save(&self, state: MigrationState) -> PortabilityResult<()>;
}

pub struct StoragePortabilityService {
    access: Arc<dyn ProviderAccessBrokerPort>,
    metadata: Arc<dyn PortabilityMetadataPort>,
    states: Arc<dyn MigrationStateStorePort>,
}

impl StoragePortabilityService {
    pub fn new(
        access: Arc<dyn ProviderAccessBrokerPort>,
        metadata: Arc<dyn PortabilityMetadataPort>,
        states: Arc<dyn MigrationStateStorePort>,
    ) -> Self {
        Self {
            access,
            metadata,
            states,
        }
    }

    pub async fn start(&self, request: StartMigrationRequest) -> PortabilityResult<MigrationState> {
        request.context.validate()?;
        if request.migration_ref.is_empty()
            || request.workspace_ref.is_empty()
            || request.source_provider_ref == request.target_provider_ref
        {
            return Err(PortabilityError::new(
                PortabilityErrorCode::InvalidState,
                "migration identity or provider pair is invalid",
                false,
            ));
        }
        let binding = self.metadata.binding(&request.workspace_ref).await?;
        if binding.provider_ref != request.source_provider_ref {
            return Err(PortabilityError::new(
                PortabilityErrorCode::Conflict,
                "source provider is not the active workspace binding",
                false,
            ));
        }
        let source = self
            .access
            .configuration(
                &request.source_provider_ref,
                request.source_access_generation,
            )
            .await?;
        let target = self
            .access
            .configuration(
                &request.target_provider_ref,
                request.target_access_generation,
            )
            .await?;
        source.capabilities.validate_profile_v1()?;
        target.capabilities.validate_profile_v1()?;
        if source.mediation != request.mediation || target.mediation != request.mediation {
            return Err(PortabilityError::new(
                PortabilityErrorCode::Denied,
                "provider mediation does not match the selected execution placement",
                false,
            ));
        }

        let resources = self.all_resources(&request.workspace_ref).await?;
        let state = MigrationState {
            migration_ref: request.migration_ref,
            workspace_ref: request.workspace_ref,
            source,
            target,
            source_binding_generation: binding.generation,
            active_binding_generation: binding.generation,
            status: MigrationStatus::Copying,
            objects: resources
                .into_iter()
                .map(|resource| MigrationObject {
                    resource,
                    status: MigrationObjectStatus::Pending,
                    target_blob_ref: None,
                    copy_receipt_ref: None,
                    delete_receipt_ref: None,
                })
                .collect(),
            switch_receipt: None,
            rollback_receipt: None,
            source_metadata_delete_receipt_ref: None,
        };
        self.states.save(state.clone()).await?;
        Ok(state)
    }

    pub async fn copy_batch(
        &self,
        migration_ref: &str,
        context: &StorageRequestContext,
        limit: usize,
    ) -> PortabilityResult<usize> {
        context.validate()?;
        if limit == 0 {
            return Err(PortabilityError::new(
                PortabilityErrorCode::InvalidState,
                "copy batch limit must be positive",
                false,
            ));
        }
        let mut state = self.states.load(migration_ref).await?;
        if state.status != MigrationStatus::Copying {
            return Err(PortabilityError::new(
                PortabilityErrorCode::InvalidState,
                "migration is not accepting copy work",
                false,
            ));
        }
        let source = self
            .access
            .resolve(&state.source.provider_ref, state.source.access_generation)
            .await?;
        let target = self
            .access
            .resolve(&state.target.provider_ref, state.target.access_generation)
            .await?;
        let pending: Vec<usize> = state
            .objects
            .iter()
            .enumerate()
            .filter(|(_, object)| object.status == MigrationObjectStatus::Pending)
            .map(|(index, _)| index)
            .take(limit)
            .collect();
        let mut copied = 0;
        for index in pending {
            let resource = state.objects[index].resource.clone();
            let mut read_context = context.clone();
            read_context.request_id = format!("{migration_ref}.read.{index}");
            let mut write_context = context.clone();
            write_context.request_id = format!("{migration_ref}.write.{index}");
            let read = source
                .read(ReadObjectRequest {
                    context: read_context,
                    object_ref: resource.blob_ref.object_ref.clone(),
                    range: None,
                })
                .await?;
            if read.descriptor.blob_ref.digest != resource.blob_ref.digest
                || read.descriptor.blob_ref.size != resource.blob_ref.size
            {
                return Err(PortabilityError::new(
                    PortabilityErrorCode::IntegrityFailed,
                    "source object does not match metadata-owned digest",
                    false,
                ));
            }
            let target_object_ref = format!("blobs/sha256/{}", resource.blob_ref.digest.value);
            let mut body = read.body;
            let receipt = target
                .put(
                    PutObjectRequest {
                        context: write_context.clone(),
                        object_ref: target_object_ref.clone(),
                        expected_digest: resource.blob_ref.digest.clone(),
                        size: resource.blob_ref.size,
                        content_type: resource.media_type.clone(),
                        metadata: Default::default(),
                        idempotency_key: format!("{migration_ref}.copy.{index}"),
                    },
                    body.as_mut(),
                )
                .await?;
            let descriptor = target
                .head(ObjectRequest {
                    context: write_context,
                    object_ref: target_object_ref,
                })
                .await?;
            if descriptor.blob_ref.digest != resource.blob_ref.digest
                || descriptor.blob_ref.size != resource.blob_ref.size
            {
                return Err(PortabilityError::new(
                    PortabilityErrorCode::IntegrityFailed,
                    "target verification did not match the source digest",
                    false,
                ));
            }
            state.objects[index].status = MigrationObjectStatus::Verified;
            state.objects[index].target_blob_ref = Some(descriptor.blob_ref);
            state.objects[index].copy_receipt_ref = Some(receipt.receipt_ref);
            copied += 1;
            self.states.save(state.clone()).await?;
        }
        if state
            .objects
            .iter()
            .all(|object| object.status == MigrationObjectStatus::Verified)
        {
            state.status = MigrationStatus::ReadyToSwitch;
            self.states.save(state).await?;
        }
        Ok(copied)
    }

    pub async fn pause(&self, migration_ref: &str) -> PortabilityResult<()> {
        let mut state = self.states.load(migration_ref).await?;
        if state.status != MigrationStatus::Copying {
            return Err(invalid_transition("only a copying migration can be paused"));
        }
        state.status = MigrationStatus::Paused;
        self.states.save(state).await
    }

    pub async fn resume(&self, migration_ref: &str) -> PortabilityResult<()> {
        let mut state = self.states.load(migration_ref).await?;
        if state.status != MigrationStatus::Paused {
            return Err(invalid_transition("only a paused migration can be resumed"));
        }
        self.access
            .configuration(&state.source.provider_ref, state.source.access_generation)
            .await?;
        self.access
            .configuration(&state.target.provider_ref, state.target.access_generation)
            .await?;
        state.status = MigrationStatus::Copying;
        self.states.save(state).await
    }

    pub async fn cutover(&self, migration_ref: &str) -> PortabilityResult<ProviderSwitchReceipt> {
        let mut state = self.states.load(migration_ref).await?;
        if state.status != MigrationStatus::ReadyToSwitch {
            return Err(invalid_transition("migration is not fully verified"));
        }
        let replacements = state
            .objects
            .iter()
            .map(|object| {
                Ok(MigratedResourceBinding {
                    resource_ref: object.resource.resource_ref.clone(),
                    expected_revision: object.resource.revision.clone(),
                    blob_ref: object.target_blob_ref.clone().ok_or_else(|| {
                        invalid_transition("verified migration object has no target blob")
                    })?,
                })
            })
            .collect::<PortabilityResult<Vec<_>>>()?;
        let receipt = self
            .metadata
            .switch_provider(
                &state.workspace_ref,
                state.source_binding_generation,
                &state.target.provider_ref,
                migration_ref,
                &replacements,
            )
            .await?;
        state.active_binding_generation = receipt.after_generation;
        state.switch_receipt = Some(receipt.clone());
        state.status = MigrationStatus::Observing;
        self.states.save(state).await?;
        Ok(receipt)
    }

    pub async fn rollback(&self, migration_ref: &str) -> PortabilityResult<ProviderSwitchReceipt> {
        let mut state = self.states.load(migration_ref).await?;
        if state.status != MigrationStatus::Observing {
            return Err(invalid_transition(
                "rollback is available only during the observation window",
            ));
        }
        let receipt = self
            .metadata
            .switch_provider(
                &state.workspace_ref,
                state.active_binding_generation,
                &state.source.provider_ref,
                migration_ref,
                &state
                    .objects
                    .iter()
                    .map(|object| MigratedResourceBinding {
                        resource_ref: object.resource.resource_ref.clone(),
                        expected_revision: object.resource.revision.clone(),
                        blob_ref: object.resource.blob_ref.clone(),
                    })
                    .collect::<Vec<_>>(),
            )
            .await?;
        state.active_binding_generation = receipt.after_generation;
        state.rollback_receipt = Some(receipt.clone());
        state.status = MigrationStatus::RolledBack;
        self.states.save(state).await?;
        Ok(receipt)
    }

    pub async fn confirm_source_deletion(
        &self,
        migration_ref: &str,
        context: &StorageRequestContext,
        confirmed_by_user: bool,
    ) -> PortabilityResult<SourceDeletionReceipt> {
        if !confirmed_by_user {
            return Err(PortabilityError::new(
                PortabilityErrorCode::Denied,
                "source deletion requires a second explicit confirmation",
                false,
            ));
        }
        let mut state = self.states.load(migration_ref).await?;
        if state.status != MigrationStatus::Observing {
            return Err(invalid_transition(
                "source deletion is available only after cutover",
            ));
        }
        let source = self
            .access
            .resolve(&state.source.provider_ref, state.source.access_generation)
            .await?;
        let candidates: Vec<String> = state
            .objects
            .iter()
            .map(|object| object.resource.blob_ref.object_ref.clone())
            .collect::<BTreeSet<_>>()
            .into_iter()
            .collect();
        let approved: BTreeSet<String> = self
            .metadata
            .source_objects_safe_to_delete(&state.workspace_ref, migration_ref, &candidates)
            .await?
            .into_iter()
            .collect();
        if !approved.is_subset(&candidates.into_iter().collect()) {
            return Err(PortabilityError::new(
                PortabilityErrorCode::Denied,
                "metadata approved an object outside the migration",
                false,
            ));
        }
        let mut provider_receipts: Vec<String> = state
            .objects
            .iter()
            .filter_map(|object| object.delete_receipt_ref.clone())
            .collect();
        for index in 0..state.objects.len() {
            if state.objects[index].status == MigrationObjectStatus::SourceDeleted {
                continue;
            }
            if !approved.contains(&state.objects[index].resource.blob_ref.object_ref) {
                continue;
            }
            let mut delete_context = context.clone();
            delete_context.request_id = format!("{migration_ref}.delete.{index}");
            let receipt = source
                .delete(ObjectRequest {
                    context: delete_context,
                    object_ref: state.objects[index].resource.blob_ref.object_ref.clone(),
                })
                .await?;
            provider_receipts.push(receipt.receipt_ref.clone());
            state.objects[index].delete_receipt_ref = Some(receipt.receipt_ref);
            state.objects[index].status = MigrationObjectStatus::SourceDeleted;
            self.states.save(state.clone()).await?;
        }
        let metadata_receipt_ref = self
            .metadata
            .record_source_deleted(&state.workspace_ref, migration_ref, &provider_receipts)
            .await?;
        state.source_metadata_delete_receipt_ref = Some(metadata_receipt_ref.clone());
        state.status = MigrationStatus::Completed;
        self.states.save(state).await?;
        Ok(SourceDeletionReceipt {
            migration_ref: migration_ref.to_owned(),
            metadata_receipt_ref,
            provider_receipt_refs: provider_receipts,
        })
    }

    pub async fn export_manifest(
        &self,
        workspace_ref: &str,
    ) -> PortabilityResult<WorkspaceExportManifest> {
        let binding = self.metadata.binding(workspace_ref).await?;
        Ok(WorkspaceExportManifest {
            schema: "openmuse.workspace-export@1".to_owned(),
            workspace_ref: workspace_ref.to_owned(),
            active_provider_ref: binding.provider_ref,
            provider_generation: binding.generation,
            resources: self.all_resources(workspace_ref).await?,
        })
    }

    async fn all_resources(
        &self,
        workspace_ref: &str,
    ) -> PortabilityResult<Vec<PortableResourceRecord>> {
        let mut resources = Vec::new();
        let mut cursor = None;
        loop {
            let page = self
                .metadata
                .list_resources(workspace_ref, cursor.as_deref(), 500)
                .await?;
            resources.extend(page.resources);
            match page.next_cursor {
                Some(next) => cursor = Some(next),
                None => return Ok(resources),
            }
        }
    }
}

fn invalid_transition(message: &str) -> PortabilityError {
    PortabilityError::new(PortabilityErrorCode::InvalidState, message, false)
}

#[derive(Clone, Default)]
pub struct InMemoryMigrationStateStore {
    states: Arc<Mutex<HashMap<String, MigrationState>>>,
}

#[async_trait]
impl MigrationStateStorePort for InMemoryMigrationStateStore {
    async fn load(&self, migration_ref: &str) -> PortabilityResult<MigrationState> {
        self.states
            .lock()
            .map_err(|_| unavailable_state())?
            .get(migration_ref)
            .cloned()
            .ok_or_else(|| {
                PortabilityError::new(
                    PortabilityErrorCode::NotFound,
                    "migration state not found",
                    false,
                )
            })
    }

    async fn save(&self, state: MigrationState) -> PortabilityResult<()> {
        self.states
            .lock()
            .map_err(|_| unavailable_state())?
            .insert(state.migration_ref.clone(), state);
        Ok(())
    }
}

fn unavailable_state() -> PortabilityError {
    PortabilityError::new(
        PortabilityErrorCode::Unavailable,
        "migration state store is unavailable",
        true,
    )
}
