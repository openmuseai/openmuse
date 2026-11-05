//! AppFlowy-owned bounded Markdown projection and policy-gated edits
//! for the current Document View.
//!
//! Public DTOs contain only markdown text, an operation, or an opaque proposal ID.
//! View identity and compare-and-set state remain inside the Host.

use std::{
  collections::{BTreeMap, HashMap},
  sync::{Arc, Weak},
  time::{SystemTime, UNIX_EPOCH},
};

use collab_document::blocks::Block;
use collab_document::document::Document;
use collab_document::document_data::PARAGRAPH_BLOCK_TYPE;
use flowy_document::{manager::DocumentManager, notification::publish_muse_markdown_domain_change};
use flowy_folder::{
  entities::view::ViewLayoutPB,
  manager::FolderManager,
};
use lib_infra::async_trait::async_trait;
use muse_document_contract::{
  DocumentChangeV2, DocumentCommitEventV2, DocumentContentV2, DocumentOriginV2, DocumentSnapshotV2,
};
use muse_host_events::HostEventHub;
use muse_host_registry::{
  Cancellation, CapabilityProvider, Effect, Idempotency, OperationDescriptor, ProviderDescriptor,
  ProviderFailure, ProviderInvocation, ResolvedHostContext, SchemaDocument,
};
use serde_json::{json, Value};
use sha2::{Digest, Sha256};
use tokio::sync::Mutex;
use uuid::Uuid;

const MAX_MARKDOWN_BYTES: usize = 64 * 1024;
const PROPOSAL_TTL_MS: u64 = 300_000;
const MAX_PROPOSALS: usize = 4096;
const MAX_PROPOSALS_PER_ACTOR: usize = 32;
const MARKDOWN_DOMAIN_CHANGE_DIGEST: &str =
  "sha256:25a07a06a54da0f8134a191784040d9f47c076048c2682cdd994a48a9c42023d";
const DOCUMENT_COMMIT_EVENT_DIGEST: &str =
  "sha256:435e9c97ad99ea7641a943d82f50a2272694b5143fdb81f86b047b5029bfd6db";

pub(crate) struct MarkdownProvider {
  document_manager: Weak<DocumentManager>,
  folder_manager: Weak<FolderManager>,
  proposals: Mutex<BTreeMap<String, Proposal>>,
  epoch_ref: String,
  events: Arc<HostEventHub>,
}

#[derive(Clone)]
struct Proposal {
  actor_ref: String,
  scope_ref: String,
  authority_epoch: u64,
  binding_id: String,
  view_id: String,
  base_markdown: String,
  proposed_markdown: String,
  expires_at_ms: u64,
}

impl MarkdownProvider {
  pub(crate) fn new(
    document_manager: Weak<DocumentManager>,
    folder_manager: Weak<FolderManager>,
    events: Arc<HostEventHub>,
  ) -> Self {
    Self {
      document_manager,
      folder_manager,
      proposals: Mutex::new(BTreeMap::new()),
      epoch_ref: format!("epoch.{}", Uuid::new_v4()),
      events,
    }
  }

  async fn read(
    &self,
    invocation: ProviderInvocation,
    context: ResolvedHostContext,
    cancellation: Cancellation,
  ) -> Result<Value, ProviderFailure> {
    if invocation
      .input
      .as_object()
      .is_none_or(|input| !input.is_empty())
    {
      return Err(ProviderFailure);
    }
    check_active(&cancellation, invocation.deadline_at_ms)?;
    let markdown = current_markdown(&self.document_manager, &context).await?;
    if cancellation.is_cancelled() {
      return Err(ProviderFailure);
    }
    let document_id = current_document_id(&context)?.to_string();
    let revision = revision_for(&markdown);
    let (markdown, truncated) = truncate_utf8(markdown, MAX_MARKDOWN_BYTES);
    serde_json::to_value(DocumentSnapshotV2 {
      protocol: "muse.document/snapshot/v2".into(),
      resource_ref: document_id,
      revision,
      content: DocumentContentV2 {
        media_type: "text/markdown".into(),
        byte_length: markdown.len() as u64,
        text: markdown,
        truncated,
      },
    })
    .map_err(|_| ProviderFailure)
  }

