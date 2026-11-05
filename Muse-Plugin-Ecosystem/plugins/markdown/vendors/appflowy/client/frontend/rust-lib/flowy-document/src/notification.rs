use flowy_derive::ProtoBuf_Enum;
use flowy_notification::NotificationBuilder;

use crate::entities::DocumentTextPB;

const DOCUMENT_OBSERVABLE_SOURCE: &str = "Document";

/// Flutter Markdown UI Facet listens on this source. It is intentionally not
/// `Document` / `DidReceiveUpdate`: those notifications still mean Collab
/// local-vs-remote, which cannot express a Host `external-command`.
pub const MUSE_MARKDOWN_NOTIFICATION_SOURCE: &str = "MuseMarkdown";
pub const MUSE_MARKDOWN_DOMAIN_CHANGE_TY: i32 = 1;

#[derive(ProtoBuf_Enum, Debug, Default)]
pub enum DocumentNotification {
  #[default]
  Unknown = 0,

  DidReceiveUpdate = 1,
  DidUpdateDocumentSnapshotState = 2,
  DidUpdateDocumentSyncState = 3,
  DidUpdateDocumentAwarenessState = 4,
}

impl std::convert::From<DocumentNotification> for i32 {
  fn from(notification: DocumentNotification) -> Self {
    notification as i32
  }
}
impl std::convert::From<i32> for DocumentNotification {
  fn from(notification: i32) -> Self {
    match notification {
      1 => DocumentNotification::DidReceiveUpdate,
      2 => DocumentNotification::DidUpdateDocumentSnapshotState,
      3 => DocumentNotification::DidUpdateDocumentSyncState,
      4 => DocumentNotification::DidUpdateDocumentAwarenessState,
      _ => DocumentNotification::Unknown,
    }
  }
}

#[tracing::instrument(level = "trace")]
pub fn document_notification_builder(id: &str, ty: DocumentNotification) -> NotificationBuilder {
  NotificationBuilder::new(id, ty, DOCUMENT_OBSERVABLE_SOURCE)
}

pub fn publish_muse_markdown_domain_change(document_id: &str, json: &str) {
  NotificationBuilder::new(
    document_id,
    MUSE_MARKDOWN_DOMAIN_CHANGE_TY,
    MUSE_MARKDOWN_NOTIFICATION_SOURCE,
  )
  .payload(DocumentTextPB {
    text: json.to_owned(),
  })
  .send();
}
