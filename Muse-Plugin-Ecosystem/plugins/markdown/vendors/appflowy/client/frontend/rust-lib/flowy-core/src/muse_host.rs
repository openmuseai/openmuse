//! AppFlowy authority adapter for the domain-neutral Muse Host registry.
//! Scope hints are untrusted selectors; User/Folder/Server state is the authority.

use std::{
  collections::BTreeMap,
  hash::{Hash, Hasher},
  sync::{Arc, Weak},
};

use flowy_folder::entities::view::ViewLayoutPB;
use flowy_folder::manager::FolderManager;
use flowy_user::user_manager::UserManager;
use hmac::{Hmac, Mac};
use lib_infra::async_trait::async_trait;
use muse_host_bridge_contract::ids::ApprovalId;
use muse_host_policy::{
  ApprovalProofVerifier, Effect as PolicyEffect, HostPolicy, PolicyConfig, PolicyEvaluator,
  PolicySubject, PolicyVerdict, RegistryPolicyAuthorizer,
};
use muse_host_registry::{
  AuthoritativeCaller, AuthorityError, AuthorityResolver, HostCapabilityRegistry,
  InvocationAdmission, InvocationAuthorizer, RegistryConfig, RegistryError, ResolvedHostContext,
  ScopeHint,
};
use sha2::Sha256;
use uuid::Uuid;

use crate::server_layer::ServerProvider;

pub(crate) fn make_host_state(
  user_manager: Weak<UserManager>,
  folder_manager: Weak<FolderManager>,
  server_provider: Weak<ServerProvider>,
) -> (Arc<HostCapabilityRegistry>, Arc<HostPolicy>, String) {
  let proofs = Arc::new(AppFlowyApprovalProofs::new());
  let approval_secret = proofs.secret.clone();
  let policy = Arc::new(
    HostPolicy::new(PolicyConfig::default(), Arc::new(AppFlowyPolicy), proofs)
      .expect("static AppFlowy Host policy must compile"),
  );
  let registry = Arc::new(HostCapabilityRegistry::new(
    Arc::new(AppFlowyAuthorityResolver {
      user_manager,
      folder_manager,
      server_provider,
    }),
    RegistryConfig {
      host_generation: "appflowy.local.1".to_string(),
      invocation_authorizer: Arc::new(AppFlowyInvocationAdmission::new(Arc::new(
        RegistryPolicyAuthorizer::new(policy.clone()),
      ))),
      ..RegistryConfig::default()
    },
  ));
  (registry, policy, approval_secret)
}

/// AppFlowy admission policy.
///
/// Every AppFlowy write still needs an approval grant. The one exception is the
/// DSH `muse.resource-presentation` family: `request`/`cancel` only resolve an
/// opaque ref and open a Host surface, and they return their own terminal
/// receipt. They mutate no user data, so they are admitted without a grant —
/// the authority resolver still binds actor, scope and authority epoch, and the
/// DSH tool layer owns the user-facing confirmation. `status` is a `read` and is
/// already admitted by the inner authorizer.
struct AppFlowyInvocationAdmission {
  policy: Arc<RegistryPolicyAuthorizer>,
}

impl AppFlowyInvocationAdmission {
  fn new(policy: Arc<RegistryPolicyAuthorizer>) -> Self {
    Self { policy }
  }
}

impl std::fmt::Debug for AppFlowyInvocationAdmission {
  fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    formatter
      .debug_struct("AppFlowyInvocationAdmission")
      .finish_non_exhaustive()
  }
}

#[async_trait]
impl InvocationAuthorizer for AppFlowyInvocationAdmission {
  async fn authorize(&self, admission: InvocationAdmission<'_>) -> Result<(), RegistryError> {
    if matches!(
      admission.operation.operation_id.as_str(),
      crate::muse_presentation::RESOURCE_PRESENTATION_REQUEST
        | crate::muse_presentation::RESOURCE_PRESENTATION_CANCEL
    ) {
      return Ok(());
    }
    self.policy.authorize(admission).await
  }
}

struct AppFlowyPolicy;
#[async_trait]
impl PolicyEvaluator for AppFlowyPolicy {
  async fn evaluate(&self, subject: &PolicySubject) -> PolicyVerdict {
    match subject.effect {
      PolicyEffect::Read => PolicyVerdict::Allow,
      PolicyEffect::Write => PolicyVerdict::ApprovalRequired {
        reason: "Changing AppFlowy data requires explicit user approval".into(),
      },
    }
  }
}

/// Host-owned HMAC proofs for the desktop sidecar. The secret never leaves AppFlowy except
/// via a private 0600 approval file that only the same-uid DSH connector may read. A Plugin
/// cannot mint this value; transport nonce is not used as evidence.
struct AppFlowyApprovalProofs {
  secret: String,
}

