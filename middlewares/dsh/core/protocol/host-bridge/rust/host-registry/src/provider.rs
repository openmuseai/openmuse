use async_trait::async_trait;
use serde_json::Value;

use crate::{authority::ResolvedHostContext, descriptor::ProviderDescriptor};

/// A deliberately tiny cancellation primitive. Stage 03 maps transport cancel/connection abort to it.
#[derive(Debug, Clone, Default)]
pub struct Cancellation(pub(crate) std::sync::Arc<std::sync::atomic::AtomicBool>);

impl Cancellation {
    pub fn cancel(&self) {
        self.0.store(true, std::sync::atomic::Ordering::Release);
    }
    pub fn is_cancelled(&self) -> bool {
        self.0.load(std::sync::atomic::Ordering::Acquire)
    }
}

#[derive(Debug, Clone)]
pub struct ProviderInvocation {
    /// Host-minted binding identity. Providers may use it to bind private
    /// proposals to the exact capability lease without exposing it to Plugins.
    pub binding_id: String,
    pub operation_id: String,
    pub input: Value,
    /// Present only for operations whose descriptor declares idempotency.
    pub idempotency_key: Option<String>,
    pub deadline_at_ms: u64,
}

#[derive(Debug, Clone)]
pub struct ProviderFailure;

#[async_trait]
pub trait CapabilityProvider: Send + Sync + 'static {
    fn descriptor(&self) -> ProviderDescriptor;
    async fn available(&self, context: &ResolvedHostContext) -> Result<bool, ProviderFailure>;
    async fn invoke(
        &self,
        invocation: ProviderInvocation,
        context: ResolvedHostContext,
        cancellation: Cancellation,
    ) -> Result<Value, ProviderFailure>;
}
