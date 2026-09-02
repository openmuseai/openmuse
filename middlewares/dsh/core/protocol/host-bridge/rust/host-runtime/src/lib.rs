use std::{
    collections::BTreeSet,
    sync::{
        Arc,
        atomic::{AtomicU64, Ordering},
    },
    time::{SystemTime, UNIX_EPOCH},
};

use async_trait::async_trait;
use muse_host_bridge_contract::{
    Envelope, NegotiatedProtocol, ProtocolLimits, ProtocolSchemas,
    ids::{ApprovalId, HostSessionId},
};
use muse_host_policy::{
    Effect as PolicyEffect, FinalizeOutcome, HostPolicy, PolicyError, PolicySubject, PolicyVerdict,
};
use muse_host_events::{EventHubConfig, EventHubError, EventRead, HostEventHub, HostEventRecord};
use muse_host_registry::{
    AuthoritativeCaller, Cancellation, DiscoverQuery, HostCapabilityRegistry, InvokeRequest,
    InvalidationCause, RegistryError, RegistryEventKind, ScopeHint,
};
use muse_host_transport::{
    DesktopEventReceiver, DesktopRequestHandler, TransportCancellation, TransportError,
};
use serde_json::{Map, Value, json};

#[async_trait]
pub trait RuntimeCallerResolver: Send + Sync + 'static {
    /// Resolve from carrier-authenticated runtime identity and current Host authority only.
    async fn caller(&self, runtime_instance_id: &str)
    -> Result<AuthoritativeCaller, RegistryError>;
}

pub struct BridgeRequestDispatcher {
    registry: Arc<HostCapabilityRegistry>,
    callers: Arc<dyn RuntimeCallerResolver>,
    schemas: ProtocolSchemas,
    limits: ProtocolLimits,
    host_session_id: HostSessionId,
    host_generation: String,
    policy: Option<Arc<HostPolicy>>,
    events: Arc<HostEventHub>,
    subscription_ids: AtomicU64,
    registry_forwarder: tokio::task::JoinHandle<()>,
}

impl BridgeRequestDispatcher {
    pub fn new(
        registry: Arc<HostCapabilityRegistry>,
        callers: Arc<dyn RuntimeCallerResolver>,
        host_generation: impl Into<String>,
        host_session_id: impl Into<String>,
    ) -> Result<Self, TransportError> {
        let events = Arc::new(
            HostEventHub::new(EventHubConfig::default()).map_err(|_| TransportError::Handler)?,
        );
        Self::new_with_events(registry, callers, host_generation, host_session_id, events)
    }

    pub fn new_with_events(
        registry: Arc<HostCapabilityRegistry>,
        callers: Arc<dyn RuntimeCallerResolver>,
        host_generation: impl Into<String>,
        host_session_id: impl Into<String>,
        events: Arc<HostEventHub>,
    ) -> Result<Self, TransportError> {
        let mut registry_events = registry.subscribe();
        let forward_events = events.clone();
        let registry_forwarder = tokio::spawn(async move {
            loop {
                match registry_events.recv().await {
                    Ok(event) => {
                        if let Some(data) = registry_event_data(event.kind) {
                            let _ = forward_events.publish(event.occurred_at_ms, data);
                        }
                    }
                    Err(tokio::sync::broadcast::error::RecvError::Lagged(_)) => continue,
                    Err(tokio::sync::broadcast::error::RecvError::Closed) => break,
                }
            }
        });
        Ok(Self {
            registry,
            callers,
            schemas: ProtocolSchemas::new().map_err(|_| TransportError::Handler)?,
            limits: ProtocolLimits::default(),
            host_session_id: host_session_id
                .into()
                .try_into()
                .map_err(|_| TransportError::Handler)?,
            host_generation: host_generation.into(),
            policy: None,
            events,
            subscription_ids: AtomicU64::new(1),
            registry_forwarder,
        })
    }

    pub fn with_policy(mut self, policy: Arc<HostPolicy>) -> Self {
        self.policy = Some(policy);
        self
    }

    pub fn events(&self) -> Arc<HostEventHub> {
        self.events.clone()
    }

    fn next_subscription_id(&self) -> String {
        format!(
            "subscription.{}",
            self.subscription_ids.fetch_add(1, Ordering::AcqRel)
        )
    }

