use std::{
    collections::{BTreeMap, BTreeSet},
    sync::{
        Arc,
        atomic::{AtomicBool, AtomicU64, Ordering},
    },
};

use async_trait::async_trait;
use muse_host_registry::*;
use serde_json::{Value, json};
use tokio::sync::{Barrier, Semaphore};

#[derive(Debug)]
struct TestClock(AtomicU64);
impl RegistryClock for TestClock {
    fn now_ms(&self) -> u64 {
        self.0.load(Ordering::Acquire)
    }
}
impl TestClock {
    fn set(&self, value: u64) {
        self.0.store(value, Ordering::Release);
    }
}

#[derive(Default)]
struct FakeAuthority {
    epoch: AtomicU64,
    deny_revalidation: AtomicBool,
}

#[async_trait]
impl AuthorityResolver for FakeAuthority {
    async fn resolve(
        &self,
        caller: &AuthoritativeCaller,
        hint: Option<&ScopeHint>,
    ) -> Result<ResolvedHostContext, AuthorityError> {
        let scope_ref = hint
            .and_then(|hint| hint.refs.get("host.scope"))
            .cloned()
            .unwrap_or_else(|| "scope.default".into());
        Ok(ResolvedHostContext {
            actor_ref: caller.actor_ref.clone(),
            scope_ref,
            authority_epoch: self.epoch.load(Ordering::Acquire),
            evidence: BTreeMap::new(),
        })
    }
    async fn revalidate(
        &self,
        caller: &AuthoritativeCaller,
        previous: &ResolvedHostContext,
    ) -> Result<ResolvedHostContext, AuthorityError> {
        if self.deny_revalidation.load(Ordering::Acquire) {
            return Err(AuthorityError::Denied);
        }
        Ok(ResolvedHostContext {
            actor_ref: caller.actor_ref.clone(),
            scope_ref: previous.scope_ref.clone(),
            authority_epoch: self.epoch.load(Ordering::Acquire),
            evidence: BTreeMap::new(),
        })
    }
}

struct FakeProvider {
    entered: Option<Arc<Barrier>>,
    release: Option<Arc<Semaphore>>,
}

struct NamedProvider {
    descriptor: ProviderDescriptor,
    output: Option<Value>,
}

struct GatedAvailabilityProvider {
    entered: Arc<Barrier>,
    release: Arc<Semaphore>,
}

struct WriteProvider {
    invoked: Arc<AtomicBool>,
}

#[async_trait]
impl CapabilityProvider for WriteProvider {
    fn descriptor(&self) -> ProviderDescriptor {
        ProviderDescriptor {
            descriptor_id: "descriptor.write".into(),
            revision: "rev.1".into(),
            family_id: "test.write".into(),
            contract_major: 1,
            contract_minor: 0,
            operations: vec![OperationDescriptor {
                operation_id: "test.write.apply".into(),
                effect: Effect::LocalWrite,
                input_schema: SchemaDocument::new(json!({"type": "object"})).unwrap(),
                output_schema: SchemaDocument::new(json!({"type": "object"})).unwrap(),
                cancellable: true,
                idempotency: Idempotency::Required,
            }],
            title: None,
            summary: None,
        }
    }

    async fn available(&self, _: &ResolvedHostContext) -> Result<bool, ProviderFailure> {
        Ok(true)
    }

    async fn invoke(
        &self,
        invocation: ProviderInvocation,
        _: ResolvedHostContext,
        _: Cancellation,
    ) -> Result<Value, ProviderFailure> {
        self.invoked.store(true, Ordering::Release);
        Ok(invocation.input)
    }
}

#[async_trait]
impl CapabilityProvider for GatedAvailabilityProvider {
    fn descriptor(&self) -> ProviderDescriptor {
        NamedProvider::new("descriptor.gated", "rev.1").descriptor
    }
    async fn available(&self, _: &ResolvedHostContext) -> Result<bool, ProviderFailure> {
        self.entered.wait().await;
        let permit = self.release.acquire().await.unwrap();
        permit.forget();
        Ok(true)
    }
    async fn invoke(
        &self,
        invocation: ProviderInvocation,
        _: ResolvedHostContext,
        _: Cancellation,
    ) -> Result<Value, ProviderFailure> {
        Ok(invocation.input)
    }
}

