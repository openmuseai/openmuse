use async_trait::async_trait;
use openmuse_contract::{PrincipalKind, PrincipalRef};
use openmuse_storage_contract::{
    BlobRef, BlobStorePort, ObjectRequest, PutObjectRequest, StorageRequestContext,
};
use openmuse_storage_portability::{
    AccessMediation, InMemoryMigrationStateStore, MigratedResourceBinding, MigrationStateStorePort,
    MigrationStatus, PortabilityError, PortabilityErrorCode, PortabilityMetadataPort,
    PortabilityResult, PortableResourcePage, PortableResourceRecord, ProviderAccessBrokerPort,
    ProviderConfiguration, ProviderSwitchReceipt, StartMigrationRequest, StoragePortabilityService,
    WorkspaceProviderBinding,
};
use openmuse_storage_tck::{InMemoryBlobStore, VecBlobStream, digest};
use std::collections::{HashMap, HashSet};
use std::sync::{Arc, Mutex};

fn context(request_id: &str) -> StorageRequestContext {
    StorageRequestContext {
        request_id: request_id.to_owned(),
        actor: PrincipalRef {
            principal_ref: "user.owner".to_owned(),
            kind: PrincipalKind::User,
        },
        caller: PrincipalRef {
            principal_ref: "plugin.storage-portability".to_owned(),
            kind: PrincipalKind::Plugin,
        },
        policy_decision_ref: format!("decision.{request_id}"),
        workspace_ref: "workspace.a".to_owned(),
        generation: 7,
        deadline_at_ms: 4_102_444_800_000,
        cancellation_ref: format!("cancel.{request_id}"),
    }
}

struct BrokerEntry {
    configuration: ProviderConfiguration,
    store: Arc<dyn BlobStorePort>,
}

#[derive(Default)]
struct FakeAccessBroker {
    entries: Mutex<HashMap<String, BrokerEntry>>,
    revoked: Mutex<HashSet<(String, u64)>>,
    resolve_calls: Mutex<usize>,
}

impl FakeAccessBroker {
    fn register(
        &self,
        provider_ref: &str,
        generation: u64,
        mediation: AccessMediation,
        store: Arc<InMemoryBlobStore>,
    ) {
        self.entries.lock().unwrap().insert(
            provider_ref.to_owned(),
            BrokerEntry {
                configuration: ProviderConfiguration {
                    provider_ref: provider_ref.to_owned(),
                    access_generation: generation,
                    mediation,
                    capabilities: store.capabilities(),
                },
                store,
            },
        );
    }

    fn revoke(&self, provider_ref: &str, generation: u64) {
        self.revoked
            .lock()
            .unwrap()
            .insert((provider_ref.to_owned(), generation));
    }

    fn check(&self, provider_ref: &str, generation: u64) -> PortabilityResult<()> {
        if self
            .revoked
            .lock()
            .unwrap()
            .contains(&(provider_ref.to_owned(), generation))
        {
            return Err(PortabilityError::new(
                PortabilityErrorCode::Denied,
                "provider access generation is revoked",
                false,
            ));
        }
        let entries = self.entries.lock().unwrap();
        let entry = entries.get(provider_ref).ok_or_else(|| {
            PortabilityError::new(PortabilityErrorCode::NotFound, "provider not found", false)
        })?;
        if entry.configuration.access_generation != generation {
            return Err(PortabilityError::new(
                PortabilityErrorCode::Denied,
                "provider access generation is stale",
                false,
            ));
        }
        Ok(())
    }
}

#[async_trait]
impl ProviderAccessBrokerPort for FakeAccessBroker {
    async fn configuration(
        &self,
        provider_ref: &str,
        expected_access_generation: u64,
    ) -> PortabilityResult<ProviderConfiguration> {
        self.check(provider_ref, expected_access_generation)?;
        Ok(self.entries.lock().unwrap()[provider_ref]
            .configuration
            .clone())
    }

    async fn resolve(
        &self,
        provider_ref: &str,
        expected_access_generation: u64,
    ) -> PortabilityResult<Arc<dyn BlobStorePort>> {
        self.check(provider_ref, expected_access_generation)?;
        *self.resolve_calls.lock().unwrap() += 1;
        Ok(self.entries.lock().unwrap()[provider_ref].store.clone())
    }
}

struct FakeMetadata {
    binding: Mutex<WorkspaceProviderBinding>,
    resources: Mutex<Vec<PortableResourceRecord>>,
    switch_sequence: Mutex<u64>,
    source_delete_receipts: Mutex<Vec<String>>,
}