impl AppFlowyApprovalProofs {
  fn new() -> Self {
    Self {
      secret: format!("{}{}", Uuid::new_v4().simple(), Uuid::new_v4().simple()),
    }
  }
}

#[async_trait]
impl ApprovalProofVerifier for AppFlowyApprovalProofs {
  async fn verify(&self, approval_id: &ApprovalId, proof_id: &str, _: &PolicySubject) -> bool {
    expected_proof(&self.secret, approval_id.as_str()) == proof_id
  }
}

pub(crate) fn expected_proof(secret: &str, approval_id: &str) -> String {
  let mut mac = Hmac::<Sha256>::new_from_slice(secret.as_bytes()).expect("HMAC key");
  mac.update(approval_id.as_bytes());
  format!(
    "proof.{}",
    mac
      .finalize()
      .into_bytes()
      .iter()
      .map(|byte| format!("{byte:02x}"))
      .collect::<String>()
  )
}

pub(crate) fn actor_ref(user_id: i64) -> String {
  opaque_ref("actor", &user_id)
}

struct AppFlowyAuthorityResolver {
  user_manager: Weak<UserManager>,
  folder_manager: Weak<FolderManager>,
  server_provider: Weak<ServerProvider>,
}

#[async_trait]
impl AuthorityResolver for AppFlowyAuthorityResolver {
  async fn resolve(
    &self,
    caller: &AuthoritativeCaller,
    hint: Option<&ScopeHint>,
  ) -> Result<ResolvedHostContext, AuthorityError> {
    self.current(caller, hint, None).await
  }
  async fn revalidate(
    &self,
    caller: &AuthoritativeCaller,
    previous: &ResolvedHostContext,
  ) -> Result<ResolvedHostContext, AuthorityError> {
    self.current(caller, None, Some(previous)).await
  }
}

impl AppFlowyAuthorityResolver {
  async fn current(
    &self,
    caller: &AuthoritativeCaller,
    hint: Option<&ScopeHint>,
    previous: Option<&ResolvedHostContext>,
  ) -> Result<ResolvedHostContext, AuthorityError> {
    let user = self
      .user_manager
      .upgrade()
      .ok_or(AuthorityError::Unavailable)?;
    let folder = self
      .folder_manager
      .upgrade()
      .ok_or(AuthorityError::Unavailable)?;
    let server = self
      .server_provider
      .upgrade()
      .ok_or(AuthorityError::Unavailable)?;
    let user_id = user.user_id().map_err(|_| AuthorityError::Unavailable)?;
    let workspace_id = user
      .workspace_id()
      .map_err(|_| AuthorityError::Unavailable)?;
    let actor_ref = actor_ref(user_id);
    if caller.actor_ref != actor_ref {
      return Err(AuthorityError::Denied);
    }

    let (selected_workspace, selected_view, current_selection) = if let Some(previous) = previous {
      let workspace = previous
        .evidence
        .get("appflowy.workspace")
        .cloned()
        .ok_or(AuthorityError::Denied)?;
      let current_selection = previous
        .evidence
        .get("appflowy.selection")
        .is_some_and(|value| value == "current");
      let view = if current_selection {
        current_view_id(&folder).await?
      } else {
        previous.evidence.get("appflowy.view").cloned()
      };
      (workspace, view, current_selection)
    } else {
      let (workspace, explicit_view, current_selection) =
        parse_hint(hint, &workspace_id.to_string())?;
      let view = if current_selection {
        current_view_id(&folder).await?
      } else {
        explicit_view
      };
      (workspace, view, current_selection)
    };
    if selected_workspace != workspace_id.to_string() {
      return Err(AuthorityError::Denied);
    }

    let auth_type = format!("{:?}", server.get_auth_type());
    let scope_ref = match &selected_view {
      Some(view_id) => opaque_ref("scope", &(user_id, workspace_id, view_id)),
      None => opaque_ref("scope", &(user_id, workspace_id)),
    };
    // Actor drift is always a denial. Scope/epoch drift is returned to the Registry so it can
    // terminally invalidate the old binding instead of leaving a repeatedly-denied live binding.
    if previous.is_some_and(|value| value.actor_ref != actor_ref) {
      return Err(AuthorityError::Denied);
    }
    let authority_epoch =
      opaque_hash(&(user_id, workspace_id, &auth_type, selected_view.as_deref()));
    let mut evidence = BTreeMap::from([
      ("appflowy.workspace".to_string(), selected_workspace),
      ("appflowy.auth_type".to_string(), auth_type),
    ]);
    if let Some(view_id) = selected_view {
      let view = folder
        .get_view_pb(&view_id)
        .await
        .map_err(|_| AuthorityError::Denied)?;
      evidence.insert("appflowy.view".to_string(), view_id);
      evidence.insert(
        "appflowy.layout".to_string(),
        layout_label(&view.layout).to_string(),
      );
    }
    if current_selection {
      evidence.insert("appflowy.selection".to_string(), "current".to_string());
    }
    Ok(ResolvedHostContext {
      actor_ref,
      scope_ref,
      authority_epoch,
      evidence,
    })
  }
}

