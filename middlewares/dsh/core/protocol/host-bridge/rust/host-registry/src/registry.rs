use std::{
    collections::{BTreeMap, BTreeSet},
    fmt::Debug,
    sync::{
        Arc,
        atomic::{AtomicBool, AtomicU64, AtomicUsize, Ordering},
    },
    time::{SystemTime, UNIX_EPOCH},
};

use muse_host_bridge_contract::{ProtocolFragment, ProtocolSchemas};
use serde_json::{Value, json};
use tokio::sync::{Mutex, Notify, RwLock, broadcast};

use crate::{
    authority::{AuthoritativeCaller, AuthorityResolver, ResolvedHostContext, ScopeHint},
    descriptor::{CompiledOperationSchemas, ProviderDescriptor, validate_json},
    error::RegistryError,
    event::{InvalidationCause, RegistryEvent, RegistryEventKind},
    policy::{DenyWrites, InvocationAdmission, InvocationAuthorizer},
    provider::{Cancellation, CapabilityProvider, ProviderInvocation},
};

pub trait RegistryClock: Debug + Send + Sync + 'static {
    fn now_ms(&self) -> u64;
}

#[derive(Debug)]
pub struct SystemRegistryClock;
impl RegistryClock for SystemRegistryClock {
    fn now_ms(&self) -> u64 {
        SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap_or_default()
            .as_millis()
            .min(u64::MAX as u128) as u64
    }
}

#[derive(Debug, Clone)]
pub struct RegistryConfig {
    pub host_generation: String,
    pub binding_ttl_ms: u64,
    pub max_bindings: usize,
    pub snapshot_ttl_ms: u64,
    pub max_snapshots: usize,
    pub max_discover_page_size: usize,
    pub max_discover_descriptors: usize,
    pub max_discover_page_bytes: usize,
    pub clock: Arc<dyn RegistryClock>,
    pub invocation_authorizer: Arc<dyn InvocationAuthorizer>,
}
impl Default for RegistryConfig {
    fn default() -> Self {
        Self {
            host_generation: "host.1".into(),
            binding_ttl_ms: 300_000,
            max_bindings: 4096,
            snapshot_ttl_ms: 30_000,
            max_snapshots: 128,
            max_discover_page_size: 256,
            max_discover_descriptors: 256,
            max_discover_page_bytes: 512 * 1024,
            clock: Arc::new(SystemRegistryClock),
            invocation_authorizer: Arc::new(DenyWrites),
        }
    }
}

#[derive(Debug, Clone)]
pub struct DiscoverQuery {
    pub families: BTreeSet<String>,
    pub scope_hint: Option<ScopeHint>,
    pub page_size: usize,
    pub cursor: Option<String>,
}
#[derive(Debug, Clone)]
pub struct DiscoveredDescriptor {
    pub descriptor: ProviderDescriptor,
    pub provider_instance_id: String,
    pub registration_generation: u64,
}
#[derive(Debug, Clone)]
pub struct DescriptorPage {
    pub registry_revision: String,
    pub descriptors: Vec<DiscoveredDescriptor>,
    pub next_cursor: Option<String>,
}
#[derive(Debug, Clone)]
pub struct BindingProjection {
    pub binding_id: String,
    pub descriptor_id: String,
    pub descriptor_revision: String,
    pub provider_instance_id: String,
    pub host_generation: String,
    pub scope_ref: String,
    pub expires_at_ms: u64,
    pub operation_ids: BTreeSet<String>,
    /// Host-private registration generation; omitted by `binding_wire`.
    pub registration_generation: u64,
}
#[derive(Debug, Clone)]
pub struct InvokeRequest {
    pub binding_id: String,
    pub operation_id: String,
    pub input: Value,
    pub deadline_at_ms: u64,
    pub cancellation: Cancellation,
    pub grant_id: Option<String>,
    pub idempotency_key: Option<String>,
    pub session_ref: Option<String>,
    pub tool_call_ref: Option<String>,
}

