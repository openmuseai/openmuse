//! Dart surface seam for the `muse.resource-presentation` family.
//!
//! `flowy_core::muse_presentation` hands every presentation request only the
//! Flutter UI can satisfy to an installed
//! [`MuseResourceSurfaceDispatcher`]; until one is installed the provider fails
//! closed with `SURFACE_UNAVAILABLE`. This module is that dispatcher for the
//! Dart host.
//!
//! Two hand-written `extern "C"` exports are the whole transport, the same
//! shape as `muse_diff_text_*` in this crate — Dart binds them with `dart:ffi`
//! and no code generation is involved:
//!
//! - [`muse_presentation_install_dart_surface`] is called once per app start
//!   with the Dart native port the surface listens on;
//! - `dispatch` projects [`MusePresentationDispatch::to_wire`] onto that port
//!   and waits for the answer, so the Flutter tab is opened while the DSH
//!   request is still in flight;
//! - [`muse_presentation_complete_dispatch`] delivers that answer, parsed with
//!   [`MusePresentationOutcome::from_wire`]. A null or inadmissible answer
//!   means "this build cannot present that resource", never "assume it worked".
//!
//! A third export, [`muse_presentation_publish_mount_roots`], pushes the Host's
//! granted Mount directories into the Rust core. A `mountRef` is opaque to the
//! core, so without those directories the Host cannot search a Mount for a
//! partial path; publishing them is what turns fuzzy resolution on. The
//! directories stay inside the Host process: nothing on this seam is a device
//! path travelling to DSH.
//!
//! The dispatch payload carries only opaque refs (a `mountRef` and a
//! Mount-relative path), and the answer is refused when it embeds a device path
//! shape, so no path travels back across the bridge.

use std::{
  collections::HashMap,
  ffi::{c_char, CStr},
  path::PathBuf,
  sync::{mpsc, Arc, Mutex, OnceLock},
  time::Duration,
};

use allo_isolate::Isolate;
use flowy_core::muse_presentation::{
  install_muse_mount_root_catalog, install_muse_resource_surface_dispatcher,
  MuseMountRootCatalog, MusePresentationDispatch, MusePresentationOutcome,
  MuseResourceSurfaceDispatcher,
};
use serde_json::Value;
use tracing::{debug, warn};

/// How long one presentation waits for the Flutter surface before failing
/// closed. The DSH caller keeps its own, longer deadline.
const SURFACE_TIMEOUT: Duration = Duration::from_secs(20);

/// Concurrent presentations one Dart surface may hold open.
const MAX_PENDING: usize = 64;

/// Largest answer the bridge accepts, in bytes.
const MAX_ANSWER_BYTES: usize = 8 * 1024;

/// Mounts one published catalog may hold.
const MAX_MOUNT_ROOTS: usize = 64;

/// Longest accepted Mount directory, in bytes; mirrors the DSH locator bound.
const MAX_MOUNT_ROOT_BYTES: usize = 4096;

/// Longest accepted Mount scope reference.
const MAX_MOUNT_REF_BYTES: usize = 256;

/// Granted Mount directories, as the Flutter layer publishes them.
struct DartMountRootCatalog {
  roots: HashMap<String, PathBuf>,
}

impl MuseMountRootCatalog for DartMountRootCatalog {
  fn mount_root(&self, mount_ref: &str) -> Option<PathBuf> {
    self.roots.get(mount_ref).cloned()
  }
}

/// Dispatches waiting for the Flutter surface, keyed by `requestRef`.
fn pending() -> &'static Mutex<HashMap<String, mpsc::Sender<Option<Value>>>> {
  static PENDING: OnceLock<Mutex<HashMap<String, mpsc::Sender<Option<Value>>>>> = OnceLock::new();
  PENDING.get_or_init(|| Mutex::new(HashMap::new()))
}

struct DartSurfaceDispatcher {
  isolate: Isolate,
}

impl MuseResourceSurfaceDispatcher for DartSurfaceDispatcher {
  fn dispatch(&self, request: MusePresentationDispatch) -> Option<MusePresentationOutcome> {
    let request_ref = request.request_ref.clone();
    let wire = serde_json::to_vec(&request.to_wire()).ok()?;
    let (sender, receiver) = mpsc::channel();
    {
      let mut waiting = pending().lock().ok()?;
      if waiting.len() >= MAX_PENDING {
        warn!("Muse presentation surface is saturated; refusing a dispatch");
        return None;
      }
      waiting.insert(request_ref.clone(), sender);
    }
    if !self.isolate.post(wire) {
      forget(&request_ref);
      warn!("Muse presentation surface port is closed; refusing a dispatch");
      return None;
    }
    debug!(request_ref = %request_ref, "Muse presentation dispatched to Flutter");
    let answer = match receiver.recv_timeout(SURFACE_TIMEOUT) {
      Ok(answer) => answer,
      Err(_) => {
        forget(&request_ref);
        warn!(request_ref = %request_ref, "Flutter surface did not answer in time");
        return None;
      },
    };
    forget(&request_ref);
    answer.and_then(|value| MusePresentationOutcome::from_wire(&value))
  }
}

