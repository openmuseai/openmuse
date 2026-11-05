//! Unix Muse Host bind: UDS + 0600 launch descriptor owned by geteuid().

use std::{
  fs::OpenOptions,
  io::Write,
  os::unix::fs::{FileTypeExt, MetadataExt, OpenOptionsExt},
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

struct SameUserPeer(u32);

impl PeerAuthenticator for SameUserPeer {
  fn authorize(&self, peer: &PeerIdentity, _runtime_instance_id: &str) -> bool {
    peer.principal == format!("uid.{}", self.0)
  }
}

pub(crate) fn prepare() -> Result<NativeHostLayout, TransportError> {
  let uid = unsafe { libc::geteuid() };
  let base = std::env::temp_dir();
  let socket = base.join(format!("appflowy-muse-host-{uid}.sock"));
  let launch_file = base.join(format!("appflowy-muse-host-{uid}.json"));
  let approval_file = base.join(format!("appflowy-muse-approval-{uid}.json"));
  remove_owned_stale_socket(&socket, uid)?;
  remove_owned_regular_file(&launch_file, uid)?;
  remove_owned_regular_file(&approval_file, uid)?;
  Ok(NativeHostLayout {
    endpoint: DesktopEndpoint {
      kind: DesktopCarrierKind::UnixDomainSocket,
      address: socket.to_string_lossy().into_owned(),
    },
    launch_file,
    approval_file,
    host_session_id: format!("host-session.appflowy.{uid}"),
    runtime_instance_id: format!("runtime.dsh-appflowy.{uid}"),
    authenticator: Arc::new(SameUserPeer(uid)),
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
    .mode(0o600)
    .open(path)
    .map_err(|_| TransportError::Unavailable)?;
  file
    .write_all(bytes)
    .and_then(|_| file.sync_all())
    .map_err(|_| TransportError::Unavailable)
}

fn remove_owned_stale_socket(path: &Path, uid: u32) -> Result<(), TransportError> {
  let Ok(metadata) = std::fs::symlink_metadata(path) else {
    return Ok(());
  };
  if metadata.uid() != uid || !metadata.file_type().is_socket() {
    return Err(TransportError::Forbidden);
  }
  std::fs::remove_file(path).map_err(|_| TransportError::Unavailable)
}

fn remove_owned_regular_file(path: &Path, uid: u32) -> Result<(), TransportError> {
  let Ok(metadata) = std::fs::symlink_metadata(path) else {
    return Ok(());
  };
  if metadata.uid() != uid || !metadata.file_type().is_file() {
    return Err(TransportError::Forbidden);
  }
  std::fs::remove_file(path).map_err(|_| TransportError::Unavailable)
}
