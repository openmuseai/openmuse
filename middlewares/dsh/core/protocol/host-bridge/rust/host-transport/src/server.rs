use std::sync::Arc;

use tokio::{sync::watch, task::JoinHandle};

use crate::{
    DesktopCarrierKind, DesktopRequestHandler, PeerAuthenticator, SecretSource, TransportConfig,
    TransportError, TransportLaunch, TransportSessionManager,
};

/// Owns the private desktop listener and its accept loop. Dropping or shutting
/// it down rejects new admission, cancels pending requests, and drains the
/// listener task. The Unix carrier also unlinks its socket path.
pub struct DesktopHostServer {
    launch: TransportLaunch,
    sessions: TransportSessionManager,
    stop: watch::Sender<bool>,
    task: JoinHandle<()>,
}

impl DesktopHostServer {
    pub fn start(
        config: TransportConfig,
        authenticator: Arc<dyn PeerAuthenticator>,
        secrets: Arc<dyn SecretSource>,
        handler: Arc<dyn DesktopRequestHandler>,
    ) -> Result<Self, TransportError> {
        match config.endpoint.kind {
            #[cfg(unix)]
            DesktopCarrierKind::UnixDomainSocket => {
                Self::listen_unix(config, authenticator, secrets, handler)
            }
            #[cfg(windows)]
            DesktopCarrierKind::WindowsNamedPipe => {
                Self::listen_windows(config, authenticator, secrets, handler)
            }
            #[allow(unreachable_patterns)]
            _ => Err(TransportError::InvalidFrame),
        }
    }

    pub fn launch(&self) -> TransportLaunch {
        self.launch.clone()
    }

    pub async fn shutdown(self) {
        self.sessions.close();
        let _ = self.stop.send(true);
        let _ = self.task.await;
    }

    fn assemble(
        sessions: TransportSessionManager,
        stop: watch::Sender<bool>,
        task: JoinHandle<()>,
    ) -> Self {
        let launch = sessions.launch();
        Self {
            launch,
            sessions,
            stop,
            task,
        }
    }
}

#[cfg(unix)]
mod unix_listen {
    use std::sync::Arc;

    use tokio::sync::watch;

    use super::DesktopHostServer;
    use crate::{
        DesktopRequestHandler, PeerAuthenticator, SecretSource, TransportConfig, TransportError,
        TransportSessionManager, UnixDomainSocketCarrier, protocol::serve_one,
    };

    impl DesktopHostServer {
        pub(super) fn listen_unix(
            config: TransportConfig,
            authenticator: Arc<dyn PeerAuthenticator>,
            secrets: Arc<dyn SecretSource>,
            handler: Arc<dyn DesktopRequestHandler>,
        ) -> Result<Self, TransportError> {
            let carrier = UnixDomainSocketCarrier::bind(&config.endpoint)?;
            let max_payload = config.max_payload_bytes;
            let max_response = config.max_response_bytes;
            let sessions = TransportSessionManager::new(config, authenticator, secrets)?;
            let (stop, mut stopped) = watch::channel(false);
            let server_sessions = sessions.clone();
            let task = tokio::spawn(async move {
                loop {
                    tokio::select! {
                        changed = stopped.changed() => {
                            if changed.is_err() || *stopped.borrow() { break; }
                        }
                        accepted = carrier.accept() => {
                            let Ok((stream, peer)) = accepted else { break };
                            let sessions = server_sessions.clone();
                            let handler = handler.clone();
                            tokio::spawn(async move {
                                let _ = serve_one(stream, peer, sessions, handler, max_payload, max_response).await;
                            });
                        }
                    }
                }
            });
            Ok(Self::assemble(sessions, stop, task))
        }
    }
}

#[cfg(windows)]
mod windows_listen {
    use std::sync::Arc;

    use tokio::sync::watch;

    use super::DesktopHostServer;
    use crate::{
        DesktopRequestHandler, PeerAuthenticator, SecretSource, TransportConfig, TransportError,
        TransportSessionManager, WindowsNamedPipeListener, protocol::serve_one,
    };

    impl DesktopHostServer {
        pub(super) fn listen_windows(
            config: TransportConfig,
            authenticator: Arc<dyn PeerAuthenticator>,
            secrets: Arc<dyn SecretSource>,
            handler: Arc<dyn DesktopRequestHandler>,
        ) -> Result<Self, TransportError> {
            let listener = WindowsNamedPipeListener::bind(&config.endpoint)?;
            let max_payload = config.max_payload_bytes;
            let max_response = config.max_response_bytes;
            let sessions = TransportSessionManager::new(config, authenticator, secrets)?;
            let (stop, mut stopped) = watch::channel(false);
            let server_sessions = sessions.clone();
            let mut incoming = listener.create_instance(true)?;
            let task = tokio::spawn(async move {
                loop {
                    tokio::select! {
                        changed = stopped.changed() => {
                            if changed.is_err() || *stopped.borrow() { break; }
                        }
                        connected = incoming.connect() => {
                            if connected.is_err() { break; }
                            let stream = incoming;
                            let peer = WindowsNamedPipeListener::peer_identity(&stream);
                            let next = listener.create_instance(false);
                            let sessions = server_sessions.clone();
                            let handler = handler.clone();
                            tokio::spawn(async move {
                                let _ = serve_one(stream, peer, sessions, handler, max_payload, max_response).await;
                            });
                            match next {
                                Ok(pipe) => incoming = pipe,
                                Err(_) => break,
                            }
                        }
                    }
                }
            });
            Ok(Self::assemble(sessions, stop, task))
        }
    }
}
