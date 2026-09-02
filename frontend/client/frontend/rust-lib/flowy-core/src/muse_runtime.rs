//! Native AppFlowy-owned Muse Bridge listener. The launch descriptor is transport identity only;
//! all actor/scope authority is re-derived from live AppFlowy managers for every request.

use std::{
  fs::OpenOptions,
  io::Write,
  os::unix::fs::{FileTypeExt, MetadataExt, OpenOptionsExt},
  path::{Path, PathBuf},
  sync::{Arc, Weak},
};

use flowy_user::user_manager::UserManager;
use lib_infra::async_trait::async_trait;
use muse_host_events::HostEventHub;
use muse_host_policy::HostPolicy;
use muse_host_registry::{AuthoritativeCaller, HostCapabilityRegistry, RegistryError};
use muse_host_runtime::{BridgeRequestDispatcher, RuntimeCallerResolver};
use muse_host_transport::{
  DesktopCarrierKind, DesktopEndpoint, DesktopHostServer, PeerAuthenticator, PeerIdentity,
  SystemSecretSource, SystemTransportClock, TransportConfig, TransportError,
};
use serde_json::json;

use crate::muse_host::actor_ref;

const HOST_GENERATION: &str = "appflowy.local.1";

struct CurrentAppFlowyCaller(Weak<UserManager>);

#[async_trait]
impl RuntimeCallerResolver for CurrentAppFlowyCaller {
  async fn caller(&self, _runtime_instance_id: &str) -> Result<AuthoritativeCaller, RegistryError> {
    let manager = self.0.upgrade().ok_or(RegistryError::Authority(
      muse_host_registry::AuthorityError::Unavailable,
    ))?;
    let user_id = manager
      .user_id()
      .map_err(|_| RegistryError::Authority(muse_host_registry::AuthorityError::Unavailable))?;
    Ok(AuthoritativeCaller {
      actor_ref: actor_ref(user_id),
    })
  }
}

struct SameUserPeer(u32);
impl PeerAuthenticator for SameUserPeer {
  fn authorize(&self, peer: &PeerIdentity, _runtime_instance_id: &str) -> bool {
    peer.principal == format!("uid.{}", self.0)
  }
}

pub(crate) struct AppFlowyMuseRuntime {
  server: DesktopHostServer,
  launch_file: PathBuf,
  approval_file: PathBuf,
}

impl AppFlowyMuseRuntime {
  pub(crate) fn start(
    registry: Arc<HostCapabilityRegistry>,
    policy: Arc<HostPolicy>,
    user_manager: Weak<UserManager>,
    approval_secret: String,
    events: Arc<HostEventHub>,
  ) -> Result<Self, TransportError> {
    let uid = unsafe { libc::geteuid() };
    let base = std::env::temp_dir();
    let socket = base.join(format!("appflowy-muse-host-{uid}.sock"));
    let launch_file = base.join(format!("appflowy-muse-host-{uid}.json"));
    let approval_file = base.join(format!("appflowy-muse-approval-{uid}.json"));
    remove_owned_stale_socket(&socket, uid)?;
    remove_owned_regular_file(&launch_file, uid)?;
    remove_owned_regular_file(&approval_file, uid)?;
    let endpoint = DesktopEndpoint {
      kind: DesktopCarrierKind::UnixDomainSocket,
      address: socket.to_string_lossy().into_owned(),
    };
    let dispatcher = Arc::new(
      BridgeRequestDispatcher::new_with_events(
        registry,
        Arc::new(CurrentAppFlowyCaller(user_manager)),
        HOST_GENERATION,
        format!("host-session.appflowy.{uid}"),
        events,
      )?
      .with_policy(policy),
    );
    let server = DesktopHostServer::start(
      TransportConfig {
        endpoint,
        host_generation: HOST_GENERATION.into(),
        connection_ttl_ms: 5 * 60_000,
        max_deadline_horizon_ms: 5 * 60_000,
        max_connections: 32,
        max_concurrent_requests: 64,
        max_payload_bytes: 2 * 1024 * 1024,
        max_response_bytes: 2 * 1024 * 1024,
        clock: Arc::new(SystemTransportClock),
      },
      Arc::new(SameUserPeer(uid)),
      Arc::new(SystemSecretSource),
      dispatcher,
    )?;
    let launch = server.launch();
    let bytes = serde_json::to_vec(&json!({
      "endpoint": launch.endpoint.address,
      "nonce": launch.nonce,
      "hostGeneration": launch.host_generation,
      "runtimeInstanceId": format!("runtime.dsh-appflowy.{uid}")
    }))
    .map_err(|_| TransportError::Handler)?;
    let mut file = OpenOptions::new()
      .write(true)
      .create_new(true)
      .mode(0o600)
      .open(&launch_file)
      .map_err(|_| TransportError::Unavailable)?;
    file
      .write_all(&bytes)
      .and_then(|_| file.sync_all())
      .map_err(|_| TransportError::Unavailable)?;
    write_private_json(&approval_file, &json!({ "secret": approval_secret }))?;
    tracing::info!(path = %launch_file.display(), "AppFlowy Muse Host runtime ready");
    Ok(Self {
      server,
      launch_file,
      approval_file,
    })
  }

  pub(crate) async fn shutdown(self) {
    self.server.shutdown().await;
    let _ = std::fs::remove_file(self.launch_file);
    let _ = std::fs::remove_file(self.approval_file);
  }
}

fn write_private_json(path: &Path, value: &serde_json::Value) -> Result<(), TransportError> {
  let bytes = serde_json::to_vec(value).map_err(|_| TransportError::Handler)?;
  let mut file = OpenOptions::new()
    .write(true)
    .create_new(true)
    .mode(0o600)
    .open(path)
    .map_err(|_| TransportError::Unavailable)?;
  file
    .write_all(&bytes)
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
