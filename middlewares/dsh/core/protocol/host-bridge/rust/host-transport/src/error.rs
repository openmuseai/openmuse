#[derive(Debug, Clone, PartialEq, Eq, thiserror::Error)]
pub enum TransportError {
    #[error("transport authentication failed")]
    Unauthenticated,
    #[error("transport peer is forbidden")]
    Forbidden,
    #[error("transport host generation is stale")]
    HostGenerationStale,
    #[error("transport deadline elapsed")]
    DeadlineExceeded,
    #[error("transport request was cancelled")]
    Cancelled,
    #[error("transport resource limit exceeded")]
    RateLimited,
    #[error("transport is unavailable")]
    Unavailable,
    #[error("transport frame or payload is invalid")]
    InvalidFrame,
    #[error("transport handler failed")]
    Handler,
}