fn parse_hint(
  hint: Option<&ScopeHint>,
  current_workspace: &str,
) -> Result<(String, Option<String>, bool), AuthorityError> {
  let Some(hint) = hint else {
    return Ok((current_workspace.to_string(), None, false));
  };
  if !hint_keys_are_known(hint) {
    return Err(AuthorityError::Denied);
  }
  let workspace = hint
    .refs
    .get("appflowy.workspace")
    .cloned()
    .unwrap_or_else(|| current_workspace.to_string());
  let view = hint.refs.get("appflowy.view").cloned();
  let selection = hint.refs.get("appflowy.selection");
  if selection.is_some_and(|value| value != "current") || (selection.is_some() && view.is_some()) {
    return Err(AuthorityError::Denied);
  }
  Ok((workspace, view, selection.is_some()))
}

/// Scope-hint keys the DSH connector always mirrors onto a request. They are
/// inert selectors — this Host derives no authority from them, and the
/// AppFlowy workspace/view keys above remain the only ones that select state.
const INERT_SCOPE_HINT_KEYS: [&str; 3] = ["muse.session", "muse.actor", "muse.workspace"];

/// True when every ref key of a hint is either an AppFlowy selector or one of the
/// inert DSH namespaces. Used by the authority resolver to reject unknown
/// namespaces without rejecting the DSH connector's mirrored refs.
fn hint_keys_are_known(hint: &ScopeHint) -> bool {
  hint.refs.keys().all(|key| {
    matches!(
      key.as_str(),
      "appflowy.workspace" | "appflowy.view" | "appflowy.selection"
    ) || INERT_SCOPE_HINT_KEYS.contains(&key.as_str())
  })
}

async fn current_view_id(folder: &FolderManager) -> Result<Option<String>, AuthorityError> {
  let setting = folder
    .get_workspace_setting_pb()
    .await
    .map_err(|_| AuthorityError::Unavailable)?;
  Ok(setting.latest_view.map(|view| view.id))
}

fn opaque_ref<T: Hash>(kind: &str, value: &T) -> String {
  format!("{}.{}", kind, opaque_hash(value))
}
fn opaque_hash<T: Hash>(value: &T) -> u64 {
  let mut hasher = std::collections::hash_map::DefaultHasher::new();
  value.hash(&mut hasher);
  hasher.finish()
}

