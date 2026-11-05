//! AppFlowy folder projection for DSH `muse.workspace@1`.
//! Lists bounded view metadata. Document bodies stay on `muse.document`.

use std::sync::Weak;

use flowy_folder::{
  entities::view::{ViewLayoutPB, ViewPB},
  manager::FolderManager,
};
use lib_infra::async_trait::async_trait;
use muse_host_registry::{
  Cancellation, CapabilityProvider, Effect, Idempotency, OperationDescriptor, ProviderDescriptor,
  ProviderFailure, ProviderInvocation, ResolvedHostContext, SchemaDocument,
};
use serde_json::{json, Value};

const MAX_TREE_LIMIT: usize = 64;
const DEFAULT_TREE_LIMIT: usize = 32;
const MAX_TREE_DEPTH: u32 = 4;
const DEFAULT_TREE_DEPTH: u32 = 3;
const TREE_PROTOCOL: &str = "muse.workspace/tree/v1";

pub(crate) struct WorkspaceProvider {
  folder_manager: Weak<FolderManager>,
}

impl WorkspaceProvider {
  pub(crate) fn new(folder_manager: Weak<FolderManager>) -> Self {
    Self { folder_manager }
  }

  async fn current(
    &self,
    invocation: ProviderInvocation,
    context: ResolvedHostContext,
    cancellation: Cancellation,
  ) -> Result<Value, ProviderFailure> {
    check_active(&cancellation, invocation.deadline_at_ms)?;
    let folder = self.folder_manager.upgrade().ok_or(ProviderFailure)?;
    let workspace = folder
      .get_current_workspace()
      .await
      .map_err(|_| ProviderFailure)?;
    if let Some(requested) = invocation
      .input
      .as_object()
      .and_then(|input| input.get("workspaceId"))
      .and_then(Value::as_str)
      .map(str::trim)
      .filter(|value| !value.is_empty())
    {
      if requested != workspace.id {
        return Err(ProviderFailure);
      }
    }
    if let Some(bound) = context.evidence.get("appflowy.workspace") {
      if bound != &workspace.id {
        return Err(ProviderFailure);
      }
    }
    Ok(json!({
      "workspaceId": workspace.id,
      "title": workspace.name,
      "role": null
    }))
  }

  async fn tree(
    &self,
    invocation: ProviderInvocation,
    context: ResolvedHostContext,
    cancellation: Cancellation,
  ) -> Result<Value, ProviderFailure> {
    check_active(&cancellation, invocation.deadline_at_ms)?;
    let folder = self.folder_manager.upgrade().ok_or(ProviderFailure)?;
    let workspace = folder
      .get_current_workspace()
      .await
      .map_err(|_| ProviderFailure)?;
    let input = invocation.input.as_object().ok_or(ProviderFailure)?;
    if let Some(requested) = input
      .get("workspaceId")
      .and_then(Value::as_str)
      .map(str::trim)
      .filter(|value| !value.is_empty())
    {
      if requested != workspace.id {
        return Err(ProviderFailure);
      }
    }
    if let Some(bound) = context.evidence.get("appflowy.workspace") {
      if bound != &workspace.id {
        return Err(ProviderFailure);
      }
    }
    let views = folder
      .get_all_views_pb()
      .await
      .map_err(|_| ProviderFailure)?;
    check_active(&cancellation, invocation.deadline_at_ms)?;
    let parent_filter = input
      .get("parentViewId")
      .and_then(Value::as_str)
      .map(str::trim)
      .filter(|value| !value.is_empty());
    let limit = input
      .get("limit")
      .and_then(Value::as_u64)
      .map(|value| value as usize)
      .unwrap_or(DEFAULT_TREE_LIMIT)
      .clamp(1, MAX_TREE_LIMIT);
    let max_depth = input
      .get("depth")
      .and_then(Value::as_u64)
      .map(|value| value as u32)
      .unwrap_or(DEFAULT_TREE_DEPTH)
      .clamp(1, MAX_TREE_DEPTH);
    let cursor = input
      .get("cursor")
      .and_then(Value::as_str)
      .map(str::trim)
      .filter(|value| !value.is_empty());
    project_tree(
      &workspace.id,
      &views,
      parent_filter,
      cursor,
      limit,
      max_depth,
    )
  }
}

#[async_trait]
impl CapabilityProvider for WorkspaceProvider {
  fn descriptor(&self) -> ProviderDescriptor {
    descriptor()
  }

  async fn available(&self, _context: &ResolvedHostContext) -> Result<bool, ProviderFailure> {
    Ok(self.folder_manager.upgrade().is_some())
  }

  async fn invoke(
    &self,
    invocation: ProviderInvocation,
    context: ResolvedHostContext,
    cancellation: Cancellation,
  ) -> Result<Value, ProviderFailure> {
    match invocation.operation_id.as_str() {
      "workspace.current.query" => self.current(invocation, context, cancellation).await,
      "workspace.tree.query" => self.tree(invocation, context, cancellation).await,
      _ => Err(ProviderFailure),
    }
  }
}