    async fn subscribe(
        &self,
        message: Value,
        runtime_instance_id: &str,
        cancellation: TransportCancellation,
    ) -> Result<DesktopEventReceiver, TransportError> {
        let bytes = serde_json::to_vec(&message).map_err(|_| TransportError::InvalidFrame)?;
        let negotiated = NegotiatedProtocol {
            major: 1,
            minor: 0,
            host_session_id: self.host_session_id.clone(),
        };
        let request = self
            .schemas
            .decode_message(&bytes, Some(&negotiated), self.limits)
            .map_err(|_| TransportError::InvalidFrame)?;
        if request.kind != "subscribe.request" {
            return Err(TransportError::InvalidFrame);
        }
        self.callers
            .caller(runtime_instance_id)
            .await
            .map_err(registry_transport)?;
        let filters = SubscriptionFilters::parse(&request.payload)?;
        let after_cursor = optional_string(&request.payload, "afterCursor")?
            .map(|value| parse_cursor(&value))
            .transpose()?;
        let subscription_id = self.next_subscription_id();
        let (mut subscription, gap) = match self.events.subscribe(after_cursor) {
            Ok(subscription) => (subscription, false),
            Err(EventHubError::CursorExpired) => (
                self.events
                    .subscribe(None)
                    .map_err(|_| TransportError::Handler)?,
                true,
            ),
            Err(EventHubError::InvalidPayload) => return Err(TransportError::Handler),
        };
        let mut start = json!({
            "subscriptionId": subscription_id,
            "startCursor": cursor(subscription.head_cursor),
            "retentionFromCursor": cursor(subscription.retention_from_cursor)
        });
        if subscription.head_cursor == 0 {
            start["startCursor"] = json!("cursor.0");
            start["retentionFromCursor"] = json!("cursor.0");
        }
        let acknowledgement = self.success(&request, "subscribe.response", start, true)?;
        let host_session_id = self.host_session_id.to_string();
        let host_generation = self.host_generation.clone();
        let (sender, receiver) = tokio::sync::mpsc::channel(64);
        tokio::spawn(async move {
            if sender.send(Ok(acknowledgement)).await.is_err() {
                return;
            }
            if gap {
                let record = HostEventRecord {
                    cursor: subscription.head_cursor,
                    occurred_at_ms: now_ms(),
                    data: json!({
                        "eventKind": "stream.gap",
                        "reason": "cursor_expired",
                        "afterCursor": after_cursor.map(cursor)
                    }),
                };
                if sender
                    .send(Ok(event_envelope(
                        &host_session_id,
                        &host_generation,
                        &subscription_id,
                        record,
                    )))
                    .await
                    .is_err()
                {
                    return;
                }
            }
            loop {
                tokio::select! {
                    _ = cancellation.cancelled() => break,
                    read = subscription.next() => match read {
                        EventRead::Event(record) => {
                            if filters.accepts(&record.data)
                                && sender.send(Ok(event_envelope(
                                    &host_session_id,
                                    &host_generation,
                                    &subscription_id,
                                    record,
                                ))).await.is_err()
                            {
                                break;
                            }
                        }
                        EventRead::Gap { .. } => {
                            let record = HostEventRecord {
                                cursor: subscription.head_cursor,
                                occurred_at_ms: now_ms(),
                                data: json!({"eventKind": "stream.gap", "reason": "retention_gap"}),
                            };
                            let _ = sender.send(Ok(event_envelope(
                                &host_session_id,
                                &host_generation,
                                &subscription_id,
                                record,
                            ))).await;
                            break;
                        }
                        EventRead::Closed => break,
                    }
                }
            }
        });
        Ok(receiver)
    }

