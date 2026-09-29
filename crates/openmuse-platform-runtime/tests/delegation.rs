use openmuse_contract::{ContractScope, PrincipalKind, PrincipalRef};
use openmuse_platform_runtime::{
    AuditOutcome, DelegatedCapability, DelegatedRequest, DelegationBroker, DelegationError,
    GrantSet, InMemoryPolicyProvider,
};
use openmuse_plugin_protocol::{Permission, PluginId};

fn grants(values: &[&str]) -> GrantSet {
    values.iter().map(|value| Permission::new(*value)).collect()
}

fn scope(workspace: &str) -> ContractScope {
    ContractScope {
        authority_ref: "authority.tenant".into(),
        workspace_ref: Some(workspace.into()),
        resource_ref: None,
    }
}

fn actor() -> PrincipalRef {
    PrincipalRef {
        principal_ref: "actor.user".into(),
        kind: PrincipalKind::User,
    }
}

fn configured_broker() -> DelegationBroker {
    let all = grants(&["workspace.read", "sandbox.execute", "office.write"]);
    let mut policy = InMemoryPolicyProvider::with_tenant_ceiling(all.clone());
    policy.set_actor_grants("actor.user", all.clone());
    for plugin in ["plugin.dsh", "provider.sandbox", "worker.office"] {
        policy.set_plugin_grants(PluginId::new(plugin), all.clone());
    }
    policy.set_workspace_grants("workspace.primary", all);
    DelegationBroker::new(policy)
}

fn handle(reference: &str, audience: &str, generation: u64) -> DelegatedCapability {
    DelegatedCapability {
        handle_ref: reference.into(),
        actor_ref: "actor.user".into(),
        audience: PluginId::new(audience),
        scope: scope("workspace.primary"),
        permissions: grants(&["workspace.read", "sandbox.execute", "office.write"]),
        generation,
        expires_at_ms: 100,
        revoked: false,
    }
}

fn request(handle_ref: &str, caller: &str, target: &str, generation: u64) -> DelegatedRequest {
    DelegatedRequest {
        request_id: format!("request.{handle_ref}"),
        actor: actor(),
        caller: PluginId::new(caller),
        target_provider: PluginId::new(target),
        scope: scope("workspace.primary"),
        revision: "revision.7".into(),
        generation,
        deadline_at_ms: 90,
        handle_ref: handle_ref.into(),
        operation: "sandbox.exec".into(),
    }
}

#[test]
fn user_dsh_sandbox_worker_chain_is_authorized_and_auditable() {
    let mut broker = configured_broker();
    broker
        .issue_handle(handle("handle.dsh", "plugin.dsh", 4))
        .unwrap();
    broker
        .issue_handle(handle("handle.sandbox", "provider.sandbox", 4))
        .unwrap();

    let first = broker
        .authorize(
            10,
            &request("handle.dsh", "plugin.dsh", "provider.sandbox", 4),
            &grants(&["workspace.read", "sandbox.execute"]),
        )
        .unwrap();
    let second = broker
        .authorize(
            11,
            &request("handle.sandbox", "provider.sandbox", "worker.office", 4),
            &grants(&["workspace.read", "office.write"]),
        )
        .unwrap();

    assert_eq!(
        first.audit_receipt.actor_ref,
        second.audit_receipt.actor_ref
    );
    assert_eq!(first.audit_receipt.target_provider, "provider.sandbox");
    assert_eq!(second.audit_receipt.target_provider, "worker.office");
    assert_eq!(broker.audit_receipts().len(), 2);
    assert!(
        broker
            .audit_receipts()
            .iter()
            .all(|receipt| receipt.outcome == AuditOutcome::Allowed)
    );
}

#[test]
fn confused_deputy_and_target_grant_escalation_fail_closed() {
    let mut broker = configured_broker();
    broker
        .issue_handle(handle("handle.dsh", "plugin.dsh", 4))
        .unwrap();

    let attacker = request("handle.dsh", "plugin.attacker", "provider.sandbox", 4);
    assert_eq!(
        broker.authorize(10, &attacker, &grants(&["workspace.read"])),
        Err(DelegationError::AudienceMismatch)
    );

    broker.policy_mut().set_plugin_grants(
        PluginId::new("provider.sandbox"),
        grants(&["workspace.read"]),
    );
    assert_eq!(
        broker.authorize(
            10,
            &request("handle.dsh", "plugin.dsh", "provider.sandbox", 4),
            &grants(&["sandbox.execute"]),
        ),
        Err(DelegationError::PermissionDenied)
    );
}

#[test]
fn revoked_expired_stale_and_cross_workspace_handles_fail() {
    let mut broker = configured_broker();
    broker
        .issue_handle(handle("handle.live", "plugin.dsh", 4))
        .unwrap();
    let live = request("handle.live", "plugin.dsh", "provider.sandbox", 4);

    let mut stale = live.clone();
    stale.generation = 3;
    assert_eq!(
        broker.authorize(10, &stale, &grants(&["workspace.read"])),
        Err(DelegationError::StaleGeneration)
    );

    let mut replay = live.clone();
    replay.scope = scope("workspace.other");
    assert_eq!(
        broker.authorize(10, &replay, &grants(&["workspace.read"])),
        Err(DelegationError::ScopeMismatch)
    );

    broker.revoke("handle.live").unwrap();
    assert_eq!(
        broker.authorize(10, &live, &grants(&["workspace.read"])),
        Err(DelegationError::HandleRevoked)
    );

    broker
        .issue_handle(handle("handle.expired", "plugin.dsh", 4))
        .unwrap();
    let mut expired = request("handle.expired", "plugin.dsh", "provider.sandbox", 4);
    expired.deadline_at_ms = 200;
    assert_eq!(
        broker.authorize(100, &expired, &grants(&["workspace.read"])),
        Err(DelegationError::HandleExpired)
    );
}

#[test]
fn audit_receipts_redact_paths_tokens_presigned_queries_and_s3_keys() {
    let mut broker = configured_broker();
    broker
        .issue_handle(handle("handle.audit", "plugin.dsh", 4))
        .unwrap();
    let mut unsafe_request = request("handle.audit", "plugin.dsh", "provider.sandbox", 4);
    unsafe_request.request_id = "token=permanent-secret".into();
    unsafe_request.revision = "s3://bucket/private/key".into();
    unsafe_request.operation = "run?X-Amz-Signature=secret".into();
    broker
        .authorize(10, &unsafe_request, &grants(&["workspace.read"]))
        .unwrap();

    let encoded = serde_json::to_string(broker.audit_receipts()).unwrap();
    assert!(!encoded.contains("permanent-secret"));
    assert!(!encoded.contains("bucket/private"));
    assert!(!encoded.contains("X-Amz"));
    assert!(encoded.contains("[redacted]"));
}

#[test]
fn delegated_request_context_round_trips_and_rejects_unknown_fields() {
    let request = request("handle.wire", "plugin.dsh", "provider.sandbox", 4);
    let mut value = serde_json::to_value(&request).unwrap();
    let decoded: openmuse_plugin_protocol::DelegatedRequestContext =
        serde_json::from_value(value.clone()).unwrap();
    assert_eq!(decoded, request);

    value
        .as_object_mut()
        .unwrap()
        .insert("ambientAuthority".into(), serde_json::Value::Bool(true));
    assert!(
        serde_json::from_value::<openmuse_plugin_protocol::DelegatedRequestContext>(value).is_err()
    );
}