fn forget(request_ref: &str) {
  if let Ok(mut waiting) = pending().lock() {
    waiting.remove(request_ref);
  }
}

/// Install the Flutter surface dispatcher that answers dispatches posted to
/// `port`.
///
/// Returns `1` when the port was accepted, `0` when it cannot be used. A
/// previous installation (and everything still waiting on it) is retired, so
/// this is safe to call once per app start.
#[no_mangle]
pub extern "C" fn muse_presentation_install_dart_surface(port: i64) -> i32 {
  if port <= 0 {
    warn!("Muse presentation surface install refused an unusable port");
    return 0;
  }
  match pending().lock() {
    Ok(mut waiting) => waiting.clear(),
    Err(_) => return 0,
  }
  let dispatcher = Arc::new(DartSurfaceDispatcher {
    isolate: Isolate::new(port),
  });
  install_muse_resource_surface_dispatcher(dispatcher);
  1
}

/// Answer the presentation whose `requestRef` is waiting on the Flutter
/// surface.
///
/// `outcome_json` is the UTF-8 JSON of a `muse.presentation/outcome/v1` answer,
/// or null for "the surface refused" — the provider then fails closed.
/// Returns `1` when the answer reached a waiting dispatch, `0` otherwise.
///
/// # Safety
///
/// Both pointers must be null or valid NUL-terminated UTF-8 strings that stay
/// alive for the duration of the call. Dart allocates them per call.
#[no_mangle]
pub unsafe extern "C" fn muse_presentation_complete_dispatch(
  request_ref: *const c_char,
  outcome_json: *const c_char,
) -> i32 {
  if request_ref.is_null() {
    return 0;
  }
  let Ok(request_ref) = CStr::from_ptr(request_ref).to_str() else {
    return 0;
  };
  let answer = if outcome_json.is_null() {
    None
  } else {
    match CStr::from_ptr(outcome_json).to_str() {
      Ok(text) if !text.is_empty() => match serde_json::from_str::<Value>(text) {
        Ok(value) if answer_is_admissible(&value) => Some(value),
        Ok(_) => {
          warn!(request_ref = %request_ref, "Muse presentation answer embedded a device path");
          None
        },
        Err(_) => {
          warn!(request_ref = %request_ref, "Muse presentation answer was not JSON");
          None
        },
      },
      _ => None,
    }
  };
  let sender = pending()
    .lock()
    .ok()
    .and_then(|mut waiting| waiting.remove(request_ref));
  let Some(sender) = sender else {
    return 0;
  };
  if sender.send(answer).is_ok() {
    1
  } else {
    0
  }
}

/// Publish the Host's granted Mount directories to the Rust core.
///
/// `roots_json` is a UTF-8 JSON object of `{"<mountRef>": "<directory>"}` — the
/// same pair the Flutter workspace controller already publishes inside
/// `DSH_HOME` for the DSH side. An empty object publishes an empty catalog,
/// which is the correct answer when the Host has no bound Mount. Returns `1`
/// when the catalog was installed, `0` when the document was unusable; an
/// unusable document never half-installs.
///
/// # Safety
///
/// `roots_json` must be null or a valid NUL-terminated UTF-8 string that stays
/// alive for the duration of the call. Dart allocates it per call.
#[no_mangle]
pub unsafe extern "C" fn muse_presentation_publish_mount_roots(roots_json: *const c_char) -> i32 {
  if roots_json.is_null() {
    return 0;
  }
  let Ok(text) = CStr::from_ptr(roots_json).to_str() else {
    warn!("Muse Mount catalog was not UTF-8");
    return 0;
  };
  let Some(catalog) = parse_mount_roots(text) else {
    warn!("Muse Mount catalog was refused");
    return 0;
  };
  install_muse_mount_root_catalog(Arc::new(catalog));
  1
}

