//! Cloud Workspace metadata authority.
//!
//! The user-visible tree, revisions, ACLs and events live in the metadata
//! transaction boundary. S3 remains an immutable blob data plane and is never
//! listed to construct the catalog.

use async_trait::async_trait;
use openmuse_storage_contract::{
    BlobDigest, BlobReadStream, BlobRef, BlobStorePort, ByteRange, PutObjectRequest, ReadObject,
    ReadObjectRequest, StorageError, StorageErrorCode, StorageRequestContext,
};
use serde::{Deserialize, Serialize};
use std::collections::{BTreeMap, BTreeSet, HashMap};
use std::sync::{Arc, Mutex};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum AuthorityErrorCode {
    Denied,
    NotFound,
    Conflict,
    Expired,
    StaleGeneration,
    MetadataUnavailable,
    BlobUnavailable,
    IntegrityFailed,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, thiserror::Error)]
#[error("{code:?}: {message}")]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct AuthorityError {
    pub code: AuthorityErrorCode,
    pub message: String,
    pub retryable: bool,
}

impl AuthorityError {
    fn new(code: AuthorityErrorCode, message: impl Into<String>, retryable: bool) -> Self {
        Self {
            code,
            message: message.into(),
            retryable,
        }
    }
}

impl From<StorageError> for AuthorityError {
    fn from(value: StorageError) -> Self {
        let code = match value.code {
            StorageErrorCode::Denied => AuthorityErrorCode::Denied,
            StorageErrorCode::NotFound => AuthorityErrorCode::NotFound,
            StorageErrorCode::Conflict => AuthorityErrorCode::Conflict,
            StorageErrorCode::Expired => AuthorityErrorCode::Expired,
            StorageErrorCode::StaleGeneration => AuthorityErrorCode::StaleGeneration,
            StorageErrorCode::IntegrityFailed => AuthorityErrorCode::IntegrityFailed,
            StorageErrorCode::Unavailable | StorageErrorCode::Transient => {
                AuthorityErrorCode::BlobUnavailable
            }
        };
        Self::new(code, value.message, value.retryable)
    }
}

pub type AuthorityResult<T> = Result<T, AuthorityError>;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct ResourceAcl {
    pub owner: String,
    pub readers: BTreeSet<String>,
    pub writers: BTreeSet<String>,
}

impl ResourceAcl {
    fn can_read(&self, principal: &str) -> bool {
        principal == self.owner || self.readers.contains(principal) || self.can_write(principal)
    }

