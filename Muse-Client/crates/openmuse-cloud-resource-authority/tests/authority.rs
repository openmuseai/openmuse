use async_trait::async_trait;
use futures::join;
use openmuse_cloud_resource_authority::{
    AuthorityError, AuthorityErrorCode, AuthorityResult, CatalogQuery, CloudResourceAuthority,
    CommitResourceRequest, InMemoryMetadataStore, MaterializeRequest, MetadataStorePort,
    ResourceAcl, ResourceChangedEvent, ResourceEventPublisherPort, ResourceRecord,
};
use openmuse_contract::{PrincipalKind, PrincipalRef};
use openmuse_storage_contract::{
    AbortMultipartRequest, BlobDigest, BlobReadStream, BlobRef, BlobStorePort, ByteRange,
    CompleteMultipartRequest, MultipartUpload, ObjectRequest, ProviderCapabilitySnapshot,
    PutObjectRequest, ReadObject, ReadObjectRequest, StorageError, StorageErrorCode,
    StorageReceipt, StorageRequestContext, UploadPartRequest, UploadedPart,
};
use openmuse_storage_tck::{InMemoryBlobStore, VecBlobStream, digest};
use std::collections::BTreeSet;
use std::sync::{Arc, Mutex};

fn context(request_id: &str) -> StorageRequestContext {
    StorageRequestContext {
        request_id: request_id.to_owned(),
        actor: PrincipalRef {
            principal_ref: "user.owner".to_owned(),
            kind: PrincipalKind::User,
        },
        caller: PrincipalRef {
            principal_ref: "service.resource-authority".to_owned(),
            kind: PrincipalKind::Service,
        },
        policy_decision_ref: format!("decision.{request_id}"),
        workspace_ref: "workspace.cloud".to_owned(),
        generation: 1,
        deadline_at_ms: 4_102_444_800_000,
        cancellation_ref: format!("cancel.{request_id}"),
    }
}

fn acl() -> ResourceAcl {
    ResourceAcl {
        owner: "user.owner".to_owned(),
        readers: BTreeSet::from(["user.reader".to_owned()]),
        writers: BTreeSet::new(),
    }
}

fn record(resource_ref: &str, revision: &str, blob_ref: BlobRef) -> ResourceRecord {
    ResourceRecord {
        workspace_ref: "workspace.cloud".to_owned(),
        resource_ref: resource_ref.to_owned(),
        parent_ref: None,
        display_name: format!("{resource_ref}.txt"),
        media_type: "text/plain".to_owned(),
        revision: revision.to_owned(),
        blob_ref,
        acl: acl(),
    }
}

fn commit_request(id: &str, expected_revision: &str, bytes: &[u8]) -> CommitResourceRequest {
    CommitResourceRequest {
        context: context(id),
        resource_ref: "resource.a".to_owned(),
        expected_revision: expected_revision.to_owned(),
        expected_digest: digest(bytes),
        size: bytes.len() as u64,
        content_type: "text/plain".to_owned(),
        idempotency_key: format!("idem.{id}"),
    }
}

#[derive(Default)]
struct RecordingPublisher {
    fail_next: Mutex<bool>,
    events: Mutex<Vec<ResourceChangedEvent>>,
}

impl RecordingPublisher {
    fn fail_once(&self) {
        *self.fail_next.lock().unwrap() = true;
    }
}

#[async_trait]
impl ResourceEventPublisherPort for RecordingPublisher {
    async fn publish(&self, event: &ResourceChangedEvent) -> AuthorityResult<()> {
        let mut fail = self.fail_next.lock().unwrap();
        if *fail {
            *fail = false;
            return Err(AuthorityError {
                code: AuthorityErrorCode::MetadataUnavailable,
                message: "event transport unavailable".to_owned(),
                retryable: true,
            });
        }
        self.events.lock().unwrap().push(event.clone());
        Ok(())
    }
}

async fn fixture() -> (
    Arc<InMemoryBlobStore>,
    Arc<InMemoryMetadataStore>,
    Arc<RecordingPublisher>,
    CloudResourceAuthority,
) {
    let blobs = Arc::new(InMemoryBlobStore::new("provider.fake.cloud"));
    let metadata = Arc::new(InMemoryMetadataStore::default());
    let publisher = Arc::new(RecordingPublisher::default());
    let initial = b"initial".to_vec();
    let initial_digest = digest(&initial);
    let mut stream = VecBlobStream::new(initial.clone(), 3);
    blobs
        .put(
            PutObjectRequest {
                context: context("seed"),
                object_ref: format!("blobs/sha256/{}", initial_digest.value),
                expected_digest: initial_digest.clone(),
                size: initial.len() as u64,
                content_type: "text/plain".to_owned(),
                metadata: Default::default(),
                idempotency_key: "blob.seed".to_owned(),
            },
            &mut stream,
        )
        .await
        .unwrap();
    metadata
        .seed(record(
            "resource.a",
            "revision.0",
            BlobRef {
                provider_ref: "provider.fake.cloud".to_owned(),
                object_ref: format!("blobs/sha256/{}", initial_digest.value),
                digest: initial_digest,
                size: initial.len() as u64,
            },
        ))
        .await
        .unwrap();
    let authority = CloudResourceAuthority::new(blobs.clone(), metadata.clone(), publisher.clone());
    (blobs, metadata, publisher, authority)
}

