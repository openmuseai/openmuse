//! Provider-neutral immutable blob storage contract.
//!
//! S3 SDK clients, byte streams, credentials, endpoints and vendor errors are
//! deliberately excluded. Workspace trees and revisions belong to Resource
//! Authority; this port stores opaque immutable bytes only.

use async_trait::async_trait;
use openmuse_contract::PrincipalRef;
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;

pub const PROFILE_NAME: &str = "openmuse.s3-data-plane";
pub const PROFILE_MAJOR: u16 = 1;
pub const MAX_STREAM_CHUNK_BYTES: usize = 1024 * 1024;
pub const MIN_MULTIPART_PART_BYTES: u64 = 5 * 1024 * 1024;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct StorageRequestContext {
    pub request_id: String,
    pub actor: PrincipalRef,
    pub caller: PrincipalRef,
    pub policy_decision_ref: String,
    pub workspace_ref: String,
    pub generation: u64,
    pub deadline_at_ms: u64,
    pub cancellation_ref: String,
}

impl StorageRequestContext {
    pub fn validate(&self) -> StorageResult<()> {
        for value in [
            self.request_id.as_str(),
            self.actor.principal_ref.as_str(),
            self.caller.principal_ref.as_str(),
            self.policy_decision_ref.as_str(),
            self.workspace_ref.as_str(),
            self.cancellation_ref.as_str(),
        ] {
            if value.is_empty() {
                return Err(StorageError::invalid(
                    "storage request context is incomplete",
                ));
            }
        }
        if self.generation == 0 || self.deadline_at_ms == 0 {
            return Err(StorageError::invalid(
                "storage request context has an invalid generation or deadline",
            ));
        }
        Ok(())
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum DigestAlgorithm {
    Sha256,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct BlobDigest {
    pub algorithm: DigestAlgorithm,
    pub value: String,
}

impl BlobDigest {
    pub fn sha256(value: impl Into<String>) -> StorageResult<Self> {
        let digest = Self {
            algorithm: DigestAlgorithm::Sha256,
            value: value.into(),
        };
        digest.validate()?;
        Ok(digest)
    }

    pub fn validate(&self) -> StorageResult<()> {
        if self.value.len() != 64
            || !self
                .value
                .bytes()
                .all(|value| value.is_ascii_digit() || (b'a'..=b'f').contains(&value))
        {
            return Err(StorageError::invalid(
                "blob digest must be lowercase SHA-256",
            ));
        }
        Ok(())
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct BlobRef {
    pub provider_ref: String,
    pub object_ref: String,
    pub digest: BlobDigest,
    pub size: u64,
}

/// Provider response validators are opaque and are never interpreted as a
/// digest. In particular, S3 ETag is not assumed to be MD5.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct ProviderValidator {
    pub value: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct BlobDescriptor {
    pub blob_ref: BlobRef,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub provider_validator: Option<ProviderValidator>,
    pub content_type: String,
    pub metadata: BTreeMap<String, String>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum StorageOperation {
    Put,
    Head,
    Read,
    Delete,
    BeginMultipart,
    UploadPart,
    CompleteMultipart,
    AbortMultipart,
    ListForMaintenance,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct StorageReceipt {
    pub receipt_ref: String,
    pub request_id: String,
    pub operation: StorageOperation,
    pub object_ref: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub digest: Option<BlobDigest>,
    pub size: u64,
    pub replayed: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct PutObjectRequest {
    pub context: StorageRequestContext,
    pub object_ref: String,
    pub expected_digest: BlobDigest,
    pub size: u64,
    pub content_type: String,
    pub metadata: BTreeMap<String, String>,
    pub idempotency_key: String,
}

impl PutObjectRequest {
    pub fn validate(&self) -> StorageResult<()> {
        self.context.validate()?;
        self.expected_digest.validate()?;
        if self.object_ref.is_empty()
            || self.content_type.is_empty()
            || self.idempotency_key.is_empty()
            || self
                .metadata
                .iter()
                .any(|(key, value)| key.is_empty() || value.is_empty())
        {
            return Err(StorageError::invalid("put request is incomplete"));
        }
        Ok(())
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct ObjectRequest {
    pub context: StorageRequestContext,
    pub object_ref: String,
}

impl ObjectRequest {
    pub fn validate(&self) -> StorageResult<()> {
        self.context.validate()?;
        require_ref(&self.object_ref, "object ref")
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct ByteRange {
    pub offset: u64,
    pub length: u64,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct ReadObjectRequest {
    pub context: StorageRequestContext,
    pub object_ref: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub range: Option<ByteRange>,
}

impl ReadObjectRequest {
    pub fn validate(&self) -> StorageResult<()> {
        self.context.validate()?;
        require_ref(&self.object_ref, "object ref")?;
        if self.range.is_some_and(|range| range.length == 0) {
            return Err(StorageError::invalid("range length must be positive"));
        }
        Ok(())
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct MultipartUpload {
    pub upload_ref: String,
    pub object_ref: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct UploadPartRequest {
    pub context: StorageRequestContext,
    pub upload_ref: String,
    pub part_number: u32,
    pub expected_digest: BlobDigest,
    pub size: u64,
}

impl UploadPartRequest {
    pub fn validate(&self) -> StorageResult<()> {
        self.context.validate()?;
        require_ref(&self.upload_ref, "upload ref")?;
        self.expected_digest.validate()?;
        if self.part_number == 0 || self.part_number > 10_000 {
            return Err(StorageError::invalid(
                "multipart part number is out of range",
            ));
        }
        Ok(())
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct UploadedPart {
    pub part_number: u32,
    pub digest: BlobDigest,
    pub size: u64,
    pub provider_part_ref: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct CompleteMultipartRequest {
    pub context: StorageRequestContext,
    pub upload_ref: String,
    pub parts: Vec<UploadedPart>,
}

impl CompleteMultipartRequest {
    pub fn validate(&self) -> StorageResult<()> {
        self.context.validate()?;
        require_ref(&self.upload_ref, "upload ref")?;
        if self.parts.is_empty() {
            return Err(StorageError::invalid("multipart completion has no parts"));
        }
        Ok(())
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct AbortMultipartRequest {
    pub context: StorageRequestContext,
    pub upload_ref: String,
}

impl AbortMultipartRequest {
    pub fn validate(&self) -> StorageResult<()> {
        self.context.validate()?;
        require_ref(&self.upload_ref, "upload ref")
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct ListForMaintenanceRequest {
    pub context: StorageRequestContext,
    pub prefix: String,
    pub cursor: Option<String>,
    pub limit: u32,
}

impl ListForMaintenanceRequest {
    pub fn validate(&self) -> StorageResult<()> {
        self.context.validate()?;
        if self.limit == 0 || self.limit > 1000 {
            return Err(StorageError::invalid("maintenance list limit is invalid"));
        }
        Ok(())
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct MaintenancePage {
    pub objects: Vec<BlobDescriptor>,
    pub next_cursor: Option<String>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ProviderKind {
    Fake,
    AwsS3,
    Minio,
    Rustfs,
    OtherS3,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum AddressingStyle {
    Path,
    VirtualHost,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct ProviderCapabilitySnapshot {
    pub profile: String,
    pub profile_major: u16,
    pub provider_ref: String,
    pub provider_kind: ProviderKind,
    pub provider_version: String,
    pub tls: bool,
    pub addressing_styles: Vec<AddressingStyle>,
    pub checksum_sha256: bool,
    pub range_read: bool,
    pub multipart: bool,
    pub maintenance_list: bool,
    pub minimum_part_size: u64,
}

impl ProviderCapabilitySnapshot {
    pub fn validate_profile_v1(&self) -> StorageResult<()> {
        if self.profile != PROFILE_NAME || self.profile_major != PROFILE_MAJOR {
            return Err(StorageError::invalid("unsupported storage profile"));
        }
        if self.provider_ref.is_empty() || self.provider_version.is_empty() {
            return Err(StorageError::invalid("provider identity is incomplete"));
        }
        if !self.checksum_sha256 || !self.range_read || !self.multipart {
            return Err(StorageError::unsupported(
                "provider lacks a required S3 profile capability",
            ));
        }
        if self.minimum_part_size > MIN_MULTIPART_PART_BYTES {
            return Err(StorageError::unsupported(
                "provider multipart minimum exceeds the profile",
            ));
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum StorageErrorCode {
    Denied,
    NotFound,
    Conflict,
    Expired,
    StaleGeneration,
    Unavailable,
    Transient,
    IntegrityFailed,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, thiserror::Error)]
#[error("{code:?}: {message}")]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct StorageError {
    pub code: StorageErrorCode,
    pub message: String,
    pub retryable: bool,
}

impl StorageError {
    pub fn new(code: StorageErrorCode, message: impl Into<String>, retryable: bool) -> Self {
        Self {
            code,
            message: message.into(),
            retryable,
        }
    }

    pub fn invalid(message: impl Into<String>) -> Self {
        Self::new(StorageErrorCode::Denied, message, false)
    }

    pub fn unsupported(message: impl Into<String>) -> Self {
        Self::new(StorageErrorCode::Unavailable, message, false)
    }

    pub fn integrity(message: impl Into<String>) -> Self {
        Self::new(StorageErrorCode::IntegrityFailed, message, false)
    }
}

pub type StorageResult<T> = Result<T, StorageError>;

#[async_trait]
pub trait BlobReadStream: Send {
    /// Returns no more than `max_bytes`. Consumers must keep this bounded and
    /// must not collect a whole object inside an adapter.
    async fn next_chunk(&mut self, max_bytes: usize) -> StorageResult<Option<Vec<u8>>>;
}

pub struct ReadObject {
    pub descriptor: BlobDescriptor,
    pub body: Box<dyn BlobReadStream>,
}

#[async_trait]
pub trait BlobStorePort: Send + Sync {
    fn capabilities(&self) -> ProviderCapabilitySnapshot;

    async fn put(
        &self,
        request: PutObjectRequest,
        body: &mut dyn BlobReadStream,
    ) -> StorageResult<StorageReceipt>;

    async fn head(&self, request: ObjectRequest) -> StorageResult<BlobDescriptor>;

    async fn read(&self, request: ReadObjectRequest) -> StorageResult<ReadObject>;

    async fn delete(&self, request: ObjectRequest) -> StorageResult<StorageReceipt>;

    async fn begin_multipart(&self, request: PutObjectRequest) -> StorageResult<MultipartUpload>;

    async fn upload_part(
        &self,
        request: UploadPartRequest,
        body: &mut dyn BlobReadStream,
    ) -> StorageResult<UploadedPart>;

    async fn complete_multipart(
        &self,
        request: CompleteMultipartRequest,
    ) -> StorageResult<StorageReceipt>;

    async fn abort_multipart(
        &self,
        request: AbortMultipartRequest,
    ) -> StorageResult<StorageReceipt>;
}

/// This port is intentionally separate: Workspace UI and Resource Authority
/// cannot depend on object listing to construct the user-visible tree.
#[async_trait]
pub trait BlobMaintenancePort: Send + Sync {
    async fn list_for_maintenance(
        &self,
        request: ListForMaintenanceRequest,
    ) -> StorageResult<MaintenancePage>;
}

fn require_ref(value: &str, field: &'static str) -> StorageResult<()> {
    if value.is_empty() {
        return Err(StorageError::invalid(format!("{field} is empty")));
    }
    Ok(())
}
