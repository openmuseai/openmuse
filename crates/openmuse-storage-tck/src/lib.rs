//! Reusable black-box TCK and deterministic in-memory reference provider.

use async_trait::async_trait;
use openmuse_contract::{PrincipalKind, PrincipalRef};
use openmuse_storage_contract::{
    AbortMultipartRequest, AddressingStyle, BlobDescriptor, BlobDigest, BlobMaintenancePort,
    BlobReadStream, BlobRef, BlobStorePort, ByteRange, CompleteMultipartRequest, DigestAlgorithm,
    ListForMaintenanceRequest, MAX_STREAM_CHUNK_BYTES, MIN_MULTIPART_PART_BYTES, MaintenancePage,
    MultipartUpload, ObjectRequest, PROFILE_MAJOR, PROFILE_NAME, ProviderCapabilitySnapshot,
    ProviderKind, ProviderValidator, PutObjectRequest, ReadObject, ReadObjectRequest, StorageError,
    StorageErrorCode, StorageOperation, StorageReceipt, StorageRequestContext, StorageResult,
    UploadPartRequest, UploadedPart,
};
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use std::collections::{BTreeMap, HashMap};
use std::sync::{Arc, Mutex};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct TckConfig {
    pub run_ref: String,
    pub workspace_ref: String,
    pub generation: u64,
    pub deadline_at_ms: u64,
    pub run_large_object: bool,
}

impl TckConfig {
    fn context(&self, request_id: impl Into<String>) -> StorageRequestContext {
        StorageRequestContext {
            request_id: request_id.into(),
            actor: PrincipalRef {
                principal_ref: "user.storage-tck".to_owned(),
                kind: PrincipalKind::User,
            },
            caller: PrincipalRef {
                principal_ref: "service.resource-authority".to_owned(),
                kind: PrincipalKind::Service,
            },
            policy_decision_ref: format!("decision.{}", self.run_ref),
            workspace_ref: self.workspace_ref.clone(),
            generation: self.generation,
            deadline_at_ms: self.deadline_at_ms,
            cancellation_ref: format!("cancel.{}", self.run_ref),
        }
    }