impl NamedProvider {
    fn new(descriptor_id: &str, revision: &str) -> Self {
        Self {
            descriptor: ProviderDescriptor {
                descriptor_id: descriptor_id.into(),
                revision: revision.into(),
                family_id: "test.named".into(),
                contract_major: 1,
                contract_minor: 0,
                operations: vec![OperationDescriptor {
                    operation_id: "test.named.run".into(),
                    effect: Effect::Read,
                    input_schema: SchemaDocument::new(
                        json!({"type": "object", "required": ["value"]}),
                    )
                    .unwrap(),
                    output_schema: SchemaDocument::new(
                        json!({"type": "object", "required": ["value"]}),
                    )
                    .unwrap(),
                    cancellable: true,
                    idempotency: Idempotency::None,
                }],
                title: None,
                summary: None,
            },
            output: None,
        }
    }
}

#[async_trait]
impl CapabilityProvider for NamedProvider {
    fn descriptor(&self) -> ProviderDescriptor {
        self.descriptor.clone()
    }
    async fn available(&self, _: &ResolvedHostContext) -> Result<bool, ProviderFailure> {
        Ok(true)
    }
    async fn invoke(
        &self,
        invocation: ProviderInvocation,
        _: ResolvedHostContext,
        _: Cancellation,
    ) -> Result<Value, ProviderFailure> {
        Ok(self.output.clone().unwrap_or(invocation.input))
    }
}
impl FakeProvider {
    fn immediate() -> Self {
        Self {
            entered: None,
            release: None,
        }
    }
}

#[async_trait]
impl CapabilityProvider for FakeProvider {
    fn descriptor(&self) -> ProviderDescriptor {
        ProviderDescriptor {
            descriptor_id: "descriptor.echo".into(),
            revision: "rev.1".into(),
            family_id: "test.echo".into(),
            contract_major: 1,
            contract_minor: 0,
            operations: vec![OperationDescriptor {
                operation_id: "test.echo.run".into(),
                effect: Effect::Read,
                input_schema: SchemaDocument::new(json!({"type": "object"})).unwrap(),
                output_schema: SchemaDocument::new(json!({"type": "object"})).unwrap(),
                cancellable: true,
                idempotency: Idempotency::None,
            }],
            title: None,
            summary: None,
        }
    }
    async fn available(&self, _: &ResolvedHostContext) -> Result<bool, ProviderFailure> {
        Ok(true)
    }
    async fn invoke(
        &self,
        invocation: ProviderInvocation,
        _: ResolvedHostContext,
        _: Cancellation,
    ) -> Result<Value, ProviderFailure> {
        if let Some(entered) = &self.entered {
            entered.wait().await;
        }
        if let Some(release) = &self.release {
            let permit = release.acquire().await.unwrap();
            permit.forget();
        }
        Ok(invocation.input)
    }
}

