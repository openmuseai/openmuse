mod carrier;
mod error;
mod handler;
mod idempotency;
#[cfg(any(unix, windows))]
mod protocol;
#[cfg(any(unix, windows))]
mod server;
mod session;

#[cfg(unix)]
pub use carrier::UnixDomainSocketCarrier;
#[cfg(windows)]
pub use carrier::WindowsNamedPipeListener;
pub use carrier::{
    DesktopCarrierKind, DesktopEndpoint, HostByteStream, PeerIdentity, read_frame,
    windows_named_pipe_address_valid, write_frame,
};
pub use error::TransportError;
pub use handler::{DesktopEventReceiver, DesktopRequestHandler};
pub use idempotency::{
    IdempotencyConfig, IdempotencyDecision, IdempotencyIdentity, IdempotencyPermit,
    IdempotencyRegistry,
};
#[cfg(any(unix, windows))]
pub use server::DesktopHostServer;
pub use session::{
    ConnectionCredential, PeerAuthenticator, RequestPermit, RuntimeProof, SecretSource,
    SystemSecretSource, SystemTransportClock, TransportCancellation, TransportClock,
    TransportConfig, TransportLaunch, TransportSessionManager,
};