    fn key(&self, suffix: &str) -> String {
        format!("tck/{}/{}", self.run_ref, suffix)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct TckReport {
    pub capability_snapshot: ProviderCapabilitySnapshot,
    pub passed: Vec<String>,
    pub failures: Vec<TckFailure>,
}

impl TckReport {
    pub fn certified(&self) -> bool {
        self.failures.is_empty()
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct TckFailure {
    pub scenario: String,
    pub code: StorageErrorCode,
    pub message: String,
}

pub async fn run_provider_tck(
    provider: &dyn BlobStorePort,
    maintenance: &dyn BlobMaintenancePort,
    config: &TckConfig,
) -> TckReport {
    let capabilities = provider.capabilities();
    let mut report = TckReport {
        capability_snapshot: capabilities.clone(),
        passed: Vec::new(),
        failures: Vec::new(),
    };

    record(
        &mut report,
        "profile-capabilities",
        validate_capabilities(&capabilities),
    );
    record(
        &mut report,
        "put-head-get-range-delete",
        object_lifecycle(provider, config).await,
    );
    record(
        &mut report,
        "zero-byte-object",
        zero_byte(provider, config).await,
    );
    record(
        &mut report,
        "five-mib-boundary",
        five_mib_boundary(provider, config).await,
    );
    record(
        &mut report,
        "interrupted-upload-retry",
        interrupted_upload_retry(provider, config).await,
    );
    record(
        &mut report,
        "checksum-failure",
        checksum_failure(provider, config).await,
    );
    record(
        &mut report,
        "multipart-abort-retry",
        multipart_abort_retry(provider, config).await,
    );
    record(
        &mut report,
        "maintenance-list",
        maintenance_list(maintenance, config).await,
    );
    if config.run_large_object {
        record(
            &mut report,
            "multipart-100-mib-plus",
            multipart_large(provider, config).await,
        );
    }
    report
}

fn record(report: &mut TckReport, scenario: &str, result: StorageResult<()>) {
    match result {
        Ok(()) => report.passed.push(scenario.to_owned()),
        Err(error) => report.failures.push(TckFailure {
            scenario: scenario.to_owned(),
            code: error.code,
            message: error.message,
        }),
    }
}

fn validate_capabilities(capabilities: &ProviderCapabilitySnapshot) -> StorageResult<()> {
    capabilities.validate_profile_v1()?;
    if !capabilities.tls {
        return Err(StorageError::unsupported(
            "TLS is required by the S3 profile",
        ));
    }
    if capabilities.addressing_styles.is_empty() {
        return Err(StorageError::unsupported("an addressing style is required"));
    }
    if !capabilities.maintenance_list {
        return Err(StorageError::unsupported("maintenance listing is required"));
    }
    Ok(())
}

async fn object_lifecycle(provider: &dyn BlobStorePort, config: &TckConfig) -> StorageResult<()> {
    let bytes = b"OpenMuse storage TCK".to_vec();
    let key = config.key("folder/space and \u{7a7a}/%23.bin");
    let request = put_request(config, "object.put", &key, &bytes, "idem.object");
    let mut body = VecBlobStream::new(bytes.clone(), 3);
    let receipt = provider.put(request.clone(), &mut body).await?;
    if receipt.digest.as_ref() != Some(&digest(&bytes)) || receipt.size != bytes.len() as u64 {
        return Err(StorageError::integrity(
            "put receipt differs from uploaded bytes",
        ));
    }

    let descriptor = provider
        .head(ObjectRequest {
            context: config.context("object.head"),
            object_ref: key.clone(),
        })
        .await?;
    if descriptor.blob_ref.digest != digest(&bytes)
        || descriptor
            .provider_validator
            .as_ref()
            .map(|item| item.value.as_str())
            == Some(descriptor.blob_ref.digest.value.as_str())
    {
        return Err(StorageError::integrity(
            "HEAD digest is wrong or provider validator was treated as content hash",
        ));
    }
    if descriptor
        .metadata
        .get("x-openmuse-test")
        .map(String::as_str)
        != Some("roundtrip")
        || descriptor.content_type != "application/octet-stream"
    {
        return Err(StorageError::integrity(
            "content type or metadata did not round trip",
        ));
    }

    let mut replay_body = VecBlobStream::new(bytes.clone(), 5);
    let replayed = provider.put(request.clone(), &mut replay_body).await?;
    if !replayed.replayed {
        return Err(StorageError::integrity(
            "idempotent immutable PUT was not reported as replayed",
        ));
    }
    let conflicting_bytes = b"different immutable content".to_vec();
    let mut conflicting_request = request;
    conflicting_request.context = config.context("object.put.conflict");
    conflicting_request.expected_digest = digest(&conflicting_bytes);
    conflicting_request.size = conflicting_bytes.len() as u64;
    let mut conflicting_body = VecBlobStream::new(conflicting_bytes, 4);
    match provider
        .put(conflicting_request, &mut conflicting_body)
        .await
    {
        Err(error) if error.code == StorageErrorCode::Conflict => {}
        _ => {
            return Err(StorageError::integrity(
                "idempotency key accepted different immutable content",
            ));
        }
    }

    let read = provider
        .read(ReadObjectRequest {
            context: config.context("object.read"),
            object_ref: key.clone(),
            range: None,
        })
        .await?;
    if collect(read.body).await? != bytes {
        return Err(StorageError::integrity("GET bytes differ"));
    }
    let range = provider
        .read(ReadObjectRequest {
            context: config.context("object.range"),
            object_ref: key.clone(),
            range: Some(ByteRange {
                offset: 4,
                length: 7,
            }),
        })
        .await?;
    if collect(range.body).await? != b"Muse st" {
        return Err(StorageError::integrity("range GET bytes differ"));
    }
    provider
        .delete(ObjectRequest {
            context: config.context("object.delete"),
            object_ref: key.clone(),
        })
        .await?;
    match provider
        .head(ObjectRequest {
            context: config.context("object.deleted.head"),
            object_ref: key,
        })
        .await
    {
        Err(error) if error.code == StorageErrorCode::NotFound => Ok(()),
        _ => Err(StorageError::integrity("deleted object remains visible")),
    }
}

async fn zero_byte(provider: &dyn BlobStorePort, config: &TckConfig) -> StorageResult<()> {
    let bytes = Vec::new();
    let key = config.key("zero.bin");
    let mut body = VecBlobStream::new(bytes.clone(), 1);
    provider
        .put(
            put_request(config, "zero.put", &key, &bytes, "idem.zero"),
            &mut body,
        )
        .await?;
    let read = provider
        .read(ReadObjectRequest {
            context: config.context("zero.read"),
            object_ref: key,
            range: None,
        })
        .await?;
    if collect(read.body).await?.is_empty() {
        Ok(())
    } else {
        Err(StorageError::integrity("zero-byte object is not empty"))
    }
}

async fn five_mib_boundary(provider: &dyn BlobStorePort, config: &TckConfig) -> StorageResult<()> {
    let bytes = vec![0x5a; MIN_MULTIPART_PART_BYTES as usize];
    let key = config.key("five-mib.bin");
    let mut body = VecBlobStream::new(bytes.clone(), 256 * 1024);
    provider
        .put(
            put_request(config, "five.put", &key, &bytes, "idem.five"),
            &mut body,
        )
        .await?;
    let descriptor = provider
        .head(ObjectRequest {
            context: config.context("five.head"),
            object_ref: key,
        })
        .await?;
    if descriptor.blob_ref.size == MIN_MULTIPART_PART_BYTES {
        Ok(())
    } else {
        Err(StorageError::integrity("5 MiB boundary size differs"))
    }
}

async fn interrupted_upload_retry(
    provider: &dyn BlobStorePort,
    config: &TckConfig,
) -> StorageResult<()> {
    let bytes = vec![0x23; 2 * MAX_STREAM_CHUNK_BYTES + 31];
    let key = config.key("retry.bin");
    let request = put_request(config, "retry.put", &key, &bytes, "idem.retry");
    let mut failing = FailingBlobStream::new(bytes.clone(), MAX_STREAM_CHUNK_BYTES);
    match provider.put(request.clone(), &mut failing).await {
        Err(error) if error.code == StorageErrorCode::Transient => {}
        _ => {
            return Err(StorageError::integrity(
                "interrupted upload did not fail transiently",
            ));
        }
    }
    match provider
        .head(ObjectRequest {
            context: config.context("retry.absent"),
            object_ref: key.clone(),
        })
        .await
    {
        Err(error) if error.code == StorageErrorCode::NotFound => {}
        _ => return Err(StorageError::integrity("partial upload became visible")),
    }
    let mut retry = VecBlobStream::new(bytes, 128 * 1024);
    provider.put(request, &mut retry).await?;
    Ok(())
}

async fn checksum_failure(provider: &dyn BlobStorePort, config: &TckConfig) -> StorageResult<()> {
    let bytes = b"actual".to_vec();
    let mut request = put_request(
        config,
        "checksum.put",
        &config.key("checksum.bin"),
        &bytes,
        "idem.checksum",
    );
    request.expected_digest = digest(b"different");
    let mut body = VecBlobStream::new(bytes, 2);
    match provider.put(request, &mut body).await {
        Err(error) if error.code == StorageErrorCode::IntegrityFailed => Ok(()),
        _ => Err(StorageError::integrity(
            "checksum mismatch did not fail closed",
        )),
    }
}

async fn multipart_abort_retry(
    provider: &dyn BlobStorePort,
    config: &TckConfig,
) -> StorageResult<()> {
    let first = vec![0x11; MIN_MULTIPART_PART_BYTES as usize];
    let last = b"last-part".to_vec();
    let mut full = first.clone();
    full.extend_from_slice(&last);
    let key = config.key("multipart-retry.bin");
    let request = put_request(config, "multipart.begin", &key, &full, "idem.multipart");
    let abandoned = provider.begin_multipart(request.clone()).await?;
    provider
        .abort_multipart(AbortMultipartRequest {
            context: config.context("multipart.abort"),
            upload_ref: abandoned.upload_ref,
        })
        .await?;

    let upload = provider.begin_multipart(request).await?;
    let part_one_request = UploadPartRequest {
        context: config.context("multipart.part.1"),
        upload_ref: upload.upload_ref.clone(),
        part_number: 1,
        expected_digest: digest(&first),
        size: first.len() as u64,
    };
    let mut part_one = VecBlobStream::new(first.clone(), 512 * 1024);
    let uploaded_one = provider
        .upload_part(part_one_request.clone(), &mut part_one)
        .await?;
    let mut repeated = VecBlobStream::new(first, 333 * 1024);
    let repeated_one = provider
        .upload_part(part_one_request, &mut repeated)
        .await?;
    if uploaded_one != repeated_one {
        return Err(StorageError::integrity(
            "multipart part retry changed its receipt",
        ));
    }
    let mut part_two = VecBlobStream::new(last.clone(), 3);
    let uploaded_two = provider
        .upload_part(
            UploadPartRequest {
                context: config.context("multipart.part.2"),
                upload_ref: upload.upload_ref.clone(),
                part_number: 2,
                expected_digest: digest(&last),
                size: last.len() as u64,
            },
            &mut part_two,
        )
        .await?;
    provider
        .complete_multipart(CompleteMultipartRequest {
            context: config.context("multipart.complete"),
            upload_ref: upload.upload_ref.clone(),
            parts: vec![uploaded_two.clone(), uploaded_one.clone()],
        })
        .await?;
    let replayed = provider
        .complete_multipart(CompleteMultipartRequest {
            context: config.context("multipart.complete.retry"),
            upload_ref: upload.upload_ref,
            parts: vec![uploaded_one, uploaded_two],
        })
        .await?;
    if !replayed.replayed {
        return Err(StorageError::integrity(
            "multipart completion retry was not marked as replayed",
        ));
    }
    Ok(())
}

async fn multipart_large(provider: &dyn BlobStorePort, config: &TckConfig) -> StorageResult<()> {
    const PART_COUNT: usize = 20;
    let part_size = MIN_MULTIPART_PART_BYTES as usize;
    let last = vec![0xa5; 1];
    let mut hasher = Sha256::new();
    for index in 0..PART_COUNT {
        hasher.update(vec![index as u8; part_size]);
    }
    hasher.update(&last);
    let expected = BlobDigest {
        algorithm: DigestAlgorithm::Sha256,
        value: format!("{:x}", hasher.finalize()),
    };
    let key = config.key("large-100-mib-plus.bin");
    let upload = provider
        .begin_multipart(PutObjectRequest {
            context: config.context("large.begin"),
            object_ref: key.clone(),
            expected_digest: expected.clone(),
            size: (PART_COUNT * part_size + 1) as u64,
            content_type: "application/octet-stream".to_owned(),
            metadata: BTreeMap::new(),
            idempotency_key: format!("{}.large", config.run_ref),
        })
        .await?;
    let mut parts = Vec::new();
    for index in 0..PART_COUNT {
        let bytes = vec![index as u8; part_size];
        let mut body = VecBlobStream::new(bytes.clone(), MAX_STREAM_CHUNK_BYTES);
        parts.push(
            provider
                .upload_part(
                    UploadPartRequest {
                        context: config.context(format!("large.part.{}", index + 1)),
                        upload_ref: upload.upload_ref.clone(),
                        part_number: (index + 1) as u32,
                        expected_digest: digest(&bytes),
                        size: part_size as u64,
                    },
                    &mut body,
                )
                .await?,
        );
    }
    let mut last_body = VecBlobStream::new(last.clone(), 1);
    parts.push(
        provider
            .upload_part(
                UploadPartRequest {
                    context: config.context("large.part.last"),
                    upload_ref: upload.upload_ref.clone(),
                    part_number: (PART_COUNT + 1) as u32,
                    expected_digest: digest(&last),
                    size: 1,
                },
                &mut last_body,
            )
            .await?,
    );
    provider
        .complete_multipart(CompleteMultipartRequest {
            context: config.context("large.complete"),
            upload_ref: upload.upload_ref,
            parts,
        })
        .await?;
    let descriptor = provider
        .head(ObjectRequest {
            context: config.context("large.head"),
            object_ref: key,
        })
        .await?;
    if descriptor.blob_ref.digest == expected && descriptor.blob_ref.size > 100 * 1024 * 1024 {
        Ok(())
    } else {
        Err(StorageError::integrity("100 MiB+ multipart result differs"))
    }
}

async fn maintenance_list(
    maintenance: &dyn BlobMaintenancePort,
    config: &TckConfig,
) -> StorageResult<()> {
    let prefix = format!("tck/{}/", config.run_ref);
    let first = maintenance
        .list_for_maintenance(ListForMaintenanceRequest {
            context: config.context("maintenance.list.1"),
            prefix: prefix.clone(),
            cursor: None,
            limit: 2,
        })
        .await?;
    if first.objects.is_empty()
        || first.objects.len() > 2
        || first
            .objects
            .iter()
            .any(|item| !item.blob_ref.object_ref.starts_with(&prefix))
    {
        return Err(StorageError::integrity(
            "maintenance page is empty, unscoped, or exceeded its limit",
        ));
    }
    if let Some(cursor) = first.next_cursor {
        maintenance
            .list_for_maintenance(ListForMaintenanceRequest {
                context: config.context("maintenance.list.2"),
                prefix,
                cursor: Some(cursor),
                limit: 2,
            })
            .await?;
    }
    Ok(())
}

fn put_request(
    config: &TckConfig,
    request_id: &str,
    object_ref: &str,
    bytes: &[u8],
    idempotency_key: &str,
) -> PutObjectRequest {
    PutObjectRequest {
        context: config.context(request_id),
        object_ref: object_ref.to_owned(),
        expected_digest: digest(bytes),
        size: bytes.len() as u64,
        content_type: "application/octet-stream".to_owned(),
        metadata: BTreeMap::from([("x-openmuse-test".to_owned(), "roundtrip".to_owned())]),
        idempotency_key: format!("{}.{}", config.run_ref, idempotency_key),
    }
}

pub fn digest(bytes: &[u8]) -> BlobDigest {
    BlobDigest {
        algorithm: DigestAlgorithm::Sha256,
        value: format!("{:x}", Sha256::digest(bytes)),
    }
}

async fn collect(mut stream: Box<dyn BlobReadStream>) -> StorageResult<Vec<u8>> {
    let mut bytes = Vec::new();
    while let Some(chunk) = stream.next_chunk(MAX_STREAM_CHUNK_BYTES).await? {
        if chunk.is_empty() || chunk.len() > MAX_STREAM_CHUNK_BYTES {
            return Err(StorageError::integrity(
                "provider emitted an empty or oversized stream chunk",
            ));
        }
        bytes.extend_from_slice(&chunk);
    }
    Ok(bytes)
}

pub struct VecBlobStream {
    bytes: Vec<u8>,
    offset: usize,
    preferred_chunk_size: usize,
}

impl VecBlobStream {
    pub fn new(bytes: Vec<u8>, preferred_chunk_size: usize) -> Self {
        Self {
            bytes,
            offset: 0,
            preferred_chunk_size: preferred_chunk_size.max(1),
        }
    }
}

#[async_trait]
impl BlobReadStream for VecBlobStream {
    async fn next_chunk(&mut self, max_bytes: usize) -> StorageResult<Option<Vec<u8>>> {
        if max_bytes == 0 || max_bytes > MAX_STREAM_CHUNK_BYTES {
            return Err(StorageError::invalid("stream chunk bound is invalid"));
        }
        if self.offset == self.bytes.len() {
            return Ok(None);
        }
        let end = self
            .offset
            .saturating_add(self.preferred_chunk_size.min(max_bytes))
            .min(self.bytes.len());
        let chunk = self.bytes[self.offset..end].to_vec();
        self.offset = end;
        Ok(Some(chunk))
    }
}

struct FailingBlobStream {
    inner: VecBlobStream,
    fail_after: usize,
    emitted: usize,
}

impl FailingBlobStream {
    fn new(bytes: Vec<u8>, fail_after: usize) -> Self {
        Self {
            inner: VecBlobStream::new(bytes, 64 * 1024),
            fail_after,
            emitted: 0,
        }
    }
}

#[async_trait]
impl BlobReadStream for FailingBlobStream {
    async fn next_chunk(&mut self, max_bytes: usize) -> StorageResult<Option<Vec<u8>>> {
        if self.emitted >= self.fail_after {
            return Err(StorageError::new(
                StorageErrorCode::Transient,
                "fixture upload interrupted",
                true,
            ));
        }
        let chunk = self.inner.next_chunk(max_bytes).await?;
        self.emitted += chunk.as_ref().map_or(0, Vec::len);
        Ok(chunk)
    }
}

#[derive(Clone)]
pub struct InMemoryBlobStore {
    provider_ref: String,
    state: Arc<Mutex<State>>,
}

#[derive(Default)]
struct State {
    sequence: u64,
    objects: BTreeMap<String, StoredObject>,
    uploads: HashMap<String, PendingUpload>,
    upload_by_idempotency: HashMap<String, String>,
    completed_uploads: HashMap<String, CompletedUpload>,
    put_receipts: HashMap<String, PutReplay>,
}

struct StoredObject {
    descriptor: BlobDescriptor,
    bytes: Vec<u8>,
}

struct PendingUpload {
    request: PutObjectRequest,
    parts: BTreeMap<u32, StoredPart>,
}

struct StoredPart {
    uploaded: UploadedPart,
    bytes: Vec<u8>,
}

struct CompletedUpload {
    parts: Vec<UploadedPart>,
    receipt: StorageReceipt,
}

struct PutReplay {
    object_ref: String,
    digest: BlobDigest,
    size: u64,
    receipt: StorageReceipt,
}

impl InMemoryBlobStore {
    pub fn new(provider_ref: impl Into<String>) -> Self {
        Self {
            provider_ref: provider_ref.into(),
            state: Arc::new(Mutex::new(State::default())),
        }
    }

    fn lock(&self) -> StorageResult<std::sync::MutexGuard<'_, State>> {
        self.state.lock().map_err(|_| {
            StorageError::new(
                StorageErrorCode::Unavailable,
                "storage state is poisoned",
                true,
            )
        })
    }

    fn descriptor(
        &self,
        request: &PutObjectRequest,
        bytes: &[u8],
        validator: String,
    ) -> BlobDescriptor {
        BlobDescriptor {
            blob_ref: BlobRef {
                provider_ref: self.provider_ref.clone(),
                object_ref: request.object_ref.clone(),
                digest: digest(bytes),
                size: bytes.len() as u64,
            },
            provider_validator: Some(ProviderValidator { value: validator }),
            content_type: request.content_type.clone(),
            metadata: request.metadata.clone(),
        }
    }
}

#[async_trait]
impl BlobStorePort for InMemoryBlobStore {
    fn capabilities(&self) -> ProviderCapabilitySnapshot {
        ProviderCapabilitySnapshot {
            profile: PROFILE_NAME.to_owned(),
            profile_major: PROFILE_MAJOR,
            provider_ref: self.provider_ref.clone(),
            provider_kind: ProviderKind::Fake,
            provider_version: "1".to_owned(),
            tls: true,
            addressing_styles: vec![AddressingStyle::Path, AddressingStyle::VirtualHost],
            checksum_sha256: true,
            range_read: true,
            multipart: true,
            maintenance_list: true,
            minimum_part_size: MIN_MULTIPART_PART_BYTES,
        }
    }

    async fn put(
        &self,
        request: PutObjectRequest,
        body: &mut dyn BlobReadStream,
    ) -> StorageResult<StorageReceipt> {
        request.validate()?;
        let bytes = read_exact_body(body, request.size, &request.expected_digest).await?;
        let mut state = self.lock()?;
        if let Some(replay) = state.put_receipts.get(&request.idempotency_key) {
            if replay.object_ref != request.object_ref
                || replay.digest != request.expected_digest
                || replay.size != request.size
            {
                return Err(StorageError::new(
                    StorageErrorCode::Conflict,
                    "idempotency key was reused for a different immutable PUT",
                    false,
                ));
            }
            let mut receipt = replay.receipt.clone();
            receipt.request_id = request.context.request_id;
            receipt.replayed = true;
            return Ok(receipt);
        }
        if let Some(existing) = state.objects.get(&request.object_ref) {
            if existing.descriptor.blob_ref.digest != request.expected_digest
                || existing.descriptor.blob_ref.size != request.size
            {
                return Err(StorageError::new(
                    StorageErrorCode::Conflict,
                    "immutable object already exists with different content",
                    false,
                ));
            }
            let result = receipt(
                &request.context,
                StorageOperation::Put,
                &existing.descriptor.blob_ref,
                true,
                "existing",
            );
            state.put_receipts.insert(
                request.idempotency_key,
                PutReplay {
                    object_ref: request.object_ref,
                    digest: request.expected_digest,
                    size: request.size,
                    receipt: result.clone(),
                },
            );
            return Ok(result);
        }
        state.sequence += 1;
        let validator = format!("opaque-validator.{}", state.sequence);
        let descriptor = self.descriptor(&request, &bytes, validator);
        let result = receipt(
            &request.context,
            StorageOperation::Put,
            &descriptor.blob_ref,
            false,
            &state.sequence.to_string(),
        );
        state.objects.insert(
            request.object_ref.clone(),
            StoredObject { descriptor, bytes },
        );
        state.put_receipts.insert(
            request.idempotency_key,
            PutReplay {
                object_ref: request.object_ref,
                digest: request.expected_digest,
                size: request.size,
                receipt: result.clone(),
            },
        );
        Ok(result)
    }

    async fn head(&self, request: ObjectRequest) -> StorageResult<BlobDescriptor> {
        request.validate()?;
        self.lock()?
            .objects
            .get(&request.object_ref)
            .map(|item| item.descriptor.clone())
            .ok_or_else(|| StorageError::new(StorageErrorCode::NotFound, "object not found", false))
    }

    async fn read(&self, request: ReadObjectRequest) -> StorageResult<ReadObject> {
        request.validate()?;
        let state = self.lock()?;
        let stored = state.objects.get(&request.object_ref).ok_or_else(|| {
            StorageError::new(StorageErrorCode::NotFound, "object not found", false)
        })?;
        let bytes = if let Some(range) = request.range {
            if range.length == 0 {
                return Err(StorageError::invalid("range length must be positive"));
            }
            let start = usize::try_from(range.offset)
                .map_err(|_| StorageError::invalid("range offset is too large"))?;
            let length = usize::try_from(range.length)
                .map_err(|_| StorageError::invalid("range length is too large"))?;
            let end = start
                .checked_add(length)
                .filter(|end| *end <= stored.bytes.len())
                .ok_or_else(|| StorageError::invalid("range exceeds object"))?;
            stored.bytes[start..end].to_vec()
        } else {
            stored.bytes.clone()
        };
        Ok(ReadObject {
            descriptor: stored.descriptor.clone(),
            body: Box::new(VecBlobStream::new(bytes, 64 * 1024)),
        })
    }

    async fn delete(&self, request: ObjectRequest) -> StorageResult<StorageReceipt> {
        request.validate()?;
        let mut state = self.lock()?;
        let removed = state.objects.remove(&request.object_ref);
        let replayed = removed.is_none();
        state.sequence += 1;
        let blob_ref = removed.map_or_else(
            || BlobRef {
                provider_ref: self.provider_ref.clone(),
                object_ref: request.object_ref,
                digest: digest(&[]),
                size: 0,
            },
            |item| item.descriptor.blob_ref,
        );
        Ok(receipt(
            &request.context,
            StorageOperation::Delete,
            &blob_ref,
            replayed,
            &state.sequence.to_string(),
        ))
    }

    async fn begin_multipart(&self, request: PutObjectRequest) -> StorageResult<MultipartUpload> {
        request.validate()?;
        let mut state = self.lock()?;
        if let Some(upload_ref) = state.upload_by_idempotency.get(&request.idempotency_key) {
            return Ok(MultipartUpload {
                upload_ref: upload_ref.clone(),
                object_ref: request.object_ref,
            });
        }
        state.sequence += 1;
        let upload_ref = format!("upload.{}", state.sequence);
        state
            .upload_by_idempotency
            .insert(request.idempotency_key.clone(), upload_ref.clone());
        state.uploads.insert(
            upload_ref.clone(),
            PendingUpload {
                request: request.clone(),
                parts: BTreeMap::new(),
            },
        );
        Ok(MultipartUpload {
            upload_ref,
            object_ref: request.object_ref,
        })
    }

    async fn upload_part(
        &self,
        request: UploadPartRequest,
        body: &mut dyn BlobReadStream,
    ) -> StorageResult<UploadedPart> {
        request.validate()?;
        let bytes = read_exact_body(body, request.size, &request.expected_digest).await?;
        let mut state = self.lock()?;
        let upload = state.uploads.get_mut(&request.upload_ref).ok_or_else(|| {
            StorageError::new(
                StorageErrorCode::NotFound,
                "multipart upload not found",
                false,
            )
        })?;
        if let Some(existing) = upload.parts.get(&request.part_number) {
            if existing.uploaded.digest == request.expected_digest
                && existing.uploaded.size == request.size
            {
                return Ok(existing.uploaded.clone());
            }
            return Err(StorageError::new(
                StorageErrorCode::Conflict,
                "multipart part retry has different content",
                false,
            ));
        }
        let uploaded = UploadedPart {
            part_number: request.part_number,
            digest: request.expected_digest,
            size: request.size,
            provider_part_ref: format!(
                "opaque-part.{}.{}",
                request.upload_ref, request.part_number
            ),
        };
        upload.parts.insert(
            request.part_number,
            StoredPart {
                uploaded: uploaded.clone(),
                bytes,
            },
        );
        Ok(uploaded)
    }

    async fn complete_multipart(
        &self,
        request: CompleteMultipartRequest,
    ) -> StorageResult<StorageReceipt> {
        request.validate()?;
        let mut state = self.lock()?;
        let mut requested = request.parts;
        requested.sort_by_key(|item| item.part_number);
        if let Some(completed) = state.completed_uploads.get(&request.upload_ref) {
            if completed.parts != requested {
                return Err(StorageError::new(
                    StorageErrorCode::Conflict,
                    "multipart completion retry names different parts",
                    false,
                ));
            }
            let mut receipt = completed.receipt.clone();
            receipt.request_id = request.context.request_id;
            receipt.replayed = true;
            return Ok(receipt);
        }

        let replay_existing = {
            let upload = state.uploads.get(&request.upload_ref).ok_or_else(|| {
                StorageError::new(
                    StorageErrorCode::NotFound,
                    "multipart upload not found",
                    false,
                )
            })?;
            if requested.is_empty() || requested.len() != upload.parts.len() {
                return Err(StorageError::invalid(
                    "multipart completion does not name every part",
                ));
            }
            let mut size = 0_u64;
            let mut hasher = Sha256::new();
            for (index, part) in requested.iter().enumerate() {
                if part.part_number != (index + 1) as u32 {
                    return Err(StorageError::invalid("multipart parts must be contiguous"));
                }
                let stored = upload
                    .parts
                    .get(&part.part_number)
                    .ok_or_else(|| StorageError::invalid("multipart part is missing"))?;
                if stored.uploaded != *part {
                    return Err(StorageError::new(
                        StorageErrorCode::Conflict,
                        "multipart completion receipt differs from uploaded part",
                        false,
                    ));
                }
                if index + 1 != requested.len() && stored.uploaded.size < MIN_MULTIPART_PART_BYTES {
                    return Err(StorageError::invalid(
                        "non-final multipart part is below 5 MiB",
                    ));
                }
                size = size.saturating_add(stored.uploaded.size);
                hasher.update(&stored.bytes);
            }
            let actual_digest = BlobDigest {
                algorithm: DigestAlgorithm::Sha256,
                value: format!("{:x}", hasher.finalize()),
            };
            if size != upload.request.size || actual_digest != upload.request.expected_digest {
                return Err(StorageError::integrity("multipart size or SHA-256 differs"));
            }
            match state.objects.get(&upload.request.object_ref) {
                Some(existing)
                    if existing.descriptor.blob_ref.digest != upload.request.expected_digest
                        || existing.descriptor.blob_ref.size != upload.request.size =>
                {
                    return Err(StorageError::new(
                        StorageErrorCode::Conflict,
                        "immutable object already exists with different content",
                        false,
                    ));
                }
                Some(_) => true,
                None => false,
            }
        };

        let mut upload = state
            .uploads
            .remove(&request.upload_ref)
            .expect("validated upload remains present while holding the state lock");
        let mut bytes = Vec::with_capacity(upload.request.size as usize);
        for part in &requested {
            let stored = upload
                .parts
                .remove(&part.part_number)
                .expect("validated part remains present while holding the state lock");
            bytes.extend_from_slice(&stored.bytes);
        }
        state.sequence += 1;
        let descriptor = if replay_existing {
            state
                .objects
                .get(&upload.request.object_ref)
                .map(|item| item.descriptor.clone())
                .ok_or_else(|| {
                    StorageError::new(
                        StorageErrorCode::Unavailable,
                        "validated immutable object disappeared",
                        true,
                    )
                })?
        } else {
            self.descriptor(
                &upload.request,
                &bytes,
                format!("opaque-multipart-validator.{}", state.sequence),
            )
        };
        let result = receipt(
            &request.context,
            StorageOperation::CompleteMultipart,
            &descriptor.blob_ref,
            replay_existing,
            &state.sequence.to_string(),
        );
        state.completed_uploads.insert(
            request.upload_ref,
            CompletedUpload {
                parts: requested,
                receipt: result.clone(),
            },
        );
        if !replay_existing {
            state.objects.insert(
                upload.request.object_ref,
                StoredObject { descriptor, bytes },
            );
        }
        Ok(result)
    }

    async fn abort_multipart(
        &self,
        request: AbortMultipartRequest,
    ) -> StorageResult<StorageReceipt> {
        request.validate()?;
        let mut state = self.lock()?;
        let removed = state.uploads.remove(&request.upload_ref);
        if let Some(upload) = &removed {
            state
                .upload_by_idempotency
                .remove(&upload.request.idempotency_key);
        }
        state.sequence += 1;
        let object_ref = removed.as_ref().map_or_else(
            || request.upload_ref.clone(),
            |item| item.request.object_ref.clone(),
        );
        Ok(StorageReceipt {
            receipt_ref: format!("receipt.{}", state.sequence),
            request_id: request.context.request_id,
            operation: StorageOperation::AbortMultipart,
            object_ref,
            digest: None,
            size: 0,
            replayed: removed.is_none(),
        })
    }
}

#[async_trait]
impl BlobMaintenancePort for InMemoryBlobStore {
    async fn list_for_maintenance(
        &self,
        request: ListForMaintenanceRequest,
    ) -> StorageResult<MaintenancePage> {
        request.validate()?;
        let offset = request
            .cursor
            .as_deref()
            .unwrap_or("0")
            .parse::<usize>()
            .map_err(|_| StorageError::invalid("maintenance cursor is invalid"))?;
        let state = self.lock()?;
        let matches = state
            .objects
            .iter()
            .filter(|(key, _)| key.starts_with(&request.prefix))
            .map(|(_, item)| item.descriptor.clone())
            .collect::<Vec<_>>();
        let end = offset
            .saturating_add(request.limit as usize)
            .min(matches.len());
        let objects = matches.get(offset..end).unwrap_or_default().to_vec();
        Ok(MaintenancePage {
            objects,
            next_cursor: (end < matches.len()).then(|| end.to_string()),
        })
    }
}

async fn read_exact_body(
    body: &mut dyn BlobReadStream,
    expected_size: u64,
    expected_digest: &BlobDigest,
) -> StorageResult<Vec<u8>> {
    let mut bytes = Vec::new();
    while let Some(chunk) = body.next_chunk(MAX_STREAM_CHUNK_BYTES).await? {
        if chunk.is_empty() || chunk.len() > MAX_STREAM_CHUNK_BYTES {
            return Err(StorageError::integrity(
                "upload source emitted an empty or oversized chunk",
            ));
        }
        bytes.extend_from_slice(&chunk);
        if bytes.len() as u64 > expected_size {
            return Err(StorageError::integrity("upload body exceeds declared size"));
        }
    }
    verify_body(&bytes, expected_size, expected_digest)?;
    Ok(bytes)
}

fn verify_body(
    bytes: &[u8],
    expected_size: u64,
    expected_digest: &BlobDigest,
) -> StorageResult<()> {
    expected_digest.validate()?;
    if bytes.len() as u64 != expected_size || &digest(bytes) != expected_digest {
        return Err(StorageError::integrity("upload size or SHA-256 differs"));
    }
    Ok(())
}

fn receipt(
    context: &StorageRequestContext,
    operation: StorageOperation,
    blob_ref: &BlobRef,
    replayed: bool,
    suffix: &str,
) -> StorageReceipt {
    StorageReceipt {
        receipt_ref: format!("receipt.{suffix}"),
        request_id: context.request_id.clone(),
        operation,
        object_ref: blob_ref.object_ref.clone(),
        digest: Some(blob_ref.digest.clone()),
        size: blob_ref.size,
        replayed,
    }
}
