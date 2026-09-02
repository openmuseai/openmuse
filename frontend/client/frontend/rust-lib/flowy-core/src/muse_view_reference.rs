//! AppFlowy-owned, read-only projection for the currently authorized View.

use std::{
  collections::BTreeMap,
  sync::{Arc, Weak},
  time::{SystemTime, UNIX_EPOCH},
};

use flowy_document::manager::DocumentManager;
use flowy_folder::{
  entities::view::{ViewLayoutPB, ViewPB},
  manager::FolderManager,
};
use lib_infra::async_trait::async_trait;
use muse_host_events::HostEventHub;
use muse_host_registry::{
  Cancellation, CapabilityProvider, Effect, HostCapabilityRegistry, Idempotency,
  OperationDescriptor, ProviderDescriptor, ProviderFailure, ProviderInvocation, RegistrationLease,
  ResolvedHostContext, SchemaDocument,
};
use serde_json::{json, Value};
use tokio::sync::Mutex;
use uuid::Uuid;

const MAX_TITLE_CHARS: usize = 256;
const MAX_CHILDREN: usize = 256;
const MAX_PAGE_SIZE: usize = 32;
const DEFAULT_PAGE_SIZE: usize = 20;
const MAX_SNAPSHOT_BYTES: usize = 64 * 1024;
const MAX_SNAPSHOTS: usize = 128;
const SNAPSHOT_TTL_MS: u64 = 60_000;

pub(crate) struct MuseHostProviders {
  registry: Arc<HostCapabilityRegistry>,
  leases: Mutex<Vec<RegistrationLease>>,
}

impl MuseHostProviders {
  pub(crate) async fn register(
    registry: Arc<HostCapabilityRegistry>,
    folder_manager: Weak<FolderManager>,
    document_manager: Weak<DocumentManager>,
    events: Arc<HostEventHub>,
  ) -> Arc<Self> {
    let owner = Arc::new(Self {
      registry: registry.clone(),
      leases: Mutex::new(Vec::new()),
    });
    let lease = registry
      .register(Arc::new(ViewReferenceProvider::new(folder_manager.clone())))
      .await
      .expect("static Muse AppFlowy View Provider registration must succeed");
    let rename_lease = registry
      .register(Arc::new(crate::muse_view_rename::ViewRenameProvider::new(
        folder_manager,
      )))
      .await
      .expect("static Muse AppFlowy View rename Provider registration must succeed");
    let markdown_lease = registry
      .register(Arc::new(crate::muse_markdown::MarkdownProvider::new(
        document_manager,
        events,
      )))
      .await
      .expect("static Muse AppFlowy Markdown Provider registration must succeed");
    owner
      .leases
      .lock()
      .await
      .extend([lease, rename_lease, markdown_lease]);
    owner
  }

  pub(crate) async fn shutdown(&self) {
    let leases = std::mem::take(&mut *self.leases.lock().await);
    for lease in leases {
      lease.dispose().await;
    }
    self.registry.shutdown().await;
  }
}

struct ViewReferenceProvider {
  folder_manager: Weak<FolderManager>,
  snapshots: Mutex<BTreeMap<String, Snapshot>>,
}

#[derive(Clone)]
struct Snapshot {
  actor_ref: String,
  scope_ref: String,
  authority_epoch: u64,
  expires_at_ms: u64,
  view: Value,
  children: Vec<Value>,
  next_offset: usize,
}

impl ViewReferenceProvider {
  fn new(folder_manager: Weak<FolderManager>) -> Self {
    Self {
      folder_manager,
      snapshots: Mutex::new(BTreeMap::new()),
    }
  }

  async fn first_page(
    &self,
    context: &ResolvedHostContext,
    limit: usize,
    cancellation: &Cancellation,
    deadline_at_ms: u64,
  ) -> Result<Value, ProviderFailure> {
    check_active(cancellation, deadline_at_ms)?;
    let view_id = context
      .evidence
      .get("appflowy.view")
      .ok_or(ProviderFailure)?;
    let folder = self.folder_manager.upgrade().ok_or(ProviderFailure)?;
    let view = folder
      .get_view_pb_with_child_limit(view_id, MAX_CHILDREN.saturating_add(1))
      .await
      .map_err(|_| ProviderFailure)?;
    check_active(cancellation, deadline_at_ms)?;
    if view.child_views.len() > MAX_CHILDREN {
      return Err(ProviderFailure);
    }
    let child_count = view.child_views.len();
    let root_view = safe_view(&view, Some(child_count))?;
    let children = view
      .child_views
      .iter()
      .map(|child| safe_view(child, None))
      .collect::<Result<Vec<_>, _>>()?;
    let snapshot_bytes = serde_json::to_vec(&json!({
      "view": &root_view,
      "children": &children,
    }))
    .map_err(|_| ProviderFailure)?
    .len();
    if snapshot_bytes > MAX_SNAPSHOT_BYTES {
      return Err(ProviderFailure);
    }
    self
      .page(
        context,
        Snapshot {
          actor_ref: context.actor_ref.clone(),
          scope_ref: context.scope_ref.clone(),
          authority_epoch: context.authority_epoch,
          expires_at_ms: unix_ms().saturating_add(SNAPSHOT_TTL_MS),
          view: root_view,
          children,
          next_offset: 0,
        },
        limit,
      )
      .await
  }

