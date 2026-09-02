use muse_host_bridge_contract::{
    JsonLimits, ProtocolFragment, ProtocolSchemas, ProviderSchemaValidator, canonicalize,
    digest_schema,
};
use serde_json::{Value, json};

use crate::error::RegistryError;

const SCHEMA_LIMITS: JsonLimits = JsonLimits {
    max_depth: 64,
    max_container_children: 10_000,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Effect {
    Read,
    LocalWrite,
    SyncWrite,
    ExternalSideEffect,
}
impl Effect {
    fn wire(self) -> &'static str {
        match self {
            Self::Read => "read",
            Self::LocalWrite => "local_write",
            Self::SyncWrite => "sync_write",
            Self::ExternalSideEffect => "external_side_effect",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Idempotency {
    None,
    Optional,
    Required,
}
impl Idempotency {
    fn wire(self) -> &'static str {
        match self {
            Self::None => "none",
            Self::Optional => "optional",
            Self::Required => "required",
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SchemaDocument {
    pub value: Value,
    pub digest: String,
}
impl SchemaDocument {
    pub fn new(value: Value) -> Result<Self, RegistryError> {
        let digest =
            digest_schema(&value, SCHEMA_LIMITS).map_err(|_| RegistryError::InvalidSchema)?;
        Ok(Self { value, digest })
    }
    fn wire(&self) -> Result<Value, RegistryError> {
        let bytes = canonicalize(&self.value, SCHEMA_LIMITS)
            .map_err(|_| RegistryError::InvalidSchema)?
            .len();
        Ok(
            json!({"sha256": self.digest, "byteLength": bytes, "draft": "2020-12", "inline": self.value}),
        )
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct OperationDescriptor {
    pub operation_id: String,
    pub effect: Effect,
    pub input_schema: SchemaDocument,
    pub output_schema: SchemaDocument,
    pub cancellable: bool,
    pub idempotency: Idempotency,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ProviderDescriptor {
    pub descriptor_id: String,
    pub revision: String,
    pub family_id: String,
    pub contract_major: u16,
    pub contract_minor: u16,
    pub operations: Vec<OperationDescriptor>,
    pub title: Option<String>,
    pub summary: Option<String>,
}

pub(crate) struct CompiledOperationSchemas {
    pub input: ProviderSchemaValidator,
    pub output: ProviderSchemaValidator,
}

impl ProviderDescriptor {
    pub(crate) fn validate_and_compile(
        &self,
        protocol: &ProtocolSchemas,
        provider_instance_id: &str,
    ) -> Result<Vec<CompiledOperationSchemas>, RegistryError> {
        if !valid_opaque(&self.descriptor_id)
            || !valid_opaque(&self.revision)
            || !valid_name(&self.family_id)
            || self.contract_major == 0
            || self.operations.is_empty()
            || self.operations.len() > 256
            || self.title.as_ref().is_some_and(|value| value.len() > 256)
            || self
                .summary
                .as_ref()
                .is_some_and(|value| value.len() > 2048)
        {
            return Err(RegistryError::InvalidDescriptor);
        }
        let mut names = std::collections::BTreeSet::new();
        let mut compiled = Vec::with_capacity(self.operations.len());
        for operation in &self.operations {
            if !valid_name(&operation.operation_id) || !names.insert(&operation.operation_id) {
                return Err(RegistryError::InvalidDescriptor);
            }
            for schema in [&operation.input_schema, &operation.output_schema] {
                let actual = digest_schema(&schema.value, SCHEMA_LIMITS)
                    .map_err(|_| RegistryError::InvalidSchema)?;
                if actual != schema.digest {
                    return Err(RegistryError::InvalidSchema);
                }
            }
            compiled.push(CompiledOperationSchemas {
                input: protocol
                    .compile_provider_schema(&operation.input_schema.value)
                    .map_err(|_| RegistryError::InvalidSchema)?,
                output: protocol
                    .compile_provider_schema(&operation.output_schema.value)
                    .map_err(|_| RegistryError::InvalidSchema)?,
            });
        }
        protocol
            .validate_fragment(
                ProtocolFragment::DiscoverDescriptor,
                &self.to_wire(provider_instance_id)?,
            )
            .map_err(|_| RegistryError::InvalidDescriptor)?;
        Ok(compiled)
    }
    pub fn operation(&self, operation_id: &str) -> Option<&OperationDescriptor> {
        self.operations
            .iter()
            .find(|value| value.operation_id == operation_id)
    }
    /// Domain-neutral Bridge v1 descriptor projection used by Host request routers.
    pub fn to_wire(&self, provider_instance_id: &str) -> Result<Value, RegistryError> {
        let operations = self.operations.iter().map(|operation| Ok(json!({
            "operationId": operation.operation_id, "effect": operation.effect.wire(),
            "inputSchema": operation.input_schema.wire()?, "outputSchema": operation.output_schema.wire()?,
            "cancellable": operation.cancellable, "idempotency": operation.idempotency.wire(),
        }))).collect::<Result<Vec<_>, RegistryError>>()?;
        let mut value = json!({
            "descriptorId": self.descriptor_id, "revision": self.revision, "familyId": self.family_id,
            "contractVersion": {"major": self.contract_major, "minor": self.contract_minor},
            "providerInstanceId": provider_instance_id, "operations": operations, "events": [],
        });
        if let Some(title) = &self.title {
            value["title"] = json!(title);
        }
        if let Some(summary) = &self.summary {
            value["summary"] = json!(summary);
        }
        Ok(value)
    }
}

pub fn validate_json(value: &Value) -> Result<(), RegistryError> {
    muse_host_bridge_contract::validate_lossless(value, SCHEMA_LIMITS)
        .map_err(|_| RegistryError::InvalidJson)
}
fn valid_opaque(value: &str) -> bool {
    !value.is_empty()
        && value.len() <= 128
        && value
            .bytes()
            .all(|byte| byte.is_ascii_alphanumeric() || matches!(byte, b'.' | b'_' | b'~' | b'-'))
}
fn valid_name(value: &str) -> bool {
    value.len() >= 3
        && value.len() <= 128
        && value.as_bytes().first().is_some_and(u8::is_ascii_lowercase)
        && value.contains('.')
        && value.bytes().all(|byte| {
            byte.is_ascii_lowercase() || byte.is_ascii_digit() || matches!(byte, b'.' | b'_' | b'-')
        })
}