fn caller() -> AuthoritativeCaller {
    AuthoritativeCaller {
        actor_ref: "actor.1".into(),
    }
}
fn registry(authority: Arc<FakeAuthority>) -> HostCapabilityRegistry {
    registry_with_clock(authority).0
}
fn registry_with_clock(authority: Arc<FakeAuthority>) -> (HostCapabilityRegistry, Arc<TestClock>) {
    let clock = Arc::new(TestClock(AtomicU64::new(10)));
    let registry = HostCapabilityRegistry::new(
        authority,
        RegistryConfig {
            host_generation: "host.1".into(),
            binding_ttl_ms: 1000,
            max_bindings: 8,
            snapshot_ttl_ms: 100,
            max_snapshots: 8,
            max_discover_page_size: 8,
            max_discover_descriptors: 16,
            max_discover_page_bytes: 64 * 1024,
            clock: clock.clone(),
            invocation_authorizer: Arc::new(muse_host_registry::DenyWrites),
        },
    );
    (registry, clock)
}
fn operations() -> BTreeSet<String> {
    ["test.echo.run".into()].into_iter().collect()
}
fn named_operations() -> BTreeSet<String> {
    ["test.named.run".into()].into_iter().collect()
}
fn discover_query(page_size: usize, cursor: Option<String>) -> DiscoverQuery {
    DiscoverQuery {
        families: BTreeSet::new(),
        scope_hint: None,
        page_size,
        cursor,
    }
}
fn invoke(binding_id: String, input: Value) -> InvokeRequest {
    InvokeRequest {
        binding_id,
        operation_id: "test.echo.run".into(),
        input,
        deadline_at_ms: 100,
        cancellation: Cancellation::default(),
        grant_id: None,
        idempotency_key: None,
        session_ref: None,
        tool_call_ref: None,
    }
}

#[tokio::test]
async fn registration_discovery_binding_and_invoke_are_exact() {
    let authority = Arc::new(FakeAuthority::default());
    let registry = registry(authority);
    let _lease = registry
        .register(Arc::new(FakeProvider::immediate()))
        .await
        .unwrap();
    let found = registry
        .discover(
            &caller(),
            DiscoverQuery {
                families: BTreeSet::new(),
                scope_hint: None,
                page_size: 8,
                cursor: None,
            },
        )
        .await
        .unwrap();
    assert_eq!(found.descriptors.len(), 1);
    assert_eq!(found.descriptors[0].descriptor.revision, "rev.1");
    let binding = registry
        .bind(&caller(), "descriptor.echo", "rev.1", operations(), None)
        .await
        .unwrap();
    let output = registry
        .invoke(
            &caller(),
            invoke(binding.binding_id.clone(), json!({"ok": true})),
        )
        .await
        .unwrap();
    assert_eq!(output, json!({"ok": true}));
    assert!(matches!(
        registry
            .bind(&caller(), "descriptor.echo", "rev.2", operations(), None)
            .await,
        Err(RegistryError::DescriptorUnavailable)
    ));
}

#[tokio::test]
async fn secure_default_rejects_write_before_provider_execution() {
    let registry = registry(Arc::new(FakeAuthority::default()));
    let invoked = Arc::new(AtomicBool::new(false));
    let _lease = registry
        .register(Arc::new(WriteProvider {
            invoked: invoked.clone(),
        }))
        .await
        .unwrap();
    let binding = registry
        .bind(
            &caller(),
            "descriptor.write",
            "rev.1",
            ["test.write.apply".into()].into_iter().collect(),
            None,
        )
        .await
        .unwrap();
    let error = registry
        .invoke(
            &caller(),
            InvokeRequest {
                binding_id: binding.binding_id,
                operation_id: "test.write.apply".into(),
                input: json!({}),
                deadline_at_ms: 100,
                cancellation: Cancellation::default(),
                grant_id: Some("grant.forged".into()),
                idempotency_key: Some("idem.1".into()),
                session_ref: Some("session.1".into()),
                tool_call_ref: Some("call.1".into()),
            },
        )
        .await
        .unwrap_err();
    assert_eq!(error, RegistryError::GrantRequired);
    assert!(!invoked.load(Ordering::Acquire));
}

#[tokio::test]
async fn authority_epoch_change_invalidates_old_binding() {
    let authority = Arc::new(FakeAuthority::default());
    let registry = registry(authority.clone());
    let _lease = registry
        .register(Arc::new(FakeProvider::immediate()))
        .await
        .unwrap();
    let binding = registry
        .bind(&caller(), "descriptor.echo", "rev.1", operations(), None)
        .await
        .unwrap();
    authority.epoch.store(1, Ordering::Release);
    let error = registry
        .invoke(&caller(), invoke(binding.binding_id.clone(), json!({})))
        .await
        .unwrap_err();
    assert_eq!(error, RegistryError::BindingInvalid);
    assert_eq!(
        registry
            .invoke(&caller(), invoke(binding.binding_id.clone(), json!({})))
            .await
            .unwrap_err(),
        RegistryError::BindingInvalid
    );
}

