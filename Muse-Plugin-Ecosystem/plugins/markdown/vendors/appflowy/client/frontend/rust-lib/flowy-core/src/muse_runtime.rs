//! Native AppFlowy-owned Muse Bridge listener. The launch descriptor is transport
//! identity only; all actor/scope authority is re-derived from live AppFlowy
//! managers for every request. Carrier bind and launch-file privacy are
//! platform-owned (`muse_native_*`).

use std::{path::PathBuf, sync::{Arc, Weak}};

use flowy_user::user_manager::UserManager;
use lib_infra::async_trait::async_trait;
use muse_host_events::HostEventHub;
use muse_host_policy::HostPolicy;
use muse_host_registry::{AuthoritativeCaller, HostCapabilityRegistry, RegistryError};
use muse_host_runtime::{BridgeRequestDispatcher, RuntimeCallerResolver};
use muse_host_transport::{
  DesktopHostServer, SystemSecretSource, SystemTransportClock, TransportConfig, TransportError,
};
use serde_json::json;

use crate::muse_host::actor_ref;
#[cfg(unix)]
use crate::muse_native_unix as native;
#[cfg(windows)]
use crate::muse_native_windows as native;

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
    let layout = native::prepare()?;
    let dispatcher = Arc::new(
      BridgeRequestDispatcher::new_with_events(
        registry,
        Arc::new(CurrentAppFlowyCaller(user_manager)),
        HOST_GENERATION,
        layout.host_session_id.clone(),
        events,
      )?
      .with_policy(policy),
    );
    let server = DesktopHostServer::start(
      TransportConfig {
        endpoint: layout.endpoint,
        host_generation: HOST_GENERATION.into(),
        connection_ttl_ms: 5 * 60_000,
        max_deadline_horizon_ms: 5 * 60_000,
        max_connections: 32,
        max_concurrent_requests: 64,
        max_payload_bytes: 2 * 1024 * 1024,
        max_response_bytes: 2 * 1024 * 1024,
        clock: Arc::new(SystemTransportClock),
      },
      layout.authenticator,
      Arc::new(SystemSecretSource),
      dispatcher,
    )?;
    let launch = server.launch();
    let bytes = serde_json::to_vec(&json!({
      "endpoint": launch.endpoint.address,
      "nonce": launch.nonce,
      "hostGeneration": launch.host_generation,
      "runtimeInstanceId": layout.runtime_instance_id,
    }))
    .map_err(|_| TransportError::Handler)?;
    native::write_private_bytes(&layout.launch_file, &bytes)?;
    native::write_private_json(
      &layout.approval_file,
      &json!({ "secret": approval_secret }),
    )?;
    tracing::info!(path = %layout.launch_file.display(), "AppFlowy Muse Host runtime ready");
    Ok(Self {
      server,
      launch_file: layout.launch_file,
      approval_file: layout.approval_file,
    })
  }

  pub(crate) async fn shutdown(self) {
    self.server.shutdown().await;
    let _ = std::fs::remove_file(self.launch_file);
    let _ = std::fs::remove_file(self.approval_file);
  }
}
