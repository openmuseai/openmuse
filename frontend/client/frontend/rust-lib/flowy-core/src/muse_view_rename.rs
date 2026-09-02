//! AppFlowy-owned current-View rename capability.
//!
//! The public DTO contains only a proposed title or an opaque proposal ID.
//! View identity and compare-and-set state remain inside the Host.

use std::{
  collections::BTreeMap,
  sync::Weak,
  time::{SystemTime, UNIX_EPOCH},
};

use flowy_folder::manager::FolderManager;
use lib_infra::async_trait::async_trait;
use muse_host_registry::{
  Cancellation, CapabilityProvider, Effect, Idempotency, OperationDescriptor, ProviderDescriptor,
  ProviderFailure, ProviderInvocation, ResolvedHostContext, SchemaDocument,
};
use serde_json::{json, Value};
use tokio::sync::Mutex;
use uuid::Uuid;

const PROPOSAL_TTL_MS: u64 = 300_000;
const MAX_PROPOSALS: usize = 4096;
const MAX_PROPOSALS_PER_ACTOR: usize = 32;
const MAX_TITLE_CHARS: usize = 256;

pub(crate) struct ViewRenameProvider {
  folder_manager: Weak<FolderManager>,
  proposals: Mutex<BTreeMap<String, Proposal>>,
}

#[derive(Clone)]
struct Proposal {
  actor_ref: String,
  scope_ref: String,
  authority_epoch: u64,
  binding_id: String,
  view_id: String,
  base_title: String,
  normalized_title: String,
  expires_at_ms: u64,
}

impl ViewRenameProvider {
  pub(crate) fn new(folder_manager: Weak<FolderManager>) -> Self {
    Self {
      folder_manager,
      proposals: Mutex::new(BTreeMap::new()),
    }
  }

  async fn propose(
    &self,
    invocation: ProviderInvocation,
    context: ResolvedHostContext,
    cancellation: Cancellation,
  ) -> Result<Value, ProviderFailure> {
    check_active(&cancellation, invocation.deadline_at_ms)?;
    require_current_selection(&context)?;
    let title = invocation
      .input
      .as_object()
      .and_then(|input| input.get("title"))
      .and_then(Value::as_str)
      .and_then(normalize_title)
      .ok_or(ProviderFailure)?;
    let view_id = context
      .evidence
      .get("appflowy.view")
      .cloned()
      .ok_or(ProviderFailure)?;
    let folder = self.folder_manager.upgrade().ok_or(ProviderFailure)?;
    let view = folder
      .get_view_pb(&view_id)
      .await
      .map_err(|_| ProviderFailure)?;
    if view.is_locked.unwrap_or(false) {
      return Err(ProviderFailure);
    }
    check_active(&cancellation, invocation.deadline_at_ms)?;

    let now = unix_ms();
    let expires_at_ms = now.saturating_add(PROPOSAL_TTL_MS);
    let proposal_id = format!("proposal.{}", Uuid::new_v4());
    let proposal = Proposal {
      actor_ref: context.actor_ref.clone(),
      scope_ref: context.scope_ref.clone(),
      authority_epoch: context.authority_epoch,
      binding_id: invocation.binding_id,
      view_id,
      base_title: view.name.clone(),
      normalized_title: title.clone(),
      expires_at_ms,
    };
    let mut proposals = self.proposals.lock().await;
    sweep(&mut proposals, now);
    if proposals.len() >= MAX_PROPOSALS
      || proposals
        .values()
        .filter(|item| item.actor_ref == context.actor_ref)
        .count()
        >= MAX_PROPOSALS_PER_ACTOR
    {
      return Err(ProviderFailure);
    }
    proposals.insert(proposal_id.clone(), proposal);
    Ok(json!({
      "proposalId": proposal_id,
      "preview": {
        "currentTitle": view.name,
        "proposedTitle": title,
        "changed": view.name != title,
        "approvalRequired": true
      },
      "expiresAtMs": expires_at_ms
    }))
  }