#[tokio::test]
async fn dispose_revokes_binding_and_emits_one_terminal_event() {
    let registry = registry(Arc::new(FakeAuthority::default()));
    let mut events = registry.subscribe();
    let lease = registry
        .register(Arc::new(FakeProvider::immediate()))
        .await
        .unwrap();
    let binding = registry
        .bind(&caller(), "descriptor.echo", "rev.1", operations(), None)
        .await
        .unwrap();
    lease.dispose().await;
    let mut invalidations = 0;
    while let Ok(event) = events.try_recv() {
        if matches!(event.kind, RegistryEventKind::BindingInvalidated { ref binding_id, .. } if binding_id == &binding.binding_id)
        {
            invalidations += 1;
        }
    }
    assert_eq!(invalidations, 1);
    assert_eq!(
        registry
            .invoke(&caller(), invoke(binding.binding_id.clone(), json!({})))
            .await
            .unwrap_err(),
        RegistryError::BindingInvalid
    );
}

#[tokio::test]
async fn dispose_waits_for_an_admitted_invocation_to_quiesce() {
    let authority = Arc::new(FakeAuthority::default());
    let registry = Arc::new(registry(authority));
    let entered = Arc::new(Barrier::new(2));
    let release = Arc::new(Semaphore::new(0));
    let lease = Arc::new(
        registry
            .register(Arc::new(FakeProvider {
                entered: Some(entered.clone()),
                release: Some(release.clone()),
            }))
            .await
            .unwrap(),
    );
    let binding = registry
        .bind(&caller(), "descriptor.echo", "rev.1", operations(), None)
        .await
        .unwrap();
    let invoke_registry = registry.clone();
    let invoke_binding = binding.binding_id.clone();
    let invoke = tokio::spawn(async move {
        invoke_registry
            .invoke(&caller(), invoke(invoke_binding, json!({"value": 1})))
            .await
    });
    entered.wait().await;
    let dispose_lease = lease.clone();
    let disposing = tokio::spawn(async move {
        dispose_lease.dispose().await;
    });
    assert!(!disposing.is_finished());
    release.add_permits(1);
    assert_eq!(invoke.await.unwrap().unwrap(), json!({"value": 1}));
    disposing.await.unwrap();
}

#[tokio::test]
async fn invalid_schema_digest_leaves_no_registration_residue() {
    let registry = registry(Arc::new(FakeAuthority::default()));
    let mut invalid = NamedProvider::new("descriptor.schema", "rev.1");
    invalid.descriptor.operations[0].input_schema.digest = format!("sha256:{}", "0".repeat(64));
    assert_eq!(
        registry.register(Arc::new(invalid)).await.err().unwrap(),
        RegistryError::InvalidSchema
    );
    let lease = registry
        .register(Arc::new(NamedProvider::new("descriptor.schema", "rev.1")))
        .await
        .unwrap();
    lease.dispose().await;
}

#[tokio::test]
async fn duplicate_is_rejected_but_identity_can_be_registered_after_dispose() {
    let registry = registry(Arc::new(FakeAuthority::default()));
    let first = registry
        .register(Arc::new(NamedProvider::new("descriptor.same", "rev.1")))
        .await
        .unwrap();
    assert_eq!(
        registry
            .register(Arc::new(NamedProvider::new("descriptor.same", "rev.1")))
            .await
            .err()
            .unwrap(),
        RegistryError::DuplicateDescriptor
    );
    first.dispose().await;
    let second = registry
        .register(Arc::new(NamedProvider::new("descriptor.same", "rev.1")))
        .await
        .unwrap();
    second.dispose().await;
}

