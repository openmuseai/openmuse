//! Bounded Word projection for the Host-selected Word view.
//! Apply stays blocked until kernel `toDocx` exists.

use std::sync::Weak;

use flowy_folder::manager::FolderManager;
use flowy_user::user_manager::UserManager;
use lib_infra::async_trait::async_trait;
use muse_host_registry::{
  Cancellation, CapabilityProvider, Effect, Idempotency, OperationDescriptor, ProviderDescriptor,
  ProviderFailure, ProviderInvocation, ResolvedHostContext, SchemaDocument,
};
use serde_json::{json, Value};
use uuid::Uuid;

use crate::deps_resolve::folder_deps::word_blob::{plain_text_from_docx, WordBlobStore};

pub(crate) struct WordProvider {
  folder_manager: Weak<FolderManager>,
  user_manager: Weak<UserManager>,
}

impl WordProvider {
  pub(crate) fn new(
    folder_manager: Weak<FolderManager>,
    user_manager: Weak<UserManager>,
  ) -> Self {
    Self {
      folder_manager,
      user_manager,
    }
  }

  async fn query(
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
    if cancellation.is_cancelled() {
      return Err(ProviderFailure);
    }
    require_current_selection(&context)?;
    require_word_layout(&context)?;
    let view_id = current_view_id(&context)?;
    if let Some(folder) = self.folder_manager.upgrade() {
      let view = folder
        .get_view_pb(&view_id.to_string())
        .await
        .map_err(|_| ProviderFailure)?;
      if view.layout != flowy_folder::entities::view::ViewLayoutPB::Word {
        return Err(ProviderFailure);
      }
    }
    let user = self.user_manager.upgrade().ok_or(ProviderFailure)?;
    let store = WordBlobStore::for_user(
      &user.user_data_dir().map_err(|_| ProviderFailure)?,
      &user.workspace_id().map_err(|_| ProviderFailure)?,
    );
    let (bytes, rev) = store.get(&view_id).await.map_err(|_| ProviderFailure)?;
    let text = plain_text_from_docx(&bytes);
    Ok(json!({
      "protocol": "muse.word/snapshot/v1",
      "resourceRef": view_id.to_string(),
      "revision": format!("sha256:{rev}"),
      "content": {
        "mediaType": "text/plain",
        "text": text,
        "truncated": false,
        "byteLength": text.len()
      }
    }))
  }
}

#[async_trait]
impl CapabilityProvider for WordProvider {
  fn descriptor(&self) -> ProviderDescriptor {
    descriptor()
  }

  async fn available(&self, context: &ResolvedHostContext) -> Result<bool, ProviderFailure> {
    Ok(
      self.user_manager.upgrade().is_some()
        && require_current_selection(context).is_ok()
        && require_word_layout(context).is_ok(),
    )
  }

  async fn invoke(
    &self,
    invocation: ProviderInvocation,
    context: ResolvedHostContext,
    cancellation: Cancellation,
  ) -> Result<Value, ProviderFailure> {
    match invocation.operation_id.as_str() {
      "word.current.query" => self.query(invocation, context, cancellation).await,
      "word.current.propose" | "word.current.apply" => Err(ProviderFailure),
      _ => Err(ProviderFailure),
    }
  }
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

fn require_word_layout(context: &ResolvedHostContext) -> Result<(), ProviderFailure> {
  match context.evidence.get("appflowy.layout").map(String::as_str) {
    Some("word") => Ok(()),
    _ => Err(ProviderFailure),
  }
}

fn current_view_id(context: &ResolvedHostContext) -> Result<Uuid, ProviderFailure> {
  context
    .evidence
    .get("appflowy.view")
    .and_then(|view| Uuid::parse_str(view).ok())
    .ok_or(ProviderFailure)
}

fn descriptor() -> ProviderDescriptor {
  let opaque_ref = json!({"type": "string", "minLength": 1, "maxLength": 128});
  let revision = json!({"type": "string", "pattern": "^sha256:[0-9a-f]{64}$"});
  ProviderDescriptor {
    descriptor_id: "appflowy.word.local".into(),
    revision: "1".into(),
    family_id: "muse.word".into(),
    contract_major: 1,
    contract_minor: 0,
    operations: vec![OperationDescriptor {
      operation_id: "word.current.query".into(),
      effect: Effect::Read,
      input_schema: schema(json!({"type": "object", "additionalProperties": false})),
      output_schema: schema(json!({
        "type": "object", "additionalProperties": false,
        "required": ["protocol", "resourceRef", "revision", "content"],
        "properties": {
          "protocol": {"const": "muse.word/snapshot/v1"},
          "resourceRef": opaque_ref,
          "revision": revision,
          "content": {"type": "object", "additionalProperties": false,
            "required": ["mediaType", "text", "truncated", "byteLength"],
            "properties": {
              "mediaType": {"const": "text/plain"},
              "text": {"type": "string", "maxLength": 32768},
              "truncated": {"type": "boolean"},
              "byteLength": {"type": "integer", "minimum": 0, "maximum": 32768}
            }
          }
        }
      })),
      cancellable: true,
      idempotency: Idempotency::None,
    }],
    title: Some("AppFlowy local muse.word@1 provider".into()),
    summary: Some("Host-selected Word blob projection. Apply is blocked until toDocx.".into()),
  }
}

fn schema(mut value: Value) -> SchemaDocument {
  value["$schema"] = json!("https://json-schema.org/draft/2020-12/schema");
  SchemaDocument::new(value).expect("static Word schema must compile")
}

#[cfg(test)]
mod tests {
  use super::*;
  use std::collections::BTreeMap;

  #[test]
  fn markdown_layout_is_wrong_for_word_tools() {
    let context = ResolvedHostContext {
      actor_ref: "actor.1".into(),
      scope_ref: "scope.1".into(),
      authority_epoch: 1,
      evidence: BTreeMap::from([
        ("appflowy.selection".into(), "current".into()),
        ("appflowy.view".into(), "view.doc".into()),
        ("appflowy.layout".into(), "document".into()),
      ]),
    };
    assert!(require_word_layout(&context).is_err());
  }

  #[test]
  fn word_layout_is_accepted() {
    let context = ResolvedHostContext {
      actor_ref: "actor.1".into(),
      scope_ref: "scope.1".into(),
      authority_epoch: 1,
      evidence: BTreeMap::from([
        ("appflowy.selection".into(), "current".into()),
        ("appflowy.view".into(), "view.word".into()),
        ("appflowy.layout".into(), "word".into()),
      ]),
    };
    assert!(require_word_layout(&context).is_ok());
  }
}
