#[derive(Debug, thiserror::Error, Clone, PartialEq, Eq)]
pub enum AuthorityError {
    #[error("caller is not authorized for the requested scope")]
    Denied,
    #[error("authoritative Host state is unavailable")]
    Unavailable,
}

#[derive(Debug, thiserror::Error, Clone, PartialEq, Eq)]
pub enum RegistryError {
    #[error("provider descriptor is invalid")]
    InvalidDescriptor,
    #[error("descriptor identity is already registered")]
    DuplicateDescriptor,
    #[error("provider JSON Schema is invalid or its digest does not match")]
    InvalidSchema,
    #[error("descriptor was not found or is not visible")]
    DescriptorUnavailable,
    #[error("descriptor revision is stale")]
    StaleDescriptor,
    #[error("discovery cursor is expired, stale, or belongs to another authority context")]
    CursorExpired,
    #[error("registry resource limit exceeded")]
    ResourceLimit,
    #[error("binding was not found or has expired")]
    BindingUnavailable,
    #[error("binding no longer matches authoritative Host state")]
    BindingInvalid,
    #[error("operation is not part of this descriptor or binding")]
    OperationUnavailable,
    #[error("provider registration is no longer active")]
    ProviderRevoked,
    #[error("caller input or provider output is outside the lossless JSON subset")]
    InvalidJson,
    #[error("invocation input does not satisfy the operation schema")]
    InvalidInput,
    #[error("provider output does not satisfy the operation schema")]
    InvalidOutput,
    #[error("provider rejected the invocation")]
    ProviderFailure,
    #[error("write operation requires a valid Host grant")]
    GrantRequired,
    #[error("Host policy rejected the invocation grant or correlation")]
    GrantInvalid,
    #[error("authority resolution failed: {0}")]
    Authority(#[from] AuthorityError),
}
