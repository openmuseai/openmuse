use tokio::io::{AsyncRead, AsyncReadExt, AsyncWrite, AsyncWriteExt};

use crate::TransportError;

#[cfg(unix)]
use std::{
    os::unix::fs::{MetadataExt, PermissionsExt},
    path::{Path, PathBuf},
};
#[cfg(unix)]
use tokio::net::{UnixListener, UnixStream};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DesktopCarrierKind {
    UnixDomainSocket,
    WindowsNamedPipe,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DesktopEndpoint {
    pub kind: DesktopCarrierKind,
    pub address: String,
}

/// Peer identity is supplied by the carrier (UDS peer credentials / named-pipe client identity),
/// never by a Bridge payload.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PeerIdentity {
    pub carrier: DesktopCarrierKind,
    pub principal: String,
}

#[cfg(unix)]
pub struct UnixDomainSocketCarrier {
    listener: UnixListener,
    path: PathBuf,
    device: u64,
    inode: u64,
}

#[cfg(unix)]
impl UnixDomainSocketCarrier {
    pub fn bind(endpoint: &DesktopEndpoint) -> Result<Self, TransportError> {
        if endpoint.kind != DesktopCarrierKind::UnixDomainSocket {
            return Err(TransportError::InvalidFrame);
        }
        let path = Path::new(&endpoint.address);
        if !path.is_absolute() || path.exists() {
            return Err(TransportError::InvalidFrame);
        }
        let listener = UnixListener::bind(path).map_err(|_| TransportError::Unavailable)?;
        if std::fs::set_permissions(path, std::fs::Permissions::from_mode(0o600)).is_err() {
            let _ = std::fs::remove_file(path);
            return Err(TransportError::Unavailable);
        }
        let metadata = match std::fs::metadata(path) {
            Ok(metadata) => metadata,
            Err(_) => {
                let _ = std::fs::remove_file(path);
                return Err(TransportError::Unavailable);
            }
        };
        Ok(Self {
            listener,
            path: path.to_path_buf(),
            device: metadata.dev(),
            inode: metadata.ino(),
        })
    }

    pub async fn accept(&self) -> Result<(UnixStream, PeerIdentity), TransportError> {
        let (stream, _) = self
            .listener
            .accept()
            .await
            .map_err(|_| TransportError::Unavailable)?;
        let credentials = stream
            .peer_cred()
            .map_err(|_| TransportError::Unauthenticated)?;
        Ok((
            stream,
            PeerIdentity {
                carrier: DesktopCarrierKind::UnixDomainSocket,
                principal: format!("uid.{}", credentials.uid()),
            },
        ))
    }
}

#[cfg(unix)]
impl Drop for UnixDomainSocketCarrier {
    fn drop(&mut self) {
        let still_owned = std::fs::metadata(&self.path)
            .is_ok_and(|metadata| metadata.dev() == self.device && metadata.ino() == self.inode);
        if still_owned {
            let _ = std::fs::remove_file(&self.path);
        }
    }
}

pub async fn write_frame<W: AsyncWrite + Unpin>(
    writer: &mut W,
    payload: &[u8],
    max_frame_bytes: usize,
) -> Result<(), TransportError> {
    if payload.is_empty() || payload.len() > max_frame_bytes || payload.len() > u32::MAX as usize {
        return Err(TransportError::InvalidFrame);
    }
    writer
        .write_all(&(payload.len() as u32).to_be_bytes())
        .await
        .map_err(|_| TransportError::Unavailable)?;
    writer
        .write_all(payload)
        .await
        .map_err(|_| TransportError::Unavailable)?;
    writer
        .flush()
        .await
        .map_err(|_| TransportError::Unavailable)
}

pub async fn read_frame<R: AsyncRead + Unpin>(
    reader: &mut R,
    max_frame_bytes: usize,
) -> Result<Vec<u8>, TransportError> {
    let mut prefix = [0_u8; 4];
    reader
        .read_exact(&mut prefix)
        .await
        .map_err(|_| TransportError::Unavailable)?;
    let length = u32::from_be_bytes(prefix) as usize;
    if length == 0 || length > max_frame_bytes {
        return Err(TransportError::InvalidFrame);
    }
    let mut payload = vec![0_u8; length];
    reader
        .read_exact(&mut payload)
        .await
        .map_err(|_| TransportError::Unavailable)?;
    Ok(payload)
}