    async fn dispatch(
        &self,
        message: Value,
        runtime_instance_id: &str,
        transport_cancel: TransportCancellation,
    ) -> Result<Value, TransportError> {
        let bytes = serde_json::to_vec(&message).map_err(|_| TransportError::InvalidFrame)?;
        let kind = message
            .get("kind")
            .and_then(Value::as_str)
            .unwrap_or_default();
        let negotiated =
            (!matches!(kind, "hello.request" | "hello.response")).then(|| NegotiatedProtocol {
                major: 1,
                minor: 0,
                host_session_id: self.host_session_id.clone(),
            });
        let request = self
            .schemas
            .decode_message(&bytes, negotiated.as_ref(), self.limits)
            .map_err(|error| match error.bridge_code() {
                "HOST_SESSION_EXPIRED" => TransportError::HostGenerationStale,
                "INVALID_ENVELOPE" | "HANDSHAKE_REQUIRED" | "UNSUPPORTED_PROTOCOL" => {
                    TransportError::InvalidFrame
                }
                _ => TransportError::Handler,
            })?;
        if request.kind == "hello.request" {
            let announced = request
                .payload
                .pointer("/runtime/instanceId")
                .and_then(Value::as_str);
            if announced != Some(runtime_instance_id) {
                return Err(TransportError::Unauthenticated);
            }
            return self.success(
                &request,
                "hello.response",
                json!({
                    "selectedVersion": {"major": 1, "minor": 0}, "features": [],
                    "limits": limits_wire(), "hostSessionId": self.host_session_id,
                    "hostGeneration": self.host_generation, "serverTime": now_ms(),
                    "clockSkewToleranceMs": 1000, "authorityRevision": "authority.current.1"
                }),
                false,
            );
        }
        let caller = self
            .callers
            .caller(runtime_instance_id)
            .await
            .map_err(registry_transport)?;
        match request.kind.as_str() {
            "discover.request" => {
                let page = self
                    .registry
                    .discover(
                        &caller,
                        DiscoverQuery {
                            families: strings(&request.payload, "families")?,
                            scope_hint: scope_hint(&request.payload)?,
                            page_size: usize_field(&request.payload, "pageSize")?,
                            cursor: optional_string(&request.payload, "cursor")?,
                        },
                    )
                    .await;
                match page {
                    Ok(page) => {
                        let descriptors = page
                            .descriptors
                            .iter()
                            .map(|item| item.descriptor.to_wire(&item.provider_instance_id))
                            .collect::<Result<Vec<_>, _>>()
                            .map_err(registry_transport)?;
                        let mut value = json!({"registryRevision": page.registry_revision, "descriptors": descriptors});
                        if let Some(cursor) = page.next_cursor {
                            value["nextCursor"] = json!(cursor);
                        }
                        self.success(&request, "discover.response", value, true)
                    }
                    Err(error) => self.failure(&request, "discover.response", error),
                }
            }
            "bind.request" => {
                let operations = strings(&request.payload, "operationIds")?;
                let result = self
                    .registry
                    .bind(
                        &caller,
                        &required_string(&request.payload, "descriptorId")?,
                        &required_string(&request.payload, "descriptorRevision")?,
                        operations,
                        scope_hint(&request.payload)?,
                    )
                    .await;
                match result {
                    Ok(binding) => {
                        let wire = self
                            .registry
                            .binding_to_wire(&binding.binding_id)
                            .await
                            .map_err(registry_transport)?;
                        self.success(&request, "bind.response", wire, true)
                    }
                    Err(error) => self.failure(&request, "bind.response", error),
                }
            }
            "invoke.request" => {
                let cancellation = Cancellation::default();
                if transport_cancel.is_cancelled() {
                    cancellation.cancel();
                }
                // DSH carries correlation in the public envelope extension. Accept the mirrored
                // payload field for v1 clients, but never invent correlation for a write.
                let extension_correlation = correlation_optional(&request);
                let payload_correlation =
                    request.payload.get("clientCorrelation").and_then(|value| {
                        Some((
                            value.get("sessionRef")?.as_str()?.to_owned(),
                            value.get("toolCallRef")?.as_str()?.to_owned(),
                        ))
                    });
                let correlation = extension_correlation.or(payload_correlation);
                let result = self
                    .registry
                    .invoke(
                        &caller,
                        InvokeRequest {
                            binding_id: required_string(&request.payload, "bindingId")?,
                            operation_id: required_string(&request.payload, "operationId")?,
                            input: request
                                .payload
                                .get("input")
                                .cloned()
                                .ok_or(TransportError::InvalidFrame)?,
                            deadline_at_ms: u64_field(&request.payload, "deadlineAt")?,
                            cancellation,
                            grant_id: optional_string(&request.payload, "grantId")?,
                            idempotency_key: optional_string(&request.payload, "idempotencyKey")?,
                            session_ref: correlation.as_ref().map(|value| value.0.clone()),
                            tool_call_ref: correlation.as_ref().map(|value| value.1.clone()),
                        },
                    )
                    .await;
                match result {
                    Ok(value) => {
                        let trace = required_string(&request.payload, "traceId")?;
                        let binding = required_string(&request.payload, "bindingId")?;
                        let operation = required_string(&request.payload, "operationId")?;
                        let mut receipt = json!({
                            "receiptId": format!("receipt.{}", request.request_id.as_ref().map(ToString::to_string).unwrap_or_else(|| "unknown".into())),
                            "requestId": request.request_id, "traceId": trace,
                            "hostSessionId": self.host_session_id, "hostGeneration": self.host_generation,
                            "bindingId": binding, "operationId": operation,
                            "policyDecision": if request.payload.get("grantId").is_some() { "approval_required" } else { "allow" },
                            "status": "applied_local", "issuedAt": now_ms()
                        });
                        if let Some((session_ref, tool_call_ref)) = correlation {
                            receipt["clientCorrelation"] = json!({
                                "sessionRef": session_ref,
                                "toolCallRef": tool_call_ref
                            });
                        }
                        if let Some(idempotency_key) = request.payload.get("idempotencyKey") {
                            receipt["idempotencyKey"] = idempotency_key.clone();
                        }
                        self.response(
                            &request,
                            "invoke.response",
                            json!({"ok": true, "value": value, "receipt": receipt}),
                            true,
                        )
                    }
                    Err(error) => self.failure(&request, "invoke.response", error),
                }
            }
            "policy.evaluate.request" => {
                let Some(policy) = &self.policy else {
                    return self.failure_code(
                        &request,
                        "policy.evaluate.response",
                        "POLICY_DENIED",
                        "Host policy is not configured",
                        false,
                    );
                };
                let binding_id = required_string(&request.payload, "bindingId")?;
                let operation_id = required_string(&request.payload, "operationId")?;
                let context = self
                    .registry
                    .policy_context(&caller, &binding_id, &operation_id)
                    .await;
                let context = match context {
                    Ok(context) => context,
                    Err(error) => return self.failure(&request, "policy.evaluate.response", error),
                };
                let requested_effect = required_string(&request.payload, "effect")?;
                let actual_effect = match context.operation.effect {
                    muse_host_registry::Effect::Read => "read",
                    muse_host_registry::Effect::LocalWrite => "local_write",
                    muse_host_registry::Effect::SyncWrite => "sync_write",
                    muse_host_registry::Effect::ExternalSideEffect => "external_side_effect",
                };
                if requested_effect != actual_effect {
                    return self.failure_code(
                        &request,
                        "policy.evaluate.response",
                        "POLICY_DENIED",
                        "requested effect does not match the bound operation",
                        false,
                    );
                }
                let correlation = correlation(&request)?;
                let subject = PolicySubject {
                    actor_ref: caller.actor_ref.clone(),
                    session_ref: correlation.0,
                    tool_call_ref: correlation.1,
                    scope_ref: context.scope_ref,
                    authority_epoch: context.authority_epoch,
                    binding_id: context.binding_id,
                    binding_generation: context.binding_generation,
                    operation_id,
                    input_digest: required_string(&request.payload, "inputDigest")?,
                    effect: if context.operation.effect == muse_host_registry::Effect::Read {
                        PolicyEffect::Read
                    } else {
                        PolicyEffect::Write
                    },
                };
                match policy
                    .evaluate(subject, u64_field(&request.payload, "deadlineAt")?)
                    .await
                {
                    Ok(evaluation) => {
                        let (decision, reason) = match evaluation.verdict {
                            PolicyVerdict::Allow => ("allow", None),
                            PolicyVerdict::Deny { reason } => ("deny", Some(reason)),
                            PolicyVerdict::ApprovalRequired { reason } => {
                                ("approval_required", Some(reason))
                            }
                        };
                        let mut value = json!({
                            "policyDecisionId": evaluation.policy_decision_id,
                            "decision": decision, "expiresAt": evaluation.expires_at_ms
                        });
                        if let Some(approval_id) = evaluation.approval_id {
                            value["approvalId"] = json!(approval_id);
                        }
                        if let Some(reason) = reason {
                            value["reason"] = json!(reason);
                        }
                        self.success(&request, "policy.evaluate.response", value, true)
                    }
                    Err(error) => self.policy_failure(&request, "policy.evaluate.response", error),
                }
            }
            "policy.finalize.request" => {
                let Some(policy) = &self.policy else {
                    return self.failure_code(
                        &request,
                        "policy.finalize.response",
                        "POLICY_DENIED",
                        "Host policy is not configured",
                        false,
                    );
                };
                let approval_id: ApprovalId = required_string(&request.payload, "approvalId")?
                    .parse()
                    .map_err(|_| TransportError::InvalidFrame)?;
                let outcome = match required_string(&request.payload, "outcome")?.as_str() {
                    "approved" => FinalizeOutcome::Approved,
                    "rejected" => FinalizeOutcome::Rejected,
                    _ => return Err(TransportError::InvalidFrame),
                };
                match policy
                    .finalize(
                        &approval_id,
                        outcome,
                        &required_string(&request.payload, "approvalProofId")?,
                        u64_field(&request.payload, "deadlineAt")?,
                    )
                    .await
                {
                    Ok(finalized) => {
                        let mut value = json!({
                            "policyDecisionId": finalized.policy_decision_id,
                            "outcome": if finalized.outcome == FinalizeOutcome::Approved { "approved" } else { "rejected" }
                        });
                        if let Some(grant) = finalized.grant_id {
                            value["grantId"] = json!(grant);
                        }
                        if let Some(expires) = finalized.expires_at_ms {
                            value["expiresAt"] = json!(expires);
                        }
                        self.success(&request, "policy.finalize.response", value, true)
                    }
                    Err(error) => self.policy_failure(&request, "policy.finalize.response", error),
                }
            }
            _ => Err(TransportError::InvalidFrame),
        }
    }