#[test]
fn concurrent_writers_use_metadata_cas_without_last_write_wins() {
    futures::executor::block_on(async {
        let (_, metadata, _, authority) = fixture().await;
        let first = b"writer-one".to_vec();
        let second = b"writer-two".to_vec();
        let mut first_stream = VecBlobStream::new(first.clone(), 2);
        let mut second_stream = VecBlobStream::new(second.clone(), 2);
        let (one, two) = join!(
            authority.commit(
                commit_request("writer-one", "revision.0", &first),
                &mut first_stream,
            ),
            authority.commit(
                commit_request("writer-two", "revision.0", &second),
                &mut second_stream,
            )
        );
        let outcomes = [one, two];
        assert_eq!(outcomes.iter().filter(|item| item.is_ok()).count(), 1);
        assert_eq!(
            outcomes
                .iter()
                .filter_map(|item| item.as_ref().err())
                .filter(|error| error.code == AuthorityErrorCode::Conflict)
                .count(),
            1
        );
        let current = metadata
            .describe("workspace.cloud", "resource.a")
            .await
            .unwrap();
        assert_ne!(current.revision, "revision.0");
    });
}

#[test]
fn blob_success_and_metadata_failure_records_an_orphan_without_success() {
    futures::executor::block_on(async {
        let (_, metadata, _, authority) = fixture().await;
        metadata.fail_next_commit().unwrap();
        let bytes = b"orphaned".to_vec();
        let mut stream = VecBlobStream::new(bytes.clone(), 2);
        let error = authority
            .commit(
                commit_request("metadata-failure", "revision.0", &bytes),
                &mut stream,
            )
            .await
            .err()
            .unwrap();
        assert_eq!(error.code, AuthorityErrorCode::MetadataUnavailable);
        assert_eq!(
            metadata
                .describe("workspace.cloud", "resource.a")
                .await
                .unwrap()
                .revision,
            "revision.0"
        );
        let orphans = metadata.list_orphans().await.unwrap();
        assert_eq!(orphans.len(), 1);
        assert_eq!(orphans[0].blob_ref.digest, digest(&bytes));
    });
}

#[test]
fn committed_metadata_outbox_replays_after_event_transport_failure() {
    futures::executor::block_on(async {
        let (_, metadata, publisher, authority) = fixture().await;
        let bytes = b"eventual-event".to_vec();
        let mut stream = VecBlobStream::new(bytes.clone(), 4);
        authority
            .commit(commit_request("outbox", "revision.0", &bytes), &mut stream)
            .await
            .unwrap();
        publisher.fail_once();
        assert_eq!(authority.dispatch_outbox(10).await.unwrap(), 0);
        let pending = metadata.pending_outbox(10).await.unwrap();
        assert_eq!(pending.len(), 1);
        assert_eq!(pending[0].attempts, 1);
        assert_eq!(authority.dispatch_outbox(10).await.unwrap(), 1);
        assert!(metadata.pending_outbox(10).await.unwrap().is_empty());
        assert_eq!(publisher.events.lock().unwrap().len(), 1);
    });
}

#[derive(Default)]
struct NoIoBlobStore;

fn unexpected_io() -> StorageError {
    StorageError::new(
        StorageErrorCode::Unavailable,
        "catalog unexpectedly called the blob data plane",
        false,
    )
}

#[async_trait]
impl BlobStorePort for NoIoBlobStore {
    fn capabilities(&self) -> ProviderCapabilitySnapshot {
        InMemoryBlobStore::new("provider.no-io").capabilities()
    }

    async fn put(
        &self,
        _request: PutObjectRequest,
        _body: &mut dyn BlobReadStream,
    ) -> Result<StorageReceipt, StorageError> {
        Err(unexpected_io())
    }

    async fn head(
        &self,
        _request: ObjectRequest,
    ) -> Result<openmuse_storage_contract::BlobDescriptor, StorageError> {
        Err(unexpected_io())
    }