#[derive(Debug, Clone)]
pub struct PolicyContextProjection {
    pub scope_ref: String,
    pub authority_epoch: u64,
    pub binding_id: String,
    pub binding_generation: u64,
    pub operation: crate::OperationDescriptor,
}

pub struct HostCapabilityRegistry {
    inner: Arc<Inner>,
}
struct Inner {
    authority: Arc<dyn AuthorityResolver>,
    config: RegistryConfig,
    protocol: ProtocolSchemas,
    state: RwLock<State>,
    counter: AtomicU64,
    generation: AtomicU64,
    events: broadcast::Sender<RegistryEvent>,
    shutdown: AtomicBool,
    shutdown_complete: AtomicBool,
    shutdown_notify: Notify,
}
#[derive(Default)]
struct State {
    revision: u64,
    registrations: BTreeMap<String, Arc<Registration>>,
    descriptor_index: BTreeMap<(String, String), String>,
    bindings: BTreeMap<String, Binding>,
    snapshots: BTreeMap<String, DiscoverySnapshot>,
}
struct Registration {
    provider_instance_id: String,
    generation: u64,
    descriptor: ProviderDescriptor,
    schemas: BTreeMap<String, CompiledOperationSchemas>,
    provider: Arc<dyn CapabilityProvider>,
    active: AtomicBool,
    in_flight: AtomicUsize,
    drained: Notify,
    dispose_lock: Mutex<()>,
    revocation_started: AtomicBool,
}
struct Binding {
    projection: BindingProjection,
    registration_id: String,
    registration_generation: u64,
    context: ResolvedHostContext,
    valid: bool,
}
#[derive(Clone)]
struct DiscoverySnapshot {
    context: ResolvedHostContext,
    registry_revision: String,
    descriptors: Vec<DiscoveredDescriptor>,
    next_offset: usize,
    page_size: usize,
    expires_at_ms: u64,
}

impl HostCapabilityRegistry {
    pub fn new(authority: Arc<dyn AuthorityResolver>, config: RegistryConfig) -> Self {
        let (events, _) = broadcast::channel(256);
        let protocol = ProtocolSchemas::new().expect("packaged Muse Bridge schemas must compile");
        Self {
            inner: Arc::new(Inner {
                authority,
                config,
                protocol,
                state: RwLock::new(State::default()),
                counter: AtomicU64::new(1),
                generation: AtomicU64::new(1),
                events,
                shutdown: AtomicBool::new(false),
                shutdown_complete: AtomicBool::new(false),
                shutdown_notify: Notify::new(),
            }),
        }
    }
    pub fn subscribe(&self) -> broadcast::Receiver<RegistryEvent> {
        self.inner.events.subscribe()
    }

    pub async fn register(
        &self,
        provider: Arc<dyn CapabilityProvider>,
    ) -> Result<RegistrationLease, RegistryError> {
        if self.inner.shutdown.load(Ordering::Acquire) {
            return Err(RegistryError::ProviderRevoked);
        }
        let descriptor = provider.descriptor();
        let id = self.mint("provider");
        let generation = self.inner.generation.fetch_add(1, Ordering::Relaxed);
        let compiled = descriptor.validate_and_compile(&self.inner.protocol, &id)?;
        let schemas = descriptor
            .operations
            .iter()
            .map(|operation| operation.operation_id.clone())
            .zip(compiled)
            .collect();
        let registration = Arc::new(Registration {
            provider_instance_id: id.clone(),
            generation,
            descriptor: descriptor.clone(),
            schemas,
            provider,
            active: AtomicBool::new(true),
            in_flight: AtomicUsize::new(0),
            drained: Notify::new(),
            dispose_lock: Mutex::new(()),
            revocation_started: AtomicBool::new(false),
        });
        let mut state = self.inner.state.write().await;
        if self.inner.shutdown.load(Ordering::Acquire) {
            return Err(RegistryError::ProviderRevoked);
        }
        let key = (
            descriptor.descriptor_id.clone(),
            descriptor.revision.clone(),
        );
        if state.descriptor_index.contains_key(&key) {
            return Err(RegistryError::DuplicateDescriptor);
        }
        state.descriptor_index.insert(key, id.clone());
        state.registrations.insert(id.clone(), registration.clone());
        let event = bump_event(
            &self.inner,
            &mut state,
            RegistryEventKind::ProviderRegistered {
                provider_instance_id: id.clone(),
                registration_generation: generation,
            },
        );
        drop(state);
        let _ = self.inner.events.send(event);
        Ok(RegistrationLease {
            inner: self.inner.clone(),
            entry: registration,
            registration_id: id,
            disposed: AtomicBool::new(false),
        })
    }

