use std::{
    collections::VecDeque,
    sync::{
        Arc, Mutex,
        atomic::{AtomicU64, Ordering},
    },
};

use muse_host_transport::*;
use async_trait::async_trait;
use serde_json::{Value, json};
use tokio::io::{AsyncWriteExt, duplex};

#[cfg(unix)]
use std::{
    os::unix::fs::PermissionsExt,
    time::{SystemTime, UNIX_EPOCH},
};
#[cfg(unix)]
use tokio::net::UnixStream;

#[cfg(unix)]
struct SameProcessPeer;
#[cfg(unix)]
impl PeerAuthenticator for SameProcessPeer {
    fn authorize(&self, peer: &PeerIdentity, _: &str) -> bool {
        peer.principal == format!("uid.{}", unsafe { libc::geteuid() })
    }
}

#[cfg(unix)]
struct StreamingHandler;
#[cfg(unix)]
#[async_trait]
impl DesktopRequestHandler for StreamingHandler {
    async fn handle(
        &self,
        message: Value,
        _: &str,
        _: TransportCancellation,
    ) -> Result<Value, TransportError> {
        Ok(message)
    }

    async fn stream(
        &self,
        message: Value,
        _: &str,
        cancellation: TransportCancellation,
    ) -> Result<DesktopEventReceiver, TransportError> {
        let (sender, receiver) = tokio::sync::mpsc::channel(2);
        tokio::spawn(async move {
            let _ = sender.send(Ok(json!({"sequence": 1, "request": message}))).await;
            let _ = sender.send(Ok(json!({"sequence": 2}))).await;
            cancellation.cancelled().await;
        });
        Ok(receiver)
    }
}

struct AllowPeer;
impl PeerAuthenticator for AllowPeer {
    fn authorize(&self, peer: &PeerIdentity, _: &str) -> bool {
        peer.principal == "uid.501"
    }
}

#[derive(Debug)]
struct TestClock(AtomicU64);
impl TransportClock for TestClock {
    fn now_ms(&self) -> u64 {
        self.0.load(Ordering::Acquire)
    }
}
impl TestClock {
    fn set(&self, value: u64) {
        self.0.store(value, Ordering::Release);
    }
}

struct Secrets(Mutex<VecDeque<String>>);
impl Secrets {
    fn new() -> Self {
        Self(Mutex::new(
            (1..32).map(|value| format!("secret.{value}")).collect(),
        ))
    }
}
impl SecretSource for Secrets {
    fn secret(&self) -> String {
        self.0.lock().unwrap().pop_front().unwrap()
    }
}

fn endpoint() -> DesktopEndpoint {
    DesktopEndpoint {
        kind: DesktopCarrierKind::UnixDomainSocket,
        address: "/tmp/muse-test.sock".into(),
    }
}
fn config(clock: Arc<dyn TransportClock>) -> TransportConfig {
    TransportConfig {
        endpoint: endpoint(),
        host_generation: "host.1".into(),
        connection_ttl_ms: 100,
        max_deadline_horizon_ms: 1_000,
        max_connections: 2,
        max_concurrent_requests: 1,
        max_payload_bytes: 1024,
        max_response_bytes: 2048,
        clock,
    }
}
fn manager() -> TransportSessionManager {
    manager_with_clock().0
}
fn manager_with_clock() -> (TransportSessionManager, Arc<TestClock>) {
    let clock = Arc::new(TestClock(AtomicU64::new(10)));
    let manager = TransportSessionManager::new(
        config(clock.clone()),
        Arc::new(AllowPeer),
        Arc::new(Secrets::new()),
    )
    .unwrap();
    (manager, clock)
}
fn peer(principal: &str) -> PeerIdentity {
    PeerIdentity {
        carrier: DesktopCarrierKind::UnixDomainSocket,
        principal: principal.into(),
    }
}
fn proof(manager: &TransportSessionManager, runtime: &str) -> RuntimeProof {
    RuntimeProof {
        runtime_instance_id: runtime.into(),
        nonce: manager.launch().nonce,
    }
}

#[test]
fn nonce_and_carrier_peer_are_both_required() {
    let manager = manager();
    assert_eq!(
        manager.connect(
            &RuntimeProof {
                runtime_instance_id: "runtime.1".into(),
                nonce: "wrong".into()
            },
            &peer("uid.501")
        ),
        Err(TransportError::Unauthenticated)
    );
    assert_eq!(
        manager.connect(&proof(&manager, "runtime.1"), &peer("uid.502")),
        Err(TransportError::Forbidden)
    );
    assert!(
        manager
            .connect(&proof(&manager, "runtime.1"), &peer("uid.501"))
            .is_ok()
    );
}