impl FakeMetadata {
    fn new(provider_ref: &str, resources: Vec<PortableResourceRecord>) -> Self {
        Self {
            binding: Mutex::new(WorkspaceProviderBinding {
                workspace_ref: "workspace.a".to_owned(),
                provider_ref: provider_ref.to_owned(),
                generation: 1,
            }),
            resources: Mutex::new(resources),
            switch_sequence: Mutex::new(0),
            source_delete_receipts: Mutex::new(Vec::new()),
        }
    }

    fn simulate_write(&self, expected_generation: u64) -> PortabilityResult<()> {
        if self.binding.lock().unwrap().generation != expected_generation {
            return Err(PortabilityError::new(
                PortabilityErrorCode::Conflict,
                "provider generation changed",
                false,
            ));
        }
        Ok(())
    }
}

#[async_trait]
impl PortabilityMetadataPort for FakeMetadata {
    async fn binding(&self, workspace_ref: &str) -> PortabilityResult<WorkspaceProviderBinding> {
        let binding = self.binding.lock().unwrap().clone();
        if binding.workspace_ref != workspace_ref {
            return Err(PortabilityError::new(
                PortabilityErrorCode::NotFound,
                "workspace not found",
                false,
            ));
        }
        Ok(binding)
    }

    async fn list_resources(
        &self,
        _workspace_ref: &str,
        cursor: Option<&str>,
        limit: usize,
    ) -> PortabilityResult<PortableResourcePage> {
        let resources = self.resources.lock().unwrap();
        let start = cursor
            .and_then(|cursor| cursor.parse::<usize>().ok())
            .unwrap_or(0);
        let page: Vec<_> = resources.iter().skip(start).take(limit).cloned().collect();
        let next = start + page.len();
        Ok(PortableResourcePage {
            resources: page,
            next_cursor: (next < resources.len()).then(|| next.to_string()),
        })
    }

    async fn switch_provider(
        &self,
        workspace_ref: &str,
        expected_generation: u64,
        target_provider_ref: &str,
        _migration_ref: &str,
        replacements: &[MigratedResourceBinding],
    ) -> PortabilityResult<ProviderSwitchReceipt> {
        let mut binding = self.binding.lock().unwrap();
        if binding.workspace_ref != workspace_ref || binding.generation != expected_generation {
            return Err(PortabilityError::new(
                PortabilityErrorCode::Conflict,
                "provider binding CAS failed",
                false,
            ));
        }
        let mut resources = self.resources.lock().unwrap();
        for replacement in replacements {
            let current = resources
                .iter()
                .find(|resource| resource.resource_ref == replacement.resource_ref)
                .ok_or_else(|| {
                    PortabilityError::new(
                        PortabilityErrorCode::NotFound,
                        "resource disappeared during migration",
                        false,
                    )
                })?;
            if current.revision != replacement.expected_revision
                || replacement.blob_ref.provider_ref != target_provider_ref
            {
                return Err(PortabilityError::new(
                    PortabilityErrorCode::Conflict,
                    "resource revision or provider changed during migration",
                    false,
                ));
            }
        }
        let before_provider_ref = binding.provider_ref.clone();
        let before_generation = binding.generation;
        for replacement in replacements {
            resources
                .iter_mut()
                .find(|resource| resource.resource_ref == replacement.resource_ref)
                .unwrap()
                .blob_ref = replacement.blob_ref.clone();
        }
        binding.provider_ref = target_provider_ref.to_owned();
        binding.generation += 1;
        let mut sequence = self.switch_sequence.lock().unwrap();
        *sequence += 1;
        Ok(ProviderSwitchReceipt {
            metadata_receipt_ref: format!("metadata-switch.{sequence}"),
            workspace_ref: workspace_ref.to_owned(),
            before_provider_ref,
            after_provider_ref: target_provider_ref.to_owned(),
            before_generation,
            after_generation: binding.generation,
        })
    }

    async fn source_objects_safe_to_delete(
        &self,
        _workspace_ref: &str,
        _migration_ref: &str,
        candidate_object_refs: &[String],
    ) -> PortabilityResult<Vec<String>> {
        // This fixture has no cross-workspace references. A production metadata
        // authority performs reference counting before returning this subset.
        Ok(candidate_object_refs.to_vec())
    }

    async fn record_source_deleted(
        &self,
        _workspace_ref: &str,
        migration_ref: &str,
        provider_receipt_refs: &[String],
    ) -> PortabilityResult<String> {
        if provider_receipt_refs.is_empty() {
            return Err(PortabilityError::new(
                PortabilityErrorCode::InvalidState,
                "provider deletion receipts are required",
                false,
            ));
        }
        let receipt = format!("metadata-delete.{migration_ref}");
        self.source_delete_receipts
            .lock()
            .unwrap()
            .push(receipt.clone());
        Ok(receipt)
    }
}

