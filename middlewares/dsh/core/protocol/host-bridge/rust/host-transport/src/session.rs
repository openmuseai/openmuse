use std::{
    collections::BTreeMap,
    fmt::Debug,
    sync::{
        Arc, Mutex,
        atomic::{AtomicBool, AtomicU64, Ordering},
    },
    time::{SystemTime, UNIX_EPOCH},
};

use base64::{Engine, engine::general_purpose::URL_SAFE_NO_PAD};
use rand::RngCore;
use sha2::{Digest, Sha256};
use subtle::ConstantTimeEq;
use tokio::sync::Notify;

use crate::{DesktopEndpoint, PeerIdentity, TransportError};

pub trait TransportClock: Debug + Send + Sync + 'static {
    fn now_ms(&self) -> u64;
}

#[derive(Debug)]
pub struct SystemTransportClock;
impl TransportClock for SystemTransportClock {
    fn now_ms(&self) -> u64 {
        SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap_or_default()
            .as_millis()
            .min(u64::MAX as u128) as u64
    }
}

#[derive(Debug, Clone)]
pub struct TransportConfig {
    pub endpoint: DesktopEndpoint,
    pub host_generation: String,
    pub connection_ttl_ms: u64,
    pub max_deadline_horizon_ms: u64,
    pub max_connections: usize,
    pub max_concurrent_requests: usize,
    pub max_payload_bytes: usize,
    pub max_response_bytes: usize,
    pub clock: Arc<dyn TransportClock>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TransportLaunch {
    pub endpoint: DesktopEndpoint,
    pub nonce: String,
    pub host_generation: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RuntimeProof {
    pub runtime_instance_id: String,
    pub nonce: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ConnectionCredential {
    pub connection_id: String,
    pub token: String,
    pub runtime_instance_id: String,
    pub host_generation: String,
    pub expires_at_ms: u64,
}

pub trait PeerAuthenticator: Send + Sync + 'static {
    fn authorize(&self, peer: &PeerIdentity, runtime_instance_id: &str) -> bool;
}

pub trait SecretSource: Send + Sync + 'static {
    fn secret(&self) -> String;
}

pub struct SystemSecretSource;
impl SecretSource for SystemSecretSource {
    fn secret(&self) -> String {
        let mut bytes = [0_u8; 32];
        rand::rng().fill_bytes(&mut bytes);
        URL_SAFE_NO_PAD.encode(bytes)
    }
}

#[derive(Debug, Clone, Default)]
pub struct TransportCancellation {
    inner: Arc<CancellationInner>,
}

#[derive(Debug, Default)]
struct CancellationInner {
    cancelled: AtomicBool,
    notify: Notify,
}

impl TransportCancellation {
    pub fn cancel(&self) {
        if !self.inner.cancelled.swap(true, Ordering::AcqRel) {
            self.inner.notify.notify_waiters();
        }
    }
    pub fn is_cancelled(&self) -> bool {
        self.inner.cancelled.load(Ordering::Acquire)
    }
    pub async fn cancelled(&self) {
        loop {
            let notified = self.inner.notify.notified();
            if self.is_cancelled() {
                return;
            }
            notified.await;
        }
    }
}

struct ConnectionRecord {
    token_digest: [u8; 32],
    runtime_instance_id: String,
    host_generation: String,
    expires_at_ms: u64,
    pending: BTreeMap<u64, TransportCancellation>,
}

#[derive(Default)]
struct State {
    closed: bool,
    active_requests: usize,
    connections: BTreeMap<String, ConnectionRecord>,
}

struct Inner {
    config: TransportConfig,
    launch: TransportLaunch,
    peer_authenticator: Arc<dyn PeerAuthenticator>,
    secrets: Arc<dyn SecretSource>,
    ids: AtomicU64,
    state: Mutex<State>,
}

#[derive(Clone)]
pub struct TransportSessionManager {
    inner: Arc<Inner>,
}

impl TransportSessionManager {
    pub fn new(
        config: TransportConfig,
        peer_authenticator: Arc<dyn PeerAuthenticator>,
        secrets: Arc<dyn SecretSource>,
    ) -> Result<Self, TransportError> {
        if config.host_generation.is_empty()
            || config.connection_ttl_ms == 0
            || config.max_deadline_horizon_ms == 0
            || config.max_connections == 0
            || config.max_concurrent_requests == 0
            || config.max_payload_bytes == 0
            || config.max_response_bytes == 0
        {
            return Err(TransportError::InvalidFrame);
        }
        let launch = TransportLaunch {
            endpoint: config.endpoint.clone(),
            nonce: secrets.secret(),
            host_generation: config.host_generation.clone(),
        };
        Ok(Self {
            inner: Arc::new(Inner {
                config,
                launch,
                peer_authenticator,
                secrets,
                ids: AtomicU64::new(1),
                state: Mutex::new(State::default()),
            }),
        })
    }