#[tokio::test]
async fn discovery_pages_are_frozen_across_registry_mutation() {
    let registry = registry(Arc::new(FakeAuthority::default()));
    let _a = registry
        .register(Arc::new(NamedProvider::new("descriptor.a", "rev.1")))
        .await
        .unwrap();
    let b = registry
        .register(Arc::new(NamedProvider::new("descriptor.b", "rev.1")))
        .await
        .unwrap();
    let first = registry
        .discover(&caller(), discover_query(1, None))
        .await
        .unwrap();
    assert_eq!(
        first.descriptors[0].descriptor.descriptor_id,
        "descriptor.a"
    );
    let cursor = first.next_cursor.unwrap();
    b.dispose().await;
    let _c = registry
        .register(Arc::new(NamedProvider::new("descriptor.c", "rev.1")))
        .await
        .unwrap();
    let second = registry
        .discover(&caller(), discover_query(1, Some(cursor)))
        .await
        .unwrap();
    assert_eq!(
        second.descriptors[0].descriptor.descriptor_id,
        "descriptor.b"
    );
    assert_eq!(first.registry_revision, second.registry_revision);
    assert!(second.next_cursor.is_none());
}

#[tokio::test]
async fn cursor_expires_and_cannot_be_used_by_another_actor() {
    let (registry, clock) = registry_with_clock(Arc::new(FakeAuthority::default()));
    let _a = registry
        .register(Arc::new(NamedProvider::new("descriptor.a", "rev.1")))
        .await
        .unwrap();
    let _b = registry
        .register(Arc::new(NamedProvider::new("descriptor.b", "rev.1")))
        .await
        .unwrap();
    let cursor = registry
        .discover(&caller(), discover_query(1, None))
        .await
        .unwrap()
        .next_cursor
        .unwrap();
    let other = AuthoritativeCaller {
        actor_ref: "actor.2".into(),
    };
    assert_eq!(
        registry
            .discover(&other, discover_query(1, Some(cursor.clone())))
            .await
            .unwrap_err(),
        RegistryError::CursorExpired
    );
    clock.set(111);
    assert_eq!(
        registry
            .discover(&caller(), discover_query(1, Some(cursor.clone())))
            .await
            .unwrap_err(),
        RegistryError::CursorExpired
    );
}

#[tokio::test]
async fn expired_binding_is_cleaned_and_emits_exactly_once() {
    let registry = registry(Arc::new(FakeAuthority::default()));
    let mut events = registry.subscribe();
    let _lease = registry
        .register(Arc::new(FakeProvider::immediate()))
        .await
        .unwrap();
    let binding = registry
        .bind(&caller(), "descriptor.echo", "rev.1", operations(), None)
        .await
        .unwrap();
    registry.sweep_expired(1010).await;
    registry.sweep_expired(1011).await;
    assert_eq!(
        registry
            .invoke(&caller(), invoke(binding.binding_id.clone(), json!({})))
            .await
            .unwrap_err(),
        RegistryError::BindingUnavailable
    );
    let mut expired = 0;
    while let Ok(event) = events.try_recv() {
        if matches!(event.kind, RegistryEventKind::BindingInvalidated { ref binding_id, cause: InvalidationCause::Expired } if binding_id == &binding.binding_id)
        {
            expired += 1;
        }
    }
    assert_eq!(expired, 1);
}

#[tokio::test]
async fn operation_schemas_validate_both_input_and_provider_output() {
    let registry = registry(Arc::new(FakeAuthority::default()));
    let mut provider = NamedProvider::new("descriptor.schema", "rev.1");
    provider.output = Some(json!({}));
    let _lease = registry.register(Arc::new(provider)).await.unwrap();
    let binding = registry
        .bind(
            &caller(),
            "descriptor.schema",
            "rev.1",
            named_operations(),
            None,
        )
        .await
        .unwrap();
    let mut request = InvokeRequest {
        binding_id: binding.binding_id.clone(),
        operation_id: "test.named.run".into(),
        input: json!({}),
        deadline_at_ms: 100,
        cancellation: Cancellation::default(),
        grant_id: None,
        idempotency_key: None,
        session_ref: None,
        tool_call_ref: None,
    };
    assert_eq!(
        registry
            .invoke(&caller(), request.clone())
            .await
            .unwrap_err(),
        RegistryError::InvalidInput
    );
    request.input = json!({"value": 1});
    assert_eq!(
        registry.invoke(&caller(), request).await.unwrap_err(),
        RegistryError::InvalidOutput
    );
}