  async fn continuation(
    &self,
    context: &ResolvedHostContext,
    cursor: &str,
    limit: usize,
    cancellation: &Cancellation,
    deadline_at_ms: u64,
  ) -> Result<Value, ProviderFailure> {
    check_active(cancellation, deadline_at_ms)?;
    let now = unix_ms();
    let snapshot = {
      let mut snapshots = self.snapshots.lock().await;
      snapshots.retain(|_, snapshot| snapshot.expires_at_ms > now);
      snapshots.remove(cursor).ok_or(ProviderFailure)?
    };
    if snapshot.expires_at_ms <= now
      || snapshot.actor_ref != context.actor_ref
      || snapshot.scope_ref != context.scope_ref
      || snapshot.authority_epoch != context.authority_epoch
    {
      return Err(ProviderFailure);
    }
    self.page(context, snapshot, limit).await
  }

  async fn page(
    &self,
    context: &ResolvedHostContext,
    mut snapshot: Snapshot,
    limit: usize,
  ) -> Result<Value, ProviderFailure> {
    if context
      .evidence
      .get("appflowy.selection")
      .map(String::as_str)
      != Some("current")
    {
      return Err(ProviderFailure);
    }
    let start = snapshot.next_offset;
    let end = snapshot.children.len().min(start.saturating_add(limit));
    let children = snapshot.children[start..end].to_vec();
    snapshot.next_offset = end;
    let has_more = end < snapshot.children.len();
    let next_cursor = if has_more {
      Some(format!("view-page.{}", Uuid::new_v4()))
    } else {
      None
    };
    let mut page = json!({
      "returned": children.len(),
      "hasMore": has_more,
    });
    if let Some(cursor) = next_cursor {
      page["nextCursor"] = json!(cursor);
    }
    let output = json!({
      "view": snapshot.view,
      "children": children,
      "page": page,
    });
    if serde_json::to_vec(&output)
      .map_err(|_| ProviderFailure)?
      .len()
      > MAX_SNAPSHOT_BYTES
    {
      return Err(ProviderFailure);
    }
    if let Some(cursor) = output["page"]["nextCursor"].as_str() {
      let mut snapshots = self.snapshots.lock().await;
      let now = unix_ms();
      snapshots.retain(|_, value| value.expires_at_ms > now);
      if snapshots.len() >= MAX_SNAPSHOTS {
        return Err(ProviderFailure);
      }
      snapshot.expires_at_ms = now.saturating_add(SNAPSHOT_TTL_MS);
      snapshots.insert(cursor.to_string(), snapshot);
    }
    Ok(output)
  }
}

#[async_trait]
impl CapabilityProvider for ViewReferenceProvider {
  fn descriptor(&self) -> ProviderDescriptor {
    view_reference_descriptor()
  }

  async fn available(&self, context: &ResolvedHostContext) -> Result<bool, ProviderFailure> {
    Ok(
      self.folder_manager.upgrade().is_some()
        && context.evidence.contains_key("appflowy.view")
        && context
          .evidence
          .get("appflowy.selection")
          .is_some_and(|value| value == "current"),
    )
  }

  async fn invoke(
    &self,
    invocation: ProviderInvocation,
    context: ResolvedHostContext,
    cancellation: Cancellation,
  ) -> Result<Value, ProviderFailure> {
    if invocation.operation_id != "view.reference.read" {
      return Err(ProviderFailure);
    }
    let object = invocation.input.as_object().ok_or(ProviderFailure)?;
    let limit = object
      .get("limit")
      .and_then(Value::as_u64)
      .map(|value| value as usize)
      .unwrap_or(DEFAULT_PAGE_SIZE);
    if !(1..=MAX_PAGE_SIZE).contains(&limit) {
      return Err(ProviderFailure);
    }
    match object.get("cursor").and_then(Value::as_str) {
      Some(cursor) => {
        self
          .continuation(
            &context,
            cursor,
            limit,
            &cancellation,
            invocation.deadline_at_ms,
          )
          .await
      },
      None => {
        self
          .first_page(&context, limit, &cancellation, invocation.deadline_at_ms)
          .await
      },
    }
  }
}