fn layout_label(layout: &ViewLayoutPB) -> &'static str {
  match layout {
    ViewLayoutPB::Document => "document",
    ViewLayoutPB::Grid => "grid",
    ViewLayoutPB::Board => "board",
    ViewLayoutPB::Calendar => "calendar",
    ViewLayoutPB::Chat => "chat",
    ViewLayoutPB::Word => "word",
    ViewLayoutPB::Excel => "excel",
    ViewLayoutPB::Slides => "slides",
    ViewLayoutPB::Pdf => "pdf",
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn hmac_proof_is_bound_to_the_approval_id() {
    let left = expected_proof("secret.1", "approval.1");
    let right = expected_proof("secret.1", "approval.2");
    assert!(left.starts_with("proof."));
    assert_ne!(left, right);
    assert_eq!(left, expected_proof("secret.1", "approval.1"));
  }

  #[test]
  fn scope_hint_accepts_only_the_appflowy_selectors_and_inert_dsh_refs() {
    let mut valid = ScopeHint::default();
    valid.refs.insert("appflowy.view".into(), "view.1".into());
    assert_eq!(
      parse_hint(Some(&valid), "workspace.1").unwrap(),
      ("workspace.1".into(), Some("view.1".into()), false)
    );

    let mut unknown = ScopeHint::default();
    unknown.refs.insert("plugin.scope".into(), "scope.1".into());
    assert_eq!(
      parse_hint(Some(&unknown), "workspace.1"),
      Err(AuthorityError::Denied)
    );

    let mut current = ScopeHint::default();
    current
      .refs
      .insert("appflowy.selection".into(), "current".into());
    assert_eq!(
      parse_hint(Some(&current), "workspace.1").unwrap(),
      ("workspace.1".into(), None, true)
    );

    current.refs.insert("appflowy.view".into(), "view.1".into());
    assert_eq!(
      parse_hint(Some(&current), "workspace.1"),
      Err(AuthorityError::Denied)
    );
  }

  /// The DSH connector mirrors `muse.session`/`muse.actor`/`muse.workspace` onto
  /// every request. They are inert selectors: they must not deny the request, and
  /// they must not change the resolved AppFlowy scope either.
  #[test]
  fn dsh_scope_hints_are_accepted_but_derive_no_authority() {
    let mut dsh = ScopeHint::default();
    dsh.refs.insert("muse.session".into(), "session.1".into());
    dsh.refs.insert("muse.actor".into(), "actor.9".into());
    dsh.refs.insert("muse.workspace".into(), "workspace.9".into());
    assert_eq!(
      parse_hint(Some(&dsh), "workspace.1").unwrap(),
      ("workspace.1".into(), None, false)
    );

    // An AppFlowy selector still wins, and an unknown namespace still denies.
    dsh.refs.insert("appflowy.view".into(), "view.1".into());
    assert_eq!(
      parse_hint(Some(&dsh), "workspace.1").unwrap(),
      ("workspace.1".into(), Some("view.1".into()), false)
    );
    dsh.refs.insert("muse.unknown".into(), "value.1".into());
    assert_eq!(
      parse_hint(Some(&dsh), "workspace.1"),
      Err(AuthorityError::Denied)
    );
  }

  /// The `muse.resource-presentation` family resolves and opens a surface but
  /// changes no user data, so it is admitted without an approval grant while
  /// every other AppFlowy write keeps requiring one.
  #[tokio::test]
  async fn presentation_writes_are_admitted_without_an_approval_grant() {
    use muse_host_registry::{
      BindingProjection, Effect, Idempotency, OperationDescriptor, SchemaDocument,
    };
    use serde_json::{Value, json};

    fn operation(operation_id: &str, effect: Effect) -> OperationDescriptor {
      OperationDescriptor {
        operation_id: operation_id.into(),
        effect,
        input_schema: SchemaDocument::new(json!({"type": "object"})).expect("input schema"),
        output_schema: SchemaDocument::new(json!({"type": "object"})).expect("output schema"),
        cancellable: true,
        idempotency: match effect {
          Effect::Read => Idempotency::None,
          _ => Idempotency::Required,
        },
      }
    }

    fn admission<'a>(
      caller: &'a AuthoritativeCaller,
      context: &'a ResolvedHostContext,
      binding: &'a BindingProjection,
      operation: &'a OperationDescriptor,
      input: &'a Value,
    ) -> InvocationAdmission<'a> {
      InvocationAdmission {
        caller,
        context,
        binding,
        operation,
        input,
        grant_id: None,
        idempotency_key: Some("request.1"),
        session_ref: Some("session.1"),
        tool_call_ref: Some("tool.1"),
        deadline_at_ms: 9_000_000_000_000_000,
      }
    }

    let policy = Arc::new(
      HostPolicy::new(
        PolicyConfig::default(),
        Arc::new(AppFlowyPolicy),
        Arc::new(AppFlowyApprovalProofs::new()),
      )
      .expect("AppFlowy policy"),
    );
    let authorizer =
      AppFlowyInvocationAdmission::new(Arc::new(RegistryPolicyAuthorizer::new(policy)));
    let caller = AuthoritativeCaller {
      actor_ref: actor_ref(7),
    };
    let context = ResolvedHostContext {
      actor_ref: caller.actor_ref.clone(),
      scope_ref: "scope.1".into(),
      authority_epoch: 1,
      evidence: std::collections::BTreeMap::new(),
    };
    let binding = BindingProjection {
      binding_id: "binding.1".into(),
      descriptor_id: "descriptor.1".into(),
      descriptor_revision: "revision.1".into(),
      provider_instance_id: "provider.1".into(),
      host_generation: "generation.1".into(),
      scope_ref: "scope.1".into(),
      expires_at_ms: u64::MAX,
      operation_ids: std::collections::BTreeSet::new(),
      registration_generation: 1,
    };
    let input = json!({"requestRef": "request.1"});

    for operation_id in [
      crate::muse_presentation::RESOURCE_PRESENTATION_REQUEST,
      crate::muse_presentation::RESOURCE_PRESENTATION_CANCEL,
    ] {
      let operation = operation(operation_id, Effect::LocalWrite);
      assert!(
        authorizer
          .authorize(admission(&caller, &context, &binding, &operation, &input))
          .await
          .is_ok(),
        "{} must be admitted without a grant",
        operation_id
      );
    }

    let read = operation(
      crate::muse_presentation::RESOURCE_PRESENTATION_STATUS,
      Effect::Read,
    );
    assert!(
      authorizer
        .authorize(admission(&caller, &context, &binding, &read, &input))
        .await
        .is_ok()
    );

    let rename = operation("appflowy.view.rename", Effect::LocalWrite);
    assert!(matches!(
      authorizer
        .authorize(admission(&caller, &context, &binding, &rename, &input))
        .await,
      Err(RegistryError::GrantRequired)
    ));
  }
}
