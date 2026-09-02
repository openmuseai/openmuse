mod carrier;
mod error;
mod idempotency;
#[cfg(unix)]
mod server;
mod session;

#[cfg(unix)]
pub use carrier::UnixDomainSocketCarrier;
pub use carrier::{DesktopCarrierKind, DesktopEndpoint, PeerIdentity, read_frame, write_frame};
pub use error::TransportError;
pub use idempotency::{
    IdempotencyConfig, IdempotencyDecision, IdempotencyIdentity, IdempotencyPermit,
    IdempotencyRegistry,
};
#[cfg(unix)]
pub use server::{DesktopEventReceiver, DesktopHostServer, DesktopRequestHandler};
pub use session::{
    ConnectionCredential, PeerAuthenticator, RequestPermit, RuntimeProof, SecretSource,
    SystemSecretSource, SystemTransportClock, TransportCancellation, TransportClock,
    TransportConfig, TransportLaunch, TransportSessionManager,
};