#[test]
fn credential_is_runtime_generation_token_and_expiry_bound() {
    let (manager, clock) = manager_with_clock();
    let credential = manager
        .connect(&proof(&manager, "runtime.1"), &peer("uid.501"))
        .unwrap();
    let mut forged = credential.clone();
    forged.runtime_instance_id = "runtime.2".into();
    assert!(matches!(
        manager.admit(&forged, 10, 50),
        Err(TransportError::Unauthenticated)
    ));
    let mut stale = credential.clone();
    stale.host_generation = "host.2".into();
    assert!(matches!(
        manager.admit(&stale, 10, 50),
        Err(TransportError::HostGenerationStale)
    ));
    clock.set(110);
    assert!(matches!(
        manager.admit(&credential, 10, 200),
        Err(TransportError::Unauthenticated)
    ));
}

#[test]
fn permits_enforce_limits_and_release_on_drop() {
    let manager = manager();
    let credential = manager
        .connect(&proof(&manager, "runtime.1"), &peer("uid.501"))
        .unwrap();
    assert_eq!(
        manager.admit(&credential, 0, 50).err().unwrap(),
        TransportError::InvalidFrame
    );
    assert_eq!(
        manager.admit(&credential, 10, 10).err().unwrap(),
        TransportError::DeadlineExceeded
    );
    assert_eq!(
        manager.admit(&credential, 10, 1_011).err().unwrap(),
        TransportError::InvalidFrame
    );
    let permit = manager.admit(&credential, 10, 50).unwrap();
    assert_eq!(
        manager.admit(&credential, 10, 50).err().unwrap(),
        TransportError::RateLimited
    );
    drop(permit);
    assert!(manager.admit(&credential, 10, 50).is_ok());
}

#[tokio::test]
async fn disconnect_and_shutdown_cancel_pending_requests() {
    let manager = manager();
    let first = manager
        .connect(&proof(&manager, "runtime.1"), &peer("uid.501"))
        .unwrap();
    let permit = manager.admit(&first, 10, 50).unwrap();
    let cancellation = permit.cancellation.clone();
    manager.disconnect(&first);
    cancellation.cancelled().await;
    assert!(cancellation.is_cancelled());

    let second = manager
        .connect(&proof(&manager, "runtime.2"), &peer("uid.501"))
        .unwrap();
    let permit = manager.admit(&second, 10, 50).unwrap();
    let cancellation = permit.cancellation.clone();
    manager.close();
    cancellation.cancelled().await;
    assert!(cancellation.is_cancelled());
    assert!(matches!(
        manager.connect(&proof(&manager, "runtime.3"), &peer("uid.501")),
        Err(TransportError::Unavailable)
    ));
}

#[test]
fn response_size_is_bounded() {
    let manager = manager();
    assert_eq!(manager.validate_response(0), Err(TransportError::Handler));
    assert_eq!(
        manager.validate_response(2049),
        Err(TransportError::Handler)
    );
    assert_eq!(manager.validate_response(2048), Ok(()));
}

#[tokio::test]
async fn length_delimited_frames_roundtrip_and_reject_oversize() {
    let (mut writer, mut reader) = duplex(64);
    let writing = tokio::spawn(async move { write_frame(&mut writer, b"hello", 16).await });
    assert_eq!(read_frame(&mut reader, 16).await.unwrap(), b"hello");
    writing.await.unwrap().unwrap();

    let (mut writer, mut reader) = duplex(64);
    writer.write_all(&100_u32.to_be_bytes()).await.unwrap();
    assert_eq!(
        read_frame(&mut reader, 16).await,
        Err(TransportError::InvalidFrame)
    );
    assert_eq!(
        write_frame(&mut writer, b"too-large", 4).await,
        Err(TransportError::InvalidFrame)
    );
}

