use crate::{
    ids::{HostSessionId, RequestId},
    json::{JsonBoundaryError, JsonLimits, canonicalize, validate_lossless},
};
use jsonschema::{Draft, Retrieve, Uri, Validator};
use serde::{Deserialize, Serialize};
use serde_json::{Map, Value, json};
use std::{collections::HashMap, sync::Arc};

const ENVELOPE_SCHEMA_ID: &str = "https://muse.dev/schemas/bridge/v1/envelope.schema.json";

const SCHEMA_DOCUMENTS: &[&str] = &[
    include_str!("../../schemas/v1/common.schema.json"),
    include_str!("../../schemas/v1/error.schema.json"),
    include_str!("../../schemas/v1/envelope.schema.json"),
    include_str!("../../schemas/v1/hello.schema.json"),
    include_str!("../../schemas/v1/discover.schema.json"),
    include_str!("../../schemas/v1/bind.schema.json"),
    include_str!("../../schemas/v1/invoke.schema.json"),
    include_str!("../../schemas/v1/subscribe.schema.json"),
    include_str!("../../schemas/v1/policy.schema.json"),
    include_str!("../../schemas/v1/cancel.schema.json"),
    include_str!("../../schemas/v1/status.schema.json"),
];

const MESSAGE_SCHEMA_REFS: &[(&str, &str)] = &[
    (
        "hello.request",
        "https://muse.dev/schemas/bridge/v1/hello.schema.json#/$defs/request",
    ),
    (
        "hello.response",
        "https://muse.dev/schemas/bridge/v1/hello.schema.json#/$defs/response",
    ),
    (
        "discover.request",
        "https://muse.dev/schemas/bridge/v1/discover.schema.json#/$defs/request",
    ),
    (
        "discover.response",
        "https://muse.dev/schemas/bridge/v1/discover.schema.json#/$defs/response",
    ),
    (
        "bind.request",
        "https://muse.dev/schemas/bridge/v1/bind.schema.json#/$defs/request",
    ),
    (
        "bind.response",
        "https://muse.dev/schemas/bridge/v1/bind.schema.json#/$defs/response",
    ),
    (
        "invoke.request",
        "https://muse.dev/schemas/bridge/v1/invoke.schema.json#/$defs/request",
    ),
    (
        "invoke.response",
        "https://muse.dev/schemas/bridge/v1/invoke.schema.json#/$defs/response",
    ),
    (
        "subscribe.request",
        "https://muse.dev/schemas/bridge/v1/subscribe.schema.json#/$defs/request",
    ),
    (
        "subscribe.response",
        "https://muse.dev/schemas/bridge/v1/subscribe.schema.json#/$defs/response",
    ),
    (
        "bridge.event",
        "https://muse.dev/schemas/bridge/v1/subscribe.schema.json#/$defs/event",
    ),
    (
        "policy.evaluate.request",
        "https://muse.dev/schemas/bridge/v1/policy.schema.json#/$defs/evaluateRequest",
    ),
    (
        "policy.evaluate.response",
        "https://muse.dev/schemas/bridge/v1/policy.schema.json#/$defs/evaluateResponse",
    ),
    (
        "policy.finalize.request",
        "https://muse.dev/schemas/bridge/v1/policy.schema.json#/$defs/finalizeRequest",
    ),
    (
        "policy.finalize.response",
        "https://muse.dev/schemas/bridge/v1/policy.schema.json#/$defs/finalizeResponse",
    ),
    (
        "cancel.request",
        "https://muse.dev/schemas/bridge/v1/cancel.schema.json#/$defs/request",
    ),
    (
        "cancel.response",
        "https://muse.dev/schemas/bridge/v1/cancel.schema.json#/$defs/response",
    ),
    (
        "status.request",
        "https://muse.dev/schemas/bridge/v1/status.schema.json#/$defs/request",
    ),
    (
        "status.response",
        "https://muse.dev/schemas/bridge/v1/status.schema.json#/$defs/response",
    ),
];

#[derive(Debug, Clone, Copy)]
pub struct ProtocolLimits {
    pub max_message_bytes: usize,
    pub max_input_bytes: usize,
    pub max_output_bytes: usize,
    pub max_error_details_bytes: usize,
    pub max_event_payload_bytes: usize,
    pub max_json_depth: usize,
    pub max_container_children: usize,
}

