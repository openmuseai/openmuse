use muse_host_bridge_contract::{digest::digest_schema, json::JsonLimits};
use serde::{Deserialize, Serialize};
use serde_json::Value;

pub const PLUGIN_DESCRIPTOR_SCHEMA: &str =
    include_str!("../../schemas/v1/plugin-descriptor.schema.json");
pub const CONTEXT_CONTRIBUTION_SCHEMA: &str =
    include_str!("../../schemas/v1/context-contribution.schema.json");
pub const DOMAIN_CHANGE_SCHEMA: &str = include_str!("../../schemas/v1/domain-change.schema.json");
pub const PRESENTATION_INTENT_SCHEMA: &str =
    include_str!("../../schemas/v1/presentation-intent.schema.json");
pub const PRESENTATION_INTENT_RESULT_SCHEMA: &str =
    include_str!("../../schemas/v1/presentation-intent-result.schema.json");

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FacetSchemaKind {
    PluginDescriptor,
    ContextContribution,
    DomainChange,
    PresentationIntent,
    PresentationIntentResult,
}

impl FacetSchemaKind {
    pub fn schema(self) -> &'static str {
        match self {
            Self::PluginDescriptor => PLUGIN_DESCRIPTOR_SCHEMA,
            Self::ContextContribution => CONTEXT_CONTRIBUTION_SCHEMA,
            Self::DomainChange => DOMAIN_CHANGE_SCHEMA,
            Self::PresentationIntent => PRESENTATION_INTENT_SCHEMA,
            Self::PresentationIntentResult => PRESENTATION_INTENT_RESULT_SCHEMA,
        }
    }
}

#[derive(Debug, thiserror::Error)]
pub enum FacetContractError {
    #[error("Facet schema document is invalid")]
    Schema,
    #[error("Facet value does not satisfy the v1 contract")]
    Invalid,
}

pub fn validate(kind: FacetSchemaKind, value: &Value) -> Result<(), FacetContractError> {
    let schema: Value =
        serde_json::from_str(kind.schema()).map_err(|_| FacetContractError::Schema)?;
    let validator = jsonschema::validator_for(&schema).map_err(|_| FacetContractError::Schema)?;
    if validator.is_valid(value) {
        Ok(())
    } else {
        Err(FacetContractError::Invalid)
    }
}

pub fn schema_digest(kind: FacetSchemaKind) -> Result<String, FacetContractError> {
    let schema: Value =
        serde_json::from_str(kind.schema()).map_err(|_| FacetContractError::Schema)?;
    digest_schema(
        &schema,
        JsonLimits {
            max_depth: 64,
            max_container_children: 4096,
        },
    )
    .map_err(|_| FacetContractError::Schema)
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum ContextLane {
    Control,
    State,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum MutationOrigin {
    UiOptimistic,
    RemoteCollab,
    ExternalCommand,
    RecoveryReplay,
    Unknown,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ContextContributionV1 {
    pub protocol: String,
    pub plugin_id: String,
    pub plugin_version: String,
    pub facet_instance_ref: String,
    pub surface_instance_ref: String,
    pub surface_kind: String,
    pub scope_ref: String,
    pub context_type: String,
    pub context_schema_digest: String,
    pub context_revision: String,
    pub epoch_ref: String,
    pub lane: ContextLane,
    pub captured_at: u64,
    pub expires_at: u64,
    pub payload: Value,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DomainChangeV1 {
    pub protocol: String,
    pub plugin_id: String,
    pub provider_instance_ref: String,
    pub scope_ref: String,
    pub resource_ref: String,
    pub event_type: String,
    pub event_schema_digest: String,
    pub domain_revision: String,
    pub epoch_ref: String,
    pub command_ref: Option<String>,
    pub origin: MutationOrigin,
    pub occurred_at: u64,
    pub payload: Value,
}