    pub async fn discover(
        &self,
        caller: &AuthoritativeCaller,
        query: DiscoverQuery,
    ) -> Result<DescriptorPage, RegistryError> {
        let now_ms = self.inner.config.clock.now_ms();
        self.sweep_expired(now_ms).await;
        if query.page_size == 0 || query.page_size > self.inner.config.max_discover_page_size {
            return Err(RegistryError::ResourceLimit);
        }
        if let Some(cursor) = query.cursor {
            let snapshot = self
                .inner
                .state
                .read()
                .await
                .snapshots
                .get(&cursor)
                .cloned()
                .ok_or(RegistryError::CursorExpired)?;
            if snapshot.expires_at_ms <= now_ms {
                self.inner.state.write().await.snapshots.remove(&cursor);
                return Err(RegistryError::CursorExpired);
            }
            let current = self
                .inner
                .authority
                .revalidate(caller, &snapshot.context)
                .await?;
            if current.actor_ref != snapshot.context.actor_ref {
                return Err(RegistryError::CursorExpired);
            }
            if current.scope_ref != snapshot.context.scope_ref
                || current.authority_epoch != snapshot.context.authority_epoch
            {
                self.inner.state.write().await.snapshots.remove(&cursor);
                return Err(RegistryError::CursorExpired);
            }
            return self.page_snapshot(cursor, snapshot).await;
        }
        let context = self
            .inner
            .authority
            .resolve(caller, query.scope_hint.as_ref())
            .await?;
        let (revision, registrations): (_, Vec<_>) = {
            let state = self.inner.state.read().await;
            (
                state.revision,
                state.registrations.values().cloned().collect(),
            )
        };
        let mut descriptors = Vec::new();
        for entry in registrations {
            if !entry.active.load(Ordering::Acquire)
                || (!query.families.is_empty()
                    && !query.families.contains(&entry.descriptor.family_id))
            {
                continue;
            }
            if entry.provider.available(&context).await.unwrap_or(false) {
                descriptors.push(DiscoveredDescriptor {
                    descriptor: entry.descriptor.clone(),
                    provider_instance_id: entry.provider_instance_id.clone(),
                    registration_generation: entry.generation,
                });
            }
        }
        descriptors.sort_by(|a, b| {
            (
                &a.descriptor.family_id,
                &a.descriptor.descriptor_id,
                &a.descriptor.revision,
            )
                .cmp(&(
                    &b.descriptor.family_id,
                    &b.descriptor.descriptor_id,
                    &b.descriptor.revision,
                ))
        });
        if descriptors.len() > self.inner.config.max_discover_descriptors {
            return Err(RegistryError::ResourceLimit);
        }
        let current = self.inner.authority.revalidate(caller, &context).await?;
        if current.actor_ref != context.actor_ref
            || current.scope_ref != context.scope_ref
            || current.authority_epoch != context.authority_epoch
        {
            return Err(RegistryError::BindingInvalid);
        }
        let snapshot = DiscoverySnapshot {
            context: current,
            registry_revision: format!("registry.{revision}"),
            descriptors,
            next_offset: 0,
            page_size: query.page_size,
            expires_at_ms: now_ms.saturating_add(self.inner.config.snapshot_ttl_ms),
        };
        self.page_snapshot(self.mint("cursor"), snapshot).await
    }

