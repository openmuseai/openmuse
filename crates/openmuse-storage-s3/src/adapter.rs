use crate::{
    EndpointGuard, EndpointResolver, S3ProviderConfig, ValidatedEndpoint, denied, unavailable,
};
use async_trait::async_trait;
use aws_sdk_s3::{
    Client,
    presigning::PresigningConfig,
    primitives::ByteStream,
    types::{CompletedMultipartUpload, CompletedPart},
};
use openmuse_storage_contract::{
    AbortMultipartRequest, BlobDescriptor, BlobDigest, BlobMaintenancePort, BlobReadStream,
    BlobRef, BlobStorePort, CompleteMultipartRequest, DigestAlgorithm, ListForMaintenanceRequest,
    MAX_STREAM_CHUNK_BYTES, MIN_MULTIPART_PART_BYTES, MaintenancePage, MultipartUpload,
    ObjectRequest, PROFILE_MAJOR, PROFILE_NAME, ProviderCapabilitySnapshot, ProviderValidator,
    PutObjectRequest, ReadObject, ReadObjectRequest, StorageError, StorageErrorCode,
    StorageOperation, StorageReceipt, StorageResult, UploadPartRequest, UploadedPart,
};
use sha2::{Digest, Sha256};
use std::{
    collections::HashMap,
    sync::{
        Arc, Mutex,
        atomic::{AtomicU64, Ordering},
    },
    time::Duration,
};
use tempfile::NamedTempFile;
use tokio::io::{AsyncWriteExt, BufWriter};

const DIGEST_META: &str = "openmuse-sha256";
const SIZE_META: &str = "openmuse-size";

#[derive(Debug, Clone)]
pub struct PresignedPutRequest {
    pub url: crate::SensitiveUrl,
    pub required_headers: std::collections::BTreeMap<String, String>,
    pub expires_in_seconds: u64,
}

#[async_trait]
pub trait OperationEndpointCheck: Send + Sync {
    async fn check(&self) -> StorageResult<()>;
}

pub struct RevalidatingEndpointCheck<R> {
    guard: EndpointGuard<R>,
    config: S3ProviderConfig,
    validated: ValidatedEndpoint,
}

impl<R> RevalidatingEndpointCheck<R>
where
    R: EndpointResolver,
{
    pub fn new(
        guard: EndpointGuard<R>,
        config: S3ProviderConfig,
        validated: ValidatedEndpoint,
    ) -> Self {
        Self {
            guard,
            config,
            validated,
        }
    }
}

#[async_trait]
impl<R> OperationEndpointCheck for RevalidatingEndpointCheck<R>
where
    R: EndpointResolver,
{
    async fn check(&self) -> StorageResult<()> {
        self.guard
            .revalidate(&self.config, &self.validated)
            .await
            .map(|_| ())
    }
}

pub struct AwsS3BlobStore {
    client: Client,
    config: S3ProviderConfig,
    endpoint_check: Arc<dyn OperationEndpointCheck>,
    uploads: Mutex<HashMap<String, ActiveUpload>>,
    parts: Mutex<HashMap<(String, u32), UploadedPart>>,
    completed: Mutex<HashMap<String, StorageReceipt>>,
    sequence: AtomicU64,
}

#[derive(Clone)]
struct ActiveUpload {
    provider_ref: String,
    object_ref: String,
    digest: BlobDigest,
    size: u64,
}

impl AwsS3BlobStore {
    pub fn with_endpoint_check(
        client: Client,
        config: S3ProviderConfig,
        check: Arc<dyn OperationEndpointCheck>,
    ) -> StorageResult<Self> {
        config.validate_static()?;
        Ok(Self {
            client,
            config,
            endpoint_check: check,
            uploads: Mutex::new(HashMap::new()),
            parts: Mutex::new(HashMap::new()),
            completed: Mutex::new(HashMap::new()),
            sequence: AtomicU64::new(1),
        })
    }

    async fn preflight(&self) -> StorageResult<()> {
        self.endpoint_check.check().await
    }

    async fn head_ref(&self, object_ref: &str) -> StorageResult<BlobDescriptor> {
        let output = self
            .client
            .head_object()
            .bucket(&self.config.bucket)
            .key(self.config.object_key(object_ref)?)
            .send()
            .await
            .map_err(|e| {
                http_error(
                    e.raw_response().map(|r| r.status().as_u16()),
                    "S3 HEAD failed",
                )
            })?;
        descriptor(
            &self.config.provider_ref,
            object_ref,
            output.metadata(),
            output.content_type(),
            output.e_tag(),
        )
    }

