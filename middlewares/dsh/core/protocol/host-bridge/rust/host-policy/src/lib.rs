//! Domain-neutral state machine behind Bridge v1 `policy.evaluate`,
//! `policy.finalize`, and command status. It owns decisions and grants; an
//! adapter supplies authoritative subject resolution and approval-proof checks.

use std::{
    collections::BTreeMap,
    fmt::Debug,
    sync::{
        Arc,
        atomic::{AtomicU64, Ordering},
    },
    time::{SystemTime, UNIX_EPOCH},
};

use async_trait::async_trait;
use muse_host_bridge_contract::ids::{ApprovalId, CommandId, GrantId, PolicyDecisionId};
use muse_host_bridge_contract::{JsonLimits, digest_input};
use muse_host_registry::{
    Effect as RegistryEffect, InvocationAdmission, InvocationAuthorizer, RegistryError,
};
use tokio::sync::Mutex;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Effect {
    Read,
    Write,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PolicySubject {
    pub actor_ref: String,
    pub session_ref: String,
    pub tool_call_ref: String,
    pub scope_ref: String,
    pub authority_epoch: u64,
    pub binding_id: String,
    pub binding_generation: u64,
    pub operation_id: String,
    pub input_digest: String,
    pub effect: Effect,
}

impl PolicySubject {
    fn valid(&self) -> bool {
        [
            &self.actor_ref,
            &self.session_ref,
            &self.tool_call_ref,
            &self.scope_ref,
            &self.binding_id,
            &self.operation_id,
            &self.input_digest,
        ]
        .iter()
        .all(|value| !value.is_empty() && value.len() <= 256)
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PolicyVerdict {
    Allow,
    Deny { reason: String },
    ApprovalRequired { reason: String },
}

#[async_trait]
pub trait PolicyEvaluator: Send + Sync + 'static {
    async fn evaluate(&self, subject: &PolicySubject) -> PolicyVerdict;
}

/// Verifies a proof issued by the approval transport, not a UI boolean and not
/// a Plugin-generated value. The proof is intentionally opaque to this crate.
#[async_trait]
pub trait ApprovalProofVerifier: Send + Sync + 'static {
    async fn verify(
        &self,
        approval_id: &ApprovalId,
        proof_id: &str,
        subject: &PolicySubject,
    ) -> bool;
}

pub trait PolicyClock: Debug + Send + Sync + 'static {
    fn now_ms(&self) -> u64;
}

#[derive(Debug)]
pub struct SystemPolicyClock;
impl PolicyClock for SystemPolicyClock {
    fn now_ms(&self) -> u64 {
        SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap_or_default()
            .as_millis()
            .min(u64::MAX as u128) as u64
    }
}

