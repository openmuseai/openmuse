//! AppFlowy Host adapter for domain-neutral Flutter context envelopes.

use std::{
  collections::HashMap,
  sync::{Arc, Mutex},
  time::{SystemTime, UNIX_EPOCH},
};

use flowy_document::muse_context::{install_muse_ui_context_publisher, MuseUiContextPublisher};
use muse_host_events::HostEventHub;
use muse_plugin_facets::{schema_digest, validate, FacetSchemaKind};
use serde_json::{json, Value};

struct AppFlowyUiContextPublisher {
  events: Arc<HostEventHub>,
  envelope_digest: String,
  latest: Mutex<HashMap<(String, String), (String, u64)>>,
}

impl MuseUiContextPublisher for AppFlowyUiContextPublisher {
  fn publish(&self, value: Value) -> Result<(), ()> {
    validate(FacetSchemaKind::ContextContribution, &value).map_err(|_| ())?;
    let now = unix_ms();
    let occurred_at = value.get("capturedAt").and_then(Value::as_u64).ok_or(())?;
    let expires_at = value.get("expiresAt").and_then(Value::as_u64).ok_or(())?;
    let lane = value.get("lane").and_then(Value::as_str).ok_or(())?;
    let max_ttl_ms = if lane == "control" {
      5 * 60_000
    } else {
      30_000
    };
    if occurred_at > now.saturating_add(5_000)
      || expires_at <= now
      || expires_at.saturating_sub(occurred_at) > max_ttl_ms
    {
      return Err(());
    }
    let surface_ref = value
      .get("surfaceInstanceRef")
      .and_then(Value::as_str)
      .ok_or(())?
      .to_owned();
    let context_type = value
      .get("contextType")
      .and_then(Value::as_str)
      .ok_or(())?
      .to_owned();
    let epoch_ref = value
      .get("epochRef")
      .and_then(Value::as_str)
      .ok_or(())?
      .to_owned();
    let revision = value
      .get("contextRevision")
      .and_then(Value::as_str)
      .and_then(|revision| revision.parse::<u64>().ok())
      .ok_or(())?;
    let event_type =
      if value.get("contextType").and_then(Value::as_str) == Some("muse.surface.closed") {
        "surface.closed"
      } else {
        "context.updated"
      };
    let payload = if event_type == "surface.closed" {
      json!({
        "surfaceInstanceRef": value.get("surfaceInstanceRef").and_then(Value::as_str).ok_or(())?
      })
    } else {
      let mut latest = self.latest.lock().map_err(|_| ())?;
      let key = (surface_ref.clone(), context_type);
      if latest
        .get(&key)
        .is_some_and(|(previous_epoch, previous_revision)| {
          previous_epoch == &epoch_ref && revision <= *previous_revision
        })
      {
        return Err(());
      }
      latest.insert(key, (epoch_ref, revision));
      value
    };
    if event_type == "surface.closed" {
      self
        .latest
        .lock()
        .map_err(|_| ())?
        .retain(|(surface, _), _| surface != &surface_ref);
    }
    self
      .events
      .publish(
        occurred_at,
        json!({
          "eventKind": "provider.event",
          "descriptorId": "muse.ui-context.local",
          "descriptorRevision": "1",
          "eventType": event_type,
          "schemaDigest": self.envelope_digest,
          "payload": payload
        }),
      )
      .map_err(|_| ())?;
    Ok(())
  }
}

pub(crate) fn install(events: Arc<HostEventHub>) {
  let envelope_digest = schema_digest(FacetSchemaKind::ContextContribution)
    .expect("static Muse context schema digest must be valid");
  install_muse_ui_context_publisher(Arc::new(AppFlowyUiContextPublisher {
    events,
    envelope_digest,
    latest: Mutex::new(HashMap::new()),
  }));
}

fn unix_ms() -> u64 {
  SystemTime::now()
    .duration_since(UNIX_EPOCH)
    .unwrap_or_default()
    .as_millis()
    .min(u64::MAX as u128) as u64
}
