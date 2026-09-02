use std::{
    collections::HashMap,
    sync::{Arc, Mutex},
};

use muse_host_events::{EventHubError, HostEventHub};
use serde::{Deserialize, Serialize};
use serde_json::{Value, json};
use uuid::Uuid;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ContextStoreConfig {
    pub max_surfaces: usize,
    pub max_contexts_per_surface: usize,
    pub max_payload_bytes: usize,
    pub max_ttl_ms: u64,
}

impl Default for ContextStoreConfig {
    fn default() -> Self {
        Self {
            max_surfaces: 128,
            max_contexts_per_surface: 32,
            max_payload_bytes: 64 * 1024,
            max_ttl_ms: 60_000,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SurfaceBinding {
    pub plugin_id: String,
    pub facet_instance_ref: String,
    pub surface_kind: String,
    pub actor_ref: String,
    pub workspace_ref: String,
    pub scope_ref: String,
    pub window_ref: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SurfaceLease {
    pub surface_instance_ref: String,
    pub generation: u64,
    secret: Uuid,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum SurfaceLifecycle {
    Background,
    Active,
    Focused,
    Closed,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ContextInput {
    pub context_type: String,
    pub schema_digest: String,
    pub revision: u64,
    pub epoch_ref: String,
    pub ttl_ms: u64,
    pub payload: Value,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ContextSnapshot {
    pub context_type: String,
    pub schema_digest: String,
    pub revision: u64,
    pub epoch_ref: String,
    pub expires_at_ms: u64,
    pub payload: Value,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SurfaceSnapshot {
    pub surface_instance_ref: String,
    pub binding: SurfaceBinding,
    pub lifecycle: SurfaceLifecycle,
    pub contexts: Vec<ContextSnapshot>,
}

#[derive(Debug, thiserror::Error, PartialEq, Eq)]
pub enum ContextStoreError {
    #[error("invalid or stale surface lease")]
    InvalidLease,
    #[error("surface is closed")]
    SurfaceClosed,
    #[error("actor, workspace or scope does not match")]
    ScopeMismatch,
    #[error("context revision is stale")]
    StaleRevision,
    #[error("configured resource limit was exceeded")]
    ResourceLimit,
    #[error("context input is invalid")]
    InvalidContext,
    #[error("event publication failed")]
    EventPublication,
}

impl From<EventHubError> for ContextStoreError {
    fn from(_: EventHubError) -> Self {
        Self::EventPublication
    }
}

struct SurfaceRecord {
    binding: SurfaceBinding,
    lease: SurfaceLease,
    lifecycle: SurfaceLifecycle,
    contexts: HashMap<String, ContextSnapshot>,
}

struct State {
    next_generation: u64,
    surfaces: HashMap<String, SurfaceRecord>,
    focused_by_window: HashMap<(String, String), String>,
}

#[derive(Clone)]
pub struct SurfaceContextStore {
    config: ContextStoreConfig,
    events: HostEventHub,
    state: Arc<Mutex<State>>,
}

impl SurfaceContextStore {
    pub fn new(config: ContextStoreConfig, events: HostEventHub) -> Result<Self, ContextStoreError> {
        if config.max_surfaces == 0
            || config.max_contexts_per_surface == 0
            || config.max_payload_bytes == 0
            || config.max_ttl_ms == 0
        {
            return Err(ContextStoreError::ResourceLimit);
        }
        Ok(Self {
            config,
            events,
            state: Arc::new(Mutex::new(State {
                next_generation: 1,
                surfaces: HashMap::new(),
                focused_by_window: HashMap::new(),
            })),
        })
    }

    pub fn bind_surface(
        &self,
        binding: SurfaceBinding,
        now_ms: u64,
    ) -> Result<SurfaceLease, ContextStoreError> {
        validate_binding(&binding)?;
        let (lease, event) = {
            let mut state = self.state.lock().expect("context store poisoned");
            if state.surfaces.len() >= self.config.max_surfaces {
                return Err(ContextStoreError::ResourceLimit);
            }
            let generation = state.next_generation;
            state.next_generation = state.next_generation.saturating_add(1);
            let surface_instance_ref = format!("surface.{}", Uuid::new_v4());
            let lease = SurfaceLease {
                surface_instance_ref: surface_instance_ref.clone(),
                generation,
                secret: Uuid::new_v4(),
            };
            state.surfaces.insert(
                surface_instance_ref.clone(),
                SurfaceRecord {
                    binding: binding.clone(),
                    lease: lease.clone(),
                    lifecycle: SurfaceLifecycle::Background,
                    contexts: HashMap::new(),
                },
            );
            (lease, json!({
                "eventKind": "provider.event",
                "eventType": "surface.opened",
                "surfaceInstanceRef": surface_instance_ref,
                "pluginId": binding.plugin_id,
                "scopeRef": binding.scope_ref,
            }))
        };
        self.events.publish(now_ms, event)?;
        Ok(lease)
    }

    pub fn transition(
        &self,
        lease: &SurfaceLease,
        lifecycle: SurfaceLifecycle,
        now_ms: u64,
    ) -> Result<(), ContextStoreError> {
        if lifecycle == SurfaceLifecycle::Closed {
            return self.close(lease, now_ms);
        }
        let event = {
            let mut state = self.state.lock().expect("context store poisoned");
            let (workspace, window, current_lifecycle) = {
                let record = checked_record(&state, lease)?;
                (
                    record.binding.workspace_ref.clone(),
                    record.binding.window_ref.clone(),
                    record.lifecycle,
                )
            };
            if current_lifecycle == SurfaceLifecycle::Closed {
                return Err(ContextStoreError::SurfaceClosed);
            }
            if lifecycle == SurfaceLifecycle::Focused {
                let key = (workspace, window);
                if let Some(previous) = state
                    .focused_by_window
                    .insert(key, lease.surface_instance_ref.clone())
                    && previous != lease.surface_instance_ref
                    && let Some(record) = state.surfaces.get_mut(&previous)
                {
                    record.lifecycle = SurfaceLifecycle::Active;
                }
            }
            checked_record_mut(&mut state, lease)?.lifecycle = lifecycle;
            json!({
                "eventKind": "provider.event",
                "eventType": "surface.transitioned",
                "surfaceInstanceRef": lease.surface_instance_ref,
                "lifecycle": lifecycle,
            })
        };
        self.events.publish(now_ms, event)?;
        Ok(())
    }

    pub fn publish(
        &self,
        lease: &SurfaceLease,
        actor_ref: &str,
        workspace_ref: &str,
        scope_ref: &str,
        input: ContextInput,
        now_ms: u64,
    ) -> Result<ContextSnapshot, ContextStoreError> {
        validate_context(&input, self.config.max_payload_bytes)?;
        let snapshot = {
            let mut state = self.state.lock().expect("context store poisoned");
            let record = checked_record_mut(&mut state, lease)?;
            if record.lifecycle == SurfaceLifecycle::Closed {
                return Err(ContextStoreError::SurfaceClosed);
            }
            if record.binding.actor_ref != actor_ref
                || record.binding.workspace_ref != workspace_ref
                || record.binding.scope_ref != scope_ref
            {
                return Err(ContextStoreError::ScopeMismatch);
            }
            if let Some(previous) = record.contexts.get(&input.context_type)
                && previous.epoch_ref == input.epoch_ref
                && input.revision <= previous.revision
            {
                return Err(ContextStoreError::StaleRevision);
            }
            if !record.contexts.contains_key(&input.context_type)
                && record.contexts.len() >= self.config.max_contexts_per_surface
            {
                return Err(ContextStoreError::ResourceLimit);
            }
            let snapshot = ContextSnapshot {
                context_type: input.context_type.clone(),
                schema_digest: input.schema_digest,
                revision: input.revision,
                epoch_ref: input.epoch_ref,
                expires_at_ms: now_ms.saturating_add(input.ttl_ms.min(self.config.max_ttl_ms)),
                payload: input.payload,
            };
            record
                .contexts
                .insert(input.context_type, snapshot.clone());
            snapshot
        };
        self.events.publish(
            now_ms,
            json!({
                "eventKind": "provider.event",
                "eventType": "context.updated",
                "surfaceInstanceRef": lease.surface_instance_ref,
                "context": snapshot,
            }),
        )?;
        Ok(snapshot)
    }

    pub fn snapshot(
        &self,
        actor_ref: &str,
        workspace_ref: &str,
        scope_ref: Option<&str>,
        now_ms: u64,
    ) -> Vec<SurfaceSnapshot> {
        let mut state = self.state.lock().expect("context store poisoned");
        expire_locked(&mut state, now_ms);
        state
            .surfaces
            .values()
            .filter(|record| {
                record.binding.actor_ref == actor_ref
                    && record.binding.workspace_ref == workspace_ref
                    && scope_ref.is_none_or(|scope| record.binding.scope_ref == scope)
                    && record.lifecycle != SurfaceLifecycle::Closed
            })
            .map(|record| SurfaceSnapshot {
                surface_instance_ref: record.lease.surface_instance_ref.clone(),
                binding: record.binding.clone(),
                lifecycle: record.lifecycle,
                contexts: record.contexts.values().cloned().collect(),
            })
            .collect()
    }

    pub fn close(&self, lease: &SurfaceLease, now_ms: u64) -> Result<(), ContextStoreError> {
        let removed = {
            let mut state = self.state.lock().expect("context store poisoned");
            checked_record(&state, lease)?;
            let record = state
                .surfaces
                .remove(&lease.surface_instance_ref)
                .expect("checked surface exists");
            state
                .focused_by_window
                .retain(|_, surface| surface != &lease.surface_instance_ref);
            record
        };
        self.events.publish(
            now_ms,
            json!({
                "eventKind": "provider.event",
                "eventType": "surface.closed",
                "surfaceInstanceRef": lease.surface_instance_ref,
                "pluginId": removed.binding.plugin_id,
                "scopeRef": removed.binding.scope_ref,
            }),
        )?;
        Ok(())
    }
}

fn checked_record<'a>(
    state: &'a State,
    lease: &SurfaceLease,
) -> Result<&'a SurfaceRecord, ContextStoreError> {
    let record = state
        .surfaces
        .get(&lease.surface_instance_ref)
        .ok_or(ContextStoreError::InvalidLease)?;
    if record.lease.generation != lease.generation || record.lease.secret != lease.secret {
        return Err(ContextStoreError::InvalidLease);
    }
    Ok(record)
}

fn checked_record_mut<'a>(
    state: &'a mut State,
    lease: &SurfaceLease,
) -> Result<&'a mut SurfaceRecord, ContextStoreError> {
    let record = state
        .surfaces
        .get_mut(&lease.surface_instance_ref)
        .ok_or(ContextStoreError::InvalidLease)?;
    if record.lease.generation != lease.generation || record.lease.secret != lease.secret {
        return Err(ContextStoreError::InvalidLease);
    }
    Ok(record)
}

fn validate_binding(binding: &SurfaceBinding) -> Result<(), ContextStoreError> {
    if [
        &binding.plugin_id,
        &binding.facet_instance_ref,
        &binding.surface_kind,
        &binding.actor_ref,
        &binding.workspace_ref,
        &binding.scope_ref,
        &binding.window_ref,
    ]
    .iter()
    .any(|value| value.trim().is_empty())
    {
        return Err(ContextStoreError::InvalidContext);
    }
    Ok(())
}

fn validate_context(input: &ContextInput, max_bytes: usize) -> Result<(), ContextStoreError> {
    if input.context_type.trim().is_empty()
        || input.epoch_ref.trim().is_empty()
        || !input.schema_digest.starts_with("sha256:")
        || input.schema_digest.len() != 71
        || input.ttl_ms == 0
        || serde_json::to_vec(&input.payload)
            .map_err(|_| ContextStoreError::InvalidContext)?
            .len()
            > max_bytes
    {
        return Err(ContextStoreError::InvalidContext);
    }
    Ok(())
}

fn expire_locked(state: &mut State, now_ms: u64) {
    for record in state.surfaces.values_mut() {
        record
            .contexts
            .retain(|_, context| context.expires_at_ms > now_ms);
    }
}