    async fn page_snapshot(
        &self,
        cursor: String,
        mut snapshot: DiscoverySnapshot,
    ) -> Result<DescriptorPage, RegistryError> {
        let start = snapshot.next_offset;
        let mut end = start;
        let hard_end = snapshot
            .descriptors
            .len()
            .min(start.saturating_add(snapshot.page_size));
        let mut bytes = 0usize;
        while end < hard_end {
            let value = snapshot.descriptors[end]
                .descriptor
                .to_wire(&snapshot.descriptors[end].provider_instance_id)?;
            let size = serde_json::to_vec(&value)
                .map_err(|_| RegistryError::InvalidDescriptor)?
                .len();
            if end == start && size > self.inner.config.max_discover_page_bytes {
                return Err(RegistryError::ResourceLimit);
            }
            if bytes.saturating_add(size) > self.inner.config.max_discover_page_bytes {
                break;
            }
            bytes += size;
            end += 1;
        }
        let descriptors = snapshot.descriptors[start..end].to_vec();
        snapshot.next_offset = end;
        let next_cursor = if end < snapshot.descriptors.len() {
            let mut state = self.inner.state.write().await;
            if !state.snapshots.contains_key(&cursor)
                && state.snapshots.len() >= self.inner.config.max_snapshots
            {
                return Err(RegistryError::ResourceLimit);
            }
            state.snapshots.insert(cursor.clone(), snapshot.clone());
            Some(cursor)
        } else {
            self.inner.state.write().await.snapshots.remove(&cursor);
            None
        };
        Ok(DescriptorPage {
            registry_revision: snapshot.registry_revision,
            descriptors,
            next_cursor,
        })
    }

    pub async fn bind(
        &self,
        caller: &AuthoritativeCaller,
        descriptor_id: &str,
        revision: &str,
        operation_ids: BTreeSet<String>,
        hint: Option<ScopeHint>,
    ) -> Result<BindingProjection, RegistryError> {
        let now_ms = self.inner.config.clock.now_ms();
        self.sweep_expired(now_ms).await;
        let context = self.inner.authority.resolve(caller, hint.as_ref()).await?;
        let entry = {
            let state = self.inner.state.read().await;
            let id = state
                .descriptor_index
                .get(&(descriptor_id.into(), revision.into()))
                .ok_or(RegistryError::DescriptorUnavailable)?;
            state
                .registrations
                .get(id)
                .cloned()
                .ok_or(RegistryError::DescriptorUnavailable)?
        };
        if !entry.active.load(Ordering::Acquire) {
            return Err(RegistryError::ProviderRevoked);
        }
        if !entry.provider.available(&context).await.unwrap_or(false) {
            return Err(RegistryError::DescriptorUnavailable);
        }
        if operation_ids.is_empty()
            || operation_ids
                .iter()
                .any(|id| entry.descriptor.operation(id).is_none())
        {
            return Err(RegistryError::OperationUnavailable);
        }
        let current = self.inner.authority.revalidate(caller, &context).await?;
        if current.actor_ref != context.actor_ref
            || current.scope_ref != context.scope_ref
            || current.authority_epoch != context.authority_epoch
        {
            return Err(RegistryError::BindingInvalid);
        }
        let binding_id = self.mint("binding");
        let projection = BindingProjection {
            binding_id: binding_id.clone(),
            descriptor_id: descriptor_id.into(),
            descriptor_revision: revision.into(),
            provider_instance_id: entry.provider_instance_id.clone(),
            host_generation: self.inner.config.host_generation.clone(),
            scope_ref: current.scope_ref.clone(),
            expires_at_ms: now_ms.saturating_add(self.inner.config.binding_ttl_ms),
            operation_ids,
            registration_generation: entry.generation,
        };
        self.inner
            .protocol
            .validate_fragment(
                ProtocolFragment::Binding,
                &binding_wire(&projection, &entry.descriptor),
            )
            .map_err(|_| RegistryError::InvalidDescriptor)?;
        let mut state = self.inner.state.write().await;
        if state.bindings.len() >= self.inner.config.max_bindings {
            return Err(RegistryError::BindingUnavailable);
        }
        if !entry.active.load(Ordering::Acquire)
            || !state
                .registrations
                .contains_key(&entry.provider_instance_id)
        {
            return Err(RegistryError::ProviderRevoked);
        }
        state.bindings.insert(
            binding_id,
            Binding {
                projection: projection.clone(),
                registration_id: entry.provider_instance_id.clone(),
                registration_generation: entry.generation,
                context: current,
                valid: true,
            },
        );
        Ok(projection)
    }

