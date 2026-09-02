use crate::json::{JsonBoundaryError, JsonLimits, canonicalize};
use serde_json::Value;
use sha2::{Digest, Sha256};

fn digest(prefix: &[u8], value: &Value, limits: JsonLimits) -> Result<String, JsonBoundaryError> {
    let canonical = canonicalize(value, limits)?;
    let mut hasher = Sha256::new();
    hasher.update(prefix);
    hasher.update(canonical);
    Ok(format!("sha256:{}", hex::encode(hasher.finalize())))
}

pub fn digest_schema(value: &Value, limits: JsonLimits) -> Result<String, JsonBoundaryError> {
    digest(b"muse-schema-v1\0", value, limits)
}

pub fn digest_input(value: &Value, limits: JsonLimits) -> Result<String, JsonBoundaryError> {
    digest(b"muse-input-v1\0", value, limits)
}

pub fn digest_grant(value: &Value, limits: JsonLimits) -> Result<String, JsonBoundaryError> {
    digest(b"muse-grant-v1\0", value, limits)
}