    fn receipt(
        &self,
        request_id: String,
        operation: StorageOperation,
        value: &BlobDescriptor,
        replayed: bool,
    ) -> StorageReceipt {
        StorageReceipt {
            receipt_ref: format!(
                "storage-receipt.{}",
                self.sequence.fetch_add(1, Ordering::Relaxed)
            ),
            request_id,
            operation,
            object_ref: value.blob_ref.object_ref.clone(),
            digest: Some(value.blob_ref.digest.clone()),
            size: value.blob_ref.size,
            replayed,
        }
    }

    fn upload(&self, upload_ref: &str) -> StorageResult<ActiveUpload> {
        self.uploads
            .lock()
            .map_err(|_| unavailable("S3 upload registry is poisoned"))?
            .get(upload_ref)
            .cloned()
            .ok_or_else(|| {
                StorageError::new(
                    StorageErrorCode::NotFound,
                    "multipart upload not found",
                    false,
                )
            })
    }

    pub async fn presign_put(
        &self,
        object_ref: &str,
        content_type: &str,
        size: u64,
        digest: &BlobDigest,
        expires_in_seconds: u64,
    ) -> StorageResult<PresignedPutRequest> {
        self.preflight().await?;
        digest.validate()?;
        if content_type.is_empty() || !(1..=900).contains(&expires_in_seconds) {
            return Err(denied("presigned PUT parameters are invalid"));
        }
        let config = PresigningConfig::expires_in(Duration::from_secs(expires_in_seconds))
            .map_err(|_| denied("presigned PUT expiry is invalid"))?;
        let request = self
            .client
            .put_object()
            .bucket(&self.config.bucket)
            .key(self.config.object_key(object_ref)?)
            .if_none_match("*")
            .content_length(i64::try_from(size).map_err(|_| denied("object is too large"))?)
            .content_type(content_type)
            .metadata(DIGEST_META, digest.value.clone())
            .metadata(SIZE_META, size.to_string())
            .presigned(config)
            .await
            .map_err(|_| unavailable("S3 presigned PUT generation failed"))?;
        Ok(PresignedPutRequest {
            url: crate::SensitiveUrl::new(request.uri().to_string()),
            required_headers: request
                .headers()
                .map(|(key, value)| (key.to_owned(), value.to_owned()))
                .collect(),
            expires_in_seconds,
        })
    }
}