    /// Returns the already validated Bridge v1 binding projection without
    /// exposing Host-private authority or registration-generation state.
    pub async fn binding_to_wire(&self, binding_id: &str) -> Result<Value, RegistryError> {
        let state = self.inner.state.read().await;
        let binding = state
            .bindings
            .get(binding_id)
            .filter(|binding| binding.valid)
            .ok_or(RegistryError::BindingUnavailable)?;
        let entry = state
            .registrations
            .get(&binding.registration_id)
            .filter(|entry| entry.generation == binding.registration_generation)
            .ok_or(RegistryError::ProviderRevoked)?;
        Ok(binding_wire(&binding.projection, &entry.descriptor))
    }

    /// Revalidates Host authority and projects only the fields needed to bind a policy decision.
    /// Provider-private evidence never leaves the Registry.
    pub async fn policy_context(
        &self,
        caller: &AuthoritativeCaller,
        binding_id: &str,
        operation_id: &str,
    ) -> Result<PolicyContextProjection, RegistryError> {
        let previous = {
            let state = self.inner.state.read().await;
            let binding = state
                .bindings
                .get(binding_id)
                .filter(|binding| binding.valid)
                .ok_or(RegistryError::BindingUnavailable)?;
            if binding.context.actor_ref != caller.actor_ref
                || !binding.projection.operation_ids.contains(operation_id)
            {
                return Err(RegistryError::BindingInvalid);
            }
            binding.context.clone()
        };
        let current = self.inner.authority.revalidate(caller, &previous).await?;
        if current.actor_ref != previous.actor_ref
            || current.scope_ref != previous.scope_ref
            || current.authority_epoch != previous.authority_epoch
        {
            self.invalidate_binding(binding_id, InvalidationCause::AuthorityChanged)
                .await;
            return Err(RegistryError::BindingInvalid);
        }
        let state = self.inner.state.read().await;
        let binding = state
            .bindings
            .get(binding_id)
            .filter(|binding| binding.valid)
            .ok_or(RegistryError::BindingUnavailable)?;
        let entry = state
            .registrations
            .get(&binding.registration_id)
            .filter(|entry| {
                entry.generation == binding.registration_generation
                    && entry.active.load(Ordering::Acquire)
            })
            .ok_or(RegistryError::ProviderRevoked)?;
        let operation = entry
            .descriptor
            .operation(operation_id)
            .cloned()
            .ok_or(RegistryError::OperationUnavailable)?;
        Ok(PolicyContextProjection {
            scope_ref: current.scope_ref,
            authority_epoch: current.authority_epoch,
            binding_id: binding.projection.binding_id.clone(),
            binding_generation: binding.registration_generation,
            operation,
        })
    }