fn project_tree(
  workspace_id: &str,
  views: &[ViewPB],
  parent_filter: Option<&str>,
  cursor: Option<&str>,
  limit: usize,
  max_depth: u32,
) -> Result<Value, ProviderFailure> {
  let parent_of: std::collections::HashMap<&str, &str> = views
    .iter()
    .map(|view| (view.id.as_str(), view.parent_view_id.as_str()))
    .collect();
  let mut items = Vec::new();
  for view in views {
    let Some(layout) = layout_name(&view.layout) else {
      continue;
    };
    let depth = depth_of(&view.id, &parent_of, workspace_id, max_depth);
    if depth > max_depth {
      continue;
    }
    if let Some(parent) = parent_filter {
      if view.parent_view_id != parent && view.id != parent {
        continue;
      }
    }
    items.push(json!({
      "viewId": view.id,
      "parentViewId": if view.parent_view_id.is_empty() {
        Value::Null
      } else {
        json!(view.parent_view_id)
      },
      "title": clip_title(&view.name),
      "layout": layout,
      "isSpace": is_space(view.extra.as_deref()),
      "depth": depth
    }));
  }
  let start = match cursor {
    Some(cursor) => {
      let at = items
        .iter()
        .position(|item| item.get("viewId").and_then(Value::as_str) == Some(cursor))
        .ok_or(ProviderFailure)?;
      at + 1
    },
    None => 0,
  };
  let end = (start + limit).min(items.len());
  let page = items[start..end].to_vec();
  let truncated = end < items.len();
  let mut value = json!({
    "protocol": TREE_PROTOCOL,
    "workspaceId": workspace_id,
    "rootViewId": parent_filter.unwrap_or(workspace_id),
    "truncated": truncated,
    "items": page
  });
  if truncated {
    if let Some(cursor) = page
      .last()
      .and_then(|item| item.get("viewId"))
      .and_then(Value::as_str)
    {
      value["nextCursor"] = json!(cursor);
    }
  }
  Ok(value)
}

fn depth_of(
  view_id: &str,
  parent_of: &std::collections::HashMap<&str, &str>,
  workspace_id: &str,
  max_depth: u32,
) -> u32 {
  let mut depth = 0;
  let mut current = view_id;
  let mut seen = std::collections::HashSet::new();
  while seen.insert(current) {
    let Some(parent) = parent_of.get(current).copied() else {
      break;
    };
    if parent.is_empty() || parent == current || parent == workspace_id {
      break;
    }
    depth += 1;
    if depth >= max_depth {
      return max_depth;
    }
    current = parent;
  }
  depth
}

fn layout_name(layout: &ViewLayoutPB) -> Option<&'static str> {
  match layout {
    ViewLayoutPB::Document => Some("document"),
    ViewLayoutPB::Grid => Some("grid"),
    ViewLayoutPB::Board => Some("board"),
    ViewLayoutPB::Calendar => Some("calendar"),
    ViewLayoutPB::Chat => Some("chat"),
    ViewLayoutPB::Word | ViewLayoutPB::Excel | ViewLayoutPB::Slides | ViewLayoutPB::Pdf => None,
  }
}

fn is_space(extra: Option<&str>) -> bool {
  extra
    .and_then(|raw| serde_json::from_str::<Value>(raw).ok())
    .and_then(|value| value.get("is_space").and_then(Value::as_bool))
    .unwrap_or(false)
}

fn clip_title(title: &str) -> String {
  title.chars().take(256).collect()
}

fn check_active(cancellation: &Cancellation, deadline_at_ms: u64) -> Result<(), ProviderFailure> {
  if cancellation.is_cancelled() || deadline_at_ms <= unix_ms() {
    Err(ProviderFailure)
  } else {
    Ok(())
  }
}

fn unix_ms() -> u64 {
  std::time::SystemTime::now()
    .duration_since(std::time::UNIX_EPOCH)
    .unwrap_or_default()
    .as_millis()
    .min(u64::MAX as u128) as u64
}

