//! Provider-neutral OpenMuse control-plane contract primitives.
//!
//! This crate owns only cross-domain wire semantics. Plugin, workspace,
//! storage, sandbox, and presentation business payloads stay in their own
//! domains and travel as opaque JSON values at this boundary.

use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::BTreeSet;

pub const PROTOCOL_NAME: &str = "openmuse.contract";
pub const MAX_SAFE_INTEGER: u64 = 9_007_199_254_740_991;
pub const PROTOCOL_VERSION: ProtocolVersion = ProtocolVersion {
    name: ProtocolName::OpenMuseContract,
    major: 1,
    minor: 0,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ProtocolName {
    #[serde(rename = "openmuse.contract")]
    OpenMuseContract,
}

impl ProtocolName {
    pub fn as_str(self) -> &'static str {
        PROTOCOL_NAME
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ProtocolVersion {
    pub name: ProtocolName,
    pub major: u16,
    pub minor: u16,
}

impl ProtocolVersion {
    pub fn accepts(self, peer: Self) -> bool {
        self.name == peer.name && self.major == peer.major && peer.minor <= self.minor
    }

    pub fn validate(self) -> Result<(), ContractViolation> {
        if PROTOCOL_VERSION.accepts(self) {
            return Ok(());
        }
        Err(ContractViolation::IncompatibleProtocol {
            actual_major: self.major,
            actual_minor: self.minor,
        })
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum PrincipalKind {
    User,
    Agent,
    Plugin,
    Service,
    Device,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct PrincipalRef {
    pub principal_ref: String,
    pub kind: PrincipalKind,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct ContractScope {
    pub authority_ref: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub workspace_ref: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_ref: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct RequestEnvelope {
    pub protocol: ProtocolVersion,
    pub kind: RequestKind,
    pub request_id: String,
    pub actor: PrincipalRef,
    pub caller: PrincipalRef,
    pub scope: ContractScope,
    pub generation: u64,
    pub deadline_at_ms: u64,
    pub cancellation_ref: String,
    pub operation: String,
    pub payload: Value,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RequestKind {
    Request,
}

impl RequestEnvelope {
    pub fn from_json_slice(
        bytes: &[u8],
        expected_generation: Option<u64>,
    ) -> Result<Self, ContractViolation> {
        let envelope: Self = serde_json::from_slice(bytes)?;
        envelope.validate(expected_generation)?;
        Ok(envelope)
    }

    pub fn validate(&self, expected_generation: Option<u64>) -> Result<(), ContractViolation> {
        validate_context(
            self.protocol,
            &self.request_id,
            &self.actor,
            &self.caller,
            &self.scope,
            self.generation,
            self.deadline_at_ms,
            &self.cancellation_ref,
            expected_generation,
        )?;
        require_ref("operation", &self.operation)
    }

    pub fn ensure_live_at(&self, now_ms: u64) -> Result<(), ContractViolation> {
        if now_ms >= self.deadline_at_ms {
            return Err(ContractViolation::Expired {
                deadline_at_ms: self.deadline_at_ms,
                now_ms,
            });
        }
        Ok(())
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct ResponseEnvelope {
    pub protocol: ProtocolVersion,
    pub kind: ResponseKind,
    pub request_id: String,
    pub actor: PrincipalRef,
    pub caller: PrincipalRef,
    pub scope: ContractScope,
    pub generation: u64,
    pub deadline_at_ms: u64,
    pub cancellation_ref: String,
    pub outcome: ContractOutcome,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ResponseKind {
    Response,
}

impl ResponseEnvelope {
    pub fn from_json_slice(
        bytes: &[u8],
        expected_generation: Option<u64>,
    ) -> Result<Self, ContractViolation> {
        let envelope: Self = serde_json::from_slice(bytes)?;
        envelope.validate(expected_generation)?;
        Ok(envelope)
    }

    pub fn validate(&self, expected_generation: Option<u64>) -> Result<(), ContractViolation> {
        validate_context(
            self.protocol,
            &self.request_id,
            &self.actor,
            &self.caller,
            &self.scope,
            self.generation,
            self.deadline_at_ms,
            &self.cancellation_ref,
            expected_generation,
        )?;
        self.outcome.validate(&self.request_id, self.generation)
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(tag = "status", rename_all = "snake_case", deny_unknown_fields)]
pub enum ContractOutcome {
    Ok {
        receipt: Receipt,
        value: Value,
    },
    Error {
        receipt: Receipt,
        error: ContractError,
    },
}

impl ContractOutcome {
    fn validate(&self, request_id: &str, generation: u64) -> Result<(), ContractViolation> {
        let receipt = match self {
            Self::Ok { receipt, .. } | Self::Error { receipt, .. } => receipt,
        };
        receipt.validate()?;
        if receipt.request_id != request_id || receipt.generation != generation {
            return Err(ContractViolation::ReceiptMismatch);
        }
        match self {
            Self::Ok { receipt, .. } if receipt.state != ReceiptState::Committed => Err(
                ContractViolation::InvalidLifecycle("ok receipt must be committed"),
            ),
            Self::Error { receipt, error } => {
                if receipt.state == ReceiptState::Committed {
                    return Err(ContractViolation::InvalidLifecycle(
                        "error receipt cannot be committed",
                    ));
                }
                error.validate()
            }
            _ => Ok(()),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum ContractErrorCode {
    Denied,
    NotFound,
    Conflict,
    Expired,
    StaleGeneration,
    Unavailable,
    Transient,
    IntegrityFailed,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct ContractError {
    pub code: ContractErrorCode,
    pub message: String,
    pub retryable: bool,
    pub details: Value,
}

impl ContractError {
    pub fn validate(&self) -> Result<(), ContractViolation> {
        if self.message.is_empty() {
            return Err(ContractViolation::InvalidField("message"));
        }
        if !self.details.is_object() {
            return Err(ContractViolation::InvalidField("details"));
        }
        Ok(())
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct Descriptor {
    pub descriptor_ref: String,
    pub generation: u64,
    pub revision: String,
    pub issued_at_ms: u64,
    pub value: Value,
}

impl Descriptor {
    pub fn validate(&self) -> Result<(), ContractViolation> {
        require_ref("descriptorRef", &self.descriptor_ref)?;
        require_generation(self.generation)?;
        require_ref("revision", &self.revision)?;
        require_timestamp("issuedAtMs", self.issued_at_ms)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum HandleState {
    Active,
    Revoked,
    Expired,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct CapabilityHandle {
    pub handle_ref: String,
    pub audience: PrincipalRef,
    pub scope: ContractScope,
    pub access: Vec<String>,
    pub generation: u64,
    pub issued_at_ms: u64,
    pub expires_at_ms: u64,
    pub state: HandleState,
}

impl CapabilityHandle {
    pub fn validate(&self) -> Result<(), ContractViolation> {
        require_ref("handleRef", &self.handle_ref)?;
        validate_principal("audience", &self.audience)?;
        validate_scope(&self.scope)?;
        require_generation(self.generation)?;
        validate_interval(self.issued_at_ms, self.expires_at_ms)?;
        if self.access.is_empty()
            || self
                .access
                .iter()
                .any(|item| require_ref("access", item).is_err())
            || self.access.iter().collect::<BTreeSet<_>>().len() != self.access.len()
        {
            return Err(ContractViolation::InvalidField("access"));
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum LeaseState {
    Active,
    Released,
    Revoked,
    Expired,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct Lease {
    pub lease_ref: String,
    pub holder: PrincipalRef,
    pub scope: ContractScope,
    pub generation: u64,
    pub issued_at_ms: u64,
    pub expires_at_ms: u64,
    pub state: LeaseState,
}

impl Lease {
    pub fn validate(&self) -> Result<(), ContractViolation> {
        require_ref("leaseRef", &self.lease_ref)?;
        validate_principal("holder", &self.holder)?;
        validate_scope(&self.scope)?;
        require_generation(self.generation)?;
        validate_interval(self.issued_at_ms, self.expires_at_ms)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ReceiptState {
    Committed,
    Rejected,
    Cancelled,
    Expired,
    Failed,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct Receipt {
    pub receipt_ref: String,
    pub request_id: String,
    pub generation: u64,
    pub state: ReceiptState,
    pub issued_at_ms: u64,
    pub effects: Vec<String>,
}

impl Receipt {
    pub fn validate(&self) -> Result<(), ContractViolation> {
        require_ref("receiptRef", &self.receipt_ref)?;
        require_ref("requestId", &self.request_id)?;
        require_generation(self.generation)?;
        require_timestamp("issuedAtMs", self.issued_at_ms)?;
        if self
            .effects
            .iter()
            .any(|item| require_ref("effects", item).is_err())
            || self.effects.iter().collect::<BTreeSet<_>>().len() != self.effects.len()
        {
            return Err(ContractViolation::InvalidField("effects"));
        }
        Ok(())
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct LifecycleSnapshot {
    pub protocol: ProtocolVersion,
    pub descriptor: Descriptor,
    pub handle: CapabilityHandle,
    pub lease: Lease,
    pub receipt: Receipt,
}

impl LifecycleSnapshot {
    pub fn validate(&self) -> Result<(), ContractViolation> {
        self.protocol.validate()?;
        self.descriptor.validate()?;
        self.handle.validate()?;
        self.lease.validate()?;
        self.receipt.validate()
    }
}

#[derive(Debug, thiserror::Error)]
pub enum ContractViolation {
    #[error("invalid contract JSON: {0}")]
    Json(#[from] serde_json::Error),
    #[error("incompatible protocol version {actual_major}.{actual_minor}")]
    IncompatibleProtocol {
        actual_major: u16,
        actual_minor: u16,
    },
    #[error("stale generation: expected {expected}, got {actual}")]
    StaleGeneration { expected: u64, actual: u64 },
    #[error("deadline {deadline_at_ms} has expired at {now_ms}")]
    Expired { deadline_at_ms: u64, now_ms: u64 },
    #[error("invalid field: {0}")]
    InvalidField(&'static str),
    #[error("invalid lifecycle: {0}")]
    InvalidLifecycle(&'static str),
    #[error("receipt does not match its envelope")]
    ReceiptMismatch,
}

#[allow(clippy::too_many_arguments)]
fn validate_context(
    protocol: ProtocolVersion,
    request_id: &str,
    actor: &PrincipalRef,
    caller: &PrincipalRef,
    scope: &ContractScope,
    generation: u64,
    deadline_at_ms: u64,
    cancellation_ref: &str,
    expected_generation: Option<u64>,
) -> Result<(), ContractViolation> {
    protocol.validate()?;
    require_ref("requestId", request_id)?;
    validate_principal("actor", actor)?;
    validate_principal("caller", caller)?;
    validate_scope(scope)?;
    require_generation(generation)?;
    require_timestamp("deadlineAtMs", deadline_at_ms)?;
    require_ref("cancellationRef", cancellation_ref)?;
    if let Some(expected) = expected_generation {
        if generation != expected {
            return Err(ContractViolation::StaleGeneration {
                expected,
                actual: generation,
            });
        }
    }
    Ok(())
}

fn validate_principal(
    field: &'static str,
    principal: &PrincipalRef,
) -> Result<(), ContractViolation> {
    require_ref(field, &principal.principal_ref)
}

fn validate_scope(scope: &ContractScope) -> Result<(), ContractViolation> {
    require_ref("authorityRef", &scope.authority_ref)?;
    if let Some(workspace_ref) = &scope.workspace_ref {
        require_ref("workspaceRef", workspace_ref)?;
    }
    if let Some(resource_ref) = &scope.resource_ref {
        require_ref("resourceRef", resource_ref)?;
    }
    Ok(())
}

fn require_ref(field: &'static str, value: &str) -> Result<(), ContractViolation> {
    if value.is_empty() || value.chars().any(char::is_whitespace) {
        return Err(ContractViolation::InvalidField(field));
    }
    Ok(())
}

fn require_generation(value: u64) -> Result<(), ContractViolation> {
    if value == 0 || value > MAX_SAFE_INTEGER {
        return Err(ContractViolation::InvalidField("generation"));
    }
    Ok(())
}

fn require_timestamp(field: &'static str, value: u64) -> Result<(), ContractViolation> {
    if value == 0 || value > MAX_SAFE_INTEGER {
        return Err(ContractViolation::InvalidField(field));
    }
    Ok(())
}

fn validate_interval(issued_at_ms: u64, expires_at_ms: u64) -> Result<(), ContractViolation> {
    require_timestamp("issuedAtMs", issued_at_ms)?;
    require_timestamp("expiresAtMs", expires_at_ms)?;
    if expires_at_ms <= issued_at_ms {
        return Err(ContractViolation::InvalidField("expiresAtMs"));
    }
    Ok(())
}