async fn put(
    store: &InMemoryBlobStore,
    provider_ref: &str,
    object_ref: &str,
    bytes: &[u8],
) -> BlobRef {
    let expected_digest = digest(bytes);
    let mut stream = VecBlobStream::new(bytes.to_vec(), 3);
    store
        .put(
            PutObjectRequest {
                context: context("seed"),
                object_ref: object_ref.to_owned(),
                expected_digest: expected_digest.clone(),
                size: bytes.len() as u64,
                content_type: "text/plain".to_owned(),
                metadata: Default::default(),
                idempotency_key: format!("seed.{object_ref}"),
            },
            &mut stream,
        )
        .await
        .unwrap();
    BlobRef {
        provider_ref: provider_ref.to_owned(),
        object_ref: object_ref.to_owned(),
        digest: expected_digest,
        size: bytes.len() as u64,
    }
}

async fn fixture(
    mediation: AccessMediation,
) -> (
    Arc<FakeAccessBroker>,
    Arc<FakeMetadata>,
    Arc<InMemoryMigrationStateStore>,
    Arc<InMemoryBlobStore>,
    Arc<InMemoryBlobStore>,
    StoragePortabilityService,
) {
    let provider_a = Arc::new(InMemoryBlobStore::new("provider.a"));
    let provider_b = Arc::new(InMemoryBlobStore::new("provider.b"));
    let first = put(&provider_a, "provider.a", "source/one", b"one").await;
    let second = put(&provider_a, "provider.a", "source/two", b"two").await;
    let resources = vec![
        PortableResourceRecord {
            resource_ref: "resource.one".to_owned(),
            revision: "revision.1".to_owned(),
            media_type: "text/plain".to_owned(),
            blob_ref: first,
        },
        PortableResourceRecord {
            resource_ref: "resource.two".to_owned(),
            revision: "revision.2".to_owned(),
            media_type: "text/plain".to_owned(),
            blob_ref: second,
        },
    ];
    let broker = Arc::new(FakeAccessBroker::default());
    broker.register("provider.a", 11, mediation, provider_a.clone());
    broker.register("provider.b", 22, mediation, provider_b.clone());
    let metadata = Arc::new(FakeMetadata::new("provider.a", resources));
    let states = Arc::new(InMemoryMigrationStateStore::default());
    let service = StoragePortabilityService::new(broker.clone(), metadata.clone(), states.clone());
    (broker, metadata, states, provider_a, provider_b, service)
}

fn start_request(
    migration_ref: &str,
    source: &str,
    source_generation: u64,
    target: &str,
    target_generation: u64,
    mediation: AccessMediation,
) -> StartMigrationRequest {
    StartMigrationRequest {
        migration_ref: migration_ref.to_owned(),
        workspace_ref: "workspace.a".to_owned(),
        source_provider_ref: source.to_owned(),
        source_access_generation: source_generation,
        target_provider_ref: target.to_owned(),
        target_access_generation: target_generation,
        mediation,
        context: context("start"),
    }
}

#[test]
fn two_providers_migrate_in_both_directions_with_digest_verification() {
    futures::executor::block_on(async {
        let (broker, metadata, states, provider_a, provider_b, service) =
            fixture(AccessMediation::Server).await;
        service
            .start(start_request(
                "migration.a-to-b",
                "provider.a",
                11,
                "provider.b",
                22,
                AccessMediation::Server,
            ))
            .await
            .unwrap();
        assert_eq!(
            service
                .copy_batch("migration.a-to-b", &context("copy-ab"), 1)
                .await
                .unwrap(),
            1
        );

        let restarted = StoragePortabilityService::new(broker, metadata.clone(), states.clone());
        assert_eq!(
            restarted
                .copy_batch("migration.a-to-b", &context("resume-ab"), 10)
                .await
                .unwrap(),
            1
        );
        restarted.cutover("migration.a-to-b").await.unwrap();
        assert_eq!(
            metadata.binding("workspace.a").await.unwrap().provider_ref,
            "provider.b"
        );
        let resources_on_b = metadata.resources.lock().unwrap().clone();
        for resource in &resources_on_b {
            assert_eq!(resource.blob_ref.provider_ref, "provider.b");
            provider_b
                .head(ObjectRequest {
                    context: context("verify-b"),
                    object_ref: resource.blob_ref.object_ref.clone(),
                })
                .await
                .unwrap();
        }

        restarted
            .start(start_request(
                "migration.b-to-a",
                "provider.b",
                22,
                "provider.a",
                11,
                AccessMediation::Server,
            ))
            .await
            .unwrap();
        assert_eq!(
            restarted
                .copy_batch("migration.b-to-a", &context("copy-ba"), 10)
                .await
                .unwrap(),
            2
        );
        restarted.cutover("migration.b-to-a").await.unwrap();
        assert_eq!(
            metadata.binding("workspace.a").await.unwrap().provider_ref,
            "provider.a"
        );
        let resources_on_a = metadata.resources.lock().unwrap().clone();
        for resource in &resources_on_a {
            assert_eq!(resource.blob_ref.provider_ref, "provider.a");
            provider_a
                .head(ObjectRequest {
                    context: context("verify-a"),
                    object_ref: resource.blob_ref.object_ref.clone(),
                })
                .await
                .unwrap();
        }
    });
}

