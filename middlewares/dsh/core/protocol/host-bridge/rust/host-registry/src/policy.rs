use async_trait::async_trait;
use serde_json::Value;
use std::fmt::Debug;

use crate::{
    AuthoritativeCaller, BindingProjection, Effect, OperationDescriptor, RegistryError,
    ResolvedHostContext,
};

#[derive(Debug)]
pub struct InvocationAdmission<'a> {
    pub caller: &'a AuthoritativeCaller,
    pub context: &'a ResolvedHostContext,
    pub binding: &'a BindingProjection,
    pub operation: &'a OperationDescriptor,
    pub input: &'a Value,
    pub grant_id: Option<&'a str>,
    pub idempotency_key: Option<&'a str>,
    pub session_ref: Option<&'a str>,
    pub tool_call_ref: Option<&'a str>,
    pub deadline_at_ms: u64,
}

#[async_trait]
pub trait InvocationAuthorizer: Debug + Send + Sync + 'static {
    async fn authorize(&self, admission: InvocationAdmission<'_>) -> Result<(), RegistryError>;
}

/// Secure default: read operations remain usable; every write requires an
/// explicitly composed Host policy implementation.
#[derive(Debug)]
pub struct DenyWrites;

#[async_trait]
impl InvocationAuthorizer for DenyWrites {
    async fn authorize(&self, admission: InvocationAdmission<'_>) -> Result<(), RegistryError> {
        if admission.operation.effect == Effect::Read {
            Ok(())
        } else {
            Err(RegistryError::GrantRequired)
        }
    }
}