/// Parse one published Mount catalog, or `None` when it may not be installed.
fn parse_mount_roots(text: &str) -> Option<DartMountRootCatalog> {
  let value: Value = serde_json::from_str(text).ok()?;
  let Value::Object(fields) = value else {
    return None;
  };
  if fields.len() > MAX_MOUNT_ROOTS {
    return None;
  }
  let mut roots = HashMap::with_capacity(fields.len());
  for (mount_ref, directory) in fields {
    let directory = directory.as_str()?;
    if mount_ref.is_empty()
      || mount_ref.len() > MAX_MOUNT_REF_BYTES
      || directory.is_empty()
      || directory.len() > MAX_MOUNT_ROOT_BYTES
      || directory.contains('\u{0}')
    {
      return None;
    }
    roots.insert(mount_ref, PathBuf::from(directory));
  }
  Some(DartMountRootCatalog { roots })
}

/// Whether an answer may cross the bridge: bounded, and free of path shapes.
///
/// The provider re-validates the opaque refs it projects into a receipt; this
/// guard is the one that makes "no device path is sent back across the bridge"
/// a property of the seam itself.
fn answer_is_admissible(value: &Value) -> bool {
  serde_json::to_vec(value)
    .map(|bytes| bytes.len() <= MAX_ANSWER_BYTES)
    .unwrap_or(false)
    && carries_no_device_path(value)
}

fn carries_no_device_path(value: &Value) -> bool {
  match value {
    Value::String(text) => !text.contains('\\') && !text.contains(":/"),
    Value::Array(items) => items.iter().all(carries_no_device_path),
    Value::Object(fields) => fields.values().all(carries_no_device_path),
    _ => true,
  }
}

#[cfg(test)]
mod tests {
  use super::*;
  use flowy_core::muse_presentation::{
    clear_muse_mount_root_catalog, muse_mount_root_catalog_available,
  };
  use serde_json::json;

  /// The granted Mount catalog is process-global, so the tests that publish one
  /// must not run concurrently with each other.
  static CATALOG_TESTS: std::sync::Mutex<()> = std::sync::Mutex::new(());

