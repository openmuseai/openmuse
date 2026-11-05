//! AppFlowy Host capability for the DSH `muse.resource-presentation` family
//! (contract major 2) and the `muse.resource-locator` family (contract major 1).
//!
//! The presentation family has three operations, all of them cancellable:
//!
//! | operation | effect | idempotency | declared input/output |
//! |---|---|---|---|
//! | `resource.presentation.request` | `local_write` | `required` | v2 request / v2 receipt |
//! | `resource.presentation.status` | `read` | `none` | v2 status input / v2 status output |
//! | `resource.presentation.cancel` | `local_write` | `optional` | v2 status input / v2 cancel output |
//!
//! The locator family has one read operation:
//!
//! | operation | effect | idempotency | declared input/output |
//! |---|---|---|---|
//! | `resource.locator.resolve` | `read` | `optional` | Mount-relative locator / opaque ref |
//!
//! # Authority
//!
//! A presentation request carries only opaque refs — never a device path. The
//! Host resolves a `resourceRef` against its own live AppFlowy state:
//!
//! - the id of the **current AppFlowy workspace** (the Host-owned Mount
//!   identity) selects the workspace root;
//! - the id of a view that `FlowyFolder::get_all_views_pb` reports for that
//!   workspace selects that view, and its layout picks the presenting adapter;
//! - any other opaque ref stays **unresolved**: it is handed to the surface
//!   dispatcher, which is the layer that owns Host mount catalogs the Rust core
//!   does not hold (for example the Flutter Project Workspace mounts).
//!
//! A ref that is not in the shared opaque grammar (`^[A-Za-z0-9._~-]{1,128}$`)
//! is rejected outright, so a path-shaped ref can never reach a surface.
//!
//! # Locator
//!
//! DSH computes a Mount-relative locator (its own `mountRef` plus a POSIX path
//! inside that Mount) and cannot mint a ref itself, so
//! [`RESOURCE_LOCATOR_RESOLVE`] is what turns "the file the user clicked" into
//! something a presentation request can name:
//!
//! - the input is validated as a Mount-relative path, never a device path:
//!   `relativePath` may not be absolute, may not carry a drive letter, a
//!   backslash, a control character, an empty segment, `.` or `..`; the empty
//!   path is the Mount root. The declared schema refuses those shapes, and the
//!   provider re-checks them;
//! - the answer is `resource.<24 hex>` — a SHA-256 over the authority (actor,
//!   scope, epoch), the Host's current workspace and the locator — so it is
//!   **stable within one Host authority generation**, and one actor can never
//!   resolve another actor's ref;
//! - the Host records the locator (bounded, 30 min, per-actor quota) and answers
//!   only when that record succeeded, so a locate can never hand out a ref that
//!   cannot later resolve.
//!
//! A presentation request naming a minted ref resolves it to the workspace's
//! Mount plus the mount-relative path — or re-derives it from the
//! `muse.mount-relative-path/v1` anchor hint the DSH mirrors onto the request,
//! which keeps a ref resolvable after its record expires. The Mount identity is
//! the Host's live AppFlowy workspace; the DSH `mountRef` is opaque here, and
//! the Flutter seam owns the mount-to-directory catalog.
//!
//! # Partial paths
//!
//! DSH cites what a reader sees, and a citation can be incomplete
//! (`architecture/08-view-support.md`) or be nothing but a file name
//! (`08-view-support.md`). The locator still mints the ref for exactly what DSH
//! sent — the ref stays opaque and stable, and the anchor hint keeps naming the
//! same locator — but the **dispatch** carries the path the Host resolved inside
//! the Mount, because the surface only opens files that exist.
//!
//! Resolution therefore happens at presentation time, in this order:
//!
//! 1. the entry's Mount-relative path, exactly as minted, when it is a file;
//! 2. a segment-suffix match, then a file-name match, inside the granted Mount —
//!    see [`crate::muse_partial_path`] for the ranking, the bounds and the
//!    containment rules;
//! 3. nothing found: the minted path is dispatched unchanged, so the surface
//!    answers its own `RESOURCE_NOT_FOUND`.
//!
//! When several files match equally well the request fails closed with a
//! `failed` receipt whose `errorCode` is `RESOURCE_AMBIGUOUS_<count>`, and no
//! surface is opened. Searching needs the directory each `mountRef` was granted
//! ([`MuseMountRootCatalog`]); without one the Host forwards the minted path
//! verbatim, which is what every build did before the catalog existed.
//!
//! # Terminal outcome
//!
//! Every invocation returns a terminal `muse.presentation/receipt/v2`; the
//! provider never reports success it cannot observe. Success (`opened`,
//! `focused`, `fallback`) is produced **only** from a
//! [`MuseResourceSurfaceDispatcher`] outcome, because the surface only exists in
//! the Flutter UI layer. When no dispatcher is installed the request fails
//! closed with one of:
//!
//! - `SURFACE_UNAVAILABLE` (`unsupported`, retryable) — the ref resolved, but
//!   this Host build has no surface dispatcher installed yet;
//! - `RESOURCE_NOT_FOUND` (`unsupported`, not retryable) — the opaque ref names
//!   nothing this Host can present;
//! - `RESOURCE_REF_INVALID` / `MOUNT_NOT_BOUND` / `VIEW_LOCKED` (`denied`);
//! - `RESOURCE_AMBIGUOUS_<count>` (`failed`, not retryable) — the minted path is
//!   partial and several files inside the Mount match it equally well.
//!
//! # Remaining seam (Dart side)
//!
//! Opening a tab is a Flutter action, so the last hop is an installable
//! in-process seam — the same pattern as `flowy-document`'s
//! `install_muse_ui_context_publisher`. Both wirings below end in the existing
//! viewer surface path, and neither may use the Windows WebView bridge in
//! `dsh_embedded_view.dart` (it is built without the `onOpenResource` callback
//! and carries a raw path + cwd):
//!
//! 1. **In-process (recommended).** The Flutter layer installs an implementation
//!    with [`install_muse_resource_surface_dispatcher`] — through a new
//!    `dart-ffi` entry point in `flowy-core`, or through the existing
//!    Flutter↔Rust plugin-event channel. The implementation projects
//!    [`MusePresentationDispatch::to_wire`] onto
//!    `MuseResourceSurfaceOpener.open(context, MuseResourceOpenRequest(...))` in
//!    `lib/plugins/resource_surface/resource_surface_dialog.dart`, which routes
//!    through `MuseLocalResourceRouter` and opens a `MuseResourceFilePlugin` tab
//!    via `TabsBloc.openExternalPlugin`; it then answers with
//!    [`MusePresentationOutcome::from_wire`] carrying the real
//!    `surfaceInstanceRef`. A `dispatch` returning `None` means "this build
//!    cannot present that resource", never "assume it worked".
//!    The catalog this needs already exists in Flutter: a dispatch carrying
//!    `mountRef`/`relativePath` maps onto
//!    `MuseWorkspaceController.requireMount(mountRef).rootLocator` plus the
//!    Mount-relative path, which is exactly what
//!    `MuseResourceOpenRequest(path:, origin: dshConversation, sessionCwd: root)`
//!    expects — the router then keeps its own containment check (the DSH
//!    conversation flow uses the same pair). Nothing in that hop needs a path on
//!    the bridge.
//! 2. **Streamed.** The provider could publish a `MuseResourcePresentation`
//!    notification (the `MuseMarkdown` precedent in `flowy-document`) and confirm
//!    the opened surface through the `muse.ui-context` contribution the markdown
//!    facet already publishes. No new Rust→Dart install call is needed, but
//!    confirmation only exists for surfaces that publish such a contribution.
//!
//! Until one of them exists the provider fails closed with
//! `SURFACE_UNAVAILABLE` (retryable), so the DSH caller sees `unsupported`
//! instead of a silent success.

use std::{
  collections::BTreeMap,
  path::PathBuf,
  sync::{Arc, RwLock, Weak},
  time::{SystemTime, UNIX_EPOCH},
};

use flowy_folder::{entities::view::ViewLayoutPB, manager::FolderManager};
use lib_infra::async_trait::async_trait;
use muse_host_bridge_contract::{digest::digest_input, json::JsonLimits};
use muse_host_registry::{
  Cancellation, CapabilityProvider, Effect, Idempotency, OperationDescriptor, ProviderDescriptor,
  ProviderFailure, ProviderInvocation, ResolvedHostContext, SchemaDocument,
};
use muse_presentation_contract::{
  PresentationContractError, PresentationOperationSchema, PresentationSchemaKind, validate,
  validate_operation,
};
use serde_json::{json, Value};
use sha2::{Digest, Sha256};
use tokio::sync::Mutex;
use uuid::Uuid;

use crate::muse_partial_path::{PartialPathResolution, resolve_partial_path};

/// Capability family id served by this module.
pub const RESOURCE_PRESENTATION_FAMILY: &str = "muse.resource-presentation";
/// Operation id of the DSH presentation request.
pub const RESOURCE_PRESENTATION_REQUEST: &str = "resource.presentation.request";
/// Operation id of the DSH presentation status lookup.
pub const RESOURCE_PRESENTATION_STATUS: &str = "resource.presentation.status";
/// Operation id of the DSH presentation cancellation.
pub const RESOURCE_PRESENTATION_CANCEL: &str = "resource.presentation.cancel";

/// Capability family id of the Host resource locator.
pub const RESOURCE_LOCATOR_FAMILY: &str = "muse.resource-locator";
/// Operation id that mints an opaque ref for a Mount-relative locator.
pub const RESOURCE_LOCATOR_RESOLVE: &str = "resource.locator.resolve";

const LOCATOR_DESCRIPTOR_ID: &str = "appflowy.resource-locator.local";
const LOCATOR_DESCRIPTOR_REVISION: &str = "1";
const LOCATOR_CONTRACT_MAJOR: u16 = 1;
const LOCATOR_CONTRACT_MINOR: u16 = 0;
/// Longest accepted Mount scope reference, matching the DSH client bound.
const MAX_MOUNT_REF_CHARS: usize = 256;
/// Longest accepted Mount-relative path, matching the DSH client bound.
const MAX_RELATIVE_PATH_CHARS: usize = 4096;
/// How long a minted locator stays resolvable by a later presentation request.
const LOCATOR_TTL_MS: u64 = 1_800_000;
const MAX_LOCATOR_ENTRIES: usize = 4096;
const MAX_LOCATOR_ENTRIES_PER_ACTOR: usize = 256;
const MAX_DISPLAY_NAME_CHARS: usize = 256;
/// Anchor-hint provider naming a Mount-relative locator (DSH mirror of the
/// locator input, so a later request can be checked against the ref we minted).
const MOUNT_RELATIVE_ANCHOR_PROVIDER: &str = "muse.mount-relative-path/v1";

const DESCRIPTOR_ID: &str = "appflowy.resource-presentation.local";
const DESCRIPTOR_REVISION: &str = "1";
const CONTRACT_MAJOR: u16 = 2;
const CONTRACT_MINOR: u16 = 0;
/// How long a completed presentation stays addressable by `status`/`cancel`.
const SESSION_TTL_MS: u64 = 300_000;
const MAX_SESSIONS: usize = 4096;
const MAX_SESSIONS_PER_ACTOR: usize = 64;
const MAX_WARNINGS: usize = 16;
const MAX_WARNING_CHARS: usize = 256;
const INPUT_LIMITS: JsonLimits = JsonLimits {
  max_depth: 64,
  max_container_children: 10_000,
};

const ADAPTER_WORKSPACE: &str = "muse.adapter.appflowy-workspace";
/// Adapter for a ref this Rust core cannot resolve itself: the surface
/// dispatcher owns the Host mount catalogs that can (for example the Flutter
/// Project Workspace mounts).
const ADAPTER_UNRESOLVED: &str = "muse.adapter.host-resolved";
const ADAPTER_DOCUMENT: &str = "muse.adapter.appflowy-document";
const ADAPTER_DATABASE: &str = "muse.adapter.appflowy-database";
const ADAPTER_CHAT: &str = "muse.adapter.appflowy-chat";
const ADAPTER_OFFICE: &str = "muse.adapter.ioffice";
const ADAPTER_VIEWER: &str = "muse.adapter.viewer";

// ---------------------------------------------------------------------------
// Surface seam
// ---------------------------------------------------------------------------

/// Host-authoritative presentation handed to the UI layer.
///
/// All fields are Host-derived. `resource_ref` is the opaque ref from the DSH
/// request and is never a device path; `workspace_id` is the Host's own Mount
/// identity, not the DSH `mountRef` namespace.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MusePresentationDispatch {
  /// Opaque `requestRef` of the v2 request; also the DSH idempotency key.
  pub request_ref: String,
  /// Opaque `resourceRef` exactly as requested.
  pub resource_ref: String,
  /// Id of the Host workspace (Mount) the request was authorized against.
  pub workspace_id: String,
  /// View id when the Host resolved the ref to one of its own views.
  pub view_id: Option<String>,
  /// Host layout label (`document`, `word`, ...) when the view resolved.
  pub layout: Option<String>,
  /// Display title when the view resolved.
  pub title: Option<String>,
  /// `open` or `focus`, from the v2 request.
  pub disposition: String,
  /// `view`, `prefer-edit` or `edit`, from the v2 request.
  pub requested_mode: String,
  /// `current-window`, `new-window`, `side-panel` or `background`.
  pub placement_hint: String,
  /// Host-selected adapter for the resolved target.
  pub adapter_ref: String,
  /// `view`, `ephemeral-edit` or `edit` the Host will try to establish.
  pub effective_mode: String,
  /// Host session scope this presentation runs under.
  pub session_ref: String,
  /// Host-known revision marker of the resolved target.
  pub revision: String,
  /// `cause.kind` of the v2 request, kept for Host-side auditing.
  pub cause_kind: String,
  /// Anchor hint of the v2 request, forwarded verbatim.
  pub anchor_hint: Option<Value>,
  /// DSH `mountRef` the Host located the resource in, when the ref was minted
  /// by [`RESOURCE_LOCATOR_FAMILY`]. Opaque; never a device path.
  pub mount_ref: Option<String>,
  /// Mount-relative POSIX path inside that Mount, empty for the Mount root.
  pub relative_path: Option<String>,
  /// True when `resourceRef` is a view of the Host's current workspace.
  pub resolved: bool,
}

/// What the UI layer actually established. Every ref is opaque.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MusePresentationOutcome {
  /// `opened`, `focused` or `fallback`.
  pub result: String,
  /// Opaque ref of the surface instance that now presents the resource.
  pub surface_instance_ref: String,
  /// Adapter that performed the presentation.
  pub selected_adapter_ref: String,
  /// `view`, `ephemeral-edit` or `edit` effectively established.
  pub effective_mode: String,
  /// Host-known revision of what is displayed.
  pub revision: String,
  /// Non-fatal Host notes (each bounded).
  pub warnings: Vec<String>,
}

/// Marker carried by a dispatch payload handed to the Flutter bridge.
pub const MUSE_PRESENTATION_DISPATCH_PROTOCOL: &str = "muse.presentation/dispatch/v1";

