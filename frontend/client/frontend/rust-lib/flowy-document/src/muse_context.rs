//! Narrow in-process seam from Flutter UI facets to the application Host.
//! The document crate transports opaque JSON and owns no Plugin schema.

use std::sync::{Arc, OnceLock, RwLock};

use serde_json::Value;

pub trait MuseUiContextPublisher: Send + Sync + 'static {
  fn publish(&self, value: Value) -> Result<(), ()>;
}

static PUBLISHER: OnceLock<RwLock<Option<Arc<dyn MuseUiContextPublisher>>>> = OnceLock::new();

pub fn install_muse_ui_context_publisher(publisher: Arc<dyn MuseUiContextPublisher>) {
  let slot = PUBLISHER.get_or_init(|| RwLock::new(None));
  *slot.write().expect("Muse UI publisher lock poisoned") = Some(publisher);
}

pub(crate) fn publish(value: Value) -> Result<(), ()> {
  let publisher = PUBLISHER
    .get()
    .and_then(|slot| slot.read().ok()?.clone())
    .ok_or(())?;
  publisher.publish(value)
}
