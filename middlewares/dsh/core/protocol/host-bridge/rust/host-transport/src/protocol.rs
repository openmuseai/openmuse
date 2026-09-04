use std::sync::Arc;

use serde_json::{Value, json};
use tokio::time::{Duration, sleep};

use crate::{
    ConnectionCredential, DesktopRequestHandler, HostByteStream, PeerIdentity, RuntimeProof,
    TransportError, TransportSessionManager, read_frame, write_frame,
};

enum ControlSuccess {
    Connected(ConnectionCredential),
    Disconnected,
    Failure(TransportError),
}

pub(crate) async fn serve_one<S: HostByteStream>(
    mut stream: S,
    peer: PeerIdentity,
    sessions: TransportSessionManager,
    handler: Arc<dyn DesktopRequestHandler>,
    max_payload: usize,
    max_response: usize,
) -> Result<(), TransportError> {
    let frame = read_frame(&mut stream, max_payload).await?;
    let request: Value =
        serde_json::from_slice(&frame).map_err(|_| TransportError::InvalidFrame)?;
    if request.get("type").and_then(Value::as_str) == Some("stream") {
        return serve_stream(
            &mut stream,
            request,
            &sessions,
            handler,
            max_payload,
            max_response,
        )
        .await;
    }
    let response = match dispatch_control(request, &peer, &sessions, handler).await {
        Ok(value) => json!({"ok": true, "message": value}),
        Err(ControlSuccess::Connected(credential)) => {
            json!({"ok": true, "connection": credential_wire(&credential)})
        }
        Err(ControlSuccess::Disconnected) => json!({"ok": true}),
        Err(ControlSuccess::Failure(error)) => error_wire(error),
    };
    let bytes = serde_json::to_vec(&response).map_err(|_| TransportError::Handler)?;
    sessions.validate_response(bytes.len())?;
    write_frame(&mut stream, &bytes, max_response).await
}

async fn serve_stream<S: HostByteStream>(
    stream: &mut S,
    request: Value,
    sessions: &TransportSessionManager,
    handler: Arc<dyn DesktopRequestHandler>,
    max_payload: usize,
    max_response: usize,
) -> Result<(), TransportError> {
    let credential = credential(
        request
            .get("connection")
            .ok_or(TransportError::InvalidFrame)?,
    )
    .map_err(control_error)?;
    let deadline = request
        .get("deadlineAt")
        .and_then(Value::as_u64)
        .ok_or(TransportError::InvalidFrame)?;
    let message = request
        .get("message")
        .cloned()
        .ok_or(TransportError::InvalidFrame)?;
    let payload_bytes = serde_json::to_vec(&message)
        .map_err(|_| TransportError::InvalidFrame)?
        .len();
    if payload_bytes == 0 || payload_bytes > max_payload {
        return Err(TransportError::InvalidFrame);
    }
    let permit = sessions.admit(&credential, payload_bytes, deadline)?;
    let cancellation = permit.cancellation.clone();
    let mut receiver = match handler
        .stream(
            message,
            &credential.runtime_instance_id,
            cancellation.clone(),
        )
        .await
    {
        Ok(receiver) => receiver,
        Err(error) => {
            write_control_frame(stream, error_wire(error), sessions, max_response).await?;
            return Ok(());
        }
    };
    let deadline_wait = deadline.saturating_sub(system_now_ms());
    let deadline_sleep = sleep(Duration::from_millis(deadline_wait));
    tokio::pin!(deadline_sleep);
    let mut probe = [0_u8; 1];
    loop {
        tokio::select! {
            _ = cancellation.cancelled() => break,
            _ = &mut deadline_sleep => {
                cancellation.cancel();
                break;
            }
            readable = stream.wait_readable() => {
                if readable.is_err() { break; }
                match stream.try_read_now(&mut probe) {
                    Ok(0) => break,
                    Ok(_) => return Err(TransportError::InvalidFrame),
                    Err(error) if error.kind() == std::io::ErrorKind::WouldBlock => {},
                    Err(_) => break,
                }
            }
            item = receiver.recv() => {
                match item {
                    Some(Ok(message)) => {
                        write_control_frame(stream, json!({"ok": true, "message": message}), sessions, max_response).await?;
                    }
                    Some(Err(error)) => {
                        write_control_frame(stream, error_wire(error), sessions, max_response).await?;
                        break;
                    }
                    None => break,
                }
            }
        }
    }
    cancellation.cancel();
    drop(permit);
    Ok(())
}