fn descriptor() -> ProviderDescriptor {
  ProviderDescriptor {
    descriptor_id: "appflowy.workspace.local".into(),
    revision: "1".into(),
    family_id: "muse.workspace".into(),
    contract_major: 1,
    contract_minor: 0,
    operations: vec![
      OperationDescriptor {
        operation_id: "workspace.current.query".into(),
        effect: Effect::Read,
        input_schema: schema(json!({
          "type": "object",
          "additionalProperties": false,
          "properties": {
            "workspaceId": { "type": "string", "minLength": 1, "maxLength": 128 }
          }
        })),
        output_schema: schema(json!({
          "type": "object",
          "additionalProperties": false,
          "required": ["workspaceId", "title"],
          "properties": {
            "workspaceId": { "type": "string" },
            "title": { "type": "string" },
            "role": { "type": ["string", "null"] }
          }
        })),
        cancellable: true,
        idempotency: Idempotency::None,
      },
      OperationDescriptor {
        operation_id: "workspace.tree.query".into(),
        effect: Effect::Read,
        input_schema: schema(json!({
          "type": "object",
          "additionalProperties": false,
          "properties": {
            "workspaceId": { "type": "string", "minLength": 1, "maxLength": 128 },
            "parentViewId": { "type": "string", "minLength": 1, "maxLength": 128 },
            "cursor": { "type": "string", "minLength": 1, "maxLength": 128 },
            "limit": { "type": "integer", "minimum": 1, "maximum": 64 },
            "depth": { "type": "integer", "minimum": 1, "maximum": 4 }
          }
        })),
        output_schema: schema(json!({
          "type": "object",
          "additionalProperties": false,
          "required": ["protocol", "workspaceId", "rootViewId", "truncated", "items"],
          "properties": {
            "protocol": { "type": "string", "const": "muse.workspace/tree/v1" },
            "workspaceId": { "type": "string" },
            "rootViewId": { "type": "string" },
            "truncated": { "type": "boolean" },
            "nextCursor": { "type": "string" },
            "items": {
              "type": "array",
              "maxItems": 64,
              "items": {
                "type": "object",
                "additionalProperties": false,
                "required": ["viewId", "title", "layout", "isSpace", "depth"],
                "properties": {
                  "viewId": { "type": "string" },
                  "parentViewId": { "type": ["string", "null"] },
                  "title": { "type": "string", "maxLength": 256 },
                  "layout": { "type": "string", "enum": ["document", "grid", "board", "calendar", "chat"] },
                  "isSpace": { "type": "boolean" },
                  "depth": { "type": "integer", "minimum": 0, "maximum": 4 }
                }
              }
            }
          }
        })),
        cancellable: true,
        idempotency: Idempotency::None,
      },
    ],
    title: Some("AppFlowy local muse.workspace@1 provider".into()),
    summary: Some("Bounded Host folder catalog. Document bodies are muse.document.".into()),
  }
}

fn schema(mut value: Value) -> SchemaDocument {
  value["$schema"] = json!("https://json-schema.org/draft/2020-12/schema");
  SchemaDocument::new(value).expect("static Workspace schema must compile")
}

#[cfg(test)]
mod tests {
  use super::*;

  fn view(id: &str, parent: &str, name: &str, layout: ViewLayoutPB) -> ViewPB {
    ViewPB {
      id: id.into(),
      parent_view_id: parent.into(),
      name: name.into(),
      layout,
      extra: Some(r#"{"is_space":false}"#.into()),
      ..Default::default()
    }
  }

  #[test]
  fn tree_projects_titles_without_bodies() {
    let workspace = "ws-1";
    let views = vec![
      view("v1", workspace, "Getting started", ViewLayoutPB::Document),
      view("v2", "v1", "Notes", ViewLayoutPB::Document),
    ];
    let projected = project_tree(workspace, &views, None, None, 32, 3).unwrap();
    let encoded = projected.to_string();
    assert!(encoded.contains("Getting started"));
    assert!(encoded.contains("Notes"));
    assert!(!encoded.contains("text/markdown"));
    assert_eq!(projected["items"][0]["depth"], 0);
    assert_eq!(projected["items"][1]["depth"], 1);
  }

  #[test]
  fn office_layouts_are_omitted_from_the_document_tree() {
    let views = vec![view("word-1", "ws-1", "Spec", ViewLayoutPB::Word)];
    let projected = project_tree("ws-1", &views, None, None, 32, 3).unwrap();
    assert_eq!(projected["items"].as_array().unwrap().len(), 0);
  }

  #[test]
  fn provider_schema_digests_match_the_typescript_workspace_plugin() {
    let descriptor = descriptor();
    let expected = [
      (
        "workspace.current.query",
        "sha256:5de689ea35cfb716063d6743f42bcc006d60e11a19f32019ec38481920c55eb4",
        "sha256:5f5402af8e1b22f490827d3730e100c92385715f20fab5742ce2773c5352ad92",
      ),
      (
        "workspace.tree.query",
        "sha256:75396529bf577ebb0162bce4f8aa98eb5cf080eac2250430e365cc203326e3b8",
        "sha256:0f96e8c22c1974261559057acc5b85201079aa1e6d3131c20c7720f60b0228eb",
      ),
    ];
    for (operation_id, input_digest, output_digest) in expected {
      let operation = descriptor.operation(operation_id).unwrap();
      assert_eq!(operation.input_schema.digest, input_digest);
      assert_eq!(operation.output_schema.digest, output_digest);
    }
  }
}
