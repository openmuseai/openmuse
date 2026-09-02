use async_trait::async_trait;
use std::collections::BTreeMap;

use crate::error::AuthorityError;

/// Host-authenticated caller; never derive this value from a Bridge payload.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AuthoritativeCaller {
    pub actor_ref: String,
}

/// Untrusted routing hint supplied by a consumer.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct ScopeHint {
    pub refs: BTreeMap<String, String>,
}

/// Opaque Host result. `evidence` is adapter-private and must never be put on the wire.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ResolvedHostContext {
    pub actor_ref: String,
    pub scope_ref: String,
    pub authority_epoch: u64,
    pub evidence: BTreeMap<String, String>,
}

#[async_trait]
pub trait AuthorityResolver: Send + Sync + 'static {
    async fn resolve(
        &self,
        caller: &AuthoritativeCaller,
        hint: Option<&ScopeHint>,
    ) -> Result<ResolvedHostContext, AuthorityError>;

    async fn revalidate(
        &self,
        caller: &AuthoritativeCaller,
        previous: &ResolvedHostContext,
    ) -> Result<ResolvedHostContext, AuthorityError>;
}