  async fn snapshot(
    &self,
    invocation: ProviderInvocation,
    _context: ResolvedHostContext,
    cancellation: Cancellation,
  ) -> Result<Value, ProviderFailure> {
    check_active(&cancellation, invocation.deadline_at_ms)?;
    let resource_ref = invocation
      .input
      .as_object()
      .and_then(|input| input.get("resourceRef"))
      .and_then(Value::as_str)
      .map(str::trim)
      .filter(|value| !value.is_empty() && value.len() <= 128)
      .ok_or(ProviderFailure)?;
    let folder = self.folder_manager.upgrade().ok_or(ProviderFailure)?;
    let view = folder
      .get_view_pb(resource_ref)
      .await
      .map_err(|_| ProviderFailure)?;
    if view.layout != ViewLayoutPB::Document {
      return Err(ProviderFailure);
    }
    let document_id = Uuid::parse_str(resource_ref).map_err(|_| ProviderFailure)?;
    let manager = self.document_manager.upgrade().ok_or(ProviderFailure)?;
    let markdown = manager
      .get_document_text(&document_id)
      .await
      .map_err(|_| ProviderFailure)?;
    if cancellation.is_cancelled() {
      return Err(ProviderFailure);
    }
    let revision = revision_for(&markdown);
    let (markdown, truncated) = truncate_utf8(markdown, MAX_MARKDOWN_BYTES);
    serde_json::to_value(DocumentSnapshotV2 {
      protocol: "muse.document/snapshot/v2".into(),
      resource_ref: resource_ref.to_string(),
      revision,
      content: DocumentContentV2 {
        media_type: "text/markdown".into(),
        byte_length: markdown.len() as u64,
        text: markdown,
        truncated,
      },
    })
    .map_err(|_| ProviderFailure)
  }