    fn success(
        &self,
        request: &Envelope,
        kind: &str,
        value: Value,
        session: bool,
    ) -> Result<Value, TransportError> {
        self.response(request, kind, json!({"ok": true, "value": value}), session)
    }
    fn failure(
        &self,
        request: &Envelope,
        kind: &str,
        error: RegistryError,
    ) -> Result<Value, TransportError> {
        self.response(
            request,
            kind,
            json!({"ok": false, "error": bridge_error(&error)}),
            true,
        )
    }
    fn failure_code(
        &self,
        request: &Envelope,
        kind: &str,
        code: &str,
        message: &str,
        retryable: bool,
    ) -> Result<Value, TransportError> {
        self.response(request, kind, json!({"ok": false, "error": {"kind": "bridge", "code": code, "message": message, "retryable": retryable}}), true)
    }
    fn policy_failure(
        &self,
        request: &Envelope,
        kind: &str,
        error: PolicyError,
    ) -> Result<Value, TransportError> {
        let (code, retryable) = match error {
            PolicyError::Deadline => ("DEADLINE_EXCEEDED", true),
            PolicyError::NotFound => ("APPROVAL_REJECTED", false),
            PolicyError::Expired => ("GRANT_EXPIRED", false),
            PolicyError::ProofRejected => ("APPROVAL_REJECTED", false),
            PolicyError::PendingLimit => ("RATE_LIMITED", true),
            _ => ("POLICY_DENIED", false),
        };
        self.failure_code(
            request,
            kind,
            code,
            "Host policy request was rejected",
            retryable,
        )
    }
    fn response(
        &self,
        request: &Envelope,
        kind: &str,
        payload: Value,
        session: bool,
    ) -> Result<Value, TransportError> {
        let envelope = Envelope {
            protocol: "muse-bridge".into(),
            major: 1,
            minor: 0,
            kind: kind.into(),
            request_id: request.request_id.clone(),
            host_session_id: session.then(|| self.host_session_id.clone()),
            sent_at: now_ms(),
            payload,
            extensions: None,
            unknown_informational: Map::new(),
        };
        let bytes = self
            .schemas
            .encode_message(&envelope, self.limits)
            .map_err(|_| TransportError::Handler)?;
        self.schemas
            .decode_message(
                &bytes,
                session
                    .then(|| NegotiatedProtocol {
                        major: 1,
                        minor: 0,
                        host_session_id: self.host_session_id.clone(),
                    })
                    .as_ref(),
                self.limits,
            )
            .map_err(|_| TransportError::Handler)?;
        serde_json::from_slice(&bytes).map_err(|_| TransportError::Handler)
    }
}