  async fn apply(
    &self,
    invocation: ProviderInvocation,
    context: ResolvedHostContext,
    cancellation: Cancellation,
  ) -> Result<Value, ProviderFailure> {
    check_active(&cancellation, invocation.deadline_at_ms)?;
    require_current_selection(&context)?;
    let proposal_id = proposal_id(&invocation.input)?;
    let now = unix_ms();
    let proposal = {
      let mut proposals = self.proposals.lock().await;
      sweep(&mut proposals, now);
      proposals.remove(proposal_id).ok_or(ProviderFailure)?
    };
    if proposal.expires_at_ms <= now
      || proposal.actor_ref != context.actor_ref
      || proposal.scope_ref != context.scope_ref
      || proposal.authority_epoch != context.authority_epoch
      || proposal.binding_id != invocation.binding_id
      || context.evidence.get("appflowy.view") != Some(&proposal.view_id)
    {
      return Err(ProviderFailure);
    }
    check_active(&cancellation, invocation.deadline_at_ms)?;
    let folder = self.folder_manager.upgrade().ok_or(ProviderFailure)?;
    let status = if proposal.base_title == proposal.normalized_title {
      "unchanged"
    } else if folder
      .rename_view_if_name(
        &proposal.view_id,
        &proposal.base_title,
        &proposal.normalized_title,
      )
      .await
      .is_ok()
    {
      "applied"
    } else {
      "conflict"
    };
    Ok(json!({
      "commandId": format!("command.{}", Uuid::new_v4()),
      "status": status
    }))
  }

  async fn status(
    &self,
    invocation: ProviderInvocation,
    context: ResolvedHostContext,
  ) -> Result<Value, ProviderFailure> {
    require_current_selection(&context)?;
    let proposal_id = proposal_id(&invocation.input)?;
    let now = unix_ms();
    let mut proposals = self.proposals.lock().await;
    sweep(&mut proposals, now);
    let proposal = proposals.get(proposal_id).ok_or(ProviderFailure)?;
    if proposal.actor_ref != context.actor_ref
      || proposal.scope_ref != context.scope_ref
      || proposal.authority_epoch != context.authority_epoch
      || proposal.binding_id != invocation.binding_id
    {
      return Err(ProviderFailure);
    }
    Ok(json!({"status": "pending", "expiresAtMs": proposal.expires_at_ms}))
  }
}

#[async_trait]
impl CapabilityProvider for ViewRenameProvider {
  fn descriptor(&self) -> ProviderDescriptor {
    descriptor()
  }

  async fn available(&self, context: &ResolvedHostContext) -> Result<bool, ProviderFailure> {
    Ok(self.folder_manager.upgrade().is_some() && require_current_selection(context).is_ok())
  }

  async fn invoke(
    &self,
    invocation: ProviderInvocation,
    context: ResolvedHostContext,
    cancellation: Cancellation,
  ) -> Result<Value, ProviderFailure> {
    match invocation.operation_id.as_str() {
      "view.rename.propose" => self.propose(invocation, context, cancellation).await,
      "view.rename.apply" => self.apply(invocation, context, cancellation).await,
      "view.rename.status" => self.status(invocation, context).await,
      _ => Err(ProviderFailure),
    }
  }
}

fn normalize_title(title: &str) -> Option<String> {
  let title = title.trim();
  let char_count = title.chars().count();
  if char_count == 0
    || char_count > MAX_TITLE_CHARS
    || title.chars().any(|character| character.is_control())
  {
    None
  } else {
    Some(title.to_owned())
  }
}

fn proposal_id(input: &Value) -> Result<&str, ProviderFailure> {
  input
    .as_object()
    .and_then(|input| input.get("proposalId"))
    .and_then(Value::as_str)
    .filter(|value| value.starts_with("proposal.") && value.len() <= 128)
    .ok_or(ProviderFailure)
}

fn require_current_selection(context: &ResolvedHostContext) -> Result<(), ProviderFailure> {
  if context
    .evidence
    .get("appflowy.selection")
    .is_some_and(|value| value == "current")
    && context.evidence.contains_key("appflowy.view")
  {
    Ok(())
  } else {
    Err(ProviderFailure)
  }
}

fn sweep(proposals: &mut BTreeMap<String, Proposal>, now: u64) {
  proposals.retain(|_, proposal| proposal.expires_at_ms > now);
}

fn check_active(cancellation: &Cancellation, deadline_at_ms: u64) -> Result<(), ProviderFailure> {
  if cancellation.is_cancelled() || deadline_at_ms <= unix_ms() {
    Err(ProviderFailure)
  } else {
    Ok(())
  }
}

fn unix_ms() -> u64 {
  SystemTime::now()
    .duration_since(UNIX_EPOCH)
    .unwrap_or_default()
    .as_millis()
    .min(u64::MAX as u128) as u64
}