    pub async fn invoke(
        &self,
        caller: &AuthoritativeCaller,
        request: InvokeRequest,
    ) -> Result<Value, RegistryError> {
        let now_ms = self.inner.config.clock.now_ms();
        self.sweep_expired(now_ms).await;
        let previous = {
            let state = self.inner.state.read().await;
            let binding = state
                .bindings
                .get(&request.binding_id)
                .ok_or(RegistryError::BindingUnavailable)?;
            if !binding.valid {
                return Err(RegistryError::BindingInvalid);
            }
            if binding.context.actor_ref != caller.actor_ref {
                return Err(RegistryError::BindingInvalid);
            }
            binding.context.clone()
        };
        let current = self.inner.authority.revalidate(caller, &previous).await?;
        if current.actor_ref != previous.actor_ref {
            return Err(RegistryError::BindingInvalid);
        }
        if current.scope_ref != previous.scope_ref
            || current.authority_epoch != previous.authority_epoch
        {
            self.invalidate_binding(&request.binding_id, InvalidationCause::AuthorityChanged)
                .await;
            return Err(RegistryError::BindingInvalid);
        }
        let (entry, binding_projection, operation) = {
            let state = self.inner.state.read().await;
            let binding = state
                .bindings
                .get(&request.binding_id)
                .ok_or(RegistryError::BindingUnavailable)?;
            if !binding.valid {
                return Err(RegistryError::BindingInvalid);
            }
            if !binding
                .projection
                .operation_ids
                .contains(&request.operation_id)
            {
                return Err(RegistryError::OperationUnavailable);
            }
            let entry = state
                .registrations
                .get(&binding.registration_id)
                .cloned()
                .ok_or(RegistryError::ProviderRevoked)?;
            if entry.generation != binding.registration_generation {
                return Err(RegistryError::ProviderRevoked);
            }
            let operation = entry
                .descriptor
                .operation(&request.operation_id)
                .cloned()
                .ok_or(RegistryError::OperationUnavailable)?;
            (entry, binding.projection.clone(), operation)
        };
        validate_json(&request.input)?;
        entry
            .schemas
            .get(&request.operation_id)
            .ok_or(RegistryError::OperationUnavailable)?
            .input
            .validate(&request.input)
            .map_err(|_| RegistryError::InvalidInput)?;
        match operation.idempotency {
            crate::Idempotency::Required if request.idempotency_key.is_none() => {
                return Err(RegistryError::InvalidInput);
            }
            crate::Idempotency::None if request.idempotency_key.is_some() => {
                return Err(RegistryError::InvalidInput);
            }
            _ => {}
        }
        self.inner
            .config
            .invocation_authorizer
            .authorize(InvocationAdmission {
                caller,
                context: &current,
                binding: &binding_projection,
                operation: &operation,
                input: &request.input,
                grant_id: request.grant_id.as_deref(),
                idempotency_key: request.idempotency_key.as_deref(),
                session_ref: request.session_ref.as_deref(),
                tool_call_ref: request.tool_call_ref.as_deref(),
                deadline_at_ms: request.deadline_at_ms,
            })
            .await?;
        if !entry.provider.available(&current).await.unwrap_or(false) {
            return Err(RegistryError::BindingInvalid);
        }
        let _permit = {
            let state = self.inner.state.read().await;
            let binding = state
                .bindings
                .get(&request.binding_id)
                .ok_or(RegistryError::BindingUnavailable)?;
            if !binding.valid
                || binding.registration_id != entry.provider_instance_id
                || binding.registration_generation != entry.generation
                || binding.context.actor_ref != current.actor_ref
                || binding.context.scope_ref != current.scope_ref
                || binding.context.authority_epoch != current.authority_epoch
            {
                return Err(RegistryError::BindingInvalid);
            }
            ExecutionPermit::acquire(entry.clone())?
        };
        if request.cancellation.is_cancelled() || request.deadline_at_ms <= now_ms {
            return Err(RegistryError::BindingInvalid);
        }
        let output = entry
            .provider
            .invoke(
                ProviderInvocation {
                    binding_id: request.binding_id,
                    operation_id: request.operation_id.clone(),
                    input: request.input,
                    idempotency_key: request.idempotency_key,
                    deadline_at_ms: request.deadline_at_ms,
                },
                current,
                request.cancellation,
            )
            .await
            .map_err(|_| RegistryError::ProviderFailure)?;
        validate_json(&output)?;
        entry
            .schemas
            .get(&request.operation_id)
            .ok_or(RegistryError::OperationUnavailable)?
            .output
            .validate(&output)
            .map_err(|_| RegistryError::InvalidOutput)?;
        Ok(output)
    }

