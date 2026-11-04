use openmuse_contract::ContractScope;
use openmuse_plugin_protocol::{DelegatedRequestContext, Permission, PluginId};
use serde::{Deserialize, Serialize};
use std::collections::{BTreeMap, BTreeSet};

pub type GrantSet = BTreeSet<Permission>;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DelegatedCapability {
    pub handle_ref: String,
    pub actor_ref: String,
    pub audience: PluginId,
    pub scope: ContractScope,
    pub permissions: GrantSet,
    pub generation: u64,
    pub expires_at_ms: u64,
    pub revoked: bool,
}

pub type DelegatedRequest = DelegatedRequestContext;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AuthorizedCall {
    pub decision_ref: String,
    pub effective_grants: GrantSet,
    pub audit_receipt: AuditReceipt,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum AuditOutcome {
    Allowed,
    Denied,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct AuditReceipt {
    pub decision_ref: String,
    pub request_id: String,
    pub actor_ref: String,
    pub caller_plugin: String,
    pub target_provider: String,
    pub authority_ref: String,
    pub workspace_ref: Option<String>,
    pub revision: String,
    pub operation: String,
    pub generation: u64,
    pub outcome: AuditOutcome,
    pub reason: Option<String>,
}

#[derive(Debug, Clone, Default)]
pub struct InMemoryPolicyProvider {
    tenant_ceiling: GrantSet,
    actor_grants: BTreeMap<String, GrantSet>,
    plugin_grants: BTreeMap<PluginId, GrantSet>,
    workspace_grants: BTreeMap<String, GrantSet>,
}

impl InMemoryPolicyProvider {
    pub fn with_tenant_ceiling(ceiling: GrantSet) -> Self {
        Self {
            tenant_ceiling: ceiling,
            ..Self::default()
        }
    }

    pub fn set_actor_grants(&mut self, actor_ref: impl Into<String>, grants: GrantSet) {
        self.actor_grants.insert(actor_ref.into(), grants);
    }

    pub fn set_plugin_grants(&mut self, plugin: PluginId, grants: GrantSet) {
        self.plugin_grants.insert(plugin, grants);
    }

    pub fn set_workspace_grants(&mut self, workspace_ref: impl Into<String>, grants: GrantSet) {
        self.workspace_grants.insert(workspace_ref.into(), grants);
    }

    fn effective_grants(
        &self,
        actor_ref: &str,
        caller: &PluginId,
        target: &PluginId,
        workspace_ref: Option<&str>,
        handle_grants: &GrantSet,
        command_required: &GrantSet,
    ) -> GrantSet {
        let empty = GrantSet::new();
        let actor = self.actor_grants.get(actor_ref).unwrap_or(&empty);
        let caller = self.plugin_grants.get(caller).unwrap_or(&empty);
        let target = self.plugin_grants.get(target).unwrap_or(&empty);
        let workspace = workspace_ref
            .and_then(|workspace| self.workspace_grants.get(workspace))
            .unwrap_or(&empty);
        command_required
            .iter()
            .filter(|permission| {
                self.tenant_ceiling.contains(*permission)
                    && actor.contains(*permission)
                    && caller.contains(*permission)
                    && target.contains(*permission)
                    && workspace.contains(*permission)
                    && handle_grants.contains(*permission)
            })
            .cloned()
            .collect()
    }
}

pub struct DelegationBroker {
    policy: InMemoryPolicyProvider,
    handles: BTreeMap<String, DelegatedCapability>,
    audit: Vec<AuditReceipt>,
    next_decision: u64,
}

impl DelegationBroker {
    pub fn new(policy: InMemoryPolicyProvider) -> Self {
        Self {
            policy,
            handles: BTreeMap::new(),
            audit: Vec::new(),
            next_decision: 1,
        }
    }

    pub fn policy_mut(&mut self) -> &mut InMemoryPolicyProvider {
        &mut self.policy
    }

    pub fn issue_handle(&mut self, handle: DelegatedCapability) -> Result<(), DelegationError> {
        if handle.handle_ref.is_empty()
            || handle.actor_ref.is_empty()
            || handle.generation == 0
            || handle.expires_at_ms == 0
            || handle.permissions.is_empty()
        {
            return Err(DelegationError::InvalidHandle);
        }
        if self.handles.contains_key(&handle.handle_ref) {
            return Err(DelegationError::HandleConflict);
        }
        self.handles.insert(handle.handle_ref.clone(), handle);
        Ok(())
    }

    pub fn revoke(&mut self, handle_ref: &str) -> Result<(), DelegationError> {
        let handle = self
            .handles
            .get_mut(handle_ref)
            .ok_or(DelegationError::HandleNotFound)?;
        handle.revoked = true;
        Ok(())
    }

    pub fn audit_receipts(&self) -> &[AuditReceipt] {
        &self.audit
    }

    pub fn authorize(
        &mut self,
        now_ms: u64,
        request: &DelegatedRequest,
        command_required: &GrantSet,
    ) -> Result<AuthorizedCall, DelegationError> {
        let decision_ref = format!("decision.{}", self.next_decision);
        self.next_decision += 1;
        let result = self.authorize_inner(now_ms, request, command_required);
        let (outcome, reason, effective) = match &result {
            Ok(grants) => (AuditOutcome::Allowed, None, grants.clone()),
            Err(error) => (
                AuditOutcome::Denied,
                Some(error.code().to_owned()),
                GrantSet::new(),
            ),
        };
        let receipt = audit_receipt(decision_ref.clone(), request, outcome, reason);
        self.audit.push(receipt.clone());
        result.map(|_| AuthorizedCall {
            decision_ref,
            effective_grants: effective,
            audit_receipt: receipt,
        })
    }

    fn authorize_inner(
        &self,
        now_ms: u64,
        request: &DelegatedRequest,
        command_required: &GrantSet,
    ) -> Result<GrantSet, DelegationError> {
        if request.request_id.is_empty()
            || request.revision.is_empty()
            || request.operation.is_empty()
        {
            return Err(DelegationError::InvalidRequest);
        }
        if now_ms >= request.deadline_at_ms {
            return Err(DelegationError::DeadlineExpired);
        }
        let handle = self
            .handles
            .get(&request.handle_ref)
            .ok_or(DelegationError::HandleNotFound)?;
        if handle.revoked {
            return Err(DelegationError::HandleRevoked);
        }
        if now_ms >= handle.expires_at_ms {
            return Err(DelegationError::HandleExpired);
        }
        if handle.actor_ref != request.actor.principal_ref {
            return Err(DelegationError::ActorMismatch);
        }
        if handle.audience != request.caller {
            return Err(DelegationError::AudienceMismatch);
        }
        if handle.scope != request.scope {
            return Err(DelegationError::ScopeMismatch);
        }
        if handle.generation != request.generation {
            return Err(DelegationError::StaleGeneration);
        }
        let effective = self.policy.effective_grants(
            &request.actor.principal_ref,
            &request.caller,
            &request.target_provider,
            request.scope.workspace_ref.as_deref(),
            &handle.permissions,
            command_required,
        );
        if effective != *command_required {
            return Err(DelegationError::PermissionDenied);
        }
        Ok(effective)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, thiserror::Error)]
pub enum DelegationError {
    #[error("invalid capability handle")]
    InvalidHandle,
    #[error("capability handle already exists")]
    HandleConflict,
    #[error("capability handle not found")]
    HandleNotFound,
    #[error("capability handle was revoked")]
    HandleRevoked,
    #[error("capability handle expired")]
    HandleExpired,
    #[error("request deadline expired")]
    DeadlineExpired,
    #[error("handle actor does not match request actor")]
    ActorMismatch,
    #[error("handle audience does not match request caller")]
    AudienceMismatch,
    #[error("handle scope does not match request scope")]
    ScopeMismatch,
    #[error("handle generation is stale")]
    StaleGeneration,
    #[error("effective policy grants do not satisfy the command")]
    PermissionDenied,
    #[error("invalid delegated request")]
    InvalidRequest,
}

impl DelegationError {
    fn code(&self) -> &'static str {
        match self {
            Self::HandleNotFound => "NOT_FOUND",
            Self::HandleExpired | Self::DeadlineExpired => "EXPIRED",
            Self::StaleGeneration => "STALE_GENERATION",
            Self::HandleConflict => "CONFLICT",
            Self::InvalidHandle | Self::InvalidRequest => "INTEGRITY_FAILED",
            Self::HandleRevoked
            | Self::ActorMismatch
            | Self::AudienceMismatch
            | Self::ScopeMismatch
            | Self::PermissionDenied => "DENIED",
        }
    }
}

fn audit_receipt(
    decision_ref: String,
    request: &DelegatedRequest,
    outcome: AuditOutcome,
    reason: Option<String>,
) -> AuditReceipt {
    AuditReceipt {
        decision_ref,
        request_id: redact(&request.request_id),
        actor_ref: redact(&request.actor.principal_ref),
        caller_plugin: redact(&request.caller.0),
        target_provider: redact(&request.target_provider.0),
        authority_ref: redact(&request.scope.authority_ref),
        workspace_ref: request.scope.workspace_ref.as_deref().map(redact),
        revision: redact(&request.revision),
        operation: redact(&request.operation),
        generation: request.generation,
        outcome,
        reason,
    }
}

fn redact(value: &str) -> String {
    let lower = value.to_ascii_lowercase();
    if value.contains('/')
        || value.contains('?')
        || lower.contains("token")
        || lower.contains("secret")
        || lower.contains("accesskey")
        || lower.starts_with("s3:")
    {
        "[redacted]".into()
    } else {
        value.into()
    }
}
