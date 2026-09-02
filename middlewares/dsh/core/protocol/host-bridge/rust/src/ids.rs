use serde::{Deserialize, Deserializer, Serialize, Serializer, de};
use std::{fmt, str::FromStr};

#[derive(Debug, Clone, PartialEq, Eq, thiserror::Error)]
#[error("opaque identifier must be 1-128 bytes of ASCII [A-Za-z0-9._~-]")]
pub struct InvalidOpaqueId;

fn valid_opaque_id(value: &str) -> bool {
    !value.is_empty()
        && value.len() <= 128
        && value
            .bytes()
            .all(|byte| byte.is_ascii_alphanumeric() || matches!(byte, b'.' | b'_' | b'~' | b'-'))
}

macro_rules! opaque_id {
    ($name:ident) => {
        #[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
        pub struct $name(String);

        impl $name {
            pub fn as_str(&self) -> &str {
                &self.0
            }
        }

        impl TryFrom<String> for $name {
            type Error = InvalidOpaqueId;

            fn try_from(value: String) -> Result<Self, Self::Error> {
                if valid_opaque_id(&value) {
                    Ok(Self(value))
                } else {
                    Err(InvalidOpaqueId)
                }
            }
        }

        impl FromStr for $name {
            type Err = InvalidOpaqueId;

            fn from_str(value: &str) -> Result<Self, Self::Err> {
                value.to_owned().try_into()
            }
        }

        impl fmt::Display for $name {
            fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
                formatter.write_str(&self.0)
            }
        }

        impl Serialize for $name {
            fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
            where
                S: Serializer,
            {
                serializer.serialize_str(&self.0)
            }
        }

        impl<'de> Deserialize<'de> for $name {
            fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
            where
                D: Deserializer<'de>,
            {
                let value = String::deserialize(deserializer)?;
                value.try_into().map_err(de::Error::custom)
            }
        }
    };
}

opaque_id!(RequestId);
opaque_id!(TraceId);
opaque_id!(HostSessionId);
opaque_id!(RuntimeInstanceId);
opaque_id!(DescriptorId);
opaque_id!(DescriptorRevision);
opaque_id!(ProviderInstanceId);
opaque_id!(HostGeneration);
opaque_id!(BindingId);
opaque_id!(CancellationId);
opaque_id!(SubscriptionId);
opaque_id!(EventCursor);
opaque_id!(PolicyDecisionId);
opaque_id!(ApprovalId);
opaque_id!(GrantId);
opaque_id!(ReceiptId);
opaque_id!(CommandId);
opaque_id!(AuditRef);
opaque_id!(IdempotencyKey);