impl MusePresentationDispatch {
  /// JSON projection the Flutter bridge transports (notification payload or FFI
  /// argument). It carries only opaque Host refs — never a device path — so the
  /// UI layer resolves nothing the Host did not already resolve.
  pub fn to_wire(&self) -> Value {
    let mut value = json!({
      "protocol": MUSE_PRESENTATION_DISPATCH_PROTOCOL,
      "requestRef": self.request_ref,
      "resourceRef": self.resource_ref,
      "workspaceId": self.workspace_id,
      "disposition": self.disposition,
      "requestedMode": self.requested_mode,
      "placementHint": self.placement_hint,
      "adapterRef": self.adapter_ref,
      "effectiveMode": self.effective_mode,
      "sessionRef": self.session_ref,
      "revision": self.revision,
      "causeKind": self.cause_kind,
      "resolved": self.resolved
    });
    if let Some(view_id) = &self.view_id {
      value["viewId"] = json!(view_id);
    }
    if let Some(layout) = &self.layout {
      value["layout"] = json!(layout);
    }
    if let Some(title) = &self.title {
      value["title"] = json!(title);
    }
    if let Some(anchor_hint) = &self.anchor_hint {
      value["anchorHint"] = anchor_hint.clone();
    }
    if let Some(mount_ref) = &self.mount_ref {
      value["mountRef"] = json!(mount_ref);
    }
    if let Some(relative_path) = &self.relative_path {
      value["relativePath"] = json!(relative_path);
    }
    value
  }
}

impl MusePresentationOutcome {
  /// Parse the Flutter bridge's reply. `None` means the payload cannot be
  /// projected, so the provider fails closed instead of inventing a result.
  pub fn from_wire(value: &Value) -> Option<Self> {
    let text = |field: &str| {
      value
        .get(field)
        .and_then(Value::as_str)
        .map(str::to_owned)
    };
    let warnings = match value.get("warnings") {
      None => Vec::new(),
      Some(warnings) => warnings
        .as_array()?
        .iter()
        .map(|warning| warning.as_str().map(str::to_owned))
        .collect::<Option<Vec<_>>>()?,
    };
    Some(Self {
      result: text("result")?,
      surface_instance_ref: text("surfaceInstanceRef")?,
      selected_adapter_ref: text("selectedAdapterRef")?,
      effective_mode: text("effectiveMode")?,
      revision: text("revision")?,
      warnings,
    })
  }
}

/// The one remaining Host-side seam of the family.
pub trait MuseResourceSurfaceDispatcher: Send + Sync + 'static {
  /// Present one resource in the Host UI.
  ///
  /// Returns `None` when this Host build cannot present the resource at all;
  /// the provider then fails closed instead of fabricating a receipt.
  fn dispatch(&self, request: MusePresentationDispatch) -> Option<MusePresentationOutcome>;
}

static SURFACE_DISPATCHER: RwLock<Option<Arc<dyn MuseResourceSurfaceDispatcher>>> =
  RwLock::new(None);

/// Install the Host surface dispatcher. Replaces any previous implementation,
/// mirroring `flowy_document::muse_context::install_muse_ui_context_publisher`.
pub fn install_muse_resource_surface_dispatcher(
  dispatcher: Arc<dyn MuseResourceSurfaceDispatcher>,
) {
  *SURFACE_DISPATCHER
    .write()
    .expect("Muse resource surface dispatcher lock poisoned") = Some(dispatcher);
}

/// Remove the Host surface dispatcher; used by the Flutter layer on teardown and
/// by tests. After this call presentations fail closed again.
pub fn clear_muse_resource_surface_dispatcher() {
  *SURFACE_DISPATCHER
    .write()
    .expect("Muse resource surface dispatcher lock poisoned") = None;
}

/// True when a Host surface dispatcher is installed.
pub fn muse_resource_surface_available() -> bool {
  SURFACE_DISPATCHER
    .read()
    .map(|slot| slot.is_some())
    .unwrap_or(false)
}

fn dispatch_surface(request: MusePresentationDispatch) -> Option<MusePresentationOutcome> {
  let dispatcher = SURFACE_DISPATCHER
    .read()
    .ok()
    .and_then(|slot| slot.as_ref().cloned())?;
  dispatcher.dispatch(request)
}

// ---------------------------------------------------------------------------
// Granted Mount catalog
// ---------------------------------------------------------------------------

/// The directory the Host granted for one DSH `mountRef`.
///
/// A `mountRef` is opaque to this crate: the Flutter layer owns the
/// mount-to-directory catalog (`MuseWorkspaceController`), so resolving a
/// partial Mount-relative path needs that catalog here. Installing one turns on
/// fuzzy resolution; without one the Host keeps dispatching the locator path
/// exactly as DSH supplied it, which is the behaviour of every build that never
/// published a catalog.
pub trait MuseMountRootCatalog: Send + Sync + 'static {
  /// Directory this Host granted for `mount_ref`, or `None` when unbound.
  fn mount_root(&self, mount_ref: &str) -> Option<PathBuf>;
}

static MOUNT_ROOT_CATALOG: RwLock<Option<Arc<dyn MuseMountRootCatalog>>> = RwLock::new(None);

/// Install (or replace) the granted Mount catalog.
pub fn install_muse_mount_root_catalog(catalog: Arc<dyn MuseMountRootCatalog>) {
  *MOUNT_ROOT_CATALOG
    .write()
    .expect("Muse Mount root catalog lock poisoned") = Some(catalog);
}

/// Remove the granted Mount catalog; partial paths stop being searched.
pub fn clear_muse_mount_root_catalog() {
  *MOUNT_ROOT_CATALOG
    .write()
    .expect("Muse Mount root catalog lock poisoned") = None;
}

/// True when this Host knows the granted directories of its Mounts.
pub fn muse_mount_root_catalog_available() -> bool {
  MOUNT_ROOT_CATALOG
    .read()
    .map(|slot| slot.is_some())
    .unwrap_or(false)
}

fn mount_root_for(mount_ref: &str) -> Option<PathBuf> {
  MOUNT_ROOT_CATALOG
    .read()
    .ok()
    .and_then(|slot| slot.as_ref().cloned())?
    .mount_root(mount_ref)
}

/// Equally good candidates for one partial path: the Host must refuse instead of
/// guessing which file the caller meant.
#[derive(Debug, Clone, PartialEq, Eq)]
struct AmbiguousPartialPath {
  candidates: usize,
  sample: Vec<String>,
  truncated: bool,
}

/// The Mount-relative path a presentation must actually dispatch.
///
/// The exact path always wins and is returned unchanged. Only when the granted
/// Mount directory is known and the exact path is not a file does the Host search
/// inside that Mount — see [`crate::muse_partial_path`] for the matching order,
/// the ranking, the bounds and the containment rules. Nothing found (or a Mount
/// whose directory this build does not know) keeps the supplied path, so the
/// surface still answers its own honest not-found.
fn mount_relative_path_for_dispatch(
  entry: &LocatorEntry,
) -> Result<String, AmbiguousPartialPath> {
  if entry.relative_path.is_empty() {
    // The Mount root itself: there is nothing to search for.
    return Ok(entry.relative_path.clone());
  }
  let Some(root) = mount_root_for(&entry.mount_ref) else {
    return Ok(entry.relative_path.clone());
  };
  match resolve_partial_path(&root, &entry.relative_path) {
    PartialPathResolution::Exact(resolved) => Ok(resolved),
    PartialPathResolution::Resolved {
      relative_path,
      matched,
      extra_segments,
    } => {
      tracing::info!(
        mount_ref = %entry.mount_ref,
        requested = %entry.relative_path,
        resolved = %relative_path,
        matched = ?matched,
        extra_segments,
        "Muse resolved a partial Mount-relative path"
      );
      Ok(relative_path)
    }
    PartialPathResolution::Ambiguous {
      candidates,
      sample,
      truncated,
    } => Err(AmbiguousPartialPath {
      candidates,
      sample,
      truncated,
    }),
    PartialPathResolution::NotFound | PartialPathResolution::Unavailable => {
      Ok(entry.relative_path.clone())
    }
  }
}

/// Distinct, readable reason code for one ambiguous partial path. It names the
/// number of equally good candidates, and `_32`-style caps mean "at least this
/// many" when the search hit its candidate bound.
fn ambiguous_partial_path_code(ambiguous: &AmbiguousPartialPath) -> String {
  format!("RESOURCE_AMBIGUOUS_{}", ambiguous.candidates.max(2))
}

// ---------------------------------------------------------------------------
// Host catalog
// ---------------------------------------------------------------------------

/// One Host-known AppFlowy view, projected for presentation resolution.
#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct PresentationView {
  pub id: String,
  pub title: String,
  pub layout: &'static str,
  pub adapter_ref: &'static str,
  pub locked: bool,
  pub last_edited_ms: i64,
}

/// The Host state a presentation request is authorized against.
#[async_trait]
pub(crate) trait PresentationCatalog: Send + Sync + 'static {
  /// False when the Host no longer holds live AppFlowy state.
  fn is_available(&self) -> bool;
  /// `(workspace_id, title, created_at_ms)` of the Host's current workspace.
  async fn workspace(&self) -> Result<(String, String, i64), ()>;
  /// Views the Host reports for its current workspace.
  async fn views(&self) -> Result<Vec<PresentationView>, ()>;
}

struct FolderCatalog {
  folder_manager: Weak<FolderManager>,
}

#[async_trait]
impl PresentationCatalog for FolderCatalog {
  fn is_available(&self) -> bool {
    self.folder_manager.upgrade().is_some()
  }

  async fn workspace(&self) -> Result<(String, String, i64), ()> {
    let folder = self.folder_manager.upgrade().ok_or(())?;
    let workspace = folder.get_current_workspace().await.map_err(|_| ())?;
    Ok((workspace.id, workspace.name, workspace.create_time))
  }

  async fn views(&self) -> Result<Vec<PresentationView>, ()> {
    let folder = self.folder_manager.upgrade().ok_or(())?;
    let views = folder.get_all_views_pb().await.map_err(|_| ())?;
    Ok(
      views
        .iter()
        .map(|view| PresentationView {
          id: view.id.clone(),
          title: view.name.clone(),
          layout: layout_label(&view.layout),
          adapter_ref: adapter_ref(&view.layout),
          locked: view.is_locked.unwrap_or(false),
          last_edited_ms: view.last_edited,
        })
        .collect(),
    )
  }
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

/// Adapter that presents one AppFlowy layout. The Flutter layer maps it onto the
/// existing `MuseLocalResourceRouter` engines and native view surfaces.
fn adapter_ref(layout: &ViewLayoutPB) -> &'static str {
  match layout {
    ViewLayoutPB::Document => ADAPTER_DOCUMENT,
    ViewLayoutPB::Grid | ViewLayoutPB::Board | ViewLayoutPB::Calendar => ADAPTER_DATABASE,
    ViewLayoutPB::Chat => ADAPTER_CHAT,
    ViewLayoutPB::Word | ViewLayoutPB::Excel | ViewLayoutPB::Slides => ADAPTER_OFFICE,
    ViewLayoutPB::Pdf => ADAPTER_VIEWER,
  }
}

// ---------------------------------------------------------------------------
// Provider
// ---------------------------------------------------------------------------

/// Host state shared by the presentation provider and the locator provider: the
/// presentation session table and the refs this Host has minted. One state per
/// registered provider pair, so a ref minted by the locator is exactly the ref
/// the presentation request later resolves.
pub(crate) struct PresentationState {
  catalog: Arc<dyn PresentationCatalog>,
  sessions: Mutex<SessionTable>,
  locators: Mutex<LocatorTable>,
}

impl PresentationState {
  pub(crate) fn new(catalog: Arc<dyn PresentationCatalog>) -> Arc<Self> {
    Arc::new(Self {
      catalog,
      sessions: Mutex::new(SessionTable::default()),
      locators: Mutex::new(LocatorTable::default()),
    })
  }

  /// State over the live AppFlowy folder manager.
  pub(crate) fn for_folder(folder_manager: Weak<FolderManager>) -> Arc<Self> {
    Self::new(Arc::new(FolderCatalog { folder_manager }))
  }
}

pub(crate) struct PresentationProvider {
  state: Arc<PresentationState>,
}

impl PresentationProvider {
  /// Build a provider over shared Host state.
  pub(crate) fn with_state(state: Arc<PresentationState>) -> Self {
    Self { state }
  }

  /// Build a provider over an explicit catalog. Production uses
  /// [`PresentationState::for_folder`]; tests use this to run without AppFlowy.
  #[cfg(test)]
  pub(crate) fn with_catalog(catalog: Arc<dyn PresentationCatalog>) -> Self {
    Self::with_state(PresentationState::new(catalog))
  }