#[cfg(unix)]
#[tokio::test]
async fn uds_endpoint_is_private_and_peer_identity_comes_from_the_kernel() {
    let unique = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_nanos();
    let path = std::env::temp_dir().join(format!("muse-{}-{unique}.sock", std::process::id()));
    let endpoint = DesktopEndpoint {
        kind: DesktopCarrierKind::UnixDomainSocket,
        address: path.to_string_lossy().into_owned(),
    };
    let carrier = UnixDomainSocketCarrier::bind(&endpoint).unwrap();
    assert_eq!(
        std::fs::metadata(&path).unwrap().permissions().mode() & 0o777,
        0o600
    );
    let accepting = tokio::spawn(async move {
        let accepted = carrier.accept().await;
        (accepted, carrier)
    });
    let client = UnixStream::connect(&path).await.unwrap();
    let (accepted, carrier) = accepting.await.unwrap();
    let (_, peer) = accepted.unwrap();
    assert_eq!(
        peer.principal,
        format!("uid.{}", unsafe { libc::geteuid() })
    );
    drop(client);
    drop(carrier);
    assert!(!path.exists());
}

#[cfg(unix)]
#[tokio::test]
async fn desktop_server_streams_multiple_authenticated_frames_and_cancels_on_disconnect() {
    let unique = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_nanos();
    let path = std::env::temp_dir().join(format!(
        "muse-stream-{}-{unique}.sock",
        std::process::id()
    ));
    let server = DesktopHostServer::start(
        TransportConfig {
            endpoint: DesktopEndpoint {
                kind: DesktopCarrierKind::UnixDomainSocket,
                address: path.to_string_lossy().into_owned(),
            },
            host_generation: "host.1".into(),
            connection_ttl_ms: 60_000,
            max_deadline_horizon_ms: 60_000,
            max_connections: 2,
            max_concurrent_requests: 2,
            max_payload_bytes: 4096,
            max_response_bytes: 4096,
            clock: Arc::new(SystemTransportClock),
        },
        Arc::new(SameProcessPeer),
        Arc::new(SystemSecretSource),
        Arc::new(StreamingHandler),
    )
    .unwrap();

    let mut connect = UnixStream::connect(&path).await.unwrap();
    let connect_request = serde_json::to_vec(&json!({
        "type": "connect",
        "proof": {
            "runtimeInstanceId": "runtime.1",
            "nonce": server.launch().nonce
        }
    }))
    .unwrap();
    write_frame(&mut connect, &connect_request, 4096).await.unwrap();
    let response: Value =
        serde_json::from_slice(&read_frame(&mut connect, 4096).await.unwrap()).unwrap();
    let credential = response["connection"].clone();
    drop(connect);

    let mut stream = UnixStream::connect(&path).await.unwrap();
    let stream_request = serde_json::to_vec(&json!({
        "type": "stream",
        "connection": credential,
        "deadlineAt": system_test_now_ms() + 30_000,
        "message": {"kind": "subscribe.request"}
    }))
    .unwrap();
    write_frame(&mut stream, &stream_request, 4096).await.unwrap();
    let first: Value =
        serde_json::from_slice(&read_frame(&mut stream, 4096).await.unwrap()).unwrap();
    let second: Value =
        serde_json::from_slice(&read_frame(&mut stream, 4096).await.unwrap()).unwrap();
    assert_eq!(first["message"]["sequence"], 1);
    assert_eq!(second["message"]["sequence"], 2);
    drop(stream);
    server.shutdown().await;
    assert!(!path.exists());
}

#[cfg(unix)]
fn system_test_now_ms() -> u64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default()
        .as_millis()
        .min(u64::MAX as u128) as u64
}

#[cfg(windows)]
struct LocalPipePeer;
#[cfg(windows)]
impl PeerAuthenticator for LocalPipePeer {
    fn authorize(&self, peer: &PeerIdentity, _: &str) -> bool {
        peer.carrier == DesktopCarrierKind::WindowsNamedPipe
    }
}

#[cfg(windows)]
struct EchoHandler;
#[cfg(windows)]
#[async_trait]
impl DesktopRequestHandler for EchoHandler {
    async fn handle(
        &self,
        message: Value,
        _: &str,
        _: TransportCancellation,
    ) -> Result<Value, TransportError> {
        Ok(message)
    }
}