    pub async fn sweep_expired(&self, now: u64) {
        let ids: Vec<_> = {
            let state = self.inner.state.read().await;
            state
                .bindings
                .iter()
                .filter(|(_, value)| value.valid && value.projection.expires_at_ms <= now)
                .map(|(id, _)| id.clone())
                .collect()
        };
        for id in ids {
            invalidate_binding(&self.inner, &id, InvalidationCause::Expired).await;
        }
        let mut state = self.inner.state.write().await;
        state
            .bindings
            .retain(|_, value| value.valid || value.projection.expires_at_ms > now);
        state.snapshots.retain(|_, value| value.expires_at_ms > now);
    }
    pub async fn invalidate_actor(&self, actor_ref: &str) {
        let ids: Vec<_> = {
            let state = self.inner.state.read().await;
            state
                .bindings
                .iter()
                .filter(|(_, b)| b.context.actor_ref == actor_ref && b.valid)
                .map(|(id, _)| id.clone())
                .collect()
        };
        for id in ids {
            self.invalidate_binding(&id, InvalidationCause::AuthorityChanged)
                .await;
        }
    }
    pub async fn shutdown(&self) {
        if self.inner.shutdown.swap(true, Ordering::AcqRel) {
            wait_shutdown_complete(&self.inner).await;
            return;
        }
        let entries: Vec<_> = self
            .inner
            .state
            .read()
            .await
            .registrations
            .values()
            .cloned()
            .collect();
        for entry in &entries {
            revoke_registration(
                &self.inner,
                &entry.provider_instance_id,
                InvalidationCause::Shutdown,
            )
            .await;
        }
        for entry in &entries {
            wait_drained(entry).await;
        }
        let mut state = self.inner.state.write().await;
        state.snapshots.clear();
        let event = bump_event(&self.inner, &mut state, RegistryEventKind::Shutdown);
        drop(state);
        let _ = self.inner.events.send(event);
        self.inner.shutdown_complete.store(true, Ordering::Release);
        self.inner.shutdown_notify.notify_waiters();
    }
    fn mint(&self, prefix: &str) -> String {
        format!(
            "{prefix}.{}",
            self.inner.counter.fetch_add(1, Ordering::Relaxed)
        )
    }
    async fn invalidate_binding(&self, id: &str, cause: InvalidationCause) {
        invalidate_binding(&self.inner, id, cause).await;
    }
}

pub struct RegistrationLease {
    inner: Arc<Inner>,
    entry: Arc<Registration>,
    registration_id: String,
    disposed: AtomicBool,
}
impl RegistrationLease {
    pub async fn dispose(&self) {
        if !self.disposed.swap(true, Ordering::AcqRel) {
            revoke_registration(
                &self.inner,
                &self.registration_id,
                InvalidationCause::ProviderDisposed,
            )
            .await;
        }
        wait_drained(&self.entry).await;
    }
}
impl Drop for RegistrationLease {
    fn drop(&mut self) {
        if self.disposed.swap(true, Ordering::AcqRel) {
            return;
        }
        // Drop cannot await. Closing admission is synchronous; cleanup and terminal events are
        // delegated to the current runtime. Normal owners must still call dispose().await.
        self.entry.active.store(false, Ordering::Release);
        if let Ok(runtime) = tokio::runtime::Handle::try_current() {
            let inner = self.inner.clone();
            let registration_id = self.registration_id.clone();
            runtime.spawn(async move {
                revoke_registration(
                    &inner,
                    &registration_id,
                    InvalidationCause::ProviderDisposed,
                )
                .await;
            });
        }
    }
}
struct ExecutionPermit(Arc<Registration>);
impl ExecutionPermit {
    fn acquire(entry: Arc<Registration>) -> Result<Self, RegistryError> {
        if !entry.active.load(Ordering::Acquire) {
            return Err(RegistryError::ProviderRevoked);
        }
        entry.in_flight.fetch_add(1, Ordering::AcqRel);
        if !entry.active.load(Ordering::Acquire) {
            release_permit(&entry);
            return Err(RegistryError::ProviderRevoked);
        }
        Ok(Self(entry))
    }
}
impl Drop for ExecutionPermit {
    fn drop(&mut self) {
        release_permit(&self.0);
    }
}
fn release_permit(entry: &Registration) {
    if entry.in_flight.fetch_sub(1, Ordering::AcqRel) == 1 {
        entry.drained.notify_waiters();
    }
}

