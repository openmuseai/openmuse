#[derive(Debug, Clone, PartialEq, Eq)]
pub enum InvalidationCause {
    ProviderDisposed,
    AuthorityChanged,
    Expired,
    Shutdown,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum RegistryEventKind {
    ProviderRegistered {
        provider_instance_id: String,
        registration_generation: u64,
    },
    ProviderRevoked {
        provider_instance_id: String,
    },
    BindingInvalidated {
        binding_id: String,
        cause: InvalidationCause,
    },
    Shutdown,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RegistryEvent {
    pub host_generation: String,
    pub registry_revision: u64,
    pub occurred_at_ms: u64,
    pub kind: RegistryEventKind,
}