async fn write_control_frame<S: HostByteStream>(
    stream: &mut S,
    response: Value,
    sessions: &TransportSessionManager,
    max_response: usize,
) -> Result<(), TransportError> {
    let bytes = serde_json::to_vec(&response).map_err(|_| TransportError::Handler)?;
    sessions.validate_response(bytes.len())?;
    write_frame(stream, &bytes, max_response).await
}

fn control_error(value: ControlSuccess) -> TransportError {
    match value {
        ControlSuccess::Failure(error) => error,
        ControlSuccess::Connected(_) | ControlSuccess::Disconnected => TransportError::InvalidFrame,
    }
}

fn system_now_ms() -> u64 {
    std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap_or_default()
        .as_millis()
        .min(u64::MAX as u128) as u64
}

async fn dispatch_control(
    request: Value,
    peer: &PeerIdentity,
    sessions: &TransportSessionManager,
    handler: Arc<dyn DesktopRequestHandler>,
) -> Result<Value, ControlSuccess> {
    let kind = request
        .get("type")
        .and_then(Value::as_str)
        .ok_or(ControlSuccess::Failure(TransportError::InvalidFrame))?;
    match kind {
        "connect" => {
            let proof = request
                .get("proof")
                .ok_or(ControlSuccess::Failure(TransportError::InvalidFrame))?;
            let proof = RuntimeProof {
                runtime_instance_id: string(proof, "runtimeInstanceId")?,
                nonce: string(proof, "nonce")?,
            };
            let credential = sessions
                .connect(&proof, peer)
                .map_err(ControlSuccess::Failure)?;
            Err(ControlSuccess::Connected(credential))
        }
        "disconnect" => {
            let credential = credential(
                request
                    .get("connection")
                    .ok_or(ControlSuccess::Failure(TransportError::InvalidFrame))?,
            )?;
            sessions.disconnect(&credential);
            Err(ControlSuccess::Disconnected)
        }
        "unary" => {
            let credential = credential(
                request
                    .get("connection")
                    .ok_or(ControlSuccess::Failure(TransportError::InvalidFrame))?,
            )?;
            let deadline = request
                .get("deadlineAt")
                .and_then(Value::as_u64)
                .ok_or(ControlSuccess::Failure(TransportError::InvalidFrame))?;
            let message = request
                .get("message")
                .cloned()
                .ok_or(ControlSuccess::Failure(TransportError::InvalidFrame))?;
            let payload_bytes = serde_json::to_vec(&message)
                .map_err(|_| ControlSuccess::Failure(TransportError::InvalidFrame))?
                .len();
            let permit = sessions
                .admit(&credential, payload_bytes, deadline)
                .map_err(ControlSuccess::Failure)?;
            handler
                .handle(
                    message,
                    &credential.runtime_instance_id,
                    permit.cancellation.clone(),
                )
                .await
                .map_err(ControlSuccess::Failure)
        }
        _ => Err(ControlSuccess::Failure(TransportError::InvalidFrame)),
    }
}

fn string(value: &Value, field: &str) -> Result<String, ControlSuccess> {
    value
        .get(field)
        .and_then(Value::as_str)
        .filter(|value| !value.is_empty())
        .map(str::to_owned)
        .ok_or(ControlSuccess::Failure(TransportError::InvalidFrame))
}

fn credential(value: &Value) -> Result<ConnectionCredential, ControlSuccess> {
    Ok(ConnectionCredential {
        connection_id: string(value, "connectionId")?,
        token: string(value, "token")?,
        runtime_instance_id: string(value, "runtimeInstanceId")?,
        host_generation: string(value, "hostGeneration")?,
        expires_at_ms: value
            .get("expiresAt")
            .and_then(Value::as_u64)
            .ok_or(ControlSuccess::Failure(TransportError::InvalidFrame))?,
    })
}

fn credential_wire(value: &ConnectionCredential) -> Value {
    json!({
        "connectionId": value.connection_id, "token": value.token,
        "runtimeInstanceId": value.runtime_instance_id, "hostGeneration": value.host_generation,
        "expiresAt": value.expires_at_ms
    })
}

fn error_wire(error: TransportError) -> Value {
    let code = match error {
        TransportError::Unauthenticated => "UNAUTHENTICATED",
        TransportError::Forbidden => "FORBIDDEN",
        TransportError::HostGenerationStale => "HOST_GENERATION_STALE",
        TransportError::DeadlineExceeded => "DEADLINE_EXCEEDED",
        TransportError::Cancelled => "CANCELLED",
        TransportError::RateLimited => "RATE_LIMITED",
        TransportError::Unavailable => "UNAVAILABLE",
        TransportError::InvalidFrame => "INVALID_ENVELOPE",
        TransportError::Handler => "INTERNAL",
    };
    json!({"ok": false, "error": {"code": code, "message": error.to_string()}})
}