async fn wait_drained(entry: &Registration) {
    loop {
        let drained = entry.drained.notified();
        if entry.in_flight.load(Ordering::Acquire) == 0 {
            break;
        }
        drained.await;
    }
}

async fn revoke_registration(inner: &Arc<Inner>, id: &str, cause: InvalidationCause) {
    let entry = inner.state.read().await.registrations.get(id).cloned();
    let Some(entry) = entry else { return };
    let _guard = entry.dispose_lock.lock().await;
    if entry.revocation_started.swap(true, Ordering::AcqRel) {
        return;
    }
    entry.active.store(false, Ordering::Release);
    let ids: Vec<_> = {
        let state = inner.state.read().await;
        state
            .bindings
            .iter()
            .filter(|(_, b)| b.registration_id == id && b.valid)
            .map(|(id, _)| id.clone())
            .collect()
    };
    for binding_id in ids {
        invalidate_binding(inner, &binding_id, cause.clone()).await;
    }
    let mut state = inner.state.write().await;
    state.registrations.remove(id);
    state.descriptor_index.remove(&(
        entry.descriptor.descriptor_id.clone(),
        entry.descriptor.revision.clone(),
    ));
    let event = bump_event(
        inner,
        &mut state,
        RegistryEventKind::ProviderRevoked {
            provider_instance_id: id.into(),
        },
    );
    drop(state);
    let _ = inner.events.send(event);
}
async fn invalidate_binding(inner: &Arc<Inner>, id: &str, cause: InvalidationCause) {
    let mut state = inner.state.write().await;
    let Some(binding) = state.bindings.get_mut(id) else {
        return;
    };
    if !binding.valid {
        return;
    }
    binding.valid = false;
    let event = bump_event(
        inner,
        &mut state,
        RegistryEventKind::BindingInvalidated {
            binding_id: id.into(),
            cause,
        },
    );
    drop(state);
    let _ = inner.events.send(event);
}
fn bump_event(inner: &Inner, state: &mut State, kind: RegistryEventKind) -> RegistryEvent {
    state.revision = state.revision.saturating_add(1);
    RegistryEvent {
        host_generation: inner.config.host_generation.clone(),
        registry_revision: state.revision,
        occurred_at_ms: inner.config.clock.now_ms(),
        kind,
    }
}
async fn wait_shutdown_complete(inner: &Inner) {
    loop {
        let notified = inner.shutdown_notify.notified();
        if inner.shutdown_complete.load(Ordering::Acquire) {
            return;
        }
        notified.await;
    }
}
fn binding_wire(binding: &BindingProjection, descriptor: &ProviderDescriptor) -> Value {
    let operations: Vec<_> = binding.operation_ids.iter().filter_map(|id| descriptor.operation(id)).map(|op| json!({
        "operationId": op.operation_id,
        "effect": match op.effect { crate::Effect::Read => "read", crate::Effect::LocalWrite => "local_write", crate::Effect::SyncWrite => "sync_write", crate::Effect::ExternalSideEffect => "external_side_effect" },
        "inputSchemaDigest": op.input_schema.digest, "outputSchemaDigest": op.output_schema.digest,
        "cancellable": op.cancellable,
        "idempotency": match op.idempotency { crate::Idempotency::None => "none", crate::Idempotency::Optional => "optional", crate::Idempotency::Required => "required" },
    })).collect();
    json!({"bindingId": binding.binding_id, "descriptorId": binding.descriptor_id, "descriptorRevision": binding.descriptor_revision, "providerInstanceId": binding.provider_instance_id, "hostGeneration": binding.host_generation, "scopeRef": binding.scope_ref, "expiresAt": binding.expires_at_ms, "operations": operations})
}