#[async_trait]
impl DesktopRequestHandler for BridgeRequestDispatcher {
    async fn handle(
        &self,
        message: Value,
        runtime_instance_id: &str,
        cancellation: TransportCancellation,
    ) -> Result<Value, TransportError> {
        self.dispatch(message, runtime_instance_id, cancellation)
            .await
    }

    async fn stream(
        &self,
        message: Value,
        runtime_instance_id: &str,
        cancellation: TransportCancellation,
    ) -> Result<DesktopEventReceiver, TransportError> {
        self.subscribe(message, runtime_instance_id, cancellation).await
    }
}

impl Drop for BridgeRequestDispatcher {
    fn drop(&mut self) {
        self.registry_forwarder.abort();
    }
}

struct SubscriptionFilters {
    event_kinds: BTreeSet<String>,
    provider_event_types: BTreeSet<String>,
    binding_ids: BTreeSet<String>,
}

impl SubscriptionFilters {
    fn parse(payload: &Value) -> Result<Self, TransportError> {
        let filters = payload
            .get("filters")
            .ok_or(TransportError::InvalidFrame)?;
        Ok(Self {
            event_kinds: strings(filters, "eventKinds")?,
            provider_event_types: strings(filters, "providerEventTypes")?,
            binding_ids: strings(filters, "bindingIds")?,
        })
    }