#[async_trait]
impl BlobStorePort for AwsS3BlobStore {
    fn capabilities(&self) -> ProviderCapabilitySnapshot {
        ProviderCapabilitySnapshot {
            profile: PROFILE_NAME.into(),
            profile_major: PROFILE_MAJOR,
            provider_ref: self.config.provider_ref.clone(),
            provider_kind: self.config.provider_kind,
            provider_version: "aws-sdk-s3/1.88".into(),
            tls: self.config.endpoint.starts_with("https://"),
            addressing_styles: vec![self.config.addressing_style],
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
        self.preflight().await?;
        match self.head_ref(&request.object_ref).await {
            Ok(old)
                if old.blob_ref.digest == request.expected_digest
                    && old.blob_ref.size == request.size =>
            {
                return Ok(self.receipt(
                    request.context.request_id,
                    StorageOperation::Put,
                    &old,
                    true,
                ));
            }
            Ok(_) => {
                return Err(StorageError::new(
                    StorageErrorCode::Conflict,
                    "immutable S3 object has different content",
                    false,
                ));
            }
            Err(e) if e.code == StorageErrorCode::NotFound => {}
            Err(e) => return Err(e),
        }
        let staged = stage(body, request.size, &request.expected_digest).await?;
        let stream = ByteStream::from_path(staged.path())
            .await
            .map_err(|_| unavailable("cannot open staged S3 upload"))?;
        let mut op = self
            .client
            .put_object()
            .bucket(&self.config.bucket)
            .key(self.config.object_key(&request.object_ref)?)
            .if_none_match("*")
            .content_length(i64::try_from(request.size).map_err(|_| denied("object is too large"))?)
            .content_type(&request.content_type)
            .metadata(DIGEST_META, request.expected_digest.value.clone())
            .metadata(SIZE_META, request.size.to_string())
            .body(stream);
        for (key, value) in &request.metadata {
            op = op.metadata(key, value);
        }
        op.send().await.map_err(|e| {
            http_error(
                e.raw_response().map(|r| r.status().as_u16()),
                "S3 PUT failed",
            )
        })?;
        let value = self.head_ref(&request.object_ref).await?;
        verify(&value, &request.expected_digest, request.size)?;
        Ok(self.receipt(
            request.context.request_id,
            StorageOperation::Put,
            &value,
            false,
        ))
    }

    async fn head(&self, request: ObjectRequest) -> StorageResult<BlobDescriptor> {
        request.validate()?;
        self.preflight().await?;
        self.head_ref(&request.object_ref).await
    }

    async fn read(&self, request: ReadObjectRequest) -> StorageResult<ReadObject> {
        request.validate()?;
        self.preflight().await?;
        let mut op = self
            .client
            .get_object()
            .bucket(&self.config.bucket)
            .key(self.config.object_key(&request.object_ref)?);
        if let Some(range) = request.range {
            let end = range
                .offset
                .checked_add(range.length - 1)
                .ok_or_else(|| denied("range overflows"))?;
            op = op.range(format!("bytes={}-{}", range.offset, end));
        }
        let output = op.send().await.map_err(|e| {
            http_error(
                e.raw_response().map(|r| r.status().as_u16()),
                "S3 GET failed",
            )
        })?;
        let value = descriptor(
            &self.config.provider_ref,
            &request.object_ref,
            output.metadata(),
            output.content_type(),
            output.e_tag(),
        )?;
        Ok(ReadObject {
            descriptor: value,
            body: Box::new(S3ReadStream {
                body: output.body,
                pending: Vec::new(),
                offset: 0,
            }),
        })
    }

    async fn delete(&self, request: ObjectRequest) -> StorageResult<StorageReceipt> {
        request.validate()?;
        self.preflight().await?;
        let old = self.head_ref(&request.object_ref).await.ok();
        let replayed = old.is_none();
        self.client
            .delete_object()
            .bucket(&self.config.bucket)
            .key(self.config.object_key(&request.object_ref)?)
            .send()
            .await
            .map_err(|e| {
                http_error(
                    e.raw_response().map(|r| r.status().as_u16()),
                    "S3 DELETE failed",
                )
            })?;
        let value = old.unwrap_or_else(|| empty_descriptor(&self.config, &request.object_ref));
        Ok(self.receipt(
            request.context.request_id,
            StorageOperation::Delete,
            &value,
            replayed,
        ))
    }

    async fn begin_multipart(&self, request: PutObjectRequest) -> StorageResult<MultipartUpload> {
        request.validate()?;
        self.preflight().await?;
        let output = self
            .client
            .create_multipart_upload()
            .bucket(&self.config.bucket)
            .key(self.config.object_key(&request.object_ref)?)
            .content_type(&request.content_type)
            .metadata(DIGEST_META, request.expected_digest.value.clone())
            .metadata(SIZE_META, request.size.to_string())
            .send()
            .await
            .map_err(|e| {
                http_error(
                    e.raw_response().map(|r| r.status().as_u16()),
                    "S3 multipart create failed",
                )
            })?;
        let provider_ref = output
            .upload_id()
            .filter(|v| !v.is_empty())
            .ok_or_else(|| unavailable("S3 returned no upload id"))?
            .to_owned();
        let upload_ref = format!(
            "s3-upload.{}",
            self.sequence.fetch_add(1, Ordering::Relaxed)
        );
        self.uploads
            .lock()
            .map_err(|_| unavailable("S3 upload registry is poisoned"))?
            .insert(
                upload_ref.clone(),
                ActiveUpload {
                    provider_ref,
                    object_ref: request.object_ref.clone(),
                    digest: request.expected_digest,
                    size: request.size,
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
        self.preflight().await?;
        let upload = self.upload(&request.upload_ref)?;
        if let Some(existing) = self
            .parts
            .lock()
            .map_err(|_| unavailable("S3 part registry is poisoned"))?
            .get(&(request.upload_ref.clone(), request.part_number))
            .cloned()
        {
            if existing.digest == request.expected_digest && existing.size == request.size {
                return Ok(existing);
            }
            return Err(StorageError::new(
                StorageErrorCode::Conflict,
                "multipart part retry has different content",
                false,
            ));
        }
        let staged = stage(body, request.size, &request.expected_digest).await?;
        let stream = ByteStream::from_path(staged.path())
            .await
            .map_err(|_| unavailable("cannot open staged S3 part"))?;
        let output = self
            .client
            .upload_part()
            .bucket(&self.config.bucket)
            .key(self.config.object_key(&upload.object_ref)?)
            .upload_id(upload.provider_ref)
            .part_number(request.part_number as i32)
            .content_length(i64::try_from(request.size).map_err(|_| denied("part is too large"))?)
            .body(stream)
            .send()
            .await
            .map_err(|e| {
                http_error(
                    e.raw_response().map(|r| r.status().as_u16()),
                    "S3 multipart part failed",
                )
            })?;
        let provider_part_ref = output
            .e_tag()
            .filter(|v| !v.is_empty())
            .ok_or_else(|| unavailable("S3 part returned no validator"))?
            .to_owned();
        let uploaded = UploadedPart {
            part_number: request.part_number,
            digest: request.expected_digest,
            size: request.size,
            provider_part_ref,
        };
        self.parts
            .lock()
            .map_err(|_| unavailable("S3 part registry is poisoned"))?
            .insert((request.upload_ref, request.part_number), uploaded.clone());
        Ok(uploaded)
    }

    async fn complete_multipart(
        &self,
        request: CompleteMultipartRequest,
    ) -> StorageResult<StorageReceipt> {
        request.validate()?;
        self.preflight().await?;
        if let Some(mut value) = self
            .completed
            .lock()
            .map_err(|_| unavailable("S3 completion registry is poisoned"))?
            .get(&request.upload_ref)
            .cloned()
        {
            value.request_id = request.context.request_id;
            value.replayed = true;
            return Ok(value);
        }
        let upload = self.upload(&request.upload_ref)?;
        let mut parts = request.parts;
        parts.sort_by_key(|v| v.part_number);
        for (index, part) in parts.iter().enumerate() {
            if part.part_number != (index + 1) as u32
                || (index + 1 != parts.len() && part.size < MIN_MULTIPART_PART_BYTES)
            {
                return Err(denied("multipart completion parts are invalid"));
            }
        }
        let completed = CompletedMultipartUpload::builder()
            .set_parts(Some(
                parts
                    .iter()
                    .map(|part| {
                        CompletedPart::builder()
                            .part_number(part.part_number as i32)
                            .e_tag(&part.provider_part_ref)
                            .build()
                    })
                    .collect(),
            ))
            .build();
        self.client
            .complete_multipart_upload()
            .bucket(&self.config.bucket)
            .key(self.config.object_key(&upload.object_ref)?)
            .upload_id(&upload.provider_ref)
            .multipart_upload(completed)
            .send()
            .await
            .map_err(|e| {
                http_error(
                    e.raw_response().map(|r| r.status().as_u16()),
                    "S3 multipart complete failed",
                )
            })?;
        let value = self.head_ref(&upload.object_ref).await?;
        verify(&value, &upload.digest, upload.size)?;
        let receipt = self.receipt(
            request.context.request_id,
            StorageOperation::CompleteMultipart,
            &value,
            false,
        );
        self.uploads
            .lock()
            .map_err(|_| unavailable("S3 upload registry is poisoned"))?
            .remove(&request.upload_ref);
        self.parts
            .lock()
            .map_err(|_| unavailable("S3 part registry is poisoned"))?
            .retain(|(upload_ref, _), _| upload_ref != &request.upload_ref);
        self.completed
            .lock()
            .map_err(|_| unavailable("S3 completion registry is poisoned"))?
            .insert(request.upload_ref, receipt.clone());
        Ok(receipt)
    }

    async fn abort_multipart(
        &self,
        request: AbortMultipartRequest,
    ) -> StorageResult<StorageReceipt> {
        request.validate()?;
        self.preflight().await?;
        let upload = self
            .uploads
            .lock()
            .map_err(|_| unavailable("S3 upload registry is poisoned"))?
            .remove(&request.upload_ref);
        self.parts
            .lock()
            .map_err(|_| unavailable("S3 part registry is poisoned"))?
            .retain(|(upload_ref, _), _| upload_ref != &request.upload_ref);
        let (object_ref, replayed) = if let Some(value) = upload {
            self.client
                .abort_multipart_upload()
                .bucket(&self.config.bucket)
                .key(self.config.object_key(&value.object_ref)?)
                .upload_id(value.provider_ref)
                .send()
                .await
                .map_err(|e| {
                    http_error(
                        e.raw_response().map(|r| r.status().as_u16()),
                        "S3 multipart abort failed",
                    )
                })?;
            (value.object_ref, false)
        } else {
            (request.upload_ref, true)
        };
        Ok(StorageReceipt {
            receipt_ref: format!(
                "storage-receipt.{}",
                self.sequence.fetch_add(1, Ordering::Relaxed)
            ),
            request_id: request.context.request_id,
            operation: StorageOperation::AbortMultipart,
            object_ref,
            digest: None,
            size: 0,
            replayed,
        })
    }
}

#[async_trait]
impl BlobMaintenancePort for AwsS3BlobStore {
    async fn list_for_maintenance(
        &self,
        request: ListForMaintenanceRequest,
    ) -> StorageResult<MaintenancePage> {
        request.validate()?;
        self.preflight().await?;
        let output = self
            .client
            .list_objects_v2()
            .bucket(&self.config.bucket)
            .prefix(self.config.object_key(&request.prefix)?)
            .set_continuation_token(request.cursor)
            .max_keys(request.limit as i32)
            .send()
            .await
            .map_err(|e| {
                http_error(
                    e.raw_response().map(|r| r.status().as_u16()),
                    "S3 list failed",
                )
            })?;
        let mut objects = Vec::new();
        for item in output.contents() {
            if let Some(key) = item.key() {
                objects.push(self.head_ref(&strip_prefix(&self.config, key)?).await?);
            }
        }
        Ok(MaintenancePage {
            objects,
            next_cursor: output.next_continuation_token().map(str::to_owned),
        })
    }
}

struct S3ReadStream {
    body: ByteStream,
    pending: Vec<u8>,
    offset: usize,
}
#[async_trait]
impl BlobReadStream for S3ReadStream {
    async fn next_chunk(&mut self, max_bytes: usize) -> StorageResult<Option<Vec<u8>>> {
        if max_bytes == 0 || max_bytes > MAX_STREAM_CHUNK_BYTES {
            return Err(denied("stream chunk bound is invalid"));
        }
        if self.offset < self.pending.len() {
            return Ok(Some(self.take(max_bytes)));
        }
        self.pending.clear();
        self.offset = 0;
        match self.body.try_next().await {
            Ok(Some(v)) if v.is_empty() => Err(integrity("S3 emitted an empty body chunk")),
            Ok(Some(v)) => {
                self.pending = v.to_vec();
                Ok(Some(self.take(max_bytes)))
            }
            Ok(None) => Ok(None),
            Err(_) => Err(unavailable("S3 response body failed")),
        }
    }
}
impl S3ReadStream {
    fn take(&mut self, max: usize) -> Vec<u8> {
        let end = (self.offset + max).min(self.pending.len());
        let value = self.pending[self.offset..end].to_vec();
        self.offset = end;
        value
    }
}

async fn stage(
    body: &mut dyn BlobReadStream,
    size: u64,
    digest: &BlobDigest,
) -> StorageResult<NamedTempFile> {
    digest.validate()?;
    let file = NamedTempFile::new().map_err(|_| unavailable("cannot create staged upload"))?;
    let writable = file
        .reopen()
        .map_err(|_| unavailable("cannot open staged upload"))?;
    let mut writer = BufWriter::new(tokio::fs::File::from_std(writable));
    let mut actual_size = 0_u64;
    let mut hasher = Sha256::new();
    while let Some(chunk) = body.next_chunk(MAX_STREAM_CHUNK_BYTES).await? {
        if chunk.is_empty() || chunk.len() > MAX_STREAM_CHUNK_BYTES {
            return Err(integrity("upload emitted an invalid chunk"));
        }
        actual_size = actual_size
            .checked_add(chunk.len() as u64)
            .ok_or_else(|| denied("upload size overflows"))?;
        if actual_size > size {
            return Err(integrity("upload exceeds declared size"));
        }
        hasher.update(&chunk);
        writer
            .write_all(&chunk)
            .await
            .map_err(|_| unavailable("cannot stage upload"))?;
    }
    writer
        .flush()
        .await
        .map_err(|_| unavailable("cannot flush staged upload"))?;
    if actual_size != size || format!("{:x}", hasher.finalize()) != digest.value {
        return Err(integrity("upload size or SHA-256 differs"));
    }
    Ok(file)
}

fn descriptor(
    provider: &str,
    object: &str,
    metadata: Option<&HashMap<String, String>>,
    content_type: Option<&str>,
    validator: Option<&str>,
) -> StorageResult<BlobDescriptor> {
    let metadata = metadata.ok_or_else(|| integrity("S3 object has no OpenMuse metadata"))?;
    let digest = BlobDigest::sha256(
        metadata
            .get(DIGEST_META)
            .ok_or_else(|| integrity("S3 object has no digest metadata"))?
            .clone(),
    )?;
    let size = metadata
        .get(SIZE_META)
        .ok_or_else(|| integrity("S3 object has no size metadata"))?
        .parse()
        .map_err(|_| integrity("S3 object has invalid size metadata"))?;
    let mut user = metadata.clone();
    user.remove(DIGEST_META);
    user.remove(SIZE_META);
    Ok(BlobDescriptor {
        blob_ref: BlobRef {
            provider_ref: provider.into(),
            object_ref: object.into(),
            digest,
            size,
        },
        provider_validator: validator.map(|v| ProviderValidator { value: v.into() }),
        content_type: content_type.unwrap_or("application/octet-stream").into(),
        metadata: user.into_iter().collect(),
    })
}
fn verify(value: &BlobDescriptor, digest: &BlobDigest, size: u64) -> StorageResult<()> {
    if &value.blob_ref.digest != digest || value.blob_ref.size != size {
        Err(integrity("S3 read-after-write verification failed"))
    } else {
        Ok(())
    }
}
fn empty_descriptor(config: &S3ProviderConfig, object: &str) -> BlobDescriptor {
    BlobDescriptor {
        blob_ref: BlobRef {
            provider_ref: config.provider_ref.clone(),
            object_ref: object.into(),
            digest: BlobDigest {
                algorithm: DigestAlgorithm::Sha256,
                value: format!("{:x}", Sha256::digest([])),
            },
            size: 0,
        },
        provider_validator: None,
        content_type: "application/octet-stream".into(),
        metadata: Default::default(),
    }
}
fn strip_prefix(config: &S3ProviderConfig, key: &str) -> StorageResult<String> {
    if config.prefix.is_empty() {
        return Ok(key.into());
    }
    key.strip_prefix(config.prefix.trim_end_matches('/'))
        .and_then(|v| v.strip_prefix('/'))
        .map(str::to_owned)
        .ok_or_else(|| denied("S3 list escaped configured prefix"))
}
fn integrity(message: &'static str) -> StorageError {
    StorageError::new(StorageErrorCode::IntegrityFailed, message, false)
}
fn http_error(status: Option<u16>, message: &'static str) -> StorageError {
    match status {
        Some(401 | 403) => StorageError::new(StorageErrorCode::Denied, message, false),
        Some(404) => StorageError::new(StorageErrorCode::NotFound, message, false),
        Some(409 | 412) => StorageError::new(StorageErrorCode::Conflict, message, false),
        Some(408 | 425 | 429 | 500..=599) => {
            StorageError::new(StorageErrorCode::Transient, message, true)
        }
        _ => unavailable(message),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use openmuse_storage_tck::{VecBlobStream, digest};

    #[test]
    fn staging_is_bounded_and_checks_digest() {
        tokio::runtime::Builder::new_current_thread()
            .enable_all()
            .build()
            .unwrap()
            .block_on(async {
                let bytes = vec![7; MAX_STREAM_CHUNK_BYTES + 9];
                let mut source = VecBlobStream::new(bytes.clone(), 13);
                let file = stage(&mut source, bytes.len() as u64, &digest(&bytes))
                    .await
                    .unwrap();
                assert_eq!(file.as_file().metadata().unwrap().len(), bytes.len() as u64);
                let mut bad = VecBlobStream::new(bytes.clone(), 17);
                assert_eq!(
                    stage(&mut bad, bytes.len() as u64, &digest(b"wrong"))
                        .await
                        .unwrap_err()
                        .code,
                    StorageErrorCode::IntegrityFailed
                );
            });
    }

    #[test]
    fn http_statuses_are_normalized() {
        assert_eq!(http_error(Some(403), "safe").code, StorageErrorCode::Denied);
        assert_eq!(
            http_error(Some(404), "safe").code,
            StorageErrorCode::NotFound
        );
        assert_eq!(
            http_error(Some(412), "safe").code,
            StorageErrorCode::Conflict
        );
        let throttled = http_error(Some(429), "safe");
        assert_eq!(throttled.code, StorageErrorCode::Transient);
        assert!(throttled.retryable);
    }
}
