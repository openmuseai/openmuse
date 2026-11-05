//! Windows Muse Host bind: local named pipe + launch descriptor in the user temp dir.
//!
//! Peer identity on this carrier is "could connect to this pipe"
//! (`named_pipe_client_token`), not UDS `SO_PEERCRED`. Launch nonce still
//! authenticates the sidecar. POSIX mode bits are not used: `%TEMP%` is already
//! per-user, and Node's `fs.stat().mode` on Windows is not a privacy signal.

use std::{
  fs::OpenOptions,
  io::Write,
  path::{Path, PathBuf},
  sync::Arc,
};

use muse_host_transport::{
  DesktopCarrierKind, DesktopEndpoint, PeerAuthenticator, PeerIdentity, TransportError,
};
use serde_json::Value;

pub(crate) struct NativeHostLayout {
  pub endpoint: DesktopEndpoint,
  pub launch_file: PathBuf,
  pub approval_file: PathBuf,
  pub host_session_id: String,
  pub runtime_instance_id: String,
  pub authenticator: Arc<dyn PeerAuthenticator>,
}

struct LocalNamedPipePeer;

impl PeerAuthenticator for LocalNamedPipePeer {
  fn authorize(&self, peer: &PeerIdentity, _runtime_instance_id: &str) -> bool {
    peer.carrier == DesktopCarrierKind::WindowsNamedPipe
  }
}

pub(crate) fn prepare() -> Result<NativeHostLayout, TransportError> {
  // Node `process.getuid?.() ?? 0` on Windows, so the sidecar looks for `-0.json`.
  const LAUNCH_ID: u32 = 0;
  let base = std::env::temp_dir();
  let launch_file = base.join(format!("appflowy-muse-host-{LAUNCH_ID}.json"));
  let approval_file = base.join(format!("appflowy-muse-approval-{LAUNCH_ID}.json"));
  remove_regular_file(&launch_file)?;
  remove_regular_file(&approval_file)?;
  let address = format!(
    r"\\.\pipe\appflowy-muse-host-{}-{}",
    std::process::id(),
    std::time::SystemTime::now()
      .duration_since(std::time::UNIX_EPOCH)
      .map(|d| d.as_nanos())
      .unwrap_or(0)
  );
  Ok(NativeHostLayout {
    endpoint: DesktopEndpoint {
      kind: DesktopCarrierKind::WindowsNamedPipe,
      address,
    },
    launch_file,
    approval_file,
    host_session_id: format!("host-session.appflowy.{LAUNCH_ID}"),
    runtime_instance_id: format!("runtime.dsh-appflowy.{LAUNCH_ID}"),
    authenticator: Arc::new(LocalNamedPipePeer),
  })
}

pub(crate) fn write_private_json(path: &Path, value: &Value) -> Result<(), TransportError> {
  let bytes = serde_json::to_vec(value).map_err(|_| TransportError::Handler)?;
  write_private_bytes(path, &bytes)
}

pub(crate) fn write_private_bytes(path: &Path, bytes: &[u8]) -> Result<(), TransportError> {
  let mut file = OpenOptions::new()
    .write(true)
    .create_new(true)
    .open(path)
    .map_err(|_| TransportError::Unavailable)?;
  file
    .write_all(bytes)
    .and_then(|_| file.sync_all())
    .map_err(|_| TransportError::Unavailable)
}

fn remove_regular_file(path: &Path) -> Result<(), TransportError> {
  let Ok(metadata) = std::fs::symlink_metadata(path) else {
    return Ok(());
  };
  if !metadata.file_type().is_file() {
    return Err(TransportError::Forbidden);
  }
  std::fs::remove_file(path).map_err(|_| TransportError::Unavailable)
}