  async fn request(
    &self,
    invocation: ProviderInvocation,
    context: ResolvedHostContext,
    cancellation: Cancellation,
  ) -> Result<Value, ProviderFailure> {
    check_active(&cancellation, invocation.deadline_at_ms)?;
    let input = RequestInput::parse(&invocation.input)?;
    if !is_opaque_ref(&input.resource_ref) {
      return self
        .record_terminal(
          &context,
          &invocation,
          &input,
          failure_receipt(&input, "denied", "RESOURCE_REF_INVALID", false),
        )
        .await;
    }
    let input_digest =
      digest_input(&invocation.input, INPUT_LIMITS).map_err(|_| ProviderFailure)?;
    {
      let mut sessions = self.state.sessions.lock().await;
      sessions.sweep(unix_ms());
      if let Some(existing) = sessions.lookup(&context, &invocation, &input, &input_digest) {
        // Same identity and same input: replay the terminal receipt.
        return Ok(existing);
      }
      sessions.begin(&context, &invocation, &input, &input_digest);
    }

    let (workspace_id, _title, created_at) = self
      .state
      .catalog
      .workspace()
      .await
      .map_err(|_| ProviderFailure)?;
    let session_ref = input
      .cause_session_ref
      .clone()
      .filter(|value| is_opaque_ref(value))
      .unwrap_or_else(|| context.scope_ref.clone());
    let mut warnings = Vec::new();
    if session_ref == context.scope_ref {
      warnings.push("session.scope-derived".to_string());
    }
    if let Some(bound) = context.evidence.get("appflowy.workspace") {
      if bound != &workspace_id {
        return self
          .record_terminal(
            &context,
            &invocation,
            &input,
            failure_receipt(&input, "denied", "MOUNT_NOT_BOUND", false),
          )
          .await;
      }
    }

    let resolution = self
      .resolve(&workspace_id, created_at, &context, &invocation, &input)
      .await?;
    let target = match resolution {
      ResolveOutcome::Target(target) => Some(target),
      ResolveOutcome::Unresolved => None,
      ResolveOutcome::Ambiguous {
        candidates,
        sample,
        truncated,
      } => {
        // Fail closed and visibly: never guess between equally good files.
        tracing::warn!(
          resource_ref = %input.resource_ref,
          candidates,
          truncated,
          sample = ?sample,
          "Muse refused an ambiguous partial Mount-relative path"
        );
        let ambiguous = AmbiguousPartialPath {
          candidates,
          sample,
          truncated,
        };
        return self
          .record_terminal(
            &context,
            &invocation,
            &input,
            failure_receipt(&input, "failed", &ambiguous_partial_path_code(&ambiguous), false),
          )
          .await;
      }
    };
    let resolved = target.is_some();
    let (effective_mode, adapter_ref, revision, locked) = match &target {
      Some(target) => {
        let (effective_mode, mode_warning) = effective_mode(&input.requested_mode, target.locked);
        if let Some(warning) = mode_warning {
          warnings.push(warning.to_string());
        }
        (
          effective_mode,
          target.adapter_ref,
          target.revision.clone(),
          target.locked,
        )
      }
      None => (
        effective_mode(&input.requested_mode, false).0,
        ADAPTER_UNRESOLVED,
        format!("revision.{created_at}"),
        false,
      ),
    };
    if input.requested_mode == "edit" && locked {
      return self
        .record_terminal(
          &context,
          &invocation,
          &input,
          failure_receipt(&input, "denied", "VIEW_LOCKED", false),
        )
        .await;
    }
    check_active(&cancellation, invocation.deadline_at_ms)?;

    let dispatch = MusePresentationDispatch {
      request_ref: input.request_ref.clone(),
      resource_ref: input.resource_ref.clone(),
      workspace_id,
      view_id: target.as_ref().map(|target| target.id.clone()),
      layout: target.as_ref().map(|target| target.layout.to_string()),
      title: target.as_ref().map(|target| target.title.clone()),
      disposition: input.disposition.clone(),
      requested_mode: input.requested_mode.clone(),
      placement_hint: input.placement_hint.clone(),
      adapter_ref: adapter_ref.to_string(),
      effective_mode: effective_mode.to_string(),
      session_ref: session_ref.clone(),
      revision: revision.clone(),
      cause_kind: input.cause_kind.clone(),
      anchor_hint: input.anchor_hint.clone(),
      mount_ref: target.as_ref().and_then(|target| target.mount_ref.clone()),
      relative_path: target
        .as_ref()
        .and_then(|target| target.relative_path.clone()),
      resolved,
    };
    let outcome = dispatch_surface(dispatch);
    let receipt = match outcome {
      Some(outcome) => match success_receipt(&input, &session_ref, &revision, outcome, warnings) {
        Some(receipt) => receipt,
        None => failure_receipt(&input, "unsupported", "SURFACE_UNAVAILABLE", true),
      },
      None if resolved => failure_receipt(&input, "unsupported", "SURFACE_UNAVAILABLE", true),
      None => failure_receipt(&input, "unsupported", "RESOURCE_NOT_FOUND", false),
    };
    let receipt = if cancellation.is_cancelled() {
      failure_receipt(&input, "cancelled", "CANCELLED", false)
    } else {
      receipt
    };
    self
      .record_terminal(&context, &invocation, &input, receipt)
      .await
  }

  /// Resolve an opaque ref to one of the Host's own targets, if it names one.
  ///
  /// Order matters: the current workspace, then one of the Host's views, then a
  /// ref this Host minted for a Mount-relative locator. Anything else stays
  /// unresolved and is offered to the surface dispatcher as-is.
  async fn resolve(
    &self,
    workspace_id: &str,
    created_at: i64,
    context: &ResolvedHostContext,
    invocation: &ProviderInvocation,
    input: &RequestInput,
  ) -> Result<ResolveOutcome, ProviderFailure> {
    if input.resource_ref == workspace_id {
      return Ok(ResolveOutcome::Target(ResolvedTarget {
        id: workspace_id.to_string(),
        title: input.resource_ref.clone(),
        layout: "workspace",
        adapter_ref: ADAPTER_WORKSPACE,
        locked: false,
        revision: format!("revision.{created_at}"),
        mount_ref: None,
        relative_path: None,
      }));
    }
    let views = self
      .state
      .catalog
      .views()
      .await
      .map_err(|_| ProviderFailure)?;
    if let Some(view) = views
      .into_iter()
      .find(|view| view.id == input.resource_ref)
    {
      return Ok(ResolveOutcome::Target(ResolvedTarget {
        id: view.id,
        title: view.title,
        layout: view.layout,
        adapter_ref: view.adapter_ref,
        locked: view.locked,
        revision: format!("revision.{}", view.last_edited_ms),
        mount_ref: None,
        relative_path: None,
      }));
    }
    let Some(entry) = self
      .locator_target(workspace_id, context, invocation, input)
      .await
    else {
      return Ok(ResolveOutcome::Unresolved);
    };
    // A partial path is resolved **before** the surface is asked to present it,
    // so the dispatch carries the real, existing Mount-relative path while the
    // opaque ref stays exactly the one DSH was given.
    let relative_path = match mount_relative_path_for_dispatch(&entry) {
      Ok(relative_path) => relative_path,
      Err(ambiguous) => {
        return Ok(ResolveOutcome::Ambiguous {
          candidates: ambiguous.candidates,
          sample: ambiguous.sample,
          truncated: ambiguous.truncated,
        })
      }
    };
    Ok(ResolveOutcome::Target(ResolvedTarget {
      id: entry.resource_ref.clone(),
      title: display_name_for(&entry.mount_ref, &relative_path),
      layout: "resource",
      // A plain Mount entry has no AppFlowy layout; the Flutter seam picks
      // the engine from the path extension, exactly as the file viewer does.
      adapter_ref: ADAPTER_UNRESOLVED,
      locked: false,
      revision: entry.revision.clone(),
      mount_ref: Some(entry.mount_ref.clone()),
      relative_path: Some(relative_path),
    }))
  }

  /// Find (or re-derive) the locator this ref was minted for.
  ///
  /// A ref is either in the Host's locator table, or provably derivable from the
  /// Mount-relative anchor hint the DSH mirrors onto the request — in which case
  /// we re-mint it and require the value to be exactly the requested ref. Both
  /// paths require the same actor, scope and authority epoch that minted it.
  async fn locator_target(
    &self,
    workspace_id: &str,
    context: &ResolvedHostContext,
    invocation: &ProviderInvocation,
    input: &RequestInput,
  ) -> Option<LocatorEntry> {
    let now = unix_ms();
    let mut locators = self.state.locators.lock().await;
    locators.sweep(now);
    if let Some(entry) = locators.authorized(context, &input.resource_ref) {
      return Some(entry.clone());
    }
    let value = input
      .anchor_hint
      .as_ref()
      .filter(|anchor| {
        anchor.get("provider").and_then(Value::as_str) == Some(MOUNT_RELATIVE_ANCHOR_PROVIDER)
      })
      .and_then(|anchor| anchor.get("value"))?;
    let mount_ref = value.get("mountRef").and_then(Value::as_str)?;
    let relative_path = value.get("path").and_then(Value::as_str)?;
    if !mount_ref_is_valid(mount_ref) || !relative_path_is_mount_relative(relative_path) {
      return None;
    }
    let resource_ref = mint_resource_ref(
      &context.actor_ref,
      &context.scope_ref,
      context.authority_epoch,
      workspace_id,
      mount_ref,
      relative_path,
    );
    if resource_ref != input.resource_ref {
      // The ref is not one this Host would mint for that locator.
      return None;
    }
    let entry = locator_entry(
      context,
      &invocation.binding_id,
      mount_ref,
      relative_path,
      disposition_of(&input.requested_mode),
      resource_ref,
      now,
    );
    locators.insert(entry.clone(), now);
    Some(entry)
  }

  async fn status(
    &self,
    invocation: ProviderInvocation,
    context: ResolvedHostContext,
    cancellation: Cancellation,
  ) -> Result<Value, ProviderFailure> {
    check_active(&cancellation, invocation.deadline_at_ms)?;
    let request_ref = string_field(&invocation.input, "requestRef")?;
    let now = unix_ms();
    let mut sessions = self.state.sessions.lock().await;
    sessions.sweep(now);
    let value = match sessions.authorized(&context, &request_ref) {
      Some(record) if record.state == "terminal" => json!({
        "requestRef": request_ref,
        "state": "terminal",
        "updatedAt": record.updated_at_ms,
        "receipt": record.receipt
      }),
      Some(record) => json!({
        "requestRef": request_ref,
        "state": "pending",
        "updatedAt": record.updated_at_ms
      }),
      None => json!({"requestRef": request_ref, "state": "unknown", "updatedAt": now}),
    };
    validate_operation(PresentationOperationSchema::StatusOutput, &value)
      .map_err(|_| ProviderFailure)?;
    Ok(value)
  }

  async fn cancel(
    &self,
    invocation: ProviderInvocation,
    context: ResolvedHostContext,
    cancellation: Cancellation,
  ) -> Result<Value, ProviderFailure> {
    check_active(&cancellation, invocation.deadline_at_ms)?;
    let request_ref = string_field(&invocation.input, "requestRef")?;
    let now = unix_ms();
    let mut sessions = self.state.sessions.lock().await;
    sessions.sweep(now);
    let value = match sessions.cancel_request(&context, &request_ref, now) {
      CancelOutcome::NotFound => json!({
        "requestRef": request_ref, "disposition": "not_found", "updatedAt": now
      }),
      CancelOutcome::AlreadyTerminal { updated_at, receipt } => json!({
        "requestRef": request_ref,
        "disposition": "already_terminal",
        "updatedAt": updated_at,
        "receipt": receipt
      }),
      CancelOutcome::Accepted => json!({
        "requestRef": request_ref,
        "disposition": "accepted",
        "updatedAt": now
      }),
    };
    validate_operation(PresentationOperationSchema::CancelOutput, &value)
      .map_err(|_| ProviderFailure)?;
    Ok(value)
  }

  /// Store one terminal receipt, unless a concurrent `cancel` won the race.
  async fn record_terminal(
    &self,
    context: &ResolvedHostContext,
    invocation: &ProviderInvocation,
    input: &RequestInput,
    receipt: Value,
  ) -> Result<Value, ProviderFailure> {
    validate(PresentationSchemaKind::Receipt, &receipt).map_err(|_| ProviderFailure)?;
    let now = unix_ms();
    let mut sessions = self.state.sessions.lock().await;
    sessions.sweep(now);
    let receipt = sessions.complete(context, invocation, input, receipt, now);
    validate(PresentationSchemaKind::Receipt, &receipt).map_err(|_| ProviderFailure)?;
    Ok(receipt)
  }
}

#[async_trait]
impl CapabilityProvider for PresentationProvider {
  fn descriptor(&self) -> ProviderDescriptor {
    descriptor()
  }

  async fn available(&self, _context: &ResolvedHostContext) -> Result<bool, ProviderFailure> {
    Ok(self.state.catalog.is_available())
  }

  async fn invoke(
    &self,
    invocation: ProviderInvocation,
    context: ResolvedHostContext,
    cancellation: Cancellation,
  ) -> Result<Value, ProviderFailure> {
    match invocation.operation_id.as_str() {
      RESOURCE_PRESENTATION_REQUEST => {
        self.request(invocation, context, cancellation).await
      }
      RESOURCE_PRESENTATION_STATUS => self.status(invocation, context, cancellation).await,
      RESOURCE_PRESENTATION_CANCEL => self.cancel(invocation, context, cancellation).await,
      _ => Err(ProviderFailure),
    }
  }
}

// ---------------------------------------------------------------------------
// Resource locator (contract major 1)
// ---------------------------------------------------------------------------

/// One ref this Host minted for a Mount-relative locator.
///
/// The entry is the Host's own record that `resource_ref` names
/// `mount_ref`/`relative_path` for exactly one authority (actor, scope, epoch);
/// it is what makes a later `resource.presentation.request` resolve without the
/// caller ever naming a device path.
#[derive(Debug, Clone, PartialEq, Eq)]
struct LocatorEntry {
  actor_ref: String,
  scope_ref: String,
  authority_epoch: u64,
  /// Binding the ref was minted for; audit only, never an authority check.
  binding_id: String,
  mount_ref: String,
  relative_path: String,
  display_name: String,
  disposition: String,
  resource_ref: String,
  revision: String,
  created_at_ms: u64,
  expires_at_ms: u64,
}

/// Bounded, authority-scoped table of minted refs.
#[derive(Debug, Default)]
struct LocatorTable {
  by_ref: BTreeMap<String, LocatorEntry>,
}

impl LocatorTable {
  fn sweep(&mut self, now: u64) {
    self.by_ref.retain(|_, entry| entry.expires_at_ms > now);
  }

  /// Record one minted ref. `false` means the quota refused it, so the caller
  /// must fail the locate instead of answering with a ref it cannot resolve.
  fn insert(&mut self, entry: LocatorEntry, now: u64) -> bool {
    self.sweep(now);
    if self.by_ref.contains_key(&entry.resource_ref) {
      // The same authority re-located the same entry: keep the original record,
      // so a replay can neither re-anchor the ref nor extend its lifetime.
      return true;
    }
    if self
      .by_ref
      .values()
      .filter(|existing| existing.actor_ref == entry.actor_ref)
      .count()
      >= MAX_LOCATOR_ENTRIES_PER_ACTOR
    {
      return false;
    }
    if self.by_ref.len() >= MAX_LOCATOR_ENTRIES {
      if let Some(oldest) = self
        .by_ref
        .iter()
        .min_by_key(|(_, existing)| existing.expires_at_ms)
        .map(|(reference, _)| reference.clone())
      {
        self.by_ref.remove(&oldest);
      }
    }
    self.by_ref.insert(entry.resource_ref.clone(), entry);
    true
  }

  fn authorized(&self, context: &ResolvedHostContext, resource_ref: &str) -> Option<&LocatorEntry> {
    self.by_ref.get(resource_ref).filter(|entry| {
      entry.actor_ref == context.actor_ref
        && entry.scope_ref == context.scope_ref
        && entry.authority_epoch == context.authority_epoch
    })
  }
}

/// The `muse.resource-locator` provider: it turns the Mount-relative locator a
/// DSH client can compute into the opaque ref only this Host can interpret.
pub(crate) struct LocatorProvider {
  state: Arc<PresentationState>,
}

impl LocatorProvider {
  pub(crate) fn with_state(state: Arc<PresentationState>) -> Self {
    Self { state }
  }
}

#[async_trait]
impl CapabilityProvider for LocatorProvider {
  fn descriptor(&self) -> ProviderDescriptor {
    locator_descriptor()
  }

  async fn available(&self, _context: &ResolvedHostContext) -> Result<bool, ProviderFailure> {
    Ok(self.state.catalog.is_available())
  }