#[tokio::test]
async fn authority_is_revalidated_before_input_schema_is_disclosed() {
    let authority = Arc::new(FakeAuthority::default());
    let registry = registry(authority.clone());
    let _lease = registry
        .register(Arc::new(NamedProvider::new("descriptor.schema", "rev.1")))
        .await
        .unwrap();
    let binding = registry
        .bind(
            &caller(),
            "descriptor.schema",
            "rev.1",
            named_operations(),
            None,
        )
        .await
        .unwrap();
    authority.deny_revalidation.store(true, Ordering::Release);
    let request = InvokeRequest {
        binding_id: binding.binding_id,
        operation_id: "test.named.run".into(),
        input: json!({}),
        deadline_at_ms: 100,
        cancellation: Cancellation::default(),
        grant_id: None,
        idempotency_key: None,
        session_ref: None,
        tool_call_ref: None,
    };
    assert_eq!(
        registry.invoke(&caller(), request).await.unwrap_err(),
        RegistryError::Authority(AuthorityError::Denied)
    );
}

#[tokio::test]
async fn dropping_a_registration_lease_closes_admission_and_cleans_up() {
    let registry = registry(Arc::new(FakeAuthority::default()));
    let lease = registry
        .register(Arc::new(FakeProvider::immediate()))
        .await
        .unwrap();
    let binding = registry
        .bind(&caller(), "descriptor.echo", "rev.1", operations(), None)
        .await
        .unwrap();
    drop(lease);
    assert_eq!(
        registry
            .invoke(&caller(), invoke(binding.binding_id, json!({})))
            .await
            .unwrap_err(),
        RegistryError::ProviderRevoked
    );
    tokio::task::yield_now().await;
    let replacement = registry
        .register(Arc::new(FakeProvider::immediate()))
        .await
        .unwrap();
    replacement.dispose().await;
}

#[tokio::test]
async fn cancellation_and_deadline_stop_before_provider_execution() {
    let registry = registry(Arc::new(FakeAuthority::default()));
    let _lease = registry
        .register(Arc::new(FakeProvider::immediate()))
        .await
        .unwrap();
    let binding = registry
        .bind(&caller(), "descriptor.echo", "rev.1", operations(), None)
        .await
        .unwrap();
    let cancellation = Cancellation::default();
    cancellation.cancel();
    let cancelled = InvokeRequest {
        cancellation,
        ..invoke(binding.binding_id.clone(), json!({}))
    };
    assert_eq!(
        registry.invoke(&caller(), cancelled).await.unwrap_err(),
        RegistryError::BindingInvalid
    );
    let mut deadline = invoke(binding.binding_id, json!({}));
    deadline.deadline_at_ms = 10;
    assert_eq!(
        registry.invoke(&caller(), deadline).await.unwrap_err(),
        RegistryError::BindingInvalid
    );
}

#[tokio::test]
async fn another_actor_cannot_use_or_invalidate_a_binding() {
    let registry = registry(Arc::new(FakeAuthority::default()));
    let _lease = registry
        .register(Arc::new(FakeProvider::immediate()))
        .await
        .unwrap();
    let binding = registry
        .bind(&caller(), "descriptor.echo", "rev.1", operations(), None)
        .await
        .unwrap();
    let other = AuthoritativeCaller {
        actor_ref: "actor.2".into(),
    };
    assert_eq!(
        registry
            .invoke(&other, invoke(binding.binding_id.clone(), json!({})))
            .await
            .unwrap_err(),
        RegistryError::BindingInvalid
    );
    assert_eq!(
        registry
            .invoke(&caller(), invoke(binding.binding_id, json!({"ok": true})))
            .await
            .unwrap(),
        json!({"ok": true})
    );
}