impl Default for ProtocolLimits {
    fn default() -> Self {
        Self {
            max_message_bytes: 2 * 1024 * 1024,
            max_input_bytes: 256 * 1024,
            max_output_bytes: 1024 * 1024,
            max_error_details_bytes: 32 * 1024,
            max_event_payload_bytes: 256 * 1024,
            max_json_depth: 64,
            max_container_children: 10_000,
        }
    }
}

impl ProtocolLimits {
    fn json(self) -> JsonLimits {
        JsonLimits {
            max_depth: self.max_json_depth,
            max_container_children: self.max_container_children,
        }
    }

    fn within_hard_ceilings(self) -> bool {
        self.max_message_bytes > 0
            && self.max_message_bytes <= 2 * 1024 * 1024
            && self.max_input_bytes > 0
            && self.max_input_bytes <= 256 * 1024
            && self.max_output_bytes > 0
            && self.max_output_bytes <= 1024 * 1024
            && self.max_error_details_bytes > 0
            && self.max_error_details_bytes <= 32 * 1024
            && self.max_event_payload_bytes > 0
            && self.max_event_payload_bytes <= 256 * 1024
            && self.max_json_depth > 0
            && self.max_json_depth <= 64
            && self.max_container_children > 0
            && self.max_container_children <= 10_000
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Envelope {
    pub protocol: String,
    pub major: u16,
    pub minor: u16,
    pub kind: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<RequestId>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub host_session_id: Option<HostSessionId>,
    pub sent_at: u64,
    pub payload: Value,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub extensions: Option<Map<String, Value>>,
    #[serde(flatten)]
    pub unknown_informational: Map<String, Value>,
}

#[derive(Debug, Clone)]
pub struct NegotiatedProtocol {
    pub major: u16,
    pub minor: u16,
    pub host_session_id: HostSessionId,
}

#[derive(Debug, thiserror::Error)]
pub enum ContractError {
    #[error("logical message exceeds byte limit")]
    Limit,
    #[error("logical message is not valid JSON: {0}")]
    InvalidJson(#[from] serde_json::Error),
    #[error("logical message is outside the lossless JSON subset: {0}")]
    Lossless(#[from] JsonBoundaryError),
    #[error("Muse Bridge protocol major is not supported")]
    UnsupportedProtocol,
    #[error("logical envelope failed validation")]
    InvalidEnvelope,
    #[error("hello must complete before this message")]
    HandshakeRequired,
    #[error("message version differs from negotiated protocol")]
    VersionMismatch,
    #[error("message host session is not current")]
    HostSessionExpired,
    #[error("message payload failed validation")]
    InvalidPayload,
    #[error("protocol schemas failed to compile: {0}")]
    SchemaCompile(String),
}

impl ContractError {
    pub fn bridge_code(&self) -> &'static str {
        match self {
            Self::UnsupportedProtocol | Self::VersionMismatch => "UNSUPPORTED_PROTOCOL",
            Self::HandshakeRequired => "HANDSHAKE_REQUIRED",
            Self::HostSessionExpired => "HOST_SESSION_EXPIRED",
            Self::Limit
            | Self::InvalidJson(_)
            | Self::Lossless(_)
            | Self::InvalidEnvelope
            | Self::InvalidPayload => "INVALID_ENVELOPE",
            Self::SchemaCompile(_) => "INTERNAL",
        }
    }
}

#[derive(Clone)]
struct InMemoryRetriever {
    schemas: Arc<HashMap<String, Value>>,
}

#[derive(Clone)]
struct RejectExternalRetriever;

impl Retrieve for RejectExternalRetriever {
    fn retrieve(
        &self,
        uri: &Uri<String>,
    ) -> Result<Value, Box<dyn std::error::Error + Send + Sync>> {
        Err(format!("external provider schema reference is not allowed: {uri}").into())
    }
}

impl Retrieve for InMemoryRetriever {
    fn retrieve(
        &self,
        uri: &Uri<String>,
    ) -> Result<Value, Box<dyn std::error::Error + Send + Sync>> {
        self.schemas
            .get(uri.as_str())
            .cloned()
            .ok_or_else(|| format!("schema is not in the packaged contract: {uri}").into())
    }
}

pub struct ProtocolSchemas {
    envelope: Validator,
    payloads: HashMap<String, Validator>,
    fragments: HashMap<ProtocolFragment, Validator>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ProtocolFragment {
    DiscoverDescriptor,
    Binding,
}

pub struct ProviderSchemaValidator(Validator);

impl ProviderSchemaValidator {
    pub fn validate(&self, value: &Value) -> Result<(), ContractError> {
        self.0
            .is_valid(value)
            .then_some(())
            .ok_or(ContractError::InvalidPayload)
    }
}

impl ProtocolSchemas {
    pub fn new() -> Result<Self, ContractError> {
        let mut documents = HashMap::new();
        for source in SCHEMA_DOCUMENTS {
            let value: Value = serde_json::from_str(source)?;
            let id = value
                .get("$id")
                .and_then(Value::as_str)
                .ok_or_else(|| ContractError::SchemaCompile("schema document has no $id".into()))?;
            documents.insert(id.to_owned(), value);
        }
        let retriever = InMemoryRetriever {
            schemas: Arc::new(documents),
        };
        let compile = |schema_ref: &str| {
            jsonschema::options()
                .with_draft(Draft::Draft202012)
                .with_retriever(retriever.clone())
                .build(&json!({
                    "$schema": "https://json-schema.org/draft/2020-12/schema",
                    "$ref": schema_ref
                }))
                .map_err(|error| ContractError::SchemaCompile(error.to_string()))
        };

        let envelope = compile(ENVELOPE_SCHEMA_ID)?;
        let mut payloads = HashMap::new();
        for (kind, schema_ref) in MESSAGE_SCHEMA_REFS {
            payloads.insert((*kind).to_owned(), compile(schema_ref)?);
        }
        let fragments = HashMap::from([
            (
                ProtocolFragment::DiscoverDescriptor,
                compile(
                    "https://muse.dev/schemas/bridge/v1/discover.schema.json#/$defs/descriptor",
                )?,
            ),
            (
                ProtocolFragment::Binding,
                compile("https://muse.dev/schemas/bridge/v1/bind.schema.json#/$defs/binding")?,
            ),
        ]);
        Ok(Self {
            envelope,
            payloads,
            fragments,
        })
    }

    pub fn validate_fragment(
        &self,
        fragment: ProtocolFragment,
        value: &Value,
    ) -> Result<(), ContractError> {
        self.fragments
            .get(&fragment)
            .is_some_and(|validator| validator.is_valid(value))
            .then_some(())
            .ok_or(ContractError::InvalidPayload)
    }

    pub fn compile_provider_schema(
        &self,
        schema: &Value,
    ) -> Result<ProviderSchemaValidator, ContractError> {
        jsonschema::options()
            .with_draft(Draft::Draft202012)
            .with_retriever(RejectExternalRetriever)
            .build(schema)
            .map(ProviderSchemaValidator)
            .map_err(|error| ContractError::SchemaCompile(error.to_string()))
    }

    pub fn decode_envelope(
        &self,
        bytes: &[u8],
        limits: ProtocolLimits,
    ) -> Result<Envelope, ContractError> {
        if !limits.within_hard_ceilings() {
            return Err(ContractError::Limit);
        }
        if bytes.len() > limits.max_message_bytes {
            return Err(ContractError::Limit);
        }
        let value: Value = serde_json::from_slice(bytes)?;
        validate_lossless(&value, limits.json())?;
        if value.get("protocol").and_then(Value::as_str) == Some("muse-bridge")
            && value
                .get("major")
                .and_then(Value::as_u64)
                .is_some_and(|major| major != 1)
        {
            return Err(ContractError::UnsupportedProtocol);
        }
        if !self.envelope.is_valid(&value) {
            return Err(ContractError::InvalidEnvelope);
        }
        Ok(serde_json::from_value(value)?)
    }

    pub fn decode_message(
        &self,
        bytes: &[u8],
        negotiated: Option<&NegotiatedProtocol>,
        limits: ProtocolLimits,
    ) -> Result<Envelope, ContractError> {
        let envelope = self.decode_envelope(bytes, limits)?;
        let is_hello = matches!(envelope.kind.as_str(), "hello.request" | "hello.response");
        if !is_hello {
            let negotiated = negotiated.ok_or(ContractError::HandshakeRequired)?;
            if envelope.major != negotiated.major || envelope.minor != negotiated.minor {
                return Err(ContractError::VersionMismatch);
            }
            if envelope.host_session_id.as_ref() != Some(&negotiated.host_session_id) {
                return Err(ContractError::HostSessionExpired);
            }
        }
        let validator = self
            .payloads
            .get(&envelope.kind)
            .ok_or(ContractError::InvalidEnvelope)?;
        if !validator.is_valid(&envelope.payload) {
            return Err(ContractError::InvalidPayload);
        }
        self.check_specific_limits(&envelope, limits)?;
        Ok(envelope)
    }

    pub fn encode_message(
        &self,
        envelope: &Envelope,
        limits: ProtocolLimits,
    ) -> Result<Vec<u8>, ContractError> {
        if !limits.within_hard_ceilings() {
            return Err(ContractError::Limit);
        }
        let value = serde_json::to_value(envelope)?;
        validate_lossless(&value, limits.json())?;
        let bytes = serde_json::to_vec(&value)?;
        if bytes.len() > limits.max_message_bytes {
            return Err(ContractError::Limit);
        }
        Ok(bytes)
    }

    fn check_specific_limits(
        &self,
        envelope: &Envelope,
        limits: ProtocolLimits,
    ) -> Result<(), ContractError> {
        let payload = &envelope.payload;
        let length = |value: &Value| -> Result<usize, ContractError> {
            Ok(canonicalize(value, limits.json())?.len())
        };
        let unsafe_provider_detail = |value: &Value| -> bool {
            fn visit(value: &Value) -> bool {
                match value {
                    Value::Array(values) => values.iter().any(visit),
                    Value::Object(values) => values.iter().any(|(key, value)| {
                        let normalized: String = key
                            .chars()
                            .filter(|character| !matches!(character, '-' | '_' | '.'))
                            .flat_map(char::to_lowercase)
                            .collect();
                        matches!(
                            normalized.as_str(),
                            "authorization"
                                | "cookie"
                                | "password"
                                | "rawbytes"
                                | "secret"
                                | "stack"
                                | "stacktrace"
                                | "token"
                        ) || [
                            "authorization",
                            "cookie",
                            "password",
                            "secret",
                            "stacktrace",
                            "token",
                        ]
                        .iter()
                        .any(|suffix| normalized.ends_with(suffix))
                            || visit(value)
                    }),
                    Value::Null | Value::Bool(_) | Value::Number(_) | Value::String(_) => false,
                }
            }
            visit(value)
        };
        if envelope.kind == "invoke.request"
            && payload
                .get("input")
                .is_some_and(|value| length(value).is_ok_and(|size| size > limits.max_input_bytes))
        {
            return Err(ContractError::Limit);
        }
        if envelope.kind == "invoke.response"
            && payload.get("ok") == Some(&Value::Bool(true))
            && payload
                .get("value")
                .is_some_and(|value| length(value).is_ok_and(|size| size > limits.max_output_bytes))
        {
            return Err(ContractError::Limit);
        }
        if let Some(error) = payload
            .get("error")
            .filter(|error| error.get("kind").and_then(Value::as_str) == Some("provider"))
        {
            if error
                .get("message")
                .and_then(Value::as_str)
                .is_some_and(|message| {
                    message.contains("\n at ")
                        || message.to_ascii_lowercase().contains("stack trace")
                })
            {
                return Err(ContractError::InvalidPayload);
            }
            if let Some(details) = error.get("details") {
                if unsafe_provider_detail(details) {
                    return Err(ContractError::InvalidPayload);
                }
                if length(details).is_ok_and(|size| size > limits.max_error_details_bytes) {
                    return Err(ContractError::Limit);
                }
            }
        }
        if envelope.kind == "bridge.event"
            && payload
                .get("data")
                .filter(|data| {
                    data.get("eventKind").and_then(Value::as_str) == Some("provider.event")
                })
                .and_then(|data| data.get("payload"))
                .is_some_and(|value| {
                    length(value).is_ok_and(|size| size > limits.max_event_payload_bytes)
                })
        {
            return Err(ContractError::Limit);
        }
        Ok(())
    }
}