fn safe_view(view: &ViewPB, child_count: Option<usize>) -> Result<Value, ProviderFailure> {
  let (title, title_truncated) = truncate_title(&view.name);
  let mut value = json!({
    "title": title,
    "titleTruncated": title_truncated,
    "layout": layout_name(&view.layout)?,
    "locked": view.is_locked,
  });
  if let Some(child_count) = child_count {
    value["childCount"] = json!(child_count);
  }
  Ok(value)
}

fn truncate_title(title: &str) -> (String, bool) {
  let mut chars = title.chars();
  let truncated: String = chars.by_ref().take(MAX_TITLE_CHARS).collect();
  let was_truncated = chars.next().is_some();
  (truncated, was_truncated)
}

fn layout_name(layout: &ViewLayoutPB) -> Result<&'static str, ProviderFailure> {
  match layout {
    ViewLayoutPB::Document => Ok("document"),
    ViewLayoutPB::Grid => Ok("grid"),
    ViewLayoutPB::Board => Ok("board"),
    ViewLayoutPB::Calendar => Ok("calendar"),
    ViewLayoutPB::Chat => Ok("chat"),
  }
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

fn view_reference_descriptor() -> ProviderDescriptor {
  ProviderDescriptor {
    descriptor_id: "appflowy.view-reference.local".into(),
    revision: "1".into(),
    family_id: "appflowy.view-reference".into(),
    contract_major: 1,
    contract_minor: 0,
    operations: vec![OperationDescriptor {
      operation_id: "view.reference.read".into(),
      effect: Effect::Read,
      input_schema: SchemaDocument::new(json!({
        "$schema": "https://json-schema.org/draft/2020-12/schema",
        "type": "object",
        "additionalProperties": false,
        "properties": {
          "limit": {"type": "integer", "minimum": 1, "maximum": 32},
          "cursor": {"type": "string", "minLength": 1, "maxLength": 256}
        }
      }))
      .expect("static View input schema must compile"),
      output_schema: SchemaDocument::new(json!({
        "$schema": "https://json-schema.org/draft/2020-12/schema",
        "type": "object",
        "additionalProperties": false,
        "required": ["view", "children", "page"],
        "properties": {
          "view": {
            "type": "object", "additionalProperties": false,
            "required": ["title", "titleTruncated", "layout", "locked", "childCount"],
            "properties": {
              "title": {"type": "string", "maxLength": 256},
              "titleTruncated": {"type": "boolean"},
              "layout": {"enum": ["document", "grid", "board", "calendar", "chat"]},
              "locked": {"type": ["boolean", "null"]},
              "childCount": {"type": "integer", "minimum": 0, "maximum": 256}
            }
          },
          "children": {
            "type": "array", "maxItems": 32,
            "items": {
              "type": "object", "additionalProperties": false,
              "required": ["title", "titleTruncated", "layout", "locked"],
              "properties": {
                "title": {"type": "string", "maxLength": 256},
                "titleTruncated": {"type": "boolean"},
                "layout": {"enum": ["document", "grid", "board", "calendar", "chat"]},
                "locked": {"type": ["boolean", "null"]}
              }
            }
          },
          "page": {
            "type": "object", "additionalProperties": false,
            "required": ["returned", "hasMore"],
            "properties": {
              "returned": {"type": "integer", "minimum": 0, "maximum": 32},
              "hasMore": {"type": "boolean"},
              "nextCursor": {"type": "string", "minLength": 1, "maxLength": 256}
            }
          }
        }
      }))
      .expect("static View output schema must compile"),
      cancellable: true,
      idempotency: Idempotency::None,
    }],
    title: Some("AppFlowy current View reference".into()),
    summary: Some(
      "Bounded metadata and immediate child summaries for the Host-selected View".into(),
    ),
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  fn context(actor_ref: &str) -> ResolvedHostContext {
    ResolvedHostContext {
      actor_ref: actor_ref.into(),
      scope_ref: "scope.current-view".into(),
      authority_epoch: 7,
      evidence: BTreeMap::from([
        ("appflowy.selection".into(), "current".into()),
        ("appflowy.view".into(), "private-view-id".into()),
      ]),
    }
  }

  fn snapshot(children: usize) -> Snapshot {
    Snapshot {
      actor_ref: "actor.1".into(),
      scope_ref: "scope.current-view".into(),
      authority_epoch: 7,
      expires_at_ms: unix_ms().saturating_add(SNAPSHOT_TTL_MS),
      view: json!({"title": "Root"}),
      children: (0..children)
        .map(|index| json!({"title": format!("Child {index}")}))
        .collect(),
      next_offset: 0,
    }
  }

  #[test]
  fn title_projection_is_unicode_safe_and_descriptor_is_valid() {
    let title = "界".repeat(MAX_TITLE_CHARS + 1);
    let (projected, truncated) = truncate_title(&title);
    assert_eq!(projected.chars().count(), MAX_TITLE_CHARS);
    assert!(truncated);
    assert_eq!(view_reference_descriptor().operations.len(), 1);
  }

  #[test]
  fn view_projection_uses_an_explicit_field_allowlist() {
    let view = ViewPB {
      id: "secret-view-id".into(),
      parent_view_id: "secret-parent-id".into(),
      name: "Visible title".into(),
      extra: Some("secret-extra".into()),
      created_by: Some(42),
      last_edited_by: Some(43),
      layout: ViewLayoutPB::Document,
      is_locked: None,
      ..Default::default()
    };
    let projected = safe_view(&view, Some(0)).unwrap();
    assert_eq!(
      projected,
      json!({
        "title": "Visible title",
        "titleTruncated": false,
        "layout": "document",
        "locked": null,
        "childCount": 0
      })
    );
    let serialized = projected.to_string();
    assert!(!serialized.contains("secret"));
    assert!(!serialized.contains("42"));
  }

  #[tokio::test]
  async fn pagination_cursor_is_opaque_single_use_and_context_bound() {
    let provider = ViewReferenceProvider::new(Weak::new());
    let first = provider
      .page(&context("actor.1"), snapshot(3), 2)
      .await
      .unwrap();
    let cursor = first["page"]["nextCursor"].as_str().unwrap().to_string();
    assert!(cursor.starts_with("view-page."));
    assert!(!cursor.contains("private-view-id"));

    let second = provider
      .continuation(
        &context("actor.1"),
        &cursor,
        2,
        &Cancellation::default(),
        unix_ms().saturating_add(1_000),
      )
      .await
      .unwrap();
    assert_eq!(second["page"]["returned"], 1);
    assert!(provider
      .continuation(
        &context("actor.1"),
        &cursor,
        2,
        &Cancellation::default(),
        unix_ms().saturating_add(1_000),
      )
      .await
      .is_err());

    let first = provider
      .page(&context("actor.1"), snapshot(3), 2)
      .await
      .unwrap();
    let cursor = first["page"]["nextCursor"].as_str().unwrap();
    assert!(provider
      .continuation(
        &context("actor.2"),
        cursor,
        2,
        &Cancellation::default(),
        unix_ms().saturating_add(1_000),
      )
      .await
      .is_err());
  }

  #[tokio::test]
  async fn expired_cursor_snapshot_quota_and_byte_limit_fail_closed() {
    let provider = ViewReferenceProvider::new(Weak::new());
    let mut expired = snapshot(2);
    expired.expires_at_ms = 0;
    provider
      .snapshots
      .lock()
      .await
      .insert("expired".into(), expired);
    assert!(provider
      .continuation(
        &context("actor.1"),
        "expired",
        1,
        &Cancellation::default(),
        unix_ms().saturating_add(1_000),
      )
      .await
      .is_err());

    {
      let mut snapshots = provider.snapshots.lock().await;
      for index in 0..MAX_SNAPSHOTS {
        snapshots.insert(format!("occupied-{index}"), snapshot(1));
      }
    }
    assert!(provider
      .page(&context("actor.1"), snapshot(2), 1)
      .await
      .is_err());

    let oversized = Snapshot {
      children: (0..MAX_CHILDREN)
        .map(|_| json!({"title": "界".repeat(MAX_TITLE_CHARS)}))
        .collect(),
      ..snapshot(0)
    };
    assert!(provider
      .page(&context("actor.1"), oversized, MAX_CHILDREN)
      .await
      .is_err());
  }

  #[test]
  fn cancellation_and_expired_deadlines_fail_closed() {
    let cancellation = Cancellation::default();
    cancellation.cancel();
    assert!(check_active(&cancellation, unix_ms().saturating_add(1_000)).is_err());
    assert!(check_active(&Cancellation::default(), unix_ms()).is_err());
  }
}