    pub fn launch(&self) -> TransportLaunch {
        self.inner.launch.clone()
    }

    pub fn connect(
        &self,
        proof: &RuntimeProof,
        peer: &PeerIdentity,
    ) -> Result<ConnectionCredential, TransportError> {
        let now_ms = self.inner.config.clock.now_ms();
        if !opaque_id(&proof.runtime_instance_id)
            || !constant_equal(&proof.nonce, &self.inner.launch.nonce)
        {
            return Err(TransportError::Unauthenticated);
        }
        if !self
            .inner
            .peer_authenticator
            .authorize(peer, &proof.runtime_instance_id)
        {
            return Err(TransportError::Forbidden);
        }
        let mut state = self.inner.state.lock().expect("transport state poisoned");
        if state.closed {
            return Err(TransportError::Unavailable);
        }
        sweep_expired(&mut state, now_ms);
        if state.connections.len() >= self.inner.config.max_connections {
            return Err(TransportError::RateLimited);
        }
        let connection_id = self.mint("connection");
        let token = self.inner.secrets.secret();
        let expires_at_ms = now_ms.saturating_add(self.inner.config.connection_ttl_ms);
        let credential = ConnectionCredential {
            connection_id: connection_id.clone(),
            token: token.clone(),
            runtime_instance_id: proof.runtime_instance_id.clone(),
            host_generation: self.inner.config.host_generation.clone(),
            expires_at_ms,
        };
        state.connections.insert(
            connection_id,
            ConnectionRecord {
                token_digest: digest(&token),
                runtime_instance_id: proof.runtime_instance_id.clone(),
                host_generation: self.inner.config.host_generation.clone(),
                expires_at_ms,
                pending: BTreeMap::new(),
            },
        );
        Ok(credential)
    }

    pub fn admit(
        &self,
        credential: &ConnectionCredential,
        payload_bytes: usize,
        deadline_at_ms: u64,
    ) -> Result<RequestPermit, TransportError> {
        let now_ms = self.inner.config.clock.now_ms();
        if payload_bytes == 0 || payload_bytes > self.inner.config.max_payload_bytes {
            return Err(TransportError::InvalidFrame);
        }
        if deadline_at_ms <= now_ms {
            return Err(TransportError::DeadlineExceeded);
        }
        if deadline_at_ms.saturating_sub(now_ms) > self.inner.config.max_deadline_horizon_ms {
            return Err(TransportError::InvalidFrame);
        }
        let mut state = self.inner.state.lock().expect("transport state poisoned");
        if state.closed {
            return Err(TransportError::Unavailable);
        }
        sweep_expired(&mut state, now_ms);
        if state.active_requests >= self.inner.config.max_concurrent_requests {
            return Err(TransportError::RateLimited);
        }
        let connection = state
            .connections
            .get(&credential.connection_id)
            .ok_or(TransportError::Unauthenticated)?;
        authenticate(connection, credential, now_ms)?;
        let request_id = self.inner.ids.fetch_add(1, Ordering::Relaxed);
        let cancellation = TransportCancellation::default();
        state
            .connections
            .get_mut(&credential.connection_id)
            .expect("connection checked above")
            .pending
            .insert(request_id, cancellation.clone());
        state.active_requests += 1;
        Ok(RequestPermit {
            inner: self.inner.clone(),
            connection_id: credential.connection_id.clone(),
            request_id,
            deadline_at_ms,
            cancellation,
            released: false,
        })
    }