#[test]
fn migration_pauses_resumes_and_stops_after_credential_revocation() {
    futures::executor::block_on(async {
        let (broker, _metadata, _states, _a, _b, service) = fixture(AccessMediation::Desktop).await;
        service
            .start(start_request(
                "migration.pause",
                "provider.a",
                11,
                "provider.b",
                22,
                AccessMediation::Desktop,
            ))
            .await
            .unwrap();
        service.pause("migration.pause").await.unwrap();
        broker.revoke("provider.a", 11);
        let error = service.resume("migration.pause").await.unwrap_err();
        assert_eq!(error.code, PortabilityErrorCode::Denied);
        assert_eq!(*broker.resolve_calls.lock().unwrap(), 0);
    });
}

#[test]
fn mediation_mismatch_is_denied_before_any_provider_io() {
    futures::executor::block_on(async {
        let (broker, _metadata, _states, _a, _b, service) = fixture(AccessMediation::Server).await;
        let error = service
            .start(start_request(
                "migration.wrong-placement",
                "provider.a",
                11,
                "provider.b",
                22,
                AccessMediation::Desktop,
            ))
            .await
            .unwrap_err();
        assert_eq!(error.code, PortabilityErrorCode::Denied);
        assert_eq!(*broker.resolve_calls.lock().unwrap(), 0);
    });
}

#[test]
fn cutover_invalidates_late_generation_and_rollback_restores_source() {
    futures::executor::block_on(async {
        let (_broker, metadata, states, _a, _b, service) = fixture(AccessMediation::Server).await;
        service
            .start(start_request(
                "migration.rollback",
                "provider.a",
                11,
                "provider.b",
                22,
                AccessMediation::Server,
            ))
            .await
            .unwrap();
        service
            .copy_batch("migration.rollback", &context("copy"), 10)
            .await
            .unwrap();
        let cutover = service.cutover("migration.rollback").await.unwrap();
        assert_eq!(cutover.before_generation, 1);
        assert_eq!(
            metadata.simulate_write(1).unwrap_err().code,
            PortabilityErrorCode::Conflict
        );

        let rollback = service.rollback("migration.rollback").await.unwrap();
        assert_eq!(rollback.after_provider_ref, "provider.a");
        assert_eq!(
            states.load("migration.rollback").await.unwrap().status,
            MigrationStatus::RolledBack
        );
        assert!(
            metadata
                .resources
                .lock()
                .unwrap()
                .iter()
                .all(|resource| resource.blob_ref.provider_ref == "provider.a")
        );
    });
}

#[test]
fn export_is_credential_free_and_source_delete_has_dual_receipts() {
    futures::executor::block_on(async {
        let (_broker, metadata, states, provider_a, _b, service) =
            fixture(AccessMediation::Server).await;
        let manifest = service.export_manifest("workspace.a").await.unwrap();
        let json = serde_json::to_string(&manifest).unwrap();
        assert_eq!(manifest.schema, "openmuse.workspace-export@1");
        assert_eq!(manifest.resources.len(), 2);
        assert!(!json.contains("credential"));
        assert!(json.contains("revision.1"));

        service
            .start(start_request(
                "migration.delete",
                "provider.a",
                11,
                "provider.b",
                22,
                AccessMediation::Server,
            ))
            .await
            .unwrap();
        service
            .copy_batch("migration.delete", &context("copy"), 10)
            .await
            .unwrap();
        service.cutover("migration.delete").await.unwrap();
        let denied = service
            .confirm_source_deletion("migration.delete", &context("delete-denied"), false)
            .await
            .unwrap_err();
        assert_eq!(denied.code, PortabilityErrorCode::Denied);

        let receipt = service
            .confirm_source_deletion("migration.delete", &context("delete"), true)
            .await
            .unwrap();
        assert_eq!(receipt.provider_receipt_refs.len(), 2);
        assert!(receipt.metadata_receipt_ref.starts_with("metadata-delete."));
        assert_eq!(
            states.load("migration.delete").await.unwrap().status,
            MigrationStatus::Completed
        );
        assert_eq!(metadata.source_delete_receipts.lock().unwrap().len(), 1);
        let error = provider_a
            .head(ObjectRequest {
                context: context("head-deleted"),
                object_ref: "source/one".to_owned(),
            })
            .await
            .unwrap_err();
        assert_eq!(
            error.code,
            openmuse_storage_contract::StorageErrorCode::NotFound
        );
    });
}