#[cfg(windows)]
#[tokio::test]
async fn named_pipe_desktop_server_exchanges_an_authenticated_unary() {
    let address = format!(
        r"\\.\pipe\muse-host-test-{}-{}",
        std::process::id(),
        std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_nanos()
    );
    let server = DesktopHostServer::start(
        TransportConfig {
            endpoint: DesktopEndpoint {
                kind: DesktopCarrierKind::WindowsNamedPipe,
                address: address.clone(),
            },
            host_generation: "host.1".into(),
            connection_ttl_ms: 60_000,
            max_deadline_horizon_ms: 60_000,
            max_connections: 2,
            max_concurrent_requests: 2,
            max_payload_bytes: 4096,
            max_response_bytes: 4096,
            clock: Arc::new(SystemTransportClock),
        },
        Arc::new(LocalPipePeer),
        Arc::new(SystemSecretSource),
        Arc::new(EchoHandler),
    )
    .unwrap();

    let mut client = {
        let mut opened = None;
        for _ in 0..50 {
            match tokio::net::windows::named_pipe::ClientOptions::new().open(&address) {
                Ok(client) => {
                    opened = Some(client);
                    break;
                }
                Err(_) => tokio::time::sleep(std::time::Duration::from_millis(20)).await,
            }
        }
        opened.expect("named pipe listener should accept a local client")
    };
    let connect_request = serde_json::to_vec(&json!({
        "type": "connect",
        "proof": {
            "runtimeInstanceId": "runtime.1",
            "nonce": server.launch().nonce
        }
    }))
    .unwrap();
    write_frame(&mut client, &connect_request, 4096).await.unwrap();
    let response: Value =
        serde_json::from_slice(&read_frame(&mut client, 4096).await.unwrap()).unwrap();
    assert_eq!(response["ok"], true);
    assert!(response["connection"]["token"].as_str().is_some());
    server.shutdown().await;
}

#[test]
fn windows_named_pipe_address_shape_is_stable() {
    assert!(windows_named_pipe_address_valid(r"\\.\pipe\appflowy-muse-host-1"));
    assert!(!windows_named_pipe_address_valid(r"\\.\pipe\"));
    assert!(!windows_named_pipe_address_valid("/tmp/appflowy-muse-host-0.sock"));
    assert!(!windows_named_pipe_address_valid("pipe\\muse"));
}

fn idempotency_identity(fingerprint: &str) -> IdempotencyIdentity {
    IdempotencyIdentity {
        runtime_instance_id: "runtime.1".into(),
        binding_id: "binding.1".into(),
        operation_id: "sample.write".into(),
        key: "idem.1".into(),
        input_fingerprint: fingerprint.into(),
    }
}

#[test]
fn idempotency_replays_completion_and_never_runs_a_concurrent_duplicate() {
    let registry = IdempotencyRegistry::new(IdempotencyConfig {
        max_entries: 2,
        ttl_ms: 100,
    })
    .unwrap();
    let permit = match registry.begin(idempotency_identity("input.1"), 10).unwrap() {
        IdempotencyDecision::Execute(permit) => permit,
        IdempotencyDecision::Replay(_) => panic!("first request cannot replay"),
    };
    assert!(matches!(
        registry.begin(idempotency_identity("input.1"), 11),
        Err(TransportError::RateLimited)
    ));
    permit.complete(b"result".to_vec(), 12).unwrap();
    match registry.begin(idempotency_identity("input.1"), 13).unwrap() {
        IdempotencyDecision::Replay(value) => assert_eq!(value, b"result"),
        IdempotencyDecision::Execute(_) => panic!("completed request must replay"),
    }
    assert!(matches!(
        registry.begin(idempotency_identity("input.2"), 13),
        Err(TransportError::InvalidFrame)
    ));
}

#[test]
fn failed_or_expired_idempotency_execution_can_run_again() {
    let registry = IdempotencyRegistry::new(IdempotencyConfig {
        max_entries: 1,
        ttl_ms: 10,
    })
    .unwrap();
    let permit = match registry.begin(idempotency_identity("input.1"), 10).unwrap() {
        IdempotencyDecision::Execute(permit) => permit,
        IdempotencyDecision::Replay(_) => unreachable!(),
    };
    drop(permit);
    let permit = match registry.begin(idempotency_identity("input.1"), 11).unwrap() {
        IdempotencyDecision::Execute(permit) => permit,
        IdempotencyDecision::Replay(_) => unreachable!(),
    };
    permit.complete(b"result".to_vec(), 12).unwrap();
    assert!(matches!(
        registry.begin(idempotency_identity("input.1"), 22),
        Ok(IdempotencyDecision::Execute(_))
    ));
}