fn descriptor() -> ProviderDescriptor {
  let proposal_id_schema = json!({"type": "string", "minLength": 1, "maxLength": 128});
  ProviderDescriptor {
    descriptor_id: "appflowy.view-rename.local".into(),
    revision: "1".into(),
    family_id: "appflowy.view-rename".into(),
    contract_major: 1,
    contract_minor: 0,
    operations: vec![
      OperationDescriptor {
        operation_id: "view.rename.propose".into(),
        effect: Effect::Read,
        input_schema: schema(json!({
          "type": "object", "additionalProperties": false, "required": ["title"],
          "properties": {"title": {"type": "string", "minLength": 1, "maxLength": 256}}
        })),
        output_schema: schema(json!({
          "type": "object", "additionalProperties": false,
          "required": ["proposalId", "preview", "expiresAtMs"],
          "properties": {
            "proposalId": proposal_id_schema.clone(),
            "preview": {
              "type": "object", "additionalProperties": false,
              "required": ["currentTitle", "proposedTitle", "changed", "approvalRequired"],
              "properties": {
                "currentTitle": {"type": "string", "maxLength": 256},
                "proposedTitle": {"type": "string", "maxLength": 256},
                "changed": {"type": "boolean"},
                "approvalRequired": {"const": true}
              }
            },
            "expiresAtMs": {"type": "integer", "minimum": 0}
          }
        })),
        cancellable: true,
        idempotency: Idempotency::None,
      },
      OperationDescriptor {
        operation_id: "view.rename.apply".into(),
        effect: Effect::LocalWrite,
        input_schema: proposal_input_schema(proposal_id_schema.clone()),
        output_schema: schema(json!({
          "type": "object", "additionalProperties": false, "required": ["commandId", "status"],
          "properties": {
            "commandId": {"type": "string", "minLength": 1, "maxLength": 128},
            "status": {"enum": ["applied", "unchanged", "conflict"]}
          }
        })),
        cancellable: true,
        idempotency: Idempotency::Required,
      },
      OperationDescriptor {
        operation_id: "view.rename.status".into(),
        effect: Effect::Read,
        input_schema: proposal_input_schema(proposal_id_schema),
        output_schema: schema(json!({
          "type": "object", "additionalProperties": false, "required": ["status", "expiresAtMs"],
          "properties": {
            "status": {"const": "pending"},
            "expiresAtMs": {"type": "integer", "minimum": 0}
          }
        })),
        cancellable: true,
        idempotency: Idempotency::None,
      },
    ],
    title: Some("Rename current AppFlowy view".into()),
    summary: Some("Propose and apply a policy-gated rename for the Host-selected view".into()),
  }
}

fn proposal_input_schema(proposal_id_schema: Value) -> SchemaDocument {
  schema(json!({
    "type": "object", "additionalProperties": false, "required": ["proposalId"],
    "properties": {"proposalId": proposal_id_schema}
  }))
}

fn schema(mut value: Value) -> SchemaDocument {
  value["$schema"] = json!("https://json-schema.org/draft/2020-12/schema");
  SchemaDocument::new(value).expect("static View rename schema must compile")
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn title_normalization_is_bounded_and_rejects_controls() {
    assert_eq!(
      normalize_title("  Q3 Roadmap  ").as_deref(),
      Some("Q3 Roadmap")
    );
    assert!(normalize_title("\n").is_none());
    assert!(normalize_title(&"a".repeat(257)).is_none());
  }

  #[test]
  fn descriptor_keeps_domain_ids_out_of_public_inputs() {
    let descriptor = descriptor();
    let encoded = descriptor
      .operations
      .iter()
      .map(|operation| operation.input_schema.value.to_string())
      .collect::<String>();
    assert!(!encoded.contains("viewId"));
    assert!(!encoded.contains("workspaceId"));
    assert!(!encoded.contains("actorId"));
    assert!(!encoded.contains("grantId"));
  }

  #[test]
  fn provider_schema_digests_match_the_hot_pluggable_typescript_plugin() {
    let descriptor = descriptor();
    let expected = [
      (
        "view.rename.propose",
        "sha256:58ae726c26aa65bf5c259e30beec71351c4bdffcf059a3569d0c7556681d47dc",
        "sha256:b005d37b902b2ffb9025201e5a14cf4225a2ea6c3a35b75696c2e30dea6af28f",
      ),
      (
        "view.rename.apply",
        "sha256:ee086f896c4fbdd7e804105503a282b8a5a7b17396cc729f1779f53f7ca10f83",
        "sha256:d47a9edc35334aa2aa45cd0aff182cf816b583b3c29af64a7448adb1b84dca6f",
      ),
      (
        "view.rename.status",
        "sha256:ee086f896c4fbdd7e804105503a282b8a5a7b17396cc729f1779f53f7ca10f83",
        "sha256:dd244450f318ac1e281a8a4fb9ef0c04b441df59e69508f62c65114fe9605a1a",
      ),
    ];
    for (operation_id, input_digest, output_digest) in expected {
      let operation = descriptor.operation(operation_id).unwrap();
      assert_eq!(operation.input_schema.digest, input_digest);
      assert_eq!(operation.output_schema.digest, output_digest);
    }
  }
}