#[tokio::test]
async fn old_binding_does_not_rebind_to_a_new_registration_generation() {
    let registry = registry(Arc::new(FakeAuthority::default()));
    let first = registry
        .register(Arc::new(FakeProvider::immediate()))
        .await
        .unwrap();
    let old = registry
        .bind(&caller(), "descriptor.echo", "rev.1", operations(), None)
        .await
        .unwrap();
    first.dispose().await;
    let _second = registry
        .register(Arc::new(FakeProvider::immediate()))
        .await
        .unwrap();
    assert_eq!(
        registry
            .invoke(&caller(), invoke(old.binding_id, json!({})))
            .await
            .unwrap_err(),
        RegistryError::BindingInvalid
    );
}

#[tokio::test]
async fn shutdown_is_idempotent_and_rejects_new_registrations() {
    let registry = registry(Arc::new(FakeAuthority::default()));
    let mut events = registry.subscribe();
    let _lease = registry
        .register(Arc::new(FakeProvider::immediate()))
        .await
        .unwrap();
    registry.shutdown().await;
    registry.shutdown().await;
    assert_eq!(
        registry
            .register(Arc::new(NamedProvider::new("descriptor.late", "rev.1")))
            .await
            .err()
            .unwrap(),
        RegistryError::ProviderRevoked
    );
    let mut shutdowns = 0;
    let mut revisions = Vec::new();
    while let Ok(event) = events.try_recv() {
        revisions.push(event.registry_revision);
        if matches!(event.kind, RegistryEventKind::Shutdown) {
            shutdowns += 1;
        }
    }
    assert_eq!(shutdowns, 1);
    assert!(revisions.windows(2).all(|pair| pair[0] < pair[1]));
}

#[tokio::test]
async fn concurrent_shutdown_callers_all_wait_for_in_flight_drain() {
    let registry = Arc::new(registry(Arc::new(FakeAuthority::default())));
    let entered = Arc::new(Barrier::new(2));
    let release = Arc::new(Semaphore::new(0));
    let _lease = registry
        .register(Arc::new(FakeProvider {
            entered: Some(entered.clone()),
            release: Some(release.clone()),
        }))
        .await
        .unwrap();
    let binding = registry
        .bind(&caller(), "descriptor.echo", "rev.1", operations(), None)
        .await
        .unwrap();
    let invoke_registry = registry.clone();
    let invocation = tokio::spawn(async move {
        invoke_registry
            .invoke(&caller(), invoke(binding.binding_id, json!({})))
            .await
    });
    entered.wait().await;
    let first_registry = registry.clone();
    let first = tokio::spawn(async move { first_registry.shutdown().await });
    tokio::task::yield_now().await;
    let second_registry = registry.clone();
    let second = tokio::spawn(async move { second_registry.shutdown().await });
    tokio::task::yield_now().await;
    assert!(!first.is_finished());
    assert!(!second.is_finished());
    release.add_permits(1);
    invocation.await.unwrap().unwrap();
    first.await.unwrap();
    second.await.unwrap();
}

#[tokio::test]
async fn bind_loses_cleanly_to_provider_dispose_at_the_linearization_point() {
    let registry = Arc::new(registry(Arc::new(FakeAuthority::default())));
    let entered = Arc::new(Barrier::new(2));
    let release = Arc::new(Semaphore::new(0));
    let lease = registry
        .register(Arc::new(GatedAvailabilityProvider {
            entered: entered.clone(),
            release: release.clone(),
        }))
        .await
        .unwrap();
    let binding_registry = registry.clone();
    let binding = tokio::spawn(async move {
        binding_registry
            .bind(
                &caller(),
                "descriptor.gated",
                "rev.1",
                named_operations(),
                None,
            )
            .await
    });
    entered.wait().await;
    lease.dispose().await;
    release.add_permits(1);
    assert_eq!(
        binding.await.unwrap().unwrap_err(),
        RegistryError::ProviderRevoked
    );
}
