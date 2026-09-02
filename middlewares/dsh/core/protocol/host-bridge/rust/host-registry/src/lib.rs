mod authority;
mod descriptor;
mod error;
mod event;
mod policy;
mod provider;
mod registry;

pub use authority::{AuthoritativeCaller, AuthorityResolver, ResolvedHostContext, ScopeHint};
pub use descriptor::{
    Effect, Idempotency, OperationDescriptor, ProviderDescriptor, SchemaDocument,
};
pub use error::{AuthorityError, RegistryError};
pub use event::{InvalidationCause, RegistryEvent, RegistryEventKind};
pub use policy::{DenyWrites, InvocationAdmission, InvocationAuthorizer};
pub use provider::{Cancellation, CapabilityProvider, ProviderFailure, ProviderInvocation};
pub use registry::{
    BindingProjection, DescriptorPage, DiscoverQuery, DiscoveredDescriptor, HostCapabilityRegistry,
    InvokeRequest, PolicyContextProjection, RegistrationLease, RegistryClock, RegistryConfig,
    SystemRegistryClock,
};