    async fn read(&self, _request: ReadObjectRequest) -> Result<ReadObject, StorageError> {
        Err(unexpected_io())
    }

    async fn delete(&self, _request: ObjectRequest) -> Result<StorageReceipt, StorageError> {
        Err(unexpected_io())
    }

    async fn begin_multipart(
        &self,
        _request: PutObjectRequest,
    ) -> Result<MultipartUpload, StorageError> {
        Err(unexpected_io())
    }

    async fn upload_part(
        &self,
        _request: UploadPartRequest,
        _body: &mut dyn BlobReadStream,
    ) -> Result<UploadedPart, StorageError> {
        Err(unexpected_io())
    }

    async fn complete_multipart(
        &self,
        _request: CompleteMultipartRequest,
    ) -> Result<StorageReceipt, StorageError> {
        Err(unexpected_io())
    }

    async fn abort_multipart(
        &self,
        _request: AbortMultipartRequest,
    ) -> Result<StorageReceipt, StorageError> {
        Err(unexpected_io())
    }
}

#[test]
fn ten_thousand_resource_catalog_reads_metadata_only() {
    futures::executor::block_on(async {
        let metadata = Arc::new(InMemoryMetadataStore::default());
        let publisher = Arc::new(RecordingPublisher::default());
        let dummy = BlobRef {
            provider_ref: "provider.no-io".to_owned(),
            object_ref: "blobs/sha256/dummy".to_owned(),
            digest: BlobDigest::sha256("0".repeat(64)).unwrap(),
            size: 1,
        };
        for index in 0..10_000 {
            metadata
                .seed(record(
                    &format!("resource.{index:05}"),
                    "revision.0",
                    dummy.clone(),
                ))
                .await
                .unwrap();
        }
        let authority = CloudResourceAuthority::new(Arc::new(NoIoBlobStore), metadata, publisher);
        let page = authority
            .list(&CatalogQuery {
                workspace_ref: "workspace.cloud".to_owned(),
                parent_ref: None,
                search: Some("resource.099".to_owned()),
                cursor: None,
                limit: 100,
                principal_ref: "user.owner".to_owned(),
            })
            .await
            .unwrap();
        assert_eq!(page.resources.len(), 100);
        assert!(
            page.resources
                .iter()
                .all(|item| item.resource_ref.contains("099"))
        );
    });
}

#[test]
fn materialization_is_revision_pinned_and_enforces_audience_generation_ttl() {
    futures::executor::block_on(async {
        let (_, _, _, authority) = fixture().await;
        let handle = authority
            .materialize(MaterializeRequest {
                workspace_ref: "workspace.cloud".to_owned(),
                resource_ref: "resource.a".to_owned(),
                principal_ref: "user.reader".to_owned(),
                audience: "plugin.viewer".to_owned(),
                generation: 7,
                now_ms: 1000,
                ttl_ms: 100,
            })
            .await
            .unwrap();
        assert_eq!(handle.revision, "revision.0");
        let updated = b"updated".to_vec();
        let mut updated_stream = VecBlobStream::new(updated.clone(), 2);
        authority
            .commit(
                commit_request("after-handle", "revision.0", &updated),
                &mut updated_stream,
            )
            .await
            .unwrap();
        let mut pinned = authority
            .read_handle(
                context("read-pinned"),
                &handle.handle_ref,
                "plugin.viewer",
                7,
                1001,
                Some(ByteRange {
                    offset: 1,
                    length: 3,
                }),
            )
            .await
            .unwrap()
            .body;
        let mut pinned_bytes = Vec::new();
        while let Some(chunk) = pinned.next_chunk(1024).await.unwrap() {
            pinned_bytes.extend(chunk);
        }
        assert_eq!(pinned_bytes, b"nit");
        let denied = authority
            .read_handle(
                context("read-denied"),
                &handle.handle_ref,
                "plugin.other",
                7,
                1001,
                None,
            )
            .await
            .err()
            .unwrap();
        assert_eq!(denied.code, AuthorityErrorCode::Denied);
        let stale = authority
            .read_handle(
                context("read-stale"),
                &handle.handle_ref,
                "plugin.viewer",
                8,
                1001,
                None,
            )
            .await
            .err()
            .unwrap();
        assert_eq!(stale.code, AuthorityErrorCode::StaleGeneration);
        let expired = authority
            .read_handle(
                context("read-expired"),
                &handle.handle_ref,
                "plugin.viewer",
                7,
                1100,
                None,
            )
            .await
            .err()
            .unwrap();
        assert_eq!(expired.code, AuthorityErrorCode::Expired);
    });
}