    fn can_write(&self, principal: &str) -> bool {
        principal == self.owner || self.writers.contains(principal)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct ResourceRecord {
    pub workspace_ref: String,
    pub resource_ref: String,
    pub parent_ref: Option<String>,
    pub display_name: String,
    pub media_type: String,
    pub revision: String,
    pub blob_ref: BlobRef,
    pub acl: ResourceAcl,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CatalogQuery {
    pub workspace_ref: String,
    pub parent_ref: Option<String>,
    pub search: Option<String>,
    pub cursor: Option<String>,
    pub limit: usize,
    pub principal_ref: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct CatalogPage {
    pub resources: Vec<ResourceRecord>,
    pub next_cursor: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CommitResourceRequest {
    pub context: StorageRequestContext,
    pub resource_ref: String,
    pub expected_revision: String,
    pub expected_digest: BlobDigest,
    pub size: u64,
    pub content_type: String,
    pub idempotency_key: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct CommitReceipt {
    pub commit_ref: String,
    pub workspace_ref: String,
    pub resource_ref: String,
    pub before_revision: String,
    pub after_revision: String,
    pub blob_ref: BlobRef,
    pub storage_receipt_ref: String,
    pub metadata_receipt_ref: String,
    pub replayed: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct ResourceChangedEvent {
    pub event_ref: String,
    pub workspace_ref: String,
    pub resource_ref: String,
    pub before_revision: String,
    pub after_revision: String,
    pub blob_ref: BlobRef,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct OutboxEntry {
    pub event: ResourceChangedEvent,
    pub attempts: u32,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MetadataCommitRequest {
    pub workspace_ref: String,
    pub resource_ref: String,
    pub expected_revision: String,
    pub blob_ref: BlobRef,
    pub storage_receipt_ref: String,
    pub actor_ref: String,
    pub idempotency_key: String,
    pub fingerprint: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct OrphanBlob {
    pub workspace_ref: String,
    pub resource_ref: String,
    pub blob_ref: BlobRef,
    pub storage_receipt_ref: String,
    pub reason: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MaterializeRequest {
    pub workspace_ref: String,
    pub resource_ref: String,
    pub principal_ref: String,
    pub audience: String,
    pub generation: u64,
    pub now_ms: u64,
    pub ttl_ms: u64,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct MaterializationHandle {
    pub handle_ref: String,
    pub workspace_ref: String,
    pub resource_ref: String,
    pub revision: String,
    pub blob_ref: BlobRef,
    pub audience: String,
    pub generation: u64,
    pub expires_at_ms: u64,
}

#[async_trait]
pub trait MetadataStorePort: Send + Sync {
    async fn seed(&self, resource: ResourceRecord) -> AuthorityResult<()>;
    async fn describe(
        &self,
        workspace_ref: &str,
        resource_ref: &str,
    ) -> AuthorityResult<ResourceRecord>;
    async fn list(&self, query: &CatalogQuery) -> AuthorityResult<CatalogPage>;
    async fn cas_commit(&self, request: MetadataCommitRequest) -> AuthorityResult<CommitReceipt>;
    async fn record_orphan(&self, orphan: OrphanBlob) -> AuthorityResult<()>;
    async fn list_orphans(&self) -> AuthorityResult<Vec<OrphanBlob>>;
    async fn issue_handle(
        &self,
        request: MaterializeRequest,
    ) -> AuthorityResult<MaterializationHandle>;
    async fn resolve_handle(
        &self,
        handle_ref: &str,
        audience: &str,
        generation: u64,
        now_ms: u64,
    ) -> AuthorityResult<MaterializationHandle>;
    async fn pending_outbox(&self, limit: usize) -> AuthorityResult<Vec<OutboxEntry>>;
    async fn record_outbox_attempt(&self, event_ref: &str) -> AuthorityResult<()>;
    async fn ack_outbox(&self, event_ref: &str) -> AuthorityResult<()>;
}

#[async_trait]
pub trait ResourceEventPublisherPort: Send + Sync {
    async fn publish(&self, event: &ResourceChangedEvent) -> AuthorityResult<()>;
}

pub struct CloudResourceAuthority {
    blobs: Arc<dyn BlobStorePort>,
    metadata: Arc<dyn MetadataStorePort>,
    publisher: Arc<dyn ResourceEventPublisherPort>,
}

impl CloudResourceAuthority {
    pub fn new(
        blobs: Arc<dyn BlobStorePort>,
        metadata: Arc<dyn MetadataStorePort>,
        publisher: Arc<dyn ResourceEventPublisherPort>,
    ) -> Self {
        Self {
            blobs,
            metadata,
            publisher,
        }
    }

    pub async fn list(&self, query: &CatalogQuery) -> AuthorityResult<CatalogPage> {
        self.metadata.list(query).await
    }

    pub async fn describe(
        &self,
        workspace_ref: &str,
        resource_ref: &str,
        principal_ref: &str,
    ) -> AuthorityResult<ResourceRecord> {
        let value = self.metadata.describe(workspace_ref, resource_ref).await?;
        if !value.acl.can_read(principal_ref) {
            return Err(denied("resource read is not granted"));
        }
        Ok(value)
    }

    pub async fn commit(
        &self,
        request: CommitResourceRequest,
        body: &mut dyn BlobReadStream,
    ) -> AuthorityResult<CommitReceipt> {
        request.context.validate()?;
        let current = self
            .metadata
            .describe(&request.context.workspace_ref, &request.resource_ref)
            .await?;
        if !current.acl.can_write(&request.context.actor.principal_ref) {
            return Err(denied("resource write is not granted"));
        }
        if current.revision != request.expected_revision {
            return Err(conflict("resource revision changed"));
        }
        let object_ref = format!("blobs/sha256/{}", request.expected_digest.value);
        let storage = self
            .blobs
            .put(
                PutObjectRequest {
                    context: request.context.clone(),
                    object_ref: object_ref.clone(),
                    expected_digest: request.expected_digest.clone(),
                    size: request.size,
                    content_type: request.content_type.clone(),
                    metadata: BTreeMap::from([
                        (
                            "workspace-ref".to_owned(),
                            request.context.workspace_ref.clone(),
                        ),
                        ("resource-ref".to_owned(), request.resource_ref.clone()),
                    ]),
                    idempotency_key: format!("blob:{}", request.idempotency_key),
                },
                body,
            )
            .await?;
        let blob_ref = BlobRef {
            provider_ref: self.blobs.capabilities().provider_ref,
            object_ref,
            digest: request.expected_digest.clone(),
            size: request.size,
        };
        let metadata_request = MetadataCommitRequest {
            workspace_ref: request.context.workspace_ref.clone(),
            resource_ref: request.resource_ref.clone(),
            expected_revision: request.expected_revision.clone(),
            blob_ref: blob_ref.clone(),
            storage_receipt_ref: storage.receipt_ref.clone(),
            actor_ref: request.context.actor.principal_ref.clone(),
            idempotency_key: request.idempotency_key.clone(),
            fingerprint: format!(
                "{}|{}|{}|{}|{}|{}",
                request.context.workspace_ref,
                request.resource_ref,
                request.expected_revision,
                request.expected_digest.value,
                request.size,
                request.context.generation
            ),
        };
        match self.metadata.cas_commit(metadata_request).await {
            Ok(value) => Ok(value),
            Err(error) => {
                self.metadata
                    .record_orphan(OrphanBlob {
                        workspace_ref: request.context.workspace_ref,
                        resource_ref: request.resource_ref,
                        blob_ref,
                        storage_receipt_ref: storage.receipt_ref,
                        reason: format!("{:?}", error.code),
                    })
                    .await?;
                Err(error)
            }
        }
    }

    pub async fn materialize(
        &self,
        request: MaterializeRequest,
    ) -> AuthorityResult<MaterializationHandle> {
        self.metadata.issue_handle(request).await
    }

    pub async fn read_handle(
        &self,
        context: StorageRequestContext,
        handle_ref: &str,
        audience: &str,
        generation: u64,
        now_ms: u64,
        range: Option<ByteRange>,
    ) -> AuthorityResult<ReadObject> {
        let handle = self
            .metadata
            .resolve_handle(handle_ref, audience, generation, now_ms)
            .await?;
        if handle.workspace_ref != context.workspace_ref {
            return Err(denied("handle belongs to another workspace"));
        }
        self.blobs
            .read(ReadObjectRequest {
                context,
                object_ref: handle.blob_ref.object_ref,
                range,
            })
            .await
            .map_err(Into::into)
    }

    pub async fn dispatch_outbox(&self, limit: usize) -> AuthorityResult<usize> {
        let entries = self.metadata.pending_outbox(limit).await?;
        let mut published = 0;
        for entry in entries {
            self.metadata
                .record_outbox_attempt(&entry.event.event_ref)
                .await?;
            if self.publisher.publish(&entry.event).await.is_ok() {
                self.metadata.ack_outbox(&entry.event.event_ref).await?;
                published += 1;
            }
        }
        Ok(published)
    }
}

#[derive(Default)]
struct MetadataState {
    sequence: u64,
    resources: BTreeMap<(String, String), ResourceRecord>,
    receipts: HashMap<String, (String, CommitReceipt)>,
    handles: HashMap<String, MaterializationHandle>,
    outbox: BTreeMap<String, OutboxEntry>,
    orphans: Vec<OrphanBlob>,
    fail_next_commit: bool,
}

#[derive(Clone, Default)]
pub struct InMemoryMetadataStore {
    state: Arc<Mutex<MetadataState>>,
}

impl InMemoryMetadataStore {
    pub fn fail_next_commit(&self) -> AuthorityResult<()> {
        self.lock()?.fail_next_commit = true;
        Ok(())
    }

    fn lock(&self) -> AuthorityResult<std::sync::MutexGuard<'_, MetadataState>> {
        self.state.lock().map_err(|_| {
            AuthorityError::new(
                AuthorityErrorCode::MetadataUnavailable,
                "metadata state is poisoned",
                true,
            )
        })
    }
}

#[async_trait]
impl MetadataStorePort for InMemoryMetadataStore {
    async fn seed(&self, resource: ResourceRecord) -> AuthorityResult<()> {
        let mut state = self.lock()?;
        let key = (
            resource.workspace_ref.clone(),
            resource.resource_ref.clone(),
        );
        match state.resources.entry(key) {
            std::collections::btree_map::Entry::Vacant(entry) => {
                entry.insert(resource);
                Ok(())
            }
            std::collections::btree_map::Entry::Occupied(_) => {
                Err(conflict("resource already exists"))
            }
        }
    }

    async fn describe(
        &self,
        workspace_ref: &str,
        resource_ref: &str,
    ) -> AuthorityResult<ResourceRecord> {
        self.lock()?
            .resources
            .get(&(workspace_ref.to_owned(), resource_ref.to_owned()))
            .cloned()
            .ok_or_else(|| not_found("resource not found"))
    }

    async fn list(&self, query: &CatalogQuery) -> AuthorityResult<CatalogPage> {
        if query.limit == 0 || query.limit > 1000 {
            return Err(denied("catalog limit is invalid"));
        }
        let offset = query
            .cursor
            .as_deref()
            .map(str::parse::<usize>)
            .transpose()
            .map_err(|_| denied("catalog cursor is invalid"))?
            .unwrap_or(0);
        let search = query.search.as_ref().map(|value| value.to_lowercase());
        let mut resources = self
            .lock()?
            .resources
            .values()
            .filter(|item| item.workspace_ref == query.workspace_ref)
            .filter(|item| item.parent_ref == query.parent_ref)
            .filter(|item| item.acl.can_read(&query.principal_ref))
            .filter(|item| {
                search
                    .as_ref()
                    .is_none_or(|term| item.display_name.to_lowercase().contains(term))
            })
            .cloned()
            .collect::<Vec<_>>();
        resources.sort_by(|a, b| {
            a.display_name
                .cmp(&b.display_name)
                .then(a.resource_ref.cmp(&b.resource_ref))
        });
        let end = offset.saturating_add(query.limit).min(resources.len());
        let page = if offset >= resources.len() {
            Vec::new()
        } else {
            resources[offset..end].to_vec()
        };
        Ok(CatalogPage {
            resources: page,
            next_cursor: (end < resources.len()).then(|| end.to_string()),
        })
    }

    async fn cas_commit(&self, request: MetadataCommitRequest) -> AuthorityResult<CommitReceipt> {
        let mut state = self.lock()?;
        if let Some((fingerprint, receipt)) = state.receipts.get(&request.idempotency_key) {
            if fingerprint != &request.fingerprint {
                return Err(conflict("idempotency key was reused"));
            }
            let mut replay = receipt.clone();
            replay.replayed = true;
            return Ok(replay);
        }
        if state.fail_next_commit {
            state.fail_next_commit = false;
            return Err(AuthorityError::new(
                AuthorityErrorCode::MetadataUnavailable,
                "metadata transaction failed",
                true,
            ));
        }
        let key = (request.workspace_ref.clone(), request.resource_ref.clone());
        let resource = state
            .resources
            .get(&key)
            .ok_or_else(|| not_found("resource not found"))?;
        if !resource.acl.can_write(&request.actor_ref) {
            return Err(denied("resource write is not granted"));
        }
        if resource.revision != request.expected_revision {
            return Err(conflict("resource revision changed"));
        }
        let before_revision = resource.revision.clone();
        state.sequence += 1;
        let after_revision = format!("revision.{}", state.sequence);
        let event_ref = format!("event.{}", state.sequence);
        let metadata_receipt_ref = format!("metadata-receipt.{}", state.sequence);
        let resource = state.resources.get_mut(&key).expect("resource exists");
        resource.revision = after_revision.clone();
        resource.blob_ref = request.blob_ref.clone();
        let receipt = CommitReceipt {
            commit_ref: format!("commit.{}", state.sequence),
            workspace_ref: request.workspace_ref.clone(),
            resource_ref: request.resource_ref.clone(),
            before_revision: before_revision.clone(),
            after_revision: after_revision.clone(),
            blob_ref: request.blob_ref.clone(),
            storage_receipt_ref: request.storage_receipt_ref,
            metadata_receipt_ref,
            replayed: false,
        };
        state.outbox.insert(
            event_ref.clone(),
            OutboxEntry {
                event: ResourceChangedEvent {
                    event_ref,
                    workspace_ref: request.workspace_ref,
                    resource_ref: request.resource_ref,
                    before_revision,
                    after_revision,
                    blob_ref: request.blob_ref,
                },
                attempts: 0,
            },
        );
        state.receipts.insert(
            request.idempotency_key,
            (request.fingerprint, receipt.clone()),
        );
        Ok(receipt)
    }

    async fn record_orphan(&self, orphan: OrphanBlob) -> AuthorityResult<()> {
        let mut state = self.lock()?;
        if !state.orphans.iter().any(|item| {
            item.blob_ref == orphan.blob_ref
                && item.storage_receipt_ref == orphan.storage_receipt_ref
        }) {
            state.orphans.push(orphan);
        }
        Ok(())
    }

    async fn list_orphans(&self) -> AuthorityResult<Vec<OrphanBlob>> {
        Ok(self.lock()?.orphans.clone())
    }

    async fn issue_handle(
        &self,
        request: MaterializeRequest,
    ) -> AuthorityResult<MaterializationHandle> {
        if request.audience.is_empty() || request.generation == 0 || request.ttl_ms == 0 {
            return Err(denied("materialization grant is incomplete"));
        }
        let mut state = self.lock()?;
        let resource = state
            .resources
            .get(&(request.workspace_ref.clone(), request.resource_ref.clone()))
            .ok_or_else(|| not_found("resource not found"))?;
        if !resource.acl.can_read(&request.principal_ref) {
            return Err(denied("resource read is not granted"));
        }
        let revision = resource.revision.clone();
        let blob_ref = resource.blob_ref.clone();
        state.sequence += 1;
        let handle = MaterializationHandle {
            handle_ref: format!("handle.{}", state.sequence),
            workspace_ref: request.workspace_ref,
            resource_ref: request.resource_ref,
            revision,
            blob_ref,
            audience: request.audience,
            generation: request.generation,
            expires_at_ms: request.now_ms.saturating_add(request.ttl_ms),
        };
        state
            .handles
            .insert(handle.handle_ref.clone(), handle.clone());
        Ok(handle)
    }

    async fn resolve_handle(
        &self,
        handle_ref: &str,
        audience: &str,
        generation: u64,
        now_ms: u64,
    ) -> AuthorityResult<MaterializationHandle> {
        let handle = self
            .lock()?
            .handles
            .get(handle_ref)
            .cloned()
            .ok_or_else(|| not_found("materialization handle not found"))?;
        if handle.audience != audience {
            return Err(denied("materialization audience differs"));
        }
        if handle.generation != generation {
            return Err(AuthorityError::new(
                AuthorityErrorCode::StaleGeneration,
                "materialization generation is stale",
                false,
            ));
        }
        if now_ms >= handle.expires_at_ms {
            return Err(AuthorityError::new(
                AuthorityErrorCode::Expired,
                "materialization handle expired",
                false,
            ));
        }
        Ok(handle)
    }

    async fn pending_outbox(&self, limit: usize) -> AuthorityResult<Vec<OutboxEntry>> {
        Ok(self.lock()?.outbox.values().take(limit).cloned().collect())
    }

    async fn record_outbox_attempt(&self, event_ref: &str) -> AuthorityResult<()> {
        let mut state = self.lock()?;
        let entry = state
            .outbox
            .get_mut(event_ref)
            .ok_or_else(|| not_found("outbox event not found"))?;
        entry.attempts = entry.attempts.saturating_add(1);
        Ok(())
    }

    async fn ack_outbox(&self, event_ref: &str) -> AuthorityResult<()> {
        if self.lock()?.outbox.remove(event_ref).is_none() {
            return Err(not_found("outbox event not found"));
        }
        Ok(())
    }
}

fn denied(message: &'static str) -> AuthorityError {
    AuthorityError::new(AuthorityErrorCode::Denied, message, false)
}

fn conflict(message: &'static str) -> AuthorityError {
    AuthorityError::new(AuthorityErrorCode::Conflict, message, false)
}

fn not_found(message: &'static str) -> AuthorityError {
    AuthorityError::new(AuthorityErrorCode::NotFound, message, false)
}