    fn accepts(&self, data: &Value) -> bool {
        let kind = data.get("eventKind").and_then(Value::as_str).unwrap_or_default();
        if !self.event_kinds.is_empty() && !self.event_kinds.contains(kind) {
            return false;
        }
        if kind == "provider.event" && !self.provider_event_types.is_empty() {
            if !data
                .get("eventType")
                .and_then(Value::as_str)
                .is_some_and(|value| self.provider_event_types.contains(value))
            {
                return false;
            }
        }
        if kind == "binding.invalidated" && !self.binding_ids.is_empty() {
            if !data
                .get("bindingId")
                .and_then(Value::as_str)
                .is_some_and(|value| self.binding_ids.contains(value))
            {
                return false;
            }
        }
        true
    }
}

fn registry_event_data(kind: RegistryEventKind) -> Option<Value> {
    match kind {
        RegistryEventKind::ProviderRegistered {
            provider_instance_id,
            registration_generation,
        } => Some(json!({
            "eventKind": "descriptor.changed",
            "descriptorId": format!("descriptor.{provider_instance_id}"),
            "revision": format!("revision.{registration_generation}"),
            "change": "added"
        })),
        RegistryEventKind::ProviderRevoked { provider_instance_id } => Some(json!({
            "eventKind": "descriptor.changed",
            "descriptorId": format!("descriptor.{provider_instance_id}"),
            "change": "removed"
        })),
        RegistryEventKind::BindingInvalidated { binding_id, cause } => Some(json!({
            "eventKind": "binding.invalidated",
            "bindingId": binding_id,
            "reason": match cause {
                InvalidationCause::ProviderDisposed => "revoked",
                InvalidationCause::AuthorityChanged => "scope_changed",
                InvalidationCause::Expired => "expired",
                InvalidationCause::Shutdown => "generation_changed",
            }
        })),
        RegistryEventKind::Shutdown => None,
    }
}

fn parse_cursor(value: &str) -> Result<u64, TransportError> {
    value
        .strip_prefix("cursor.")
        .and_then(|value| value.parse().ok())
        .ok_or(TransportError::InvalidFrame)
}

fn cursor(value: u64) -> String {
    format!("cursor.{value}")
}

fn event_envelope(
    host_session_id: &str,
    host_generation: &str,
    subscription_id: &str,
    record: HostEventRecord,
) -> Value {
    json!({
        "protocol": "muse-bridge",
        "major": 1,
        "minor": 0,
        "kind": "bridge.event",
        "hostSessionId": host_session_id,
        "sentAt": now_ms(),
        "payload": {
            "subscriptionId": subscription_id,
            "cursor": cursor(record.cursor),
            "occurredAt": record.occurred_at_ms,
            "hostGeneration": host_generation,
            "data": record.data
        }
    })
}