    pub fn disconnect(&self, credential: &ConnectionCredential) {
        let now_ms = self.inner.config.clock.now_ms();
        let mut state = self.inner.state.lock().expect("transport state poisoned");
        let authenticated = state
            .connections
            .get(&credential.connection_id)
            .is_some_and(|record| authenticate(record, credential, now_ms).is_ok());
        if authenticated {
            if let Some(record) = state.connections.remove(&credential.connection_id) {
                state.active_requests = state.active_requests.saturating_sub(record.pending.len());
                for cancellation in record.pending.values() {
                    cancellation.cancel();
                }
            }
        }
    }

    pub fn close(&self) {
        let mut state = self.inner.state.lock().expect("transport state poisoned");
        if state.closed {
            return;
        }
        state.closed = true;
        for connection in state.connections.values() {
            for cancellation in connection.pending.values() {
                cancellation.cancel();
            }
        }
        state.active_requests = 0;
        state.connections.clear();
    }

    pub fn validate_response(&self, response_bytes: usize) -> Result<(), TransportError> {
        if response_bytes == 0 || response_bytes > self.inner.config.max_response_bytes {
            Err(TransportError::Handler)
        } else {
            Ok(())
        }
    }

    fn mint(&self, prefix: &str) -> String {
        format!(
            "{prefix}.{}",
            self.inner.ids.fetch_add(1, Ordering::Relaxed)
        )
    }
}

pub struct RequestPermit {
    inner: Arc<Inner>,
    connection_id: String,
    request_id: u64,
    pub deadline_at_ms: u64,
    pub cancellation: TransportCancellation,
    released: bool,
}

impl RequestPermit {
    pub fn cancel(&self) {
        self.cancellation.cancel();
    }
}

impl Drop for RequestPermit {
    fn drop(&mut self) {
        if self.released {
            return;
        }
        self.released = true;
        let mut state = self.inner.state.lock().expect("transport state poisoned");
        if let Some(connection) = state.connections.get_mut(&self.connection_id) {
            if connection.pending.remove(&self.request_id).is_some() {
                state.active_requests = state.active_requests.saturating_sub(1);
            }
        }
    }
}

fn authenticate(
    record: &ConnectionRecord,
    credential: &ConnectionCredential,
    now_ms: u64,
) -> Result<(), TransportError> {
    if record.host_generation != credential.host_generation {
        return Err(TransportError::HostGenerationStale);
    }
    if record.runtime_instance_id != credential.runtime_instance_id
        || record.expires_at_ms != credential.expires_at_ms
        || !bool::from(record.token_digest.ct_eq(&digest(&credential.token)))
    {
        return Err(TransportError::Unauthenticated);
    }
    if record.expires_at_ms <= now_ms {
        return Err(TransportError::Unauthenticated);
    }
    Ok(())
}

fn sweep_expired(state: &mut State, now_ms: u64) {
    let expired: Vec<_> = state
        .connections
        .iter()
        .filter(|(_, connection)| connection.expires_at_ms <= now_ms)
        .map(|(id, _)| id.clone())
        .collect();
    for id in expired {
        if let Some(connection) = state.connections.remove(&id) {
            state.active_requests = state
                .active_requests
                .saturating_sub(connection.pending.len());
            for cancellation in connection.pending.values() {
                cancellation.cancel();
            }
        }
    }
}

fn digest(value: &str) -> [u8; 32] {
    Sha256::digest(value.as_bytes()).into()
}

fn constant_equal(left: &str, right: &str) -> bool {
    let left = digest(left);
    let right = digest(right);
    bool::from(left.ct_eq(&right))
}

fn opaque_id(value: &str) -> bool {
    !value.is_empty()
        && value.len() <= 128
        && value
            .bytes()
            .all(|byte| byte.is_ascii_alphanumeric() || matches!(byte, b'.' | b'_' | b'~' | b'-'))
}