  async fn propose(
    &self,
    invocation: ProviderInvocation,
    context: ResolvedHostContext,
    cancellation: Cancellation,
  ) -> Result<Value, ProviderFailure> {
    check_active(&cancellation, invocation.deadline_at_ms)?;
    require_current_selection(&context)?;
    let input = invocation.input.as_object().ok_or(ProviderFailure)?;
    let expected_revision = input
      .get("expectedRevision")
      .and_then(Value::as_str)
      .ok_or(ProviderFailure)?;
    let mutation = input
      .get("mutation")
      .and_then(Value::as_object)
      .ok_or(ProviderFailure)?;
    let op = mutation
      .get("kind")
      .and_then(Value::as_str)
      .filter(|value| matches!(*value, "insert" | "replace" | "delete"))
      .ok_or(ProviderFailure)?
      .to_string();
    let markdown = mutation.get("text").and_then(Value::as_str);
    let find = mutation.get("find").and_then(Value::as_str);
    let view_id = context
      .evidence
      .get("appflowy.view")
      .cloned()
      .ok_or(ProviderFailure)?;
    let current = current_markdown(&self.document_manager, &context).await?;
    if expected_revision != revision_for(&current) {
      return Err(ProviderFailure);
    }
    if current.len() > MAX_MARKDOWN_BYTES {
      return Err(ProviderFailure);
    }
    check_active(&cancellation, invocation.deadline_at_ms)?;
    let proposed = apply_op(&current, &op, markdown, find).ok_or(ProviderFailure)?;
    if proposed.len() > MAX_MARKDOWN_BYTES {
      return Err(ProviderFailure);
    }

    let now = unix_ms();
    let expires_at_ms = now.saturating_add(PROPOSAL_TTL_MS);
    let proposal_id = format!("proposal.{}", Uuid::new_v4());
    let proposal = Proposal {
      actor_ref: context.actor_ref.clone(),
      scope_ref: context.scope_ref.clone(),
      authority_epoch: context.authority_epoch,
      binding_id: invocation.binding_id,
      view_id: view_id.clone(),
      base_markdown: current.clone(),
      proposed_markdown: proposed.clone(),
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
      "protocol": "muse.document/proposal/v2",
      "proposalRef": proposal_id,
      "resourceRef": view_id,
      "expectedRevision": expected_revision,
      "preview": {
        "before": current,
        "after": proposed,
        "changed": current != proposed
      },
      "approvalRequired": true,
      "expiresAt": expires_at_ms
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
    let document_id = current_document_id(&context)?;
    let manager = self.document_manager.upgrade().ok_or(ProviderFailure)?;
    manager
      .open_document(&document_id)
      .await
      .map_err(|_| ProviderFailure)?;
    check_active(&cancellation, invocation.deadline_at_ms)?;
    let document = manager
      .editable_document(&document_id)
      .await
      .map_err(|_| ProviderFailure)?;
    let mut document = document.write().await;
    let current = document.paragraphs().join("\n");
    let status = if proposal.base_markdown == proposal.proposed_markdown {
      "unchanged"
    } else if current != proposal.base_markdown {
      "conflict"
    } else {
      replace_paragraphs(&mut document, &proposal.proposed_markdown)?;
      "applied"
    };
    drop(document);
    let command_id = format!("command.{}", Uuid::new_v4());
    let mut domain_revision = revision_for(&current);
    let previous_revision = domain_revision.clone();
    let mut event_cursor: Option<String> = None;
    let mut event_publication_status = "not-needed";
    if status == "applied" {
      domain_revision = revision_for(&proposal.proposed_markdown);

      let occurred_at = unix_ms();
      let domain_change = json!({
        "protocol": "muse.domain-change/v1",
        "pluginId": "muse.appflowy.markdown",
        "providerInstanceRef": "provider.appflowy.markdown.local.1",
        "scopeRef": context.scope_ref,
        "resourceRef": document_id.to_string(),
        "eventType": "markdown.document.changed",
        "eventSchemaDigest": MARKDOWN_DOMAIN_CHANGE_DIGEST,
        "domainRevision": domain_revision,
        "epochRef": self.epoch_ref,
        "commandRef": command_id,
        "origin": "external-command",
        "occurredAt": occurred_at,
        "payload": {
          "resourceRef": document_id.to_string(),
          "changeKind": "operations-applied",
          "changedBlockRefs": [],
          "structureChanged": true,
          "selectionRebaseHint": "sync-required"
        }
      });
      // Host-local Collab writes are isRemote=false. Notify the Markdown UI Facet
      // with origin=external-command instead of pretending this was cloud collab.
      publish_muse_markdown_domain_change(&document_id.to_string(), &domain_change.to_string());
      let document_event = serde_json::to_value(DocumentCommitEventV2 {
        protocol: "muse.document/event/v2".into(),
        event_id: format!("event.{}", Uuid::new_v4()),
        cursor: "host-assigned".into(),
        resource_ref: document_id.to_string(),
        revision: domain_revision.clone(),
        command_ref: command_id.clone(),
        origin: DocumentOriginV2::Agent,
        change: DocumentChangeV2 {
          kind: "content-replaced".into(),
        },
        occurred_at,
      })
      .map_err(|_| ProviderFailure)?;
      let published = self.events.publish(
        occurred_at,
        json!({
          "eventKind": "provider.event",
          "descriptorId": "appflowy.document.local",
          "descriptorRevision": "3",
          "eventType": "document.commit.v2",
          "schemaDigest": DOCUMENT_COMMIT_EVENT_DIGEST,
          "payload": document_event
        }),
      );
      match published {
        Ok(record) => {
          event_publication_status = "published";
          event_cursor = Some(record.cursor.to_string());
        },
        Err(_) => event_publication_status = "degraded",
      }
    }
    Ok(json!({
      "protocol": "muse.document/receipt/v2",
      "commandRef": command_id,
      "status": status,
      "resourceRef": document_id.to_string(),
      "previousRevision": previous_revision,
      "revision": domain_revision,
      "eventCursor": event_cursor,
      "eventPublicationStatus": event_publication_status,
      "idempotencyKey": invocation.idempotency_key.unwrap_or_default()
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
    Ok(json!({"status": "pending", "expiresAt": proposal.expires_at_ms}))
  }
}

#[async_trait]
impl CapabilityProvider for MarkdownProvider {
  fn descriptor(&self) -> ProviderDescriptor {
    descriptor()
  }

  async fn available(&self, _context: &ResolvedHostContext) -> Result<bool, ProviderFailure> {
    Ok(self.document_manager.upgrade().is_some() && self.folder_manager.upgrade().is_some())
  }

  async fn invoke(
    &self,
    invocation: ProviderInvocation,
    context: ResolvedHostContext,
    cancellation: Cancellation,
  ) -> Result<Value, ProviderFailure> {
    match invocation.operation_id.as_str() {
      "document.current.query" => self.read(invocation, context, cancellation).await,
      "document.resource.snapshot" => self.snapshot(invocation, context, cancellation).await,
      "document.current.propose" => self.propose(invocation, context, cancellation).await,
      "document.current.apply" => self.apply(invocation, context, cancellation).await,
      "document.command.status" => self.status(invocation, context).await,
      _ => Err(ProviderFailure),
    }
  }
}

async fn current_markdown(
  document_manager: &Weak<DocumentManager>,
  context: &ResolvedHostContext,
) -> Result<String, ProviderFailure> {
  require_document_layout(context)?;
  let document_id = current_document_id(context)?;
  let manager = document_manager.upgrade().ok_or(ProviderFailure)?;
  manager
    .get_document_text(&document_id)
    .await
    .map_err(|_| ProviderFailure)
}

fn apply_op(current: &str, op: &str, markdown: Option<&str>, find: Option<&str>) -> Option<String> {
  match op {
    "insert" => {
      let chunk = normalize_markdown(markdown?)?;
      if current.is_empty() {
        Some(chunk)
      } else {
        Some(format!("{current}\n{chunk}"))
      }
    },
    "replace" => {
      let chunk = normalize_markdown(markdown?)?;
      match find {
        Some(find) if !find.is_empty() => {
          if !current.contains(find) {
            None
          } else {
            Some(current.replacen(find, &chunk, 1))
          }
        },
        _ => Some(chunk),
      }
    },
    "delete" => {
      let find = find.filter(|value| !value.is_empty())?;
      if !current.contains(find) {
        None
      } else {
        Some(current.replacen(find, "", 1))
      }
    },
    _ => None,
  }
}

fn normalize_markdown(value: &str) -> Option<String> {
  if value.len() > MAX_MARKDOWN_BYTES || value.chars().any(|character| character == '\0') {
    None
  } else {
    Some(value.to_owned())
  }
}

fn revision_for(value: &str) -> String {
  format!("sha256:{:x}", Sha256::digest(value.as_bytes()))
}

fn replace_paragraphs(document: &mut Document, markdown: &str) -> Result<(), ProviderFailure> {
  let page_id = document.get_page_id().ok_or(ProviderFailure)?;
  let existing = document.get_block_children_ids(&page_id);
  for child_id in existing {
    document
      .delete_block(&child_id)
      .map_err(|_| ProviderFailure)?;
  }
  let lines: Vec<&str> = if markdown.is_empty() {
    vec![""]
  } else {
    markdown.split('\n').collect()
  };
  let mut prev_id = None;
  for line in lines {
    let block_id = new_id();
    let children_id = new_id();
    let text_id = new_id();
    document
      .insert_block(
        Block {
          id: block_id.clone(),
          ty: PARAGRAPH_BLOCK_TYPE.to_string(),
          parent: page_id.clone(),
          children: children_id,
          external_id: Some(text_id.clone()),
          external_type: Some("text".to_string()),
          data: HashMap::new(),
        },
        prev_id,
      )
      .map_err(|_| ProviderFailure)?;
    if !line.is_empty() {
      document.apply_text_delta(&text_id, json!([{ "insert": line }]).to_string());
    }
    prev_id = Some(block_id);
  }
  Ok(())
}

fn new_id() -> String {
  Uuid::new_v4().simple().to_string()[..10].to_string()
}

fn current_document_id(context: &ResolvedHostContext) -> Result<Uuid, ProviderFailure> {
  require_current_selection(context)?;
  context
    .evidence
    .get("appflowy.view")
    .and_then(|view| Uuid::parse_str(view).ok())
    .ok_or(ProviderFailure)
}

fn proposal_id(input: &Value) -> Result<&str, ProviderFailure> {
  input
    .as_object()
    .and_then(|input| input.get("proposalRef"))
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

fn require_document_layout(context: &ResolvedHostContext) -> Result<(), ProviderFailure> {
  match context.evidence.get("appflowy.layout").map(String::as_str) {
    Some("document") | None => Ok(()),
    Some(_) => Err(ProviderFailure),
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

fn truncate_utf8(mut value: String, max_bytes: usize) -> (String, bool) {
  if value.len() <= max_bytes {
    return (value, false);
  }
  let mut end = max_bytes;
  while !value.is_char_boundary(end) {
    end -= 1;
  }
  value.truncate(end);
  (value, true)
}

fn descriptor() -> ProviderDescriptor {
  let opaque_ref = json!({"type": "string", "minLength": 1, "maxLength": 128});
  let revision = json!({"type": "string", "pattern": "^sha256:[0-9a-f]{64}$"});
  ProviderDescriptor {
    descriptor_id: "appflowy.document.local".into(),
    revision: "4".into(),
    family_id: "muse.document".into(),
    contract_major: 2,
    contract_minor: 0,
    operations: vec![
      OperationDescriptor {
        operation_id: "document.current.query".into(),
        effect: Effect::Read,
        input_schema: schema(json!({"type": "object", "additionalProperties": false})),
        output_schema: schema(json!({
          "type": "object", "additionalProperties": false,
          "required": ["protocol", "resourceRef", "revision", "content"],
          "properties": {
            "protocol": {"const": "muse.document/snapshot/v2"},
            "resourceRef": opaque_ref.clone(), "revision": revision.clone(),
            "content": {"type": "object", "additionalProperties": false,
              "required": ["mediaType", "text", "truncated", "byteLength"],
              "properties": {
                "mediaType": {"const": "text/markdown"}, "text": {"type": "string", "maxLength": 65536},
                "truncated": {"type": "boolean"}, "byteLength": {"type": "integer", "minimum": 0, "maximum": 65536}
              }
            }
          }
        })),
        cancellable: true,
        idempotency: Idempotency::None,
      },
      OperationDescriptor {
        operation_id: "document.resource.snapshot".into(),
        effect: Effect::Read,
        input_schema: schema(json!({
          "type": "object", "additionalProperties": false, "required": ["resourceRef"],
          "properties": { "resourceRef": opaque_ref.clone() }
        })),
        output_schema: schema(json!({
          "type": "object", "additionalProperties": false,
          "required": ["protocol", "resourceRef", "revision", "content"],
          "properties": {
            "protocol": {"const": "muse.document/snapshot/v2"},
            "resourceRef": opaque_ref.clone(), "revision": revision.clone(),
            "content": {"type": "object", "additionalProperties": false,
              "required": ["mediaType", "text", "truncated", "byteLength"],
              "properties": {
                "mediaType": {"const": "text/markdown"}, "text": {"type": "string", "maxLength": 65536},
                "truncated": {"type": "boolean"}, "byteLength": {"type": "integer", "minimum": 0, "maximum": 65536}
              }
            }
          }
        })),
        cancellable: true,
        idempotency: Idempotency::None,
      },
      OperationDescriptor {
        operation_id: "document.current.propose".into(),
        effect: Effect::Read,
        input_schema: schema(json!({
          "type": "object", "additionalProperties": false, "required": ["expectedRevision", "mutation"],
          "properties": {
            "expectedRevision": revision.clone(),
            "mutation": {"type": "object", "required": ["kind"], "properties": {
              "kind": {"enum": ["insert", "replace", "delete"]},
              "text": {"type": "string", "maxLength": 65536},
              "find": {"type": "string", "minLength": 1, "maxLength": 65536}
            }}
          }
        })),
        output_schema: schema(json!({
          "type": "object", "additionalProperties": false,
          "required": ["protocol", "proposalRef", "resourceRef", "expectedRevision", "preview", "approvalRequired", "expiresAt"],
          "properties": {
            "protocol": {"const": "muse.document/proposal/v2"}, "proposalRef": opaque_ref.clone(),
            "resourceRef": opaque_ref.clone(), "expectedRevision": revision.clone(),
            "preview": {"type": "object", "additionalProperties": false,
              "required": ["before", "after", "changed"], "properties": {
                "before": {"type": "string", "maxLength": 65536}, "after": {"type": "string", "maxLength": 65536}, "changed": {"type": "boolean"}
              }},
            "approvalRequired": {"const": true}, "expiresAt": {"type": "integer", "minimum": 0}
          }
        })),
        cancellable: true,
        idempotency: Idempotency::None,
      },
      OperationDescriptor {
        operation_id: "document.current.apply".into(),
        effect: Effect::LocalWrite,
        input_schema: proposal_input_schema(opaque_ref.clone()),
        output_schema: schema(json!({
          "type": "object", "additionalProperties": false,
          "required": ["protocol", "commandRef", "status", "resourceRef", "previousRevision", "revision", "eventPublicationStatus", "idempotencyKey"],
          "properties": {
            "protocol": {"const": "muse.document/receipt/v2"}, "commandRef": opaque_ref.clone(),
            "status": {"enum": ["applied", "unchanged", "conflict"]},
            "resourceRef": opaque_ref.clone(), "previousRevision": revision.clone(), "revision": revision.clone(),
            "eventCursor": {"type": ["string", "null"]},
            "eventPublicationStatus": {"enum": ["published", "not-needed", "degraded"]}, "idempotencyKey": {"type": "string"}
          }
        })),
        cancellable: true,
        idempotency: Idempotency::Required,
      },
      OperationDescriptor {
        operation_id: "document.command.status".into(),
        effect: Effect::Read,
        input_schema: proposal_input_schema(opaque_ref),
        output_schema: schema(json!({
          "type": "object", "additionalProperties": false, "required": ["status", "expiresAt"],
          "properties": {
            "status": {"const": "pending"},
            "expiresAt": {"type": "integer", "minimum": 0}
          }
        })),
        cancellable: true,
        idempotency: Idempotency::None,
      },
    ],
    title: Some("AppFlowy local muse.document@2 provider".into()),
    summary: Some(
      "Host-selected Document projection with propose/approve/apply/event semantics".into(),
    ),
  }
}

fn proposal_input_schema(proposal_id_schema: Value) -> SchemaDocument {
  schema(json!({
    "type": "object", "additionalProperties": false, "required": ["proposalRef"],
    "properties": {"proposalRef": proposal_id_schema}
  }))
}

fn schema(mut value: Value) -> SchemaDocument {
  value["$schema"] = json!("https://json-schema.org/draft/2020-12/schema");
  SchemaDocument::new(value).expect("static Markdown schema must compile")
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn truncation_preserves_utf8_boundaries() {
    let (value, truncated) = truncate_utf8("ab中".into(), 4);
    assert_eq!(value, "ab");
    assert!(truncated);
  }

  #[test]
  fn public_read_input_has_no_target_identifier() {
    let schema = &descriptor().operations[0].input_schema.value;
    assert_eq!(
      schema,
      &json!({
        "$schema": "https://json-schema.org/draft/2020-12/schema",
        "type": "object", "additionalProperties": false
      })
    );
    let ops = descriptor().operations;
    let snapshot = ops
      .iter()
      .find(|operation| operation.operation_id == "document.resource.snapshot")
      .unwrap();
    assert!(snapshot.input_schema.value.to_string().contains("resourceRef"));
    assert!(!snapshot.input_schema.value.to_string().contains("viewId"));
  }

  #[test]
  fn markdown_ops_are_bounded_insert_update_delete() {
    assert_eq!(
      apply_op("Hello", "insert", Some("World"), None).as_deref(),
      Some("Hello\nWorld")
    );
    assert_eq!(
      apply_op("Hello World", "replace", Some("Muse"), Some("World")).as_deref(),
      Some("Hello Muse")
    );
    assert_eq!(
      apply_op("Hello World", "delete", None, Some(" World")).as_deref(),
      Some("Hello")
    );
    assert!(apply_op("Hello", "delete", None, Some("missing")).is_none());
    assert!(normalize_markdown(&"a".repeat(MAX_MARKDOWN_BYTES + 1)).is_none());
  }

  #[test]
  fn descriptor_keeps_domain_ids_out_of_public_inputs() {
    let encoded = descriptor()
      .operations
      .iter()
      .map(|operation| operation.input_schema.value.to_string())
      .collect::<String>();
    assert!(!encoded.contains("viewId"));
    assert!(!encoded.contains("workspaceId"));
    assert!(!encoded.contains("documentId"));
    assert!(!encoded.contains("actorId"));
    assert!(!encoded.contains("grantId"));
  }

  #[test]
  fn document_v2_schema_digests_match_generated_typescript_sdk() {
    let descriptor = descriptor();
    let snapshot = descriptor
      .operation("document.resource.snapshot")
      .expect("snapshot op");
    assert_eq!(
      snapshot.input_schema.digest,
      "sha256:03183e81cbda72d9f3739453d8b8f33b0c301f16e85087cf2905591f0ac23d7b"
    );
    assert_eq!(
      snapshot.output_schema.digest,
      "sha256:5440b5f0cb01eae9fb706b1886ef71b2140b38b409ae5f42257d0090ea20bef1"
    );
    let actual = descriptor
      .operations
      .into_iter()
      .flat_map(|operation| {
        [
          operation.input_schema.digest,
          operation.output_schema.digest,
        ]
      })
      .collect::<Vec<_>>();
    assert_eq!(actual.len(), 10, "{actual:?}");
    // Query output and snapshot output share muse.document/snapshot/v2.
    assert_eq!(actual[1], actual[3]);
  }

  #[test]
  fn applied_domain_change_satisfies_facet_contract() {
    let value = json!({
      "protocol": "muse.domain-change/v1",
      "pluginId": "muse.appflowy.markdown",
      "providerInstanceRef": "provider.appflowy.markdown.local.1",
      "scopeRef": "scope.123",
      "resourceRef": "9ffadd30-5a73-4a3c-9caf-7c8e191f8b65",
      "eventType": "markdown.document.changed",
      "eventSchemaDigest": MARKDOWN_DOMAIN_CHANGE_DIGEST,
      "domainRevision": "1",
      "epochRef": "epoch.1",
      "commandRef": "command.1",
      "origin": "external-command",
      "occurredAt": 1100,
      "payload": {
        "resourceRef": "9ffadd30-5a73-4a3c-9caf-7c8e191f8b65",
        "changeKind": "operations-applied",
        "changedBlockRefs": [],
        "structureChanged": true,
        "selectionRebaseHint": "sync-required"
      }
    });
    assert!(muse_plugin_facets::validate(
      muse_plugin_facets::FacetSchemaKind::DomainChange,
      &value
    )
    .is_ok());
  }

  #[test]
  fn word_layout_is_wrong_layout_for_markdown() {
    let mut evidence = BTreeMap::new();
    evidence.insert("appflowy.selection".into(), "current".into());
    evidence.insert("appflowy.view".into(), "view.word".into());
    evidence.insert("appflowy.layout".into(), "word".into());
    let context = ResolvedHostContext {
      actor_ref: "actor.1".into(),
      scope_ref: "scope.1".into(),
      authority_epoch: 1,
      evidence,
    };
    assert!(require_document_layout(&context).is_err());
  }
}