#[derive(Debug, Clone)]
pub struct PolicyConfig {
    pub decision_ttl_ms: u64,
    pub grant_ttl_ms: u64,
    pub command_retention_ms: u64,
    pub max_pending: usize,
    pub clock: Arc<dyn PolicyClock>,
}
impl Default for PolicyConfig {
    fn default() -> Self {
        Self {
            decision_ttl_ms: 300_000,
            grant_ttl_ms: 60_000,
            command_retention_ms: 86_400_000,
            max_pending: 4096,
            clock: Arc::new(SystemPolicyClock),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Evaluation {
    pub policy_decision_id: PolicyDecisionId,
    pub verdict: PolicyVerdict,
    pub expires_at_ms: u64,
    pub approval_id: Option<ApprovalId>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FinalizeOutcome {
    Approved,
    Rejected,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Finalization {
    pub policy_decision_id: PolicyDecisionId,
    pub outcome: FinalizeOutcome,
    pub grant_id: Option<GrantId>,
    pub expires_at_ms: Option<u64>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum CommandStatus {
    Applied,
    Conflict,
    Denied,
    Cancelled,
    Failed,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CommandTerminal {
    pub command_id: CommandId,
    pub subject: PolicySubject,
    pub idempotency_key: String,
    pub status: CommandStatus,
    pub updated_at_ms: u64,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PolicyError {
    Deadline,
    InvalidSubject,
    PendingLimit,
    NotFound,
    Expired,
    ProofRejected,
    GrantRejected,
    IdempotencyConflict,
}

struct Pending {
    subject: PolicySubject,
    decision_id: PolicyDecisionId,
    approval_id: ApprovalId,
    expires_at_ms: u64,
}
struct Grant {
    subject: PolicySubject,
    expires_at_ms: u64,
}
#[derive(Default)]
struct State {
    pending: BTreeMap<ApprovalId, Pending>,
    grants: BTreeMap<GrantId, Grant>,
    commands: BTreeMap<(String, String), CommandTerminal>,
}

pub struct HostPolicy {
    config: PolicyConfig,
    evaluator: Arc<dyn PolicyEvaluator>,
    verifier: Arc<dyn ApprovalProofVerifier>,
    counter: AtomicU64,
    state: Mutex<State>,
}

#[derive(Clone)]
pub struct RegistryPolicyAuthorizer {
    policy: Arc<HostPolicy>,
}

impl std::fmt::Debug for RegistryPolicyAuthorizer {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter
            .debug_struct("RegistryPolicyAuthorizer")
            .finish_non_exhaustive()
    }
}

impl RegistryPolicyAuthorizer {
    pub fn new(policy: Arc<HostPolicy>) -> Self {
        Self { policy }
    }
}

const INPUT_LIMITS: JsonLimits = JsonLimits {
    max_depth: 64,
    max_container_children: 10_000,
};

pub fn subject_from_admission(
    admission: &InvocationAdmission<'_>,
) -> Result<PolicySubject, RegistryError> {
    let session_ref = admission.session_ref.ok_or(RegistryError::GrantInvalid)?;
    let tool_call_ref = admission.tool_call_ref.ok_or(RegistryError::GrantInvalid)?;
    let effect = match admission.operation.effect {
        RegistryEffect::Read => Effect::Read,
        RegistryEffect::LocalWrite
        | RegistryEffect::SyncWrite
        | RegistryEffect::ExternalSideEffect => Effect::Write,
    };
    Ok(PolicySubject {
        actor_ref: admission.caller.actor_ref.clone(),
        session_ref: session_ref.into(),
        tool_call_ref: tool_call_ref.into(),
        scope_ref: admission.context.scope_ref.clone(),
        authority_epoch: admission.context.authority_epoch,
        binding_id: admission.binding.binding_id.clone(),
        binding_generation: admission.binding.registration_generation,
        operation_id: admission.operation.operation_id.clone(),
        input_digest: digest_input(admission.input, INPUT_LIMITS)
            .map_err(|_| RegistryError::InvalidJson)?,
        effect,
    })
}

#[async_trait]
impl InvocationAuthorizer for RegistryPolicyAuthorizer {
    async fn authorize(&self, admission: InvocationAdmission<'_>) -> Result<(), RegistryError> {
        if admission.operation.effect == RegistryEffect::Read {
            return Ok(());
        }
        let grant = admission.grant_id.ok_or(RegistryError::GrantRequired)?;
        let grant: GrantId = grant.parse().map_err(|_| RegistryError::GrantInvalid)?;
        let subject = subject_from_admission(&admission)?;
        self.policy
            .consume_grant(&grant, &subject, admission.deadline_at_ms)
            .await
            .map_err(|_| RegistryError::GrantInvalid)
    }
}

impl HostPolicy {
    pub fn new(
        config: PolicyConfig,
        evaluator: Arc<dyn PolicyEvaluator>,
        verifier: Arc<dyn ApprovalProofVerifier>,
    ) -> Result<Self, PolicyError> {
        if config.decision_ttl_ms == 0
            || config.grant_ttl_ms == 0
            || config.command_retention_ms == 0
            || config.max_pending == 0
        {
            return Err(PolicyError::PendingLimit);
        }
        Ok(Self {
            config,
            evaluator,
            verifier,
            counter: AtomicU64::new(1),
            state: Mutex::new(State::default()),
        })
    }

    pub async fn evaluate(
        &self,
        subject: PolicySubject,
        deadline_at_ms: u64,
    ) -> Result<Evaluation, PolicyError> {
        let now = self.now();
        if deadline_at_ms <= now {
            return Err(PolicyError::Deadline);
        }
        if !subject.valid() {
            return Err(PolicyError::InvalidSubject);
        }
        let verdict = self.evaluator.evaluate(&subject).await;
        let decision_id: PolicyDecisionId = self.id("policy");
        let expires_at_ms = now.saturating_add(self.config.decision_ttl_ms);
        let approval_id = match &verdict {
            PolicyVerdict::ApprovalRequired { .. } => {
                let approval: ApprovalId = self.id("approval");
                let mut state = self.state.lock().await;
                self.sweep(&mut state, now);
                if state.pending.len() >= self.config.max_pending {
                    return Err(PolicyError::PendingLimit);
                }
                state.pending.insert(
                    approval.clone(),
                    Pending {
                        subject: subject.clone(),
                        decision_id: decision_id.clone(),
                        approval_id: approval.clone(),
                        expires_at_ms,
                    },
                );
                Some(approval)
            }
            _ => None,
        };
        Ok(Evaluation {
            policy_decision_id: decision_id,
            verdict,
            expires_at_ms,
            approval_id,
        })
    }

    pub async fn finalize(
        &self,
        approval_id: &ApprovalId,
        outcome: FinalizeOutcome,
        proof_id: &str,
        deadline_at_ms: u64,
    ) -> Result<Finalization, PolicyError> {
        let now = self.now();
        if deadline_at_ms <= now {
            return Err(PolicyError::Deadline);
        }
        let pending = {
            let mut state = self.state.lock().await;
            self.sweep(&mut state, now);
            state
                .pending
                .remove(approval_id)
                .ok_or(PolicyError::NotFound)?
        };
        if pending.expires_at_ms <= now {
            return Err(PolicyError::Expired);
        }
        if outcome == FinalizeOutcome::Rejected {
            return Ok(Finalization {
                policy_decision_id: pending.decision_id,
                outcome,
                grant_id: None,
                expires_at_ms: None,
            });
        }
        if proof_id.is_empty()
            || !self
                .verifier
                .verify(&pending.approval_id, proof_id, &pending.subject)
                .await
        {
            return Err(PolicyError::ProofRejected);
        }
        let grant_id: GrantId = self.id("grant");
        let expires_at_ms = now.saturating_add(self.config.grant_ttl_ms);
        let mut state = self.state.lock().await;
        self.sweep(&mut state, now);
        state.grants.insert(
            grant_id.clone(),
            Grant {
                subject: pending.subject,
                expires_at_ms,
            },
        );
        Ok(Finalization {
            policy_decision_id: pending.decision_id,
            outcome,
            grant_id: Some(grant_id),
            expires_at_ms: Some(expires_at_ms),
        })
    }

    /// Consumes the grant before returning. The caller must call this at its
    /// final domain-command admission point, after authority revalidation.
    pub async fn consume_grant(
        &self,
        grant_id: &GrantId,
        subject: &PolicySubject,
        deadline_at_ms: u64,
    ) -> Result<(), PolicyError> {
        let now = self.now();
        if deadline_at_ms <= now {
            return Err(PolicyError::Deadline);
        }
        let mut state = self.state.lock().await;
        self.sweep(&mut state, now);
        let grant = state
            .grants
            .remove(grant_id)
            .ok_or(PolicyError::GrantRejected)?;
        if grant.expires_at_ms <= now || grant.subject != *subject {
            return Err(PolicyError::GrantRejected);
        }
        Ok(())
    }

    pub async fn terminal(
        &self,
        subject: PolicySubject,
        idempotency_key: String,
        status: CommandStatus,
    ) -> Result<CommandTerminal, PolicyError> {
        if !subject.valid() || idempotency_key.is_empty() || idempotency_key.len() > 256 {
            return Err(PolicyError::InvalidSubject);
        }
        let now = self.now();
        let key = (subject.actor_ref.clone(), idempotency_key.clone());
        let mut state = self.state.lock().await;
        self.sweep(&mut state, now);
        if let Some(previous) = state.commands.get(&key) {
            if previous.subject == subject {
                return Ok(previous.clone());
            }
            return Err(PolicyError::IdempotencyConflict);
        }
        let terminal = CommandTerminal {
            command_id: self.id("command"),
            subject,
            idempotency_key,
            status,
            updated_at_ms: now,
        };
        state.commands.insert(key, terminal.clone());
        Ok(terminal)
    }

    pub async fn status(&self, actor_ref: &str, idempotency_key: &str) -> Option<CommandTerminal> {
        let now = self.now();
        let mut state = self.state.lock().await;
        self.sweep(&mut state, now);
        state
            .commands
            .get(&(actor_ref.to_owned(), idempotency_key.to_owned()))
            .cloned()
    }

    fn now(&self) -> u64 {
        self.config.clock.now_ms()
    }
    fn id<T>(&self, prefix: &str) -> T
    where
        T: TryFrom<String>,
        <T as TryFrom<String>>::Error: Debug,
    {
        format!("{prefix}.{}", self.counter.fetch_add(1, Ordering::Relaxed))
            .try_into()
            .expect("generated opaque id")
    }
    fn sweep(&self, state: &mut State, now: u64) {
        state.pending.retain(|_, value| value.expires_at_ms > now);
        state.grants.retain(|_, value| value.expires_at_ms > now);
        state.commands.retain(|_, value| {
            value
                .updated_at_ms
                .saturating_add(self.config.command_retention_ms)
                > now
        });
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use muse_host_registry::{
        AuthoritativeCaller, AuthorityError, AuthorityResolver, Cancellation, CapabilityProvider,
        HostCapabilityRegistry, Idempotency, InvokeRequest, OperationDescriptor,
        ProviderDescriptor, ProviderFailure, ProviderInvocation, RegistryClock, RegistryConfig,
        ResolvedHostContext, SchemaDocument, ScopeHint,
    };
    use serde_json::{Value, json};
    use std::{
        collections::{BTreeMap, BTreeSet},
        sync::atomic::{AtomicBool, AtomicU64},
    };

    #[derive(Debug)]
    struct Clock(AtomicU64);
    impl Clock {
        fn set(&self, value: u64) {
            self.0.store(value, Ordering::SeqCst);
        }
    }
    impl PolicyClock for Clock {
        fn now_ms(&self) -> u64 {
            self.0.load(Ordering::SeqCst)
        }
    }
    impl RegistryClock for Clock {
        fn now_ms(&self) -> u64 {
            self.0.load(Ordering::SeqCst)
        }
    }
    struct Ask;
    #[async_trait]
    impl PolicyEvaluator for Ask {
        async fn evaluate(&self, _: &PolicySubject) -> PolicyVerdict {
            PolicyVerdict::ApprovalRequired {
                reason: "write".into(),
            }
        }
    }
    struct Proof;
    #[async_trait]
    impl ApprovalProofVerifier for Proof {
        async fn verify(&self, _: &ApprovalId, proof: &str, _: &PolicySubject) -> bool {
            proof == "proof.1"
        }
    }
    fn subject() -> PolicySubject {
        PolicySubject {
            actor_ref: "actor.1".into(),
            session_ref: "session.1".into(),
            tool_call_ref: "call.1".into(),
            scope_ref: "scope.1".into(),
            authority_epoch: 1,
            binding_id: "binding.1".into(),
            binding_generation: 1,
            operation_id: "view.rename.apply".into(),
            input_digest: "sha256:input".into(),
            effect: Effect::Write,
        }
    }
    fn policy(clock: Arc<Clock>) -> HostPolicy {
        HostPolicy::new(
            PolicyConfig {
                clock,
                ..PolicyConfig::default()
            },
            Arc::new(Ask),
            Arc::new(Proof),
        )
        .unwrap()
    }

    struct Authority;
    #[async_trait]
    impl AuthorityResolver for Authority {
        async fn resolve(
            &self,
            caller: &AuthoritativeCaller,
            _: Option<&ScopeHint>,
        ) -> Result<ResolvedHostContext, AuthorityError> {
            Ok(ResolvedHostContext {
                actor_ref: caller.actor_ref.clone(),
                scope_ref: "scope.1".into(),
                authority_epoch: 1,
                evidence: BTreeMap::new(),
            })
        }

        async fn revalidate(
            &self,
            caller: &AuthoritativeCaller,
            previous: &ResolvedHostContext,
        ) -> Result<ResolvedHostContext, AuthorityError> {
            Ok(ResolvedHostContext {
                actor_ref: caller.actor_ref.clone(),
                ..previous.clone()
            })
        }
    }

    struct WriteProvider(Arc<AtomicBool>);
    #[async_trait]
    impl CapabilityProvider for WriteProvider {
        fn descriptor(&self) -> ProviderDescriptor {
            ProviderDescriptor {
                descriptor_id: "descriptor.write".into(),
                revision: "1".into(),
                family_id: "test.write".into(),
                contract_major: 1,
                contract_minor: 0,
                operations: vec![OperationDescriptor {
                    operation_id: "view.rename.apply".into(),
                    effect: RegistryEffect::LocalWrite,
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
            self.0.store(true, Ordering::Release);
            Ok(invocation.input)
        }
    }

    #[tokio::test]
    async fn approved_grant_is_exact_and_one_use() {
        let clock = Arc::new(Clock(AtomicU64::new(10)));
        let policy = policy(clock);
        let subject = subject();
        let evaluation = policy.evaluate(subject.clone(), 20).await.unwrap();
        let approval = evaluation.approval_id.unwrap();
        let grant = policy
            .finalize(&approval, FinalizeOutcome::Approved, "proof.1", 20)
            .await
            .unwrap()
            .grant_id
            .unwrap();
        assert!(policy.consume_grant(&grant, &subject, 20).await.is_ok());
        assert_eq!(
            policy.consume_grant(&grant, &subject, 20).await,
            Err(PolicyError::GrantRejected)
        );
    }

    #[tokio::test]
    async fn registry_consumes_exact_grant_before_provider_and_rejects_replay() {
        let clock = Arc::new(Clock(AtomicU64::new(10)));
        let policy = Arc::new(policy(clock.clone()));
        let registry = HostCapabilityRegistry::new(
            Arc::new(Authority),
            RegistryConfig {
                clock: clock.clone(),
                invocation_authorizer: Arc::new(RegistryPolicyAuthorizer::new(policy.clone())),
                ..RegistryConfig::default()
            },
        );
        let invoked = Arc::new(AtomicBool::new(false));
        let _lease = registry
            .register(Arc::new(WriteProvider(invoked.clone())))
            .await
            .unwrap();
        let caller = AuthoritativeCaller {
            actor_ref: "actor.1".into(),
        };
        let binding = registry
            .bind(
                &caller,
                "descriptor.write",
                "1",
                BTreeSet::from(["view.rename.apply".into()]),
                None,
            )
            .await
            .unwrap();
        let input = json!({"proposalId": "proposal.1"});
        let subject = PolicySubject {
            actor_ref: caller.actor_ref.clone(),
            session_ref: "session.1".into(),
            tool_call_ref: "call.1".into(),
            scope_ref: binding.scope_ref.clone(),
            authority_epoch: 1,
            binding_id: binding.binding_id.clone(),
            binding_generation: binding.registration_generation,
            operation_id: "view.rename.apply".into(),
            input_digest: digest_input(&input, INPUT_LIMITS).unwrap(),
            effect: Effect::Write,
        };
        let approval = policy
            .evaluate(subject, 100)
            .await
            .unwrap()
            .approval_id
            .unwrap();
        let grant = policy
            .finalize(&approval, FinalizeOutcome::Approved, "proof.1", 100)
            .await
            .unwrap()
            .grant_id
            .unwrap();
        let request = InvokeRequest {
            binding_id: binding.binding_id,
            operation_id: "view.rename.apply".into(),
            input,
            deadline_at_ms: 100,
            cancellation: Cancellation::default(),
            grant_id: Some(grant.to_string()),
            idempotency_key: Some("idem.1".into()),
            session_ref: Some("session.1".into()),
            tool_call_ref: Some("call.1".into()),
        };
        assert!(registry.invoke(&caller, request.clone()).await.is_ok());
        assert!(invoked.load(Ordering::Acquire));
        invoked.store(false, Ordering::Release);
        assert_eq!(
            registry.invoke(&caller, request).await,
            Err(RegistryError::GrantInvalid)
        );
        assert!(!invoked.load(Ordering::Acquire));
    }

    #[tokio::test]
    async fn wrong_subject_proof_and_expiry_fail_closed() {
        let clock = Arc::new(Clock(AtomicU64::new(10)));
        let policy = policy(clock.clone());
        let original = subject();
        let approval = policy
            .evaluate(original.clone(), 20)
            .await
            .unwrap()
            .approval_id
            .unwrap();
        assert_eq!(
            policy
                .finalize(&approval, FinalizeOutcome::Approved, "forged", 20)
                .await,
            Err(PolicyError::ProofRejected)
        );
        let approval = policy
            .evaluate(original.clone(), 20)
            .await
            .unwrap()
            .approval_id
            .unwrap();
        let grant = policy
            .finalize(&approval, FinalizeOutcome::Approved, "proof.1", 20)
            .await
            .unwrap()
            .grant_id
            .unwrap();
        let mut wrong = original.clone();
        wrong.tool_call_ref = "call.2".into();
        assert_eq!(
            policy.consume_grant(&grant, &wrong, 20).await,
            Err(PolicyError::GrantRejected)
        );
        let approval = policy
            .evaluate(original, 20)
            .await
            .unwrap()
            .approval_id
            .unwrap();
        clock.set(400_000);
        assert_eq!(
            policy
                .finalize(&approval, FinalizeOutcome::Approved, "proof.1", 500_000)
                .await,
            Err(PolicyError::NotFound)
        );
    }

    #[tokio::test]
    async fn idempotency_returns_same_terminal_only_for_same_subject() {
        let clock = Arc::new(Clock(AtomicU64::new(10)));
        let policy = policy(clock);
        let subject = subject();
        let first = policy
            .terminal(subject.clone(), "idem.1".into(), CommandStatus::Applied)
            .await
            .unwrap();
        assert_eq!(
            policy
                .terminal(subject.clone(), "idem.1".into(), CommandStatus::Conflict)
                .await
                .unwrap(),
            first
        );
        let mut other = subject;
        other.scope_ref = "scope.2".into();
        assert_eq!(
            policy
                .terminal(other, "idem.1".into(), CommandStatus::Applied)
                .await,
            Err(PolicyError::IdempotencyConflict)
        );
    }
}
