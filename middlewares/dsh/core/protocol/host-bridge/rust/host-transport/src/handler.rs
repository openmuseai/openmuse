use async_trait::async_trait;
use serde_json::Value;
use tokio::sync::mpsc;

use crate::{TransportCancellation, TransportError};

#[async_trait]
pub trait DesktopRequestHandler: Send + Sync + 'static {
    async fn handle(
        &self,
        message: Value,
        runtime_instance_id: &str,
        cancellation: TransportCancellation,
    ) -> Result<Value, TransportError>;

    async fn stream(
        &self,
        _message: Value,
        _runtime_instance_id: &str,
        _cancellation: TransportCancellation,
    ) -> Result<DesktopEventReceiver, TransportError> {
        Err(TransportError::Unavailable)
    }
}

pub type DesktopEventReceiver = mpsc::Receiver<Result<Value, TransportError>>;