fn required_string(value: &Value, field: &str) -> Result<String, TransportError> {
    value
        .get(field)
        .and_then(Value::as_str)
        .map(str::to_owned)
        .ok_or(TransportError::InvalidFrame)
}
fn optional_string(value: &Value, field: &str) -> Result<Option<String>, TransportError> {
    match value.get(field) {
        None => Ok(None),
        Some(value) => value
            .as_str()
            .map(|v| Some(v.to_owned()))
            .ok_or(TransportError::InvalidFrame),
    }
}
fn u64_field(value: &Value, field: &str) -> Result<u64, TransportError> {
    value
        .get(field)
        .and_then(Value::as_u64)
        .ok_or(TransportError::InvalidFrame)
}
fn usize_field(value: &Value, field: &str) -> Result<usize, TransportError> {
    usize::try_from(u64_field(value, field)?).map_err(|_| TransportError::InvalidFrame)
}
fn strings(value: &Value, field: &str) -> Result<BTreeSet<String>, TransportError> {
    value
        .get(field)
        .map(|value| {
            value
                .as_array()
                .ok_or(TransportError::InvalidFrame)?
                .iter()
                .map(|value| {
                    value
                        .as_str()
                        .map(str::to_owned)
                        .ok_or(TransportError::InvalidFrame)
                })
                .collect()
        })
        .unwrap_or_else(|| Ok(BTreeSet::new()))
}
fn scope_hint(value: &Value) -> Result<Option<ScopeHint>, TransportError> {
    let Some(hint) = value.get("scopeHint") else {
        return Ok(None);
    };
    let refs = hint
        .get("refs")
        .and_then(Value::as_object)
        .ok_or(TransportError::InvalidFrame)?;
    Ok(Some(ScopeHint {
        refs: refs
            .iter()
            .map(|(key, value)| {
                value
                    .as_str()
                    .map(|value| (key.clone(), value.to_owned()))
                    .ok_or(TransportError::InvalidFrame)
            })
            .collect::<Result<_, _>>()?,
    }))
}
fn correlation(request: &Envelope) -> Result<(String, String), TransportError> {
    correlation_optional(request).ok_or(TransportError::InvalidFrame)
}
fn correlation_optional(request: &Envelope) -> Option<(String, String)> {
    let value = request
        .extensions
        .as_ref()
        .and_then(|extensions| extensions.get("muse.client-correlation"))?;
    Some((
        value.get("sessionRef")?.as_str()?.to_owned(),
        value.get("toolCallRef")?.as_str()?.to_owned(),
    ))
}
fn registry_transport(error: RegistryError) -> TransportError {
    match error {
        RegistryError::ResourceLimit => TransportError::RateLimited,
        RegistryError::Authority(_) => TransportError::Forbidden,
        _ => TransportError::Handler,
    }
}
fn bridge_error(error: &RegistryError) -> Value {
    let (code, retryable) = match error {
        RegistryError::DescriptorUnavailable => ("CAPABILITY_NOT_FOUND", false),
        RegistryError::StaleDescriptor => ("DESCRIPTOR_STALE", false),
        RegistryError::CursorExpired => ("CURSOR_EXPIRED", true),
        RegistryError::ResourceLimit => ("RATE_LIMITED", true),
        RegistryError::BindingUnavailable => ("BINDING_NOT_FOUND", false),
        RegistryError::BindingInvalid | RegistryError::ProviderRevoked => {
            ("BINDING_REVOKED", false)
        }
        RegistryError::OperationUnavailable => ("OPERATION_NOT_FOUND", false),
        RegistryError::InvalidInput | RegistryError::InvalidJson => ("INPUT_INVALID", false),
        RegistryError::InvalidOutput => ("OUTPUT_INVALID", false),
        RegistryError::GrantRequired => ("APPROVAL_REQUIRED", false),
        RegistryError::GrantInvalid => ("GRANT_INVALID", false),
        RegistryError::Authority(_) => ("FORBIDDEN", false),
        _ => ("INTERNAL", false),
    };
    json!({"kind": "bridge", "code": code, "message": error.to_string(), "retryable": retryable})
}
fn now_ms() -> u64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default()
        .as_millis()
        .min(u64::MAX as u128) as u64
}
fn limits_wire() -> Value {
    json!({
        "maxMessageBytes": 2097152, "maxInlineSchemaBytes": 65536, "maxResolvedSchemaBytes": 1048576,
        "maxDiscoverPageBytes": 524288, "maxDiscoverDescriptors": 256, "maxInputBytes": 262144,
        "maxOutputBytes": 1048576, "maxErrorDetailsBytes": 32768, "maxEventPayloadBytes": 262144,
        "maxJsonDepth": 64, "maxContainerChildren": 10000
    })
}