  fn catalog_guard() -> std::sync::MutexGuard<'static, ()> {
    CATALOG_TESTS
      .lock()
      .unwrap_or_else(|poisoned| poisoned.into_inner())
  }

  /// The dispatch JSON the Flutter seam parses. The same literal is asserted
  /// from Dart in
  /// `appflowy_flutter/test/plugins/resource_surface/muse_presentation_dispatch_test.dart`,
  /// so a key rename on either side fails a test instead of silently degrading
  /// every presentation to `SURFACE_UNAVAILABLE`.
  const DISPATCH_WIRE_FIXTURE: &str = r#"{
    "protocol": "muse.presentation/dispatch/v1",
    "requestRef": "request.9f",
    "resourceRef": "resource.7a3",
    "workspaceId": "workspace.appflowy.1",
    "disposition": "open",
    "requestedMode": "view",
    "placementHint": "current-window",
    "adapterRef": "muse.adapter.host-resolved",
    "effectiveMode": "view",
    "sessionRef": "session.4c",
    "revision": "revision.12",
    "causeKind": "deliverable",
    "resolved": true,
    "mountRef": "mount:test",
    "relativePath": "notes/spec.md"
  }"#;

  /// The reply JSON `MusePresentationOutcome.toWireJson()` sends back, pinned
  /// byte for byte (Dart's `jsonEncode` preserves this key order).
  const DART_OPENED_REPLY_FIXTURE: &str = concat!(
    r#"{"result":"opened","#,
    r#""surfaceInstanceRef":"surface.muse.resource.0f1e2d3c4b5a69788796a5b4c3d2e1f0","#,
    r#""selectedAdapterRef":"muse.adapter.host-resolved","#,
    r#""effectiveMode":"view","revision":"revision.12","#,
    r#""warnings":[],"layout":"resource","title":"spec.md"}"#,
  );

  fn host_dispatch() -> MusePresentationDispatch {
    MusePresentationDispatch {
      request_ref: "request.9f".into(),
      resource_ref: "resource.7a3".into(),
      workspace_id: "workspace.appflowy.1".into(),
      view_id: None,
      layout: None,
      title: None,
      disposition: "open".into(),
      requested_mode: "view".into(),
      placement_hint: "current-window".into(),
      adapter_ref: "muse.adapter.host-resolved".into(),
      effective_mode: "view".into(),
      session_ref: "session.4c".into(),
      revision: "revision.12".into(),
      cause_kind: "deliverable".into(),
      anchor_hint: None,
      mount_ref: Some("mount:test".into()),
      relative_path: Some("notes/spec.md".into()),
      resolved: true,
    }
  }

  #[test]
  fn the_dispatch_wire_is_the_fixture_the_dart_seam_parses() {
    let expected: Value = serde_json::from_str(DISPATCH_WIRE_FIXTURE).unwrap();
    assert_eq!(host_dispatch().to_wire(), expected);
  }

  #[test]
  fn the_dart_reply_fixture_is_an_admissible_answer() {
    let value: Value = serde_json::from_str(DART_OPENED_REPLY_FIXTURE).unwrap();
    assert!(answer_is_admissible(&value));
    let outcome = MusePresentationOutcome::from_wire(&value).expect("admissible reply");
    assert_eq!(outcome.result, "opened");
    assert_eq!(outcome.effective_mode, "view");
    assert_eq!(outcome.revision, "revision.12");
    assert_eq!(
      outcome.surface_instance_ref,
      "surface.muse.resource.0f1e2d3c4b5a69788796a5b4c3d2e1f0"
    );
    assert_eq!(outcome.selected_adapter_ref, "muse.adapter.host-resolved");
  }

  #[test]
  fn answers_with_path_shapes_are_refused() {
    assert!(answer_is_admissible(&json!({
      "result": "opened",
      "surfaceInstanceRef": "surface.muse.resource.0f0f",
      "selectedAdapterRef": "muse.adapter.host-resolved",
      "effectiveMode": "view",
      "revision": "revision.17",
      "warnings": []
    })));
    assert!(!answer_is_admissible(&json!({
      "result": "opened",
      "surfaceInstanceRef": "C:/Users/me/secret.md",
    })));
    assert!(!answer_is_admissible(&json!({
      "result": "opened",
      "surfaceInstanceRef": "surface.muse.resource.0f0f",
      "warnings": ["opened \\\\server\\share\\secret.md"],
    })));
    assert!(!answer_is_admissible(&json!({
      "result": "opened",
      "surfaceInstanceRef": "surface.muse.resource.0f0f",
      "warnings": ["opened file:/tmp/secret.md"],
    })));
  }

  #[test]
  fn oversized_answers_are_refused() {
    let oversized = json!({
      "result": "opened",
      "surfaceInstanceRef": "surface.muse.resource.0f0f",
      "warnings": ["x".repeat(MAX_ANSWER_BYTES)],
    });
    assert!(!answer_is_admissible(&oversized));
  }

  /// The export the Flutter layer calls: a well-formed catalog installs, an
  /// unusable one installs nothing.
  #[test]
  fn publishing_a_mount_catalog_accepts_only_a_usable_document() {
    let _guard = catalog_guard();
    let catalog = parse_mount_roots(r#"{"mount:0d71":"D:\\agentic\\src\\openmuse-io\\vendors\\helix"}"#)
      .expect("a published catalog must parse");
    assert_eq!(
      catalog.mount_root("mount:0d71"),
      Some(PathBuf::from(r"D:\agentic\src\openmuse-io\vendors\helix"))
    );
    assert_eq!(catalog.mount_root("mount:other"), None);

    assert!(parse_mount_roots("{}").is_some(), "an empty catalog is valid");
    for refused in [
      "[]",
      "not json",
      r#"{"mount:0d71":42}"#,
      r#"{"mount:0d71":""}"#,
      r#"{"":"D:\\dir"}"#,
      &format!(r#"{{"mount:0d71":"{}"}}"#, "x".repeat(MAX_MOUNT_ROOT_BYTES + 1)),
    ] {
      assert!(
        parse_mount_roots(refused).is_none(),
        "an unusable catalog must be refused: {}",
        refused
      );
    }

    // Through the real export, with a real C string.
    clear_muse_mount_root_catalog();
    assert!(!muse_mount_root_catalog_available());
    let body = std::ffi::CString::new(r#"{"mount:0d71":"D:\\helix"}"#).unwrap();
    let published = unsafe { muse_presentation_publish_mount_roots(body.as_ptr()) };
    assert_eq!(published, 1);
    assert!(muse_mount_root_catalog_available());
    let refused = std::ffi::CString::new("[]").unwrap();
    assert_eq!(
      unsafe { muse_presentation_publish_mount_roots(refused.as_ptr()) },
      0
    );
    assert_eq!(
      unsafe { muse_presentation_publish_mount_roots(std::ptr::null()) },
      0
    );
    assert!(
      muse_mount_root_catalog_available(),
      "a refused document must not uninstall the catalog the Host is using"
    );
    clear_muse_mount_root_catalog();
  }

  #[test]
  fn completing_an_unknown_request_reports_no_waiter() {
    let request_ref = std::ffi::CString::new("request.unknown").unwrap();
    let outcome = std::ffi::CString::new("{}").unwrap();
    let delivered = unsafe {
      muse_presentation_complete_dispatch(request_ref.as_ptr(), outcome.as_ptr())
    };
    assert_eq!(delivered, 0);
    assert_eq!(
      unsafe { muse_presentation_complete_dispatch(std::ptr::null(), std::ptr::null()) },
      0
    );
  }
}