  async fn invoke(
    &self,
    invocation: ProviderInvocation,
    context: ResolvedHostContext,
    cancellation: Cancellation,
  ) -> Result<Value, ProviderFailure> {
    if invocation.operation_id != RESOURCE_LOCATOR_RESOLVE {
      return Err(ProviderFailure);
    }
    check_active(&cancellation, invocation.deadline_at_ms)?;
    let mount_ref = string_field(&invocation.input, "mountRef")?;
    let relative_path = string_field(&invocation.input, "relativePath")?;
    // Defence in depth: the declared schema already refuses these shapes.
    if !mount_ref_is_valid(&mount_ref) || !relative_path_is_mount_relative(&relative_path) {
      return Err(ProviderFailure);
    }
    let disposition = disposition_of(
      invocation
        .input
        .get("disposition")
        .and_then(Value::as_str)
        .unwrap_or("view"),
    );
    let (workspace_id, _, _) = self
      .state
      .catalog
      .workspace()
      .await
      .map_err(|_| ProviderFailure)?;
    let resource_ref = mint_resource_ref(
      &context.actor_ref,
      &context.scope_ref,
      context.authority_epoch,
      &workspace_id,
      &mount_ref,
      &relative_path,
    );
    let display_name = display_name_for(&mount_ref, &relative_path);
    let now = unix_ms();
    let entry = locator_entry(
      &context,
      &invocation.binding_id,
      &mount_ref,
      &relative_path,
      disposition,
      resource_ref.clone(),
      now,
    );
    check_active(&cancellation, invocation.deadline_at_ms)?;
    let recorded = self
      .state
      .locators
      .lock()
      .await
      .insert(entry, now);
    if !recorded {
      // Answering with a ref this Host cannot resolve later would be a lie.
      return Err(ProviderFailure);
    }
    Ok(json!({"resourceRef": resource_ref, "displayName": display_name}))
  }
}

/// Build the locator record for one authority and locator.
fn locator_entry(
  context: &ResolvedHostContext,
  binding_id: &str,
  mount_ref: &str,
  relative_path: &str,
  disposition: &str,
  resource_ref: String,
  now: u64,
) -> LocatorEntry {
  LocatorEntry {
    actor_ref: context.actor_ref.clone(),
    scope_ref: context.scope_ref.clone(),
    authority_epoch: context.authority_epoch,
    binding_id: binding_id.to_string(),
    mount_ref: mount_ref.to_string(),
    relative_path: relative_path.to_string(),
    display_name: display_name_for(mount_ref, relative_path),
    disposition: disposition.to_string(),
    revision: resource_revision(&resource_ref),
    resource_ref,
    created_at_ms: now,
    expires_at_ms: now.saturating_add(LOCATOR_TTL_MS),
  }
}

/// Stable opaque ref for one locator, scope and Host authority generation.
///
/// Deterministic, so re-locating the same entry always yields the same ref; the
/// authority and the current Host workspace are part of the input, so a ref
/// never survives a workspace switch or an authority change, and one actor can
/// never resolve another actor's ref.
fn mint_resource_ref(
  actor_ref: &str,
  scope_ref: &str,
  authority_epoch: u64,
  workspace_id: &str,
  mount_ref: &str,
  relative_path: &str,
) -> String {
  let mut hasher = Sha256::new();
  hasher.update(b"muse.resource-locator/v1\0");
  for part in [actor_ref, scope_ref, workspace_id, mount_ref, relative_path] {
    hasher.update(part.as_bytes());
    hasher.update([0u8]);
  }
  hasher.update(authority_epoch.to_le_bytes());
  format!("resource.{}", hex(&hasher.finalize()[..12]))
}

/// Opaque revision marker for one minted ref, derived from the ref itself.
fn resource_revision(resource_ref: &str) -> String {
  match resource_ref.strip_prefix("resource.") {
    Some(suffix) if suffix.len() <= 96 => format!("revision.{suffix}"),
    _ => format!("revision.{}", hex(&Sha256::digest(resource_ref.as_bytes())[..12])),
  }
}

fn hex(bytes: &[u8]) -> String {
  let mut out = String::with_capacity(bytes.len() * 2);
  for byte in bytes {
    out.push_str(&format!("{:02x}", byte));
  }
  out
}

/// Node name shown for one locator; the Mount label stands in for its root.
fn display_name_for(mount_ref: &str, relative_path: &str) -> String {
  let name = relative_path
    .rsplit('/')
    .find(|segment| !segment.is_empty())
    .unwrap_or(mount_ref);
  name.chars().take(MAX_DISPLAY_NAME_CHARS).collect()
}

fn disposition_of(requested_mode: &str) -> &'static str {
  if requested_mode == "edit" { "edit" } else { "view" }
}

fn mount_ref_is_valid(mount_ref: &str) -> bool {
  !mount_ref.is_empty()
    && mount_ref.chars().count() <= MAX_MOUNT_REF_CHARS
    && !mount_ref.chars().any(char::is_control)
}

/// Whether a value is a Mount-relative POSIX path: no device path, no absolute
/// path, no traversal, no empty segment. The empty path is the Mount root.
fn relative_path_is_mount_relative(relative_path: &str) -> bool {
  if relative_path.is_empty() {
    return true;
  }
  if relative_path.chars().count() > MAX_RELATIVE_PATH_CHARS
    || relative_path.starts_with('/')
    || relative_path.contains('\\')
    || relative_path.chars().any(char::is_control)
  {
    return false;
  }
  let mut chars = relative_path.chars();
  if let (Some(first), Some(':')) = (chars.next(), chars.next()) {
    if first.is_ascii_alphabetic() {
      return false;
    }
  }
  relative_path
    .split('/')
    .all(|segment| !segment.is_empty() && segment != "." && segment != "..")
}

fn locator_descriptor() -> ProviderDescriptor {
  ProviderDescriptor {
    descriptor_id: LOCATOR_DESCRIPTOR_ID.into(),
    revision: LOCATOR_DESCRIPTOR_REVISION.into(),
    family_id: RESOURCE_LOCATOR_FAMILY.into(),
    contract_major: LOCATOR_CONTRACT_MAJOR,
    contract_minor: LOCATOR_CONTRACT_MINOR,
    operations: vec![OperationDescriptor {
      operation_id: RESOURCE_LOCATOR_RESOLVE.into(),
      effect: Effect::Read,
      input_schema: SchemaDocument::new(json!({
        "$schema": "https://json-schema.org/draft/2020-12/schema",
        "type": "object",
        "additionalProperties": false,
        "required": ["mountRef", "relativePath"],
        "properties": {
          "mountRef": {
            "type": "string",
            "minLength": 1,
            "maxLength": MAX_MOUNT_REF_CHARS,
            "not": {"pattern": "[\\u0000-\\u001f]"}
          },
          "relativePath": {
            "type": "string",
            "maxLength": MAX_RELATIVE_PATH_CHARS,
            "allOf": [
              {"not": {"pattern": "^/"}},
              {"not": {"pattern": "^[A-Za-z]:"}},
              {"not": {"pattern": "\\\\"}},
              {"not": {"pattern": "(^|/)\\.\\.($|/)"}},
              {"not": {"pattern": "(^|/)\\.($|/)"}},
              {"not": {"pattern": "//|/$"}},
              {"not": {"pattern": "[\\u0000-\\u001f]"}}
            ]
          },
          "disposition": {"enum": ["view", "edit"]}
        }
      }))
      .expect("static Locator input schema must compile"),
      output_schema: SchemaDocument::new(json!({
        "$schema": "https://json-schema.org/draft/2020-12/schema",
        "type": "object",
        "additionalProperties": false,
        "required": ["resourceRef"],
        "properties": {
          "resourceRef": {"type": "string", "pattern": "^[A-Za-z0-9._~-]{1,128}$"},
          "displayName": {"type": "string", "minLength": 1, "maxLength": MAX_DISPLAY_NAME_CHARS}
        }
      }))
      .expect("static Locator output schema must compile"),
      cancellable: true,
      idempotency: Idempotency::Optional,
    }],
    title: Some("AppFlowy Mount-relative resource locator".into()),
    summary: Some(
      "Mints the opaque resourceRef for a Mount scope and a Mount-relative POSIX path".into(),
    ),
  }
}

#[derive(Debug, Clone)]
struct ResolvedTarget {
  id: String,
  title: String,
  layout: &'static str,
  adapter_ref: &'static str,
  locked: bool,
  revision: String,
  /// DSH `mountRef` when the target came from a Host-minted locator.
  mount_ref: Option<String>,
  /// Mount-relative POSIX path inside that Mount, empty for the Mount root.
  relative_path: Option<String>,
}

/// What one presentation request resolved to.
#[derive(Debug, Clone)]
enum ResolveOutcome {
  /// The ref names a Host target; present it.
  Target(ResolvedTarget),
  /// The ref names nothing this Host holds: the surface decides.
  Unresolved,
  /// The ref names a Mount-relative locator whose partial path matched several
  /// equally good files: refuse instead of guessing.
  Ambiguous {
    candidates: usize,
    sample: Vec<String>,
    truncated: bool,
  },
}

/// Parsed subset of the v2 presentation request the Host acts on. The registry
/// has already validated the payload against the published request schema.
#[derive(Debug, Clone, PartialEq, Eq)]
struct RequestInput {
  request_ref: String,
  resource_ref: String,
  disposition: String,
  requested_mode: String,
  placement_hint: String,
  cause_kind: String,
  cause_session_ref: Option<String>,
  anchor_hint: Option<Value>,
}

impl RequestInput {
  fn parse(input: &Value) -> Result<Self, ProviderFailure> {
    let cause = input.get("cause").ok_or(ProviderFailure)?;
    Ok(Self {
      request_ref: string_field(input, "requestRef")?,
      resource_ref: string_field(input, "resourceRef")?,
      disposition: string_field(input, "disposition")?,
      requested_mode: string_field(input, "requestedMode")?,
      placement_hint: string_field(input, "placementHint")?,
      cause_kind: string_field(cause, "kind")?,
      cause_session_ref: cause
        .get("sessionRef")
        .and_then(Value::as_str)
        .map(str::to_owned),
      anchor_hint: input.get("anchorHint").cloned(),
    })
  }
}

/// `(effectiveMode, warning)` for one requested mode.
fn effective_mode(requested: &str, locked: bool) -> (&'static str, Option<&'static str>) {
  match requested {
    "view" => ("view", None),
    "prefer-edit" if !locked => ("view", Some("mode.edit-not-requested-by-host")),
    "prefer-edit" => ("view", Some("mode.locked")),
    _ if locked => ("view", Some("mode.locked")),
    _ => ("edit", None),
  }
}

// ---------------------------------------------------------------------------
// Receipts
// ---------------------------------------------------------------------------

fn receipt_refs() -> (String, String, String) {
  (
    format!("receipt.{}", Uuid::new_v4()),
    format!("attempt.{}", Uuid::new_v4()),
    format!("trace.{}", Uuid::new_v4()),
  )
}

/// Success receipt, or `None` when the dispatcher outcome cannot be projected
/// into the published receipt contract (the provider then fails closed).
fn success_receipt(
  input: &RequestInput,
  session_ref: &str,
  fallback_revision: &str,
  outcome: MusePresentationOutcome,
  warnings: Vec<String>,
) -> Option<Value> {
  if !matches!(outcome.result.as_str(), "opened" | "focused" | "fallback") {
    return None;
  }
  if !matches!(outcome.effective_mode.as_str(), "view" | "ephemeral-edit" | "edit") {
    return None;
  }
  for value in [
    &outcome.surface_instance_ref,
    &outcome.selected_adapter_ref,
    session_ref,
    fallback_revision,
  ] {
    if !is_opaque_ref(value) {
      return None;
    }
  }
  let revision = if is_opaque_ref(&outcome.revision) {
    outcome.revision
  } else {
    fallback_revision.to_string()
  };
  let mut warnings: Vec<String> = warnings
    .into_iter()
    .filter(|warning| !warning.is_empty() && warning.chars().count() <= MAX_WARNING_CHARS)
    .take(MAX_WARNINGS)
    .collect();
  let remaining = MAX_WARNINGS.saturating_sub(warnings.len());
  warnings.extend(
    outcome
      .warnings
      .into_iter()
      .filter(|warning| !warning.is_empty() && warning.chars().count() <= MAX_WARNING_CHARS)
      .take(remaining),
  );
  let (receipt_ref, attempt_ref, trace_ref) = receipt_refs();
  Some(json!({
    "protocol": "muse.presentation/receipt/v2",
    "receiptRef": receipt_ref,
    "requestRef": input.request_ref,
    "attemptRef": attempt_ref,
    "completedAt": unix_ms(),
    "traceRef": trace_ref,
    "result": outcome.result,
    "effectiveMode": outcome.effective_mode,
    "selectedAdapterRef": outcome.selected_adapter_ref,
    "resourceRef": input.resource_ref,
    "revision": revision,
    "sessionRef": session_ref,
    "surfaceInstanceRef": outcome.surface_instance_ref,
    "warnings": warnings
  }))
}

fn failure_receipt(
  input: &RequestInput,
  result: &str,
  error_code: &str,
  retryable: bool,
) -> Value {
  let (receipt_ref, attempt_ref, trace_ref) = receipt_refs();
  json!({
    "protocol": "muse.presentation/receipt/v2",
    "receiptRef": receipt_ref,
    "requestRef": input.request_ref,
    "attemptRef": attempt_ref,
    "completedAt": unix_ms(),
    "traceRef": trace_ref,
    "result": result,
    "errorCode": error_code,
    "retryable": retryable
  })
}

// ---------------------------------------------------------------------------
// Session records
// ---------------------------------------------------------------------------

#[derive(Debug, Clone)]
enum CancelOutcome {
  Accepted,
  AlreadyTerminal { updated_at: u64, receipt: Value },
  NotFound,
}

#[derive(Debug, Clone)]
struct SessionRecord {
  actor_ref: String,
  scope_ref: String,
  authority_epoch: u64,
  binding_id: String,
  idempotency_key: Option<String>,
  input_digest: String,
  state: &'static str,
  cancel_requested: bool,
  receipt: Value,
  updated_at_ms: u64,
  expires_at_ms: u64,
}

#[derive(Debug, Default)]
struct SessionTable {
  /// `requestRef` -> record.
  by_request: BTreeMap<String, SessionRecord>,
  /// DSH idempotency key -> `requestRef`.
  by_idempotency: BTreeMap<(String, String), String>,
}

impl SessionTable {
  fn sweep(&mut self, now: u64) {
    self.by_request.retain(|_, record| record.expires_at_ms > now);
    let live: std::collections::BTreeSet<String> = self.by_request.keys().cloned().collect();
    self.by_idempotency.retain(|_, request_ref| live.contains(request_ref));
    while self.by_request.len() >= MAX_SESSIONS {
      let Some(oldest) = self
        .by_request
        .iter()
        .filter(|(_, record)| record.state == "terminal")
        .min_by_key(|(_, record)| record.updated_at_ms)
        .map(|(request_ref, _)| request_ref.clone())
      else {
        return;
      };
      self.by_request.remove(&oldest);
      self.by_idempotency.retain(|_, value| value != &oldest);
    }
  }

  fn authorized<'a>(
    &'a self,
    context: &ResolvedHostContext,
    request_ref: &str,
  ) -> Option<&'a SessionRecord> {
    self
      .by_request
      .get(request_ref)
      .filter(|record| is_same_authority(record, context))
  }

  /// Apply a cancellation to whatever state the command is in. A command that is
  /// still running records the request; a terminal one reports its receipt.
  fn cancel_request(
    &mut self,
    context: &ResolvedHostContext,
    request_ref: &str,
    now: u64,
  ) -> CancelOutcome {
    let Some(record) = self
      .by_request
      .get_mut(request_ref)
      .filter(|record| is_same_authority(record, context))
    else {
      return CancelOutcome::NotFound;
    };
    if record.state == "terminal" {
      return CancelOutcome::AlreadyTerminal {
        updated_at: record.updated_at_ms,
        receipt: record.receipt.clone(),
      };
    }
    record.cancel_requested = true;
    record.updated_at_ms = now;
    CancelOutcome::Accepted
  }

  /// Replay a terminal receipt for an identical retry.
  fn lookup(
    &self,
    context: &ResolvedHostContext,
    invocation: &ProviderInvocation,
    input: &RequestInput,
    input_digest: &str,
  ) -> Option<Value> {
    let record = self.authorized(context, &input.request_ref)?;
    if record.binding_id != invocation.binding_id
      || record.idempotency_key != invocation.idempotency_key
    {
      return None;
    }
    (record.state == "terminal" && record.input_digest == input_digest)
      .then(|| record.receipt.clone())
  }

  fn begin(
    &mut self,
    context: &ResolvedHostContext,
    invocation: &ProviderInvocation,
    input: &RequestInput,
    input_digest: &str,
  ) {
    self.by_request.insert(
      input.request_ref.clone(),
      SessionRecord {
        actor_ref: context.actor_ref.clone(),
        scope_ref: context.scope_ref.clone(),
        authority_epoch: context.authority_epoch,
        binding_id: invocation.binding_id.clone(),
        idempotency_key: invocation.idempotency_key.clone(),
        input_digest: input_digest.to_string(),
        state: "pending",
        cancel_requested: false,
        receipt: Value::Null,
        updated_at_ms: unix_ms(),
        expires_at_ms: unix_ms().saturating_add(SESSION_TTL_MS),
      },
    );
    if let Some(key) = &invocation.idempotency_key {
      self
        .by_idempotency
        .insert((context.actor_ref.clone(), key.clone()), input.request_ref.clone());
    }
    let per_actor = self
      .by_request
      .values()
      .filter(|record| record.actor_ref == context.actor_ref)
      .count();
    if per_actor > MAX_SESSIONS_PER_ACTOR {
      if let Some(evicted) = self
        .by_request
        .iter()
        .filter(|(_, record)| {
          record.actor_ref == context.actor_ref && record.state == "terminal"
        })
        .min_by_key(|(_, record)| record.updated_at_ms)
        .map(|(request_ref, _)| request_ref.clone())
      {
        self.by_request.remove(&evicted);
        self.by_idempotency.retain(|_, value| value != &evicted);
      }
    }
  }

  /// Terminal transition. A granted cancellation always wins, so a request that
  /// lost the race can never record a success receipt.
  fn complete(
    &mut self,
    context: &ResolvedHostContext,
    invocation: &ProviderInvocation,
    input: &RequestInput,
    receipt: Value,
    now: u64,
  ) -> Value {
    let cancelled = self
      .authorized(context, &input.request_ref)
      .is_some_and(|record| record.cancel_requested);
    let cancelled_receipt = failure_receipt(input, "cancelled", "CANCELLED", false);
    let (receipt, state) = if cancelled {
      (cancelled_receipt, "terminal")
    } else {
      (receipt, "terminal")
    };
    let record = self
      .by_request
      .entry(input.request_ref.clone())
      .or_insert_with(|| SessionRecord {
        actor_ref: context.actor_ref.clone(),
        scope_ref: context.scope_ref.clone(),
        authority_epoch: context.authority_epoch,
        binding_id: invocation.binding_id.clone(),
        idempotency_key: invocation.idempotency_key.clone(),
        input_digest: String::new(),
        state: "pending",
        cancel_requested: false,
        receipt: Value::Null,
        updated_at_ms: now,
        expires_at_ms: now.saturating_add(SESSION_TTL_MS),
      });
    record.state = state;
    record.receipt = receipt.clone();
    record.updated_at_ms = now;
    record.expires_at_ms = now.saturating_add(SESSION_TTL_MS);
    receipt
  }
}

fn is_same_authority(record: &SessionRecord, context: &ResolvedHostContext) -> bool {
  record.actor_ref == context.actor_ref
    && record.scope_ref == context.scope_ref
    && record.authority_epoch == context.authority_epoch
}

// ---------------------------------------------------------------------------
// Schema declaration
// ---------------------------------------------------------------------------

fn descriptor() -> ProviderDescriptor {
  ProviderDescriptor {
    descriptor_id: DESCRIPTOR_ID.into(),
    revision: DESCRIPTOR_REVISION.into(),
    family_id: RESOURCE_PRESENTATION_FAMILY.into(),
    contract_major: CONTRACT_MAJOR,
    contract_minor: CONTRACT_MINOR,
    operations: vec![
      OperationDescriptor {
        operation_id: RESOURCE_PRESENTATION_REQUEST.into(),
        effect: Effect::LocalWrite,
        input_schema: schema(PresentationSchemaKind::Request.value()),
        output_schema: schema(PresentationSchemaKind::Receipt.value()),
        cancellable: true,
        idempotency: Idempotency::Required,
      },
      OperationDescriptor {
        operation_id: RESOURCE_PRESENTATION_STATUS.into(),
        effect: Effect::Read,
        input_schema: schema(PresentationOperationSchema::StatusInput.value()),
        output_schema: schema(PresentationOperationSchema::StatusOutput.value()),
        cancellable: true,
        idempotency: Idempotency::None,
      },
      OperationDescriptor {
        operation_id: RESOURCE_PRESENTATION_CANCEL.into(),
        effect: Effect::LocalWrite,
        input_schema: schema(PresentationOperationSchema::StatusInput.value()),
        output_schema: schema(PresentationOperationSchema::CancelOutput.value()),
        cancellable: true,
        idempotency: Idempotency::Optional,
      },
    ],
    title: Some("AppFlowy Host resource presentation".into()),
    summary: Some(
      "Contract-major-2 muse.resource-presentation: Host-resolved open/focus with terminal receipts."
        .into(),
    ),
  }
}

fn schema(value: Result<Value, PresentationContractError>) -> SchemaDocument {
  SchemaDocument::new(value.expect("static presentation schema document must parse"))
    .expect("static presentation schema must digest")
}

// ---------------------------------------------------------------------------
// Helpers
// ---------------------------------------------------------------------------

/// Shared opaque-reference grammar of the presentation contract.
fn is_opaque_ref(value: &str) -> bool {
  !value.is_empty()
    && value.len() <= 128
    && value
      .bytes()
      .all(|byte| byte.is_ascii_alphanumeric() || matches!(byte, b'.' | b'_' | b'~' | b'-'))
}

fn string_field(value: &Value, field: &str) -> Result<String, ProviderFailure> {
  value
    .get(field)
    .and_then(Value::as_str)
    .map(str::to_owned)
    .ok_or(ProviderFailure)
}

fn check_active(cancellation: &Cancellation, deadline_at_ms: u64) -> Result<(), ProviderFailure> {
  if cancellation.is_cancelled() || deadline_at_ms <= unix_ms() {
    return Err(ProviderFailure);
  }
  Ok(())
}

fn unix_ms() -> u64 {
  SystemTime::now()
    .duration_since(UNIX_EPOCH)
    .unwrap_or_default()
    .as_millis()
    .min(u64::MAX as u128) as u64
}

#[cfg(test)]
mod tests {
  use super::*;
  use muse_presentation_contract::OPERATION_DIGEST_FIXTURE;
  use std::sync::Mutex as StdMutex;

  /// The surface dispatcher is process-global, so dispatcher tests must not run
  /// concurrently with each other.
  static DISPATCHER_TESTS: StdMutex<()> = StdMutex::new(());

  fn dispatcher_guard() -> std::sync::MutexGuard<'static, ()> {
    DISPATCHER_TESTS
      .lock()
      .unwrap_or_else(|poisoned| poisoned.into_inner())
  }

  struct TestCatalog {
    available: bool,
    views: Vec<PresentationView>,
  }

  #[async_trait]
  impl PresentationCatalog for TestCatalog {
    fn is_available(&self) -> bool {
      self.available
    }

    async fn workspace(&self) -> Result<(String, String, i64), ()> {
      Ok(("workspace.1".into(), "Test workspace".into(), 1_700_000_000_000))
    }

    async fn views(&self) -> Result<Vec<PresentationView>, ()> {
      Ok(self.views.clone())
    }
  }

  struct TestDispatcher {
    outcome: Option<MusePresentationOutcome>,
    seen: StdMutex<Vec<MusePresentationDispatch>>,
  }

  impl TestDispatcher {
    fn opened() -> Arc<Self> {
      Arc::new(Self {
        outcome: Some(MusePresentationOutcome {
          result: "opened".into(),
          surface_instance_ref: "surface.1".into(),
          selected_adapter_ref: ADAPTER_DOCUMENT.into(),
          effective_mode: "view".into(),
          revision: "revision.1700000000123".into(),
          warnings: vec![],
        }),
        seen: StdMutex::new(Vec::new()),
      })
    }

    fn seen(&self) -> Vec<MusePresentationDispatch> {
      self.seen.lock().expect("seen lock").clone()
    }

    /// A dispatcher that reports no revision of its own, so the Host's own
    /// revision marker is what reaches the receipt.
    fn without_revision() -> Arc<Self> {
      Arc::new(Self {
        outcome: Some(MusePresentationOutcome {
          result: "opened".into(),
          surface_instance_ref: "surface.1".into(),
          selected_adapter_ref: ADAPTER_DOCUMENT.into(),
          effective_mode: "view".into(),
          revision: String::new(),
          warnings: vec![],
        }),
        seen: StdMutex::new(Vec::new()),
      })
    }
  }

  impl MuseResourceSurfaceDispatcher for TestDispatcher {
    fn dispatch(&self, request: MusePresentationDispatch) -> Option<MusePresentationOutcome> {
      self.seen.lock().expect("seen lock").push(request);
      self.outcome.clone()
    }
  }

  fn document_view(id: &str, locked: bool) -> PresentationView {
    PresentationView {
      id: id.into(),
      title: "A document".into(),
      layout: "document",
      adapter_ref: ADAPTER_DOCUMENT,
      locked,
      last_edited_ms: 1_700_000_000_123,
    }
  }

  fn catalog(views: Vec<PresentationView>) -> Arc<dyn PresentationCatalog> {
    Arc::new(TestCatalog {
      available: true,
      views,
    })
  }

  fn context() -> ResolvedHostContext {
    ResolvedHostContext {
      actor_ref: "actor.1".into(),
      scope_ref: "scope.1".into(),
      authority_epoch: 7,
      evidence: std::collections::BTreeMap::from([(
        "appflowy.workspace".to_string(),
        "workspace.1".to_string(),
      )]),
    }
  }

  fn request_payload(request_ref: &str, resource_ref: &str, mode: &str) -> Value {
    json!({
      "protocol": "muse.presentation/request/v2",
      "requestRef": request_ref,
      "resourceRef": resource_ref,
      "disposition": "open",
      "requestedMode": mode,
      "placementHint": "current-window",
      "cause": {"kind": "agent-tool"},
      "requestedAt": 1_700_000_000_000u64
    })
  }

  fn invocation(operation_id: &str, input: Value) -> ProviderInvocation {
    ProviderInvocation {
      binding_id: "binding.1".into(),
      operation_id: operation_id.into(),
      input,
      idempotency_key: Some("request.1".into()),
      deadline_at_ms: unix_ms() + 60_000,
    }
  }

  fn descriptor_operations() -> Vec<OperationDescriptor> {
    descriptor().operations
  }

  /// Requirement 5(a): the descriptor declares exactly the digests the DSH wire
  /// publishes, so `descriptorCompatible()` accepts this Host.
  #[test]
  fn descriptor_declares_the_published_schema_digests() {
    let fixture: BTreeMap<String, String> =
      serde_json::from_str(OPERATION_DIGEST_FIXTURE).expect("digest fixture must parse");
    assert_eq!(fixture.len(), 6);
    let descriptor = descriptor();
    assert_eq!(descriptor.family_id, RESOURCE_PRESENTATION_FAMILY);
    assert_eq!(descriptor.contract_major, 2);
    assert_eq!(descriptor.operations.len(), 3);
    for operation in &descriptor.operations {
      let input = fixture
        .get(&format!("{}.input", operation.operation_id))
        .expect("fixture must pin the input digest");
      let output = fixture
        .get(&format!("{}.output", operation.operation_id))
        .expect("fixture must pin the output digest");
      assert_eq!(&operation.input_schema.digest, input);
      assert_eq!(&operation.output_schema.digest, output);
      assert!(operation.cancellable);
    }
    let request = descriptor
      .operations
      .iter()
      .find(|operation| operation.operation_id == RESOURCE_PRESENTATION_REQUEST)
      .expect("request operation");
    assert_eq!(request.effect, Effect::LocalWrite);
    assert_eq!(request.idempotency, Idempotency::Required);
    let status = descriptor
      .operations
      .iter()
      .find(|operation| operation.operation_id == RESOURCE_PRESENTATION_STATUS)
      .expect("status operation");
    assert_eq!(status.effect, Effect::Read);
    let cancel = descriptor
      .operations
      .iter()
      .find(|operation| operation.operation_id == RESOURCE_PRESENTATION_CANCEL)
      .expect("cancel operation");
    assert_eq!(cancel.effect, Effect::LocalWrite);
    // `status` and `cancel` share one input document by contract.
    assert_eq!(status.input_schema, cancel.input_schema);
  }

  /// The declared receipt schema is the canonical v2 document.
  #[test]
  fn declared_receipt_schema_is_the_canonical_v2_receipt() {
    let operation = descriptor_operations()
      .into_iter()
      .find(|operation| operation.operation_id == RESOURCE_PRESENTATION_REQUEST)
      .expect("request operation");
    assert_eq!(
      operation.output_schema.value,
      PresentationSchemaKind::Receipt
        .value()
        .expect("canonical receipt schema must parse")
    );
  }

  #[test]
  fn test_request_payloads_satisfy_the_v2_request_schema() {
    for mode in ["view", "prefer-edit", "edit"] {
      let payload = request_payload("request.1", "view.1", mode);
      validate(PresentationSchemaKind::Request, &payload).expect("fixture request must be valid");
    }
  }

  /// Requirement 5(b): a resolvable request returns a schema-valid v2 receipt and
  /// hands the resolved Host facts to the surface dispatcher.
  #[tokio::test]
  async fn request_opens_a_view_and_returns_a_contract_valid_receipt() {
    let _guard = dispatcher_guard();
    clear_muse_resource_surface_dispatcher();
    let dispatcher = TestDispatcher::opened();
    install_muse_resource_surface_dispatcher(dispatcher.clone());
    let provider = PresentationProvider::with_catalog(catalog(vec![document_view("view.1", false)]));
    let receipt = provider
      .invoke(
        invocation(
          RESOURCE_PRESENTATION_REQUEST,
          request_payload("request.1", "view.1", "view"),
        ),
        context(),
        Cancellation::default(),
      )
      .await
      .expect("request must succeed");
    validate(PresentationSchemaKind::Receipt, &receipt).expect("receipt must satisfy v2");
    assert_eq!(receipt["result"], json!("opened"));
    assert_eq!(receipt["requestRef"], json!("request.1"));
    assert_eq!(receipt["resourceRef"], json!("view.1"));
    assert_eq!(receipt["surfaceInstanceRef"], json!("surface.1"));
    assert_eq!(receipt["effectiveMode"], json!("view"));
    assert_eq!(receipt["revision"], json!("revision.1700000000123"));
    let seen = dispatcher.seen();
    assert_eq!(seen.len(), 1);
    assert_eq!(seen[0].workspace_id, "workspace.1");
    assert_eq!(seen[0].view_id.as_deref(), Some("view.1"));
    assert_eq!(seen[0].layout.as_deref(), Some("document"));
    assert_eq!(seen[0].adapter_ref, ADAPTER_DOCUMENT);
    assert!(seen[0].resolved);
    assert_eq!(seen[0].session_ref, "scope.1");
    clear_muse_resource_surface_dispatcher();
  }

  /// Without a Host surface dispatcher the provider fails closed instead of
  /// claiming an open it cannot observe.
  #[tokio::test]
  async fn request_fails_closed_without_a_surface_dispatcher() {
    let _guard = dispatcher_guard();
    clear_muse_resource_surface_dispatcher();
    let provider = PresentationProvider::with_catalog(catalog(vec![document_view("view.1", false)]));
    let receipt = provider
      .invoke(
        invocation(
          RESOURCE_PRESENTATION_REQUEST,
          request_payload("request.1", "view.1", "view"),
        ),
        context(),
        Cancellation::default(),
      )
      .await
      .expect("request must produce a receipt");
    validate(PresentationSchemaKind::Receipt, &receipt).expect("receipt must satisfy v2");
    assert_eq!(receipt["result"], json!("unsupported"));
    assert_eq!(receipt["errorCode"], json!("SURFACE_UNAVAILABLE"));
    assert_eq!(receipt["retryable"], json!(true));
  }

  #[tokio::test]
  async fn a_path_shaped_ref_is_denied_before_any_surface_dispatch() {
    let _guard = dispatcher_guard();
    clear_muse_resource_surface_dispatcher();
    let dispatcher = TestDispatcher::opened();
    install_muse_resource_surface_dispatcher(dispatcher.clone());
    let provider = PresentationProvider::with_catalog(catalog(vec![document_view("view.1", false)]));
    let receipt = provider
      .invoke(
        invocation(
          RESOURCE_PRESENTATION_REQUEST,
          request_payload("request.1", "C:/Users/example/secret.docx", "view"),
        ),
        context(),
        Cancellation::default(),
      )
      .await
      .expect("request must produce a receipt");
    validate(PresentationSchemaKind::Receipt, &receipt).expect("receipt must satisfy v2");
    assert_eq!(receipt["result"], json!("denied"));
    assert_eq!(receipt["errorCode"], json!("RESOURCE_REF_INVALID"));
    assert!(dispatcher.seen().is_empty());
    clear_muse_resource_surface_dispatcher();
  }

  #[tokio::test]
  async fn an_unresolvable_opaque_ref_is_offered_to_the_dispatcher_unresolved() {
    let _guard = dispatcher_guard();
    clear_muse_resource_surface_dispatcher();
    let dispatcher = TestDispatcher::opened();
    install_muse_resource_surface_dispatcher(dispatcher.clone());
    let provider = PresentationProvider::with_catalog(catalog(vec![document_view("view.1", false)]));
    let receipt = provider
      .invoke(
        invocation(
          RESOURCE_PRESENTATION_REQUEST,
          request_payload("request.1", "mount.6f2a1c", "view"),
        ),
        context(),
        Cancellation::default(),
      )
      .await
      .expect("request must produce a receipt");
    validate(PresentationSchemaKind::Receipt, &receipt).expect("receipt must satisfy v2");
    let seen = dispatcher.seen();
    assert_eq!(seen.len(), 1);
    assert!(!seen[0].resolved);
    assert_eq!(seen[0].view_id, None);
    assert_eq!(seen[0].adapter_ref, ADAPTER_UNRESOLVED);
    clear_muse_resource_surface_dispatcher();

    // With no dispatcher the same unresolved ref is an honest not-found.
    let receipt = provider
      .invoke(
        invocation(
          RESOURCE_PRESENTATION_REQUEST,
          request_payload("request.2", "mount.6f2a1c", "view"),
        ),
        context(),
        Cancellation::default(),
      )
      .await
      .expect("request must produce a receipt");
    assert_eq!(receipt["result"], json!("unsupported"));
    assert_eq!(receipt["errorCode"], json!("RESOURCE_NOT_FOUND"));
  }

  #[tokio::test]
  async fn a_request_bound_to_another_workspace_is_denied() {
    let _guard = dispatcher_guard();
    clear_muse_resource_surface_dispatcher();
    let provider = PresentationProvider::with_catalog(catalog(vec![document_view("view.1", false)]));
    let mut bound = context();
    bound.evidence.insert(
      "appflowy.workspace".to_string(),
      "workspace.other".to_string(),
    );
    let receipt = provider
      .invoke(
        invocation(
          RESOURCE_PRESENTATION_REQUEST,
          request_payload("request.1", "view.1", "view"),
        ),
        bound,
        Cancellation::default(),
      )
      .await
      .expect("request must produce a receipt");
    assert_eq!(receipt["result"], json!("denied"));
    assert_eq!(receipt["errorCode"], json!("MOUNT_NOT_BOUND"));
  }

  #[tokio::test]
  async fn edit_on_a_locked_view_is_denied() {
    let _guard = dispatcher_guard();
    clear_muse_resource_surface_dispatcher();
    let dispatcher = TestDispatcher::opened();
    install_muse_resource_surface_dispatcher(dispatcher.clone());
    let provider = PresentationProvider::with_catalog(catalog(vec![document_view("view.1", true)]));
    let receipt = provider
      .invoke(
        invocation(
          RESOURCE_PRESENTATION_REQUEST,
          request_payload("request.1", "view.1", "edit"),
        ),
        context(),
        Cancellation::default(),
      )
      .await
      .expect("request must produce a receipt");
    assert_eq!(receipt["result"], json!("denied"));
    assert_eq!(receipt["errorCode"], json!("VIEW_LOCKED"));
    assert!(dispatcher.seen().is_empty());
    clear_muse_resource_surface_dispatcher();
  }

  /// A retried request with the same identity and input replays one receipt.
  #[tokio::test]
  async fn an_identical_retry_replays_the_recorded_receipt() {
    let _guard = dispatcher_guard();
    clear_muse_resource_surface_dispatcher();
    let dispatcher = TestDispatcher::opened();
    install_muse_resource_surface_dispatcher(dispatcher.clone());
    let provider = PresentationProvider::with_catalog(catalog(vec![document_view("view.1", false)]));
    let payload = request_payload("request.1", "view.1", "view");
    let first = provider
      .invoke(
        invocation(RESOURCE_PRESENTATION_REQUEST, payload.clone()),
        context(),
        Cancellation::default(),
      )
      .await
      .expect("first request");
    let second = provider
      .invoke(
        invocation(RESOURCE_PRESENTATION_REQUEST, payload),
        context(),
        Cancellation::default(),
      )
      .await
      .expect("retry");
    assert_eq!(first, second);
    assert_eq!(dispatcher.seen().len(), 1);
    clear_muse_resource_surface_dispatcher();
  }

  #[tokio::test]
  async fn status_and_cancel_report_honest_terminal_state() {
    let _guard = dispatcher_guard();
    clear_muse_resource_surface_dispatcher();
    let dispatcher = TestDispatcher::opened();
    install_muse_resource_surface_dispatcher(dispatcher);
    let provider = PresentationProvider::with_catalog(catalog(vec![document_view("view.1", false)]));
    provider
      .invoke(
        invocation(
          RESOURCE_PRESENTATION_REQUEST,
          request_payload("request.1", "view.1", "view"),
        ),
        context(),
        Cancellation::default(),
      )
      .await
      .expect("request");

    let status = provider
      .invoke(
        invocation(RESOURCE_PRESENTATION_STATUS, json!({"requestRef": "request.1"})),
        context(),
        Cancellation::default(),
      )
      .await
      .expect("status");
    validate_operation(PresentationOperationSchema::StatusOutput, &status).expect("status output");
    assert_eq!(status["state"], json!("terminal"));
    assert_eq!(status["receipt"]["requestRef"], json!("request.1"));

    let cancel = provider
      .invoke(
        invocation(RESOURCE_PRESENTATION_CANCEL, json!({"requestRef": "request.1"})),
        context(),
        Cancellation::default(),
      )
      .await
      .expect("cancel");
    validate_operation(PresentationOperationSchema::CancelOutput, &cancel).expect("cancel output");
    assert_eq!(cancel["disposition"], json!("already_terminal"));
    assert_eq!(cancel["receipt"]["requestRef"], json!("request.1"));

    let unknown = provider
      .invoke(
        invocation(RESOURCE_PRESENTATION_STATUS, json!({"requestRef": "request.404"})),
        context(),
        Cancellation::default(),
      )
      .await
      .expect("status");
    assert_eq!(unknown["state"], json!("unknown"));
    assert!(unknown.get("receipt").is_none());

    let missing = provider
      .invoke(
        invocation(RESOURCE_PRESENTATION_CANCEL, json!({"requestRef": "request.404"})),
        context(),
        Cancellation::default(),
      )
      .await
      .expect("cancel");
    assert_eq!(missing["disposition"], json!("not_found"));
    clear_muse_resource_surface_dispatcher();
  }

  /// Status never leaks another actor's command.
  #[tokio::test]
  async fn status_does_not_leak_another_actors_request() {
    let _guard = dispatcher_guard();
    clear_muse_resource_surface_dispatcher();
    let dispatcher = TestDispatcher::opened();
    install_muse_resource_surface_dispatcher(dispatcher);
    let provider = PresentationProvider::with_catalog(catalog(vec![document_view("view.1", false)]));
    provider
      .invoke(
        invocation(
          RESOURCE_PRESENTATION_REQUEST,
          request_payload("request.1", "view.1", "view"),
        ),
        context(),
        Cancellation::default(),
      )
      .await
      .expect("request");
    let mut other = context();
    other.actor_ref = "actor.2".into();
    let status = provider
      .invoke(
        invocation(RESOURCE_PRESENTATION_STATUS, json!({"requestRef": "request.1"})),
        other,
        Cancellation::default(),
      )
      .await
      .expect("status");
    assert_eq!(status["state"], json!("unknown"));
    clear_muse_resource_surface_dispatcher();
  }

  /// A granted `cancel` always wins, even if the request is about to be recorded.
  #[tokio::test]
  async fn a_cancelled_request_never_records_success() {
    let provider = PresentationProvider::with_catalog(catalog(vec![document_view("view.1", false)]));
    let context = context();
    let invocation = invocation(
      RESOURCE_PRESENTATION_REQUEST,
      request_payload("request.1", "view.1", "view"),
    );
    let input = RequestInput::parse(&invocation.input).expect("input");
    {
      let mut sessions = provider.state.sessions.lock().await;
      sessions.begin(&context, &invocation, &input, "digest.1");
      assert!(
        matches!(
          sessions.cancel_request(&context, "request.1", unix_ms()),
          CancelOutcome::Accepted
        ),
        "a pending command accepts cancellation"
      );
      assert!(matches!(
        sessions.cancel_request(&context, "request.1", unix_ms()),
        CancelOutcome::Accepted
      ));
    }
    let receipt = provider
      .record_terminal(
        &context,
        &invocation,
        &input,
        success_receipt(
          &input,
          "scope.1",
          "revision.1",
          MusePresentationOutcome {
            result: "opened".into(),
            surface_instance_ref: "surface.1".into(),
            selected_adapter_ref: ADAPTER_DOCUMENT.into(),
            effective_mode: "view".into(),
            revision: "revision.1".into(),
            warnings: vec![],
          },
          vec![],
        )
        .expect("success receipt"),
      )
      .await
      .expect("terminal receipt");
    validate(PresentationSchemaKind::Receipt, &receipt).expect("receipt must satisfy v2");
    assert_eq!(receipt["result"], json!("cancelled"));
    assert_eq!(receipt["errorCode"], json!("CANCELLED"));
  }

  #[tokio::test]
  async fn an_unavailable_catalog_is_not_available() {
    let provider = PresentationProvider::with_catalog(Arc::new(TestCatalog {
      available: false,
      views: vec![],
    }));
    assert!(!provider
      .available(&context())
      .await
      .expect("availability"));
  }

  /// A dispatcher outcome that cannot be projected fails closed.
  #[test]
  fn an_unprojectable_dispatcher_outcome_fails_closed() {
    let input = RequestInput::parse(&request_payload("request.1", "view.1", "view")).expect("input");
    assert!(success_receipt(
      &input,
      "scope.1",
      "revision.1",
      MusePresentationOutcome {
        result: "opened".into(),
        surface_instance_ref: "../etc/passwd".into(),
        selected_adapter_ref: ADAPTER_DOCUMENT.into(),
        effective_mode: "view".into(),
        revision: "revision.1".into(),
        warnings: vec![],
      },
      vec![],
    )
    .is_none());
    assert!(success_receipt(
      &input,
      "scope.1",
      "revision.1",
      MusePresentationOutcome {
        result: "maybe".into(),
        surface_instance_ref: "surface.1".into(),
        selected_adapter_ref: ADAPTER_DOCUMENT.into(),
        effective_mode: "view".into(),
        revision: "revision.1".into(),
        warnings: vec![],
      },
      vec![],
    )
    .is_none());
  }

  /// The Flutter bridge payload is a closed projection: it can only carry what
  /// the Host already resolved, and an unprojectable reply fails closed.
  #[test]
  fn the_flutter_bridge_payload_round_trips() {
    let dispatch = MusePresentationDispatch {
      request_ref: "request.1".into(),
      resource_ref: "view.1".into(),
      workspace_id: "workspace.1".into(),
      view_id: Some("view.1".into()),
      layout: Some("document".into()),
      title: Some("A document".into()),
      disposition: "open".into(),
      requested_mode: "view".into(),
      placement_hint: "current-window".into(),
      adapter_ref: ADAPTER_DOCUMENT.into(),
      effective_mode: "view".into(),
      session_ref: "scope.1".into(),
      revision: "revision.1".into(),
      cause_kind: "agent-tool".into(),
      anchor_hint: None,
      mount_ref: None,
      relative_path: None,
      resolved: true,
    };
    let wire = dispatch.to_wire();
    assert_eq!(wire["protocol"], json!(MUSE_PRESENTATION_DISPATCH_PROTOCOL));
    assert_eq!(wire["resourceRef"], json!("view.1"));
    assert_eq!(wire["viewId"], json!("view.1"));
    assert!(wire.get("anchorHint").is_none());
    for value in wire.as_object().expect("object").values() {
      if let Some(text) = value.as_str() {
        assert!(
          !text.contains('\\') && !text.contains(":/"),
          "the bridge payload may only carry opaque refs: {}",
          text
        );
      }
    }

    let outcome = MusePresentationOutcome::from_wire(&json!({
      "result": "opened",
      "surfaceInstanceRef": "surface.1",
      "selectedAdapterRef": ADAPTER_DOCUMENT,
      "effectiveMode": "view",
      "revision": "revision.1",
      "warnings": ["surface.dispatch-unconfirmed"]
    }))
    .expect("a complete reply projects");
    assert_eq!(outcome.result, "opened");
    assert_eq!(outcome.warnings, vec!["surface.dispatch-unconfirmed"]);

    assert!(MusePresentationOutcome::from_wire(&json!({"result": "opened"})).is_none());
    assert!(MusePresentationOutcome::from_wire(&json!({
      "result": "opened",
      "surfaceInstanceRef": "surface.1",
      "selectedAdapterRef": ADAPTER_DOCUMENT,
      "effectiveMode": "view",
      "revision": "revision.1",
      "warnings": "not-an-array"
    }))
    .is_none());
  }

  #[test]
  fn warnings_are_bounded() {
    let input = RequestInput::parse(&request_payload("request.1", "view.1", "view")).expect("input");
    let receipt = success_receipt(
      &input,
      "scope.1",
      "revision.1",
      MusePresentationOutcome {
        result: "opened".into(),
        surface_instance_ref: "surface.1".into(),
        selected_adapter_ref: ADAPTER_DOCUMENT.into(),
        effective_mode: "view".into(),
        revision: "revision.1".into(),
        warnings: (0..64).map(|index| format!("warning.{index}")).collect(),
      },
      vec!["x".repeat(MAX_WARNING_CHARS + 1), "kept".to_string()],
    )
    .expect("success receipt");
    validate(PresentationSchemaKind::Receipt, &receipt).expect("receipt must satisfy v2");
    let warnings = receipt["warnings"].as_array().expect("warnings array");
    assert!(warnings.len() <= MAX_WARNINGS);
    assert!(warnings.iter().any(|value| value == "kept"));
    assert!(!warnings.iter().any(|value| value.as_str() == Some("kept-by-oversize")));
  }

  // -------------------------------------------------------------------------
  // Resource locator
  // -------------------------------------------------------------------------

  fn locator_input(mount_ref: &str, relative_path: &str) -> Value {
    json!({"mountRef": mount_ref, "relativePath": relative_path, "disposition": "view"})
  }

  /// The anchor hint the DSH mirrors onto a presentation request for a Mount
  /// entry (see `mountRelativeAnchor` in `@muse/plugin-appflowy-workspace`).
  fn anchor_hint(mount_ref: &str, path: &str) -> Value {
    json!({
      "provider": MOUNT_RELATIVE_ANCHOR_PROVIDER,
      "value": {"mountRef": mount_ref, "path": path}
    })
  }

  fn locator_provider(catalog: Arc<dyn PresentationCatalog>) -> LocatorProvider {
    LocatorProvider::with_state(PresentationState::new(catalog))
  }

  async fn locate(provider: &LocatorProvider, input: Value) -> Result<Value, ProviderFailure> {
    provider
      .invoke(
        invocation(RESOURCE_LOCATOR_RESOLVE, input),
        context(),
        Cancellation::default(),
      )
      .await
  }

  /// The locator mints an opaque, deterministic ref, and it never mints one for
  /// a device path or a traversal — an idempotency key is optional for a read.
  #[tokio::test]
  async fn locator_mints_a_stable_opaque_ref_and_refuses_device_paths() {
    let provider = locator_provider(catalog(vec![]));
    let first = locate(&provider, locator_input("mount.6f2a1c", "notes/roadmap.md"))
      .await
      .expect("a mount-relative path must locate");
    let resource_ref = first["resourceRef"].as_str().expect("ref").to_string();
    assert!(
      is_opaque_ref(&resource_ref),
      "ref must be opaque: {}",
      resource_ref
    );
    assert_eq!(first["displayName"], json!("roadmap.md"));

    let second = locate(&provider, locator_input("mount.6f2a1c", "notes/roadmap.md"))
      .await
      .expect("re-locating the same entry must succeed");
    assert_eq!(
      second["resourceRef"], first["resourceRef"],
      "a ref must be stable within one Host authority generation"
    );

    let other_mount = locate(&provider, locator_input("mount.other", "notes/roadmap.md"))
      .await
      .expect("another Mount locates");
    assert_ne!(other_mount["resourceRef"], first["resourceRef"]);

    let root = locate(&provider, locator_input("mount.6f2a1c", ""))
      .await
      .expect("the Mount root locates");
    assert_eq!(root["displayName"], json!("mount.6f2a1c"));

    // A different actor in the same workspace must not be able to name the same
    // resource by guessing the locator.
    let stranger = provider
      .invoke(
        invocation(RESOURCE_LOCATOR_RESOLVE, locator_input("mount.6f2a1c", "notes/roadmap.md")),
        ResolvedHostContext {
          actor_ref: "actor.2".into(),
          ..context()
        },
        Cancellation::default(),
      )
      .await
      .expect("another actor may locate for itself");
    assert_ne!(stranger["resourceRef"], first["resourceRef"]);

    for relative_path in [
      "/etc/passwd",
      "C:/Users/example/secret.docx",
      "c:/Users/example/secret.docx",
      "..",
      "notes/../../secret.md",
      "notes/./roadmap.md",
      "notes//roadmap.md",
      "notes/roadmap.md/",
      "\\\\server\\share\\file.md",
      "notes\u{0}/roadmap.md",
    ] {
      assert!(
        locate(&provider, locator_input("mount.6f2a1c", relative_path))
          .await
          .is_err(),
        "a device or traversal path must be refused: {:?}",
        relative_path
      );
    }
    assert!(locate(&provider, locator_input("", "notes/roadmap.md"))
      .await
      .is_err());
  }

  /// Defence in depth around the shared table: a ref that cannot be recorded is
  /// never answered, so a locate can never hand out a ref that cannot resolve.
  #[tokio::test]
  async fn the_locator_quota_refuses_to_mint_an_unresolvable_ref() {
    let provider = locator_provider(catalog(vec![]));
    let now = unix_ms();
    {
      let mut locators = provider.state.locators.lock().await;
      for index in 0..MAX_LOCATOR_ENTRIES_PER_ACTOR {
        let entry = locator_entry(
          &context(),
          "binding.1",
          "mount.6f2a1c",
          &format!("notes/{index}.md"),
          "view",
          format!("resource.{index:024x}"),
          now,
        );
        assert!(locators.insert(entry, now));
      }
    }
    assert!(locate(&provider, locator_input("mount.6f2a1c", "one-more.md"))
      .await
      .is_err());
  }

  /// Requirement (b)+(c): the ref the locator mints is the ref a later
  /// presentation request resolves, and the dispatch carries the Mount locator
  /// instead of any device path.
  #[tokio::test]
  async fn a_minted_ref_presents_through_the_shared_host_state() {
    let _guard = dispatcher_guard();
    clear_muse_resource_surface_dispatcher();
    let dispatcher = TestDispatcher::without_revision();
    install_muse_resource_surface_dispatcher(dispatcher.clone());

    let state = PresentationState::new(catalog(vec![document_view("view.1", false)]));
    let locator = LocatorProvider::with_state(state.clone());
    let provider = PresentationProvider::with_state(state);
    let located = locate(&locator, locator_input("mount.6f2a1c", "notes/roadmap.md"))
      .await
      .expect("locate");
    let resource_ref = located["resourceRef"].as_str().unwrap().to_string();

    let receipt = provider
      .invoke(
        invocation(
          RESOURCE_PRESENTATION_REQUEST,
          request_payload("request.1", &resource_ref, "view"),
        ),
        context(),
        Cancellation::default(),
      )
      .await
      .expect("the minted ref must present");
    validate(PresentationSchemaKind::Receipt, &receipt).expect("receipt must satisfy v2");
    assert_eq!(receipt["result"], json!("opened"));
    assert_eq!(receipt["resourceRef"], json!(resource_ref));
    assert_eq!(
      receipt["revision"],
      json!(resource_revision(&resource_ref)),
      "the locator's revision marker is the Host revision for a plain Mount entry"
    );

    let seen = dispatcher.seen();
    assert_eq!(seen.len(), 1);
    assert!(seen[0].resolved);
    assert_eq!(seen[0].mount_ref.as_deref(), Some("mount.6f2a1c"));
    assert_eq!(seen[0].relative_path.as_deref(), Some("notes/roadmap.md"));
    assert_eq!(seen[0].adapter_ref, ADAPTER_UNRESOLVED);
    assert_eq!(seen[0].layout.as_deref(), Some("resource"));
    assert_eq!(seen[0].title.as_deref(), Some("roadmap.md"));
    let wire = seen[0].to_wire();
    assert_eq!(wire["mountRef"], json!("mount.6f2a1c"));
    assert_eq!(wire["relativePath"], json!("notes/roadmap.md"));
    clear_muse_resource_surface_dispatcher();
  }

  /// A ref stays resolvable after its locator entry is gone, but only when the
  /// Mount-relative anchor proves it is the ref this Host would mint, and only
  /// for the authority that minted it.
  #[tokio::test]
  async fn a_minted_ref_rehydrates_from_the_anchor_but_never_crosses_authority() {
    let _guard = dispatcher_guard();
    clear_muse_resource_surface_dispatcher();
    let dispatcher = TestDispatcher::opened();
    install_muse_resource_surface_dispatcher(dispatcher.clone());

    let state = PresentationState::new(catalog(vec![]));
    let locator = LocatorProvider::with_state(state.clone());
    let provider = PresentationProvider::with_state(state.clone());
    let located = locate(&locator, locator_input("mount.6f2a1c", "notes/roadmap.md"))
      .await
      .expect("locate");
    let resource_ref = located["resourceRef"].as_str().unwrap().to_string();

    // Simulate an expired entry: the durable proof is the anchor hint the DSH
    // carries, so the request still resolves.
    state.locators.lock().await.by_ref.clear();
    let mut payload = request_payload("request.1", &resource_ref, "view");
    payload["anchorHint"] = anchor_hint("mount.6f2a1c", "notes/roadmap.md");
    let receipt = provider
      .invoke(
        invocation(RESOURCE_PRESENTATION_REQUEST, payload),
        context(),
        Cancellation::default(),
      )
      .await
      .expect("the anchor proves the ref");
    assert_eq!(receipt["result"], json!("opened"));
    assert_eq!(dispatcher.seen()[0].mount_ref.as_deref(), Some("mount.6f2a1c"));

    // A mismatched anchor cannot resurrect a ref: the Host hands the unresolved
    // ref to the seam, it does not resolve it as a Mount entry.
    state.locators.lock().await.by_ref.clear();
    let mut forged = request_payload("request.2", &resource_ref, "view");
    forged["anchorHint"] = anchor_hint("mount.6f2a1c", "notes/other.md");
    let receipt = provider
      .invoke(
        invocation(RESOURCE_PRESENTATION_REQUEST, forged),
        context(),
        Cancellation::default(),
      )
      .await
      .expect("a receipt is still returned");
    validate(PresentationSchemaKind::Receipt, &receipt).expect("receipt must satisfy v2");
    let forged_dispatch = dispatcher.seen().pop().expect("a dispatch happened");
    assert!(
      !forged_dispatch.resolved,
      "the Host must not resolve a locator it never minted for this ref"
    );
    assert!(forged_dispatch.mount_ref.is_none());
    assert!(forged_dispatch.relative_path.is_none());
    assert_eq!(
      forged_dispatch.revision, "revision.1700000000000",
      "an unresolved ref carries the workspace revision, not a locator revision"
    );

    // Another actor cannot present the ref even with the matching anchor.
    state.locators.lock().await.by_ref.clear();
    let mut payload = request_payload("request.3", &resource_ref, "view");
    payload["anchorHint"] = anchor_hint("mount.6f2a1c", "notes/roadmap.md");
    let receipt = provider
      .invoke(
        invocation(RESOURCE_PRESENTATION_REQUEST, payload),
        ResolvedHostContext {
          actor_ref: "actor.2".into(),
          ..context()
        },
        Cancellation::default(),
      )
      .await
      .expect("a receipt is still returned");
    validate(PresentationSchemaKind::Receipt, &receipt).expect("receipt must satisfy v2");
    assert!(
      !dispatcher.seen().pop().expect("a dispatch happened").resolved,
      "a ref must never cross actors"
    );
    assert!(
      state.locators.lock().await.by_ref.is_empty(),
      "a refused resolution must write no locator state"
    );
    clear_muse_resource_surface_dispatcher();
  }

  /// Requirement (a): both families are resolvable through one registry, the
  /// locator's declared schema refuses a device path with `InvalidInput`, and the
  /// registry itself accepts the minted ref and the receipt.
  #[tokio::test]
  async fn the_registry_serves_both_families() {
    use muse_host_registry::{
      AuthoritativeCaller, AuthorityResolver, AuthorityError, DiscoverQuery,
      HostCapabilityRegistry, InvocationAdmission, InvocationAuthorizer, InvokeRequest,
      RegistryConfig, RegistryError, ScopeHint,
    };

    struct TestAuthority;
    #[async_trait]
    impl AuthorityResolver for TestAuthority {
      async fn resolve(
        &self,
        _caller: &AuthoritativeCaller,
        _hint: Option<&ScopeHint>,
      ) -> Result<ResolvedHostContext, AuthorityError> {
        Ok(context())
      }

      async fn revalidate(
        &self,
        _caller: &AuthoritativeCaller,
        previous: &ResolvedHostContext,
      ) -> Result<ResolvedHostContext, AuthorityError> {
        Ok(previous.clone())
      }
    }

    struct AllowAll;
    impl std::fmt::Debug for AllowAll {
      fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter.write_str("AllowAll")
      }
    }

    #[async_trait]
    impl InvocationAuthorizer for AllowAll {
      async fn authorize(&self, _admission: InvocationAdmission<'_>) -> Result<(), RegistryError> {
        Ok(())
      }
    }

    let _guard = dispatcher_guard();
    clear_muse_resource_surface_dispatcher();
    let dispatcher = TestDispatcher::opened();
    install_muse_resource_surface_dispatcher(dispatcher.clone());

    let registry = Arc::new(HostCapabilityRegistry::new(
      Arc::new(TestAuthority),
      RegistryConfig {
        host_generation: "appflowy.local.1".to_string(),
        invocation_authorizer: Arc::new(AllowAll),
        ..RegistryConfig::default()
      },
    ));
    let state = PresentationState::new(catalog(vec![]));
    let locator_lease = registry
      .register(Arc::new(LocatorProvider::with_state(state.clone())))
      .await
      .expect("locator registration");
    let presentation_lease = registry
      .register(Arc::new(PresentationProvider::with_state(state)))
      .await
      .expect("presentation registration");
    let caller = AuthoritativeCaller {
      actor_ref: "actor.1".into(),
    };

    let page = registry
      .discover(
        &caller,
        DiscoverQuery {
          families: vec![RESOURCE_LOCATOR_FAMILY.to_string()].into_iter().collect(),
          scope_hint: None,
          page_size: 16,
          cursor: None,
        },
      )
      .await
      .expect("discover the locator family");
    assert_eq!(page.descriptors.len(), 1);
    let locator_descriptor = &page.descriptors[0].descriptor;
    assert_eq!(locator_descriptor.contract_major, 1);
    assert_eq!(
      locator_descriptor
        .operation(RESOURCE_LOCATOR_RESOLVE)
        .expect("locator operation")
        .effect,
      Effect::Read
    );

    let locator_binding = registry
      .bind(
        &caller,
        LOCATOR_DESCRIPTOR_ID,
        LOCATOR_DESCRIPTOR_REVISION,
        vec![RESOURCE_LOCATOR_RESOLVE.to_string()].into_iter().collect(),
        None,
      )
      .await
      .expect("bind the locator");

    let denied = registry
      .invoke(
        &caller,
        InvokeRequest {
          binding_id: locator_binding.binding_id.clone(),
          operation_id: RESOURCE_LOCATOR_RESOLVE.into(),
          input: locator_input("mount.6f2a1c", "C:/Users/example/secret.docx"),
          deadline_at_ms: unix_ms() + 30_000,
          cancellation: Cancellation::default(),
          grant_id: None,
          idempotency_key: None,
          session_ref: Some("session.1".into()),
          tool_call_ref: Some("tool.1".into()),
        },
      )
      .await;
    assert_eq!(denied, Err(RegistryError::InvalidInput));

    let located = registry
      .invoke(
        &caller,
        InvokeRequest {
          binding_id: locator_binding.binding_id.clone(),
          operation_id: RESOURCE_LOCATOR_RESOLVE.into(),
          input: locator_input("mount.6f2a1c", "notes/roadmap.md"),
          deadline_at_ms: unix_ms() + 30_000,
          cancellation: Cancellation::default(),
          grant_id: None,
          idempotency_key: None,
          session_ref: Some("session.1".into()),
          tool_call_ref: Some("tool.1".into()),
        },
      )
      .await
      .expect("the registry must accept the locator input");
    let resource_ref = located["resourceRef"].as_str().unwrap().to_string();

    let presentation_binding = registry
      .bind(
        &caller,
        DESCRIPTOR_ID,
        DESCRIPTOR_REVISION,
        vec![
          RESOURCE_PRESENTATION_REQUEST.to_string(),
          RESOURCE_PRESENTATION_STATUS.to_string(),
          RESOURCE_PRESENTATION_CANCEL.to_string(),
        ]
        .into_iter()
        .collect(),
        None,
      )
      .await
      .expect("bind presentation");
    let receipt = registry
      .invoke(
        &caller,
        InvokeRequest {
          binding_id: presentation_binding.binding_id.clone(),
          operation_id: RESOURCE_PRESENTATION_REQUEST.into(),
          input: request_payload("request.1", &resource_ref, "view"),
          deadline_at_ms: unix_ms() + 30_000,
          cancellation: Cancellation::default(),
          grant_id: None,
          idempotency_key: Some("request.1".into()),
          session_ref: Some("session.1".into()),
          tool_call_ref: Some("tool.1".into()),
        },
      )
      .await
      .expect("the registry must accept the minted ref");
    validate(PresentationSchemaKind::Receipt, &receipt).expect("receipt must satisfy v2");
    assert_eq!(receipt["result"], json!("opened"));
    assert_eq!(receipt["resourceRef"], json!(resource_ref));
    assert_eq!(dispatcher.seen()[0].mount_ref.as_deref(), Some("mount.6f2a1c"));

    locator_lease.dispose().await;
    presentation_lease.dispose().await;
    clear_muse_resource_surface_dispatcher();
  }

  // -------------------------------------------------------------------------
  // Partial (fuzzy) Mount-relative resolution
  // -------------------------------------------------------------------------

  /// The granted Mount catalog, in the shape the Flutter layer publishes.
  struct TestMountRoots {
    roots: BTreeMap<String, PathBuf>,
  }

  impl MuseMountRootCatalog for TestMountRoots {
    fn mount_root(&self, mount_ref: &str) -> Option<PathBuf> {
      self.roots.get(mount_ref).cloned()
    }
  }

  /// A temporary Mount directory that removes itself when the test ends.
  struct MountTree {
    root: PathBuf,
  }

  impl MountTree {
    fn with(name: &str, files: &[&str]) -> Self {
      let root = std::env::temp_dir().join(format!("muse-presentation-{name}-{}", Uuid::new_v4()));
      std::fs::create_dir_all(&root).expect("temp mount root");
      let tree = Self { root };
      for file in files {
        let path = tree.dir(file);
        std::fs::create_dir_all(path.parent().expect("parent")).expect("temp parent");
        std::fs::write(&path, "x").expect("temp file");
      }
      tree
    }

    fn dir(&self, relative: &str) -> PathBuf {
      let mut path = self.root.clone();
      for segment in relative.split('/') {
        path.push(segment);
      }
      path
    }

    /// Grant this directory as `mount_ref`, as the Flutter layer does.
    fn grant(&self, mount_ref: &str) {
      install_muse_mount_root_catalog(Arc::new(TestMountRoots {
        roots: BTreeMap::from([(mount_ref.to_string(), self.root.clone())]),
      }));
      assert!(muse_mount_root_catalog_available());
    }
  }

  impl Drop for MountTree {
    fn drop(&mut self) {
      let _ = std::fs::remove_dir_all(&self.root);
    }
  }

  /// Present one minted ref and return the receipt, with a recording surface.
  async fn present(
    provider: &PresentationProvider,
    request_ref: &str,
    resource_ref: &str,
    anchor: Option<Value>,
  ) -> Value {
    let mut payload = request_payload(request_ref, resource_ref, "view");
    if let Some(anchor) = anchor {
      payload["anchorHint"] = anchor;
    }
    let receipt = provider
      .invoke(
        invocation(RESOURCE_PRESENTATION_REQUEST, payload),
        context(),
        Cancellation::default(),
      )
      .await
      .expect("a receipt is always returned");
    validate(PresentationSchemaKind::Receipt, &receipt).expect("receipt must satisfy v2");
    receipt
  }

  /// An exact path is never rewritten: the surface still receives exactly what
  /// DSH minted the ref for.
  #[tokio::test]
  async fn an_exact_mount_path_reaches_the_surface_unchanged() {
    let _guard = dispatcher_guard();
    clear_muse_resource_surface_dispatcher();
    clear_muse_mount_root_catalog();
    let dispatcher = TestDispatcher::without_revision();
    install_muse_resource_surface_dispatcher(dispatcher.clone());
    let tree = MountTree::with(
      "exact",
      &["docs/architecture/08-view-support.md", "docs/architecture/07-lsp.md"],
    );
    tree.grant("mount.6f2a1c");

    let state = PresentationState::new(catalog(vec![document_view("view.1", false)]));
    let locator = LocatorProvider::with_state(state.clone());
    let provider = PresentationProvider::with_state(state);
    let located = locate(
      &locator,
      locator_input("mount.6f2a1c", "docs/architecture/08-view-support.md"),
    )
    .await
    .expect("an exact mount path locates");
    let resource_ref = located["resourceRef"].as_str().unwrap().to_string();

    let receipt = present(&provider, "request.1", &resource_ref, None).await;
    assert_eq!(receipt["result"], json!("opened"));
    let seen = dispatcher.seen();
    assert_eq!(seen.len(), 1);
    assert_eq!(
      seen[0].relative_path.as_deref(),
      Some("docs/architecture/08-view-support.md")
    );
    assert_eq!(seen[0].title.as_deref(), Some("08-view-support.md"));
    clear_muse_mount_root_catalog();
    clear_muse_resource_surface_dispatcher();
  }

  /// A partial path is resolved inside the granted Mount **before** the surface
  /// is asked to present it, so the dispatch carries the real, existing path.
  #[tokio::test]
  async fn a_partial_mount_path_reaches_the_surface_resolved() {
    let _guard = dispatcher_guard();
    clear_muse_resource_surface_dispatcher();
    clear_muse_mount_root_catalog();
    let dispatcher = TestDispatcher::without_revision();
    install_muse_resource_surface_dispatcher(dispatcher.clone());
    let tree = MountTree::with(
      "partial",
      &[
        "docs/architecture/08-view-support.md",
        "docs/architecture/07-lsp.md",
      ],
    );
    tree.grant("mount.6f2a1c");

    let state = PresentationState::new(catalog(vec![]));
    let locator = LocatorProvider::with_state(state.clone());
    let provider = PresentationProvider::with_state(state);

    // (a) missing leading segments
    let located = locate(&locator, locator_input("mount.6f2a1c", "architecture/08-view-support.md"))
      .await
      .expect("a partial path still locates");
    let resource_ref = located["resourceRef"].as_str().unwrap().to_string();
    let receipt = present(&provider, "request.1", &resource_ref, None).await;
    assert_eq!(receipt["result"], json!("opened"));
    assert_eq!(
      receipt["resourceRef"],
      json!(resource_ref),
      "the opaque ref the caller was given never changes"
    );

    // (b) a bare file name
    let located = locate(&locator, locator_input("mount.6f2a1c", "08-view-support.md"))
      .await
      .expect("a bare file name still locates");
    let bare_ref = located["resourceRef"].as_str().unwrap().to_string();
    assert_ne!(bare_ref, resource_ref);
    let receipt = present(&provider, "request.2", &bare_ref, None).await;
    assert_eq!(receipt["result"], json!("opened"));

    let seen = dispatcher.seen();
    assert_eq!(seen.len(), 2);
    for dispatch in &seen {
      assert_eq!(
        dispatch.relative_path.as_deref(),
        Some("docs/architecture/08-view-support.md"),
        "the surface receives the resolved, existing path"
      );
      assert_eq!(dispatch.mount_ref.as_deref(), Some("mount.6f2a1c"));
      assert_eq!(dispatch.title.as_deref(), Some("08-view-support.md"));
    }
    clear_muse_mount_root_catalog();
    clear_muse_resource_surface_dispatcher();
  }

  /// Equally good candidates are never guessed between: the request fails closed
  /// with a distinct reason code that names the candidate count, and no surface
  /// is opened.
  #[tokio::test]
  async fn an_ambiguous_mount_path_fails_closed_with_a_named_count() {
    let _guard = dispatcher_guard();
    clear_muse_resource_surface_dispatcher();
    clear_muse_mount_root_catalog();
    let dispatcher = TestDispatcher::opened();
    install_muse_resource_surface_dispatcher(dispatcher.clone());
    let tree = MountTree::with(
      "ambiguous",
      &[
        "docs/architecture/08-view-support.md",
        "docs/reference/08-view-support.md",
        "notes/third/08-view-support.md",
      ],
    );
    tree.grant("mount.6f2a1c");

    let state = PresentationState::new(catalog(vec![]));
    let locator = LocatorProvider::with_state(state.clone());
    let provider = PresentationProvider::with_state(state);
    let located = locate(&locator, locator_input("mount.6f2a1c", "08-view-support.md"))
      .await
      .expect("locating a partial path never resolves it");
    let resource_ref = located["resourceRef"].as_str().unwrap().to_string();

    let receipt = present(&provider, "request.1", &resource_ref, None).await;
    assert_eq!(receipt["result"], json!("failed"));
    assert_eq!(receipt["errorCode"], json!("RESOURCE_AMBIGUOUS_3"));
    assert_eq!(receipt["retryable"], json!(false));
    assert!(
      receipt.get("resourceRef").is_none(),
      "the v2 failure receipt carries a reason code, not resource identity"
    );
    assert!(
      dispatcher.seen().is_empty(),
      "an ambiguous path must never reach the surface"
    );

    // A path that matches nothing keeps today's behaviour: the supplied path is
    // dispatched and the surface answers its own not-found.
    let located = locate(&locator, locator_input("mount.6f2a1c", "not-in-this-mount.md"))
      .await
      .expect("locate");
    let missing = located["resourceRef"].as_str().unwrap().to_string();
    let receipt = present(&provider, "request.2", &missing, None).await;
    assert_eq!(receipt["result"], json!("opened"));
    assert_eq!(
      dispatcher.seen()[0].relative_path.as_deref(),
      Some("not-in-this-mount.md")
    );
    clear_muse_mount_root_catalog();
    clear_muse_resource_surface_dispatcher();
  }

  /// The resolved path is what a later, anchor-rehydrated request dispatches too:
  /// the *partial* path is what DSH mirrors into the anchor, and the Host resolves
  /// it again, deterministically, to the same file.
  #[tokio::test]
  async fn a_fuzzy_resolved_ref_rehydrates_from_the_partial_anchor() {
    let _guard = dispatcher_guard();
    clear_muse_resource_surface_dispatcher();
    clear_muse_mount_root_catalog();
    let dispatcher = TestDispatcher::without_revision();
    install_muse_resource_surface_dispatcher(dispatcher.clone());
    let tree = MountTree::with("rehydrate", &["docs/architecture/08-view-support.md"]);
    tree.grant("mount.6f2a1c");

    let state = PresentationState::new(catalog(vec![]));
    let locator = LocatorProvider::with_state(state.clone());
    let provider = PresentationProvider::with_state(state.clone());
    let located = locate(&locator, locator_input("mount.6f2a1c", "08-view-support.md"))
      .await
      .expect("locate");
    let resource_ref = located["resourceRef"].as_str().unwrap().to_string();

    // Expire the locator table: only the anchor hint is left.
    state.locators.lock().await.by_ref.clear();
    let receipt = present(
      &provider,
      "request.1",
      &resource_ref,
      Some(anchor_hint("mount.6f2a1c", "08-view-support.md")),
    )
    .await;
    assert_eq!(receipt["result"], json!("opened"));
    assert_eq!(receipt["resourceRef"], json!(resource_ref));
    assert_eq!(
      dispatcher.seen()[0].relative_path.as_deref(),
      Some("docs/architecture/08-view-support.md")
    );
    clear_muse_mount_root_catalog();
    clear_muse_resource_surface_dispatcher();
  }

  /// Without a granted Mount catalog this Host cannot search, so nothing about a
  /// dispatch changes: the supplied path is forwarded verbatim.
  #[tokio::test]
  async fn without_a_granted_mount_catalog_a_partial_path_is_forwarded_verbatim() {
    let _guard = dispatcher_guard();
    clear_muse_resource_surface_dispatcher();
    clear_muse_mount_root_catalog();
    let dispatcher = TestDispatcher::without_revision();
    install_muse_resource_surface_dispatcher(dispatcher.clone());
    // The file exists inside a real directory, but this Host was never granted
    // that Mount, so it has nothing to search.
    let _tree = MountTree::with("unsearched", &["docs/architecture/08-view-support.md"]);

    let state = PresentationState::new(catalog(vec![]));
    let locator = LocatorProvider::with_state(state.clone());
    let provider = PresentationProvider::with_state(state);
    let located = locate(&locator, locator_input("mount.6f2a1c", "08-view-support.md"))
      .await
      .expect("locate");
    let resource_ref = located["resourceRef"].as_str().unwrap().to_string();

    let receipt = present(&provider, "request.1", &resource_ref, None).await;
    assert_eq!(receipt["result"], json!("opened"));
    assert_eq!(
      dispatcher.seen()[0].relative_path.as_deref(),
      Some("08-view-support.md"),
      "an unsearchable Host must not rewrite the path"
    );
    clear_muse_resource_surface_dispatcher();
  }
}

