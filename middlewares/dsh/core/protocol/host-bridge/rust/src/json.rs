use serde_json::Value;

#[derive(Debug, Clone, Copy)]
pub struct JsonLimits {
    pub max_depth: usize,
    pub max_container_children: usize,
}

#[derive(Debug, thiserror::Error)]
pub enum JsonBoundaryError {
    #[error("maximum JSON depth exceeded")]
    Depth,
    #[error("JSON container child limit exceeded")]
    Children,
    #[error("negative zero is not lossless JSON")]
    NegativeZero,
    #[error("canonical JSON serialization failed: {0}")]
    Canonical(#[from] serde_json::Error),
}

pub fn validate_lossless(value: &Value, limits: JsonLimits) -> Result<(), JsonBoundaryError> {
    fn visit(value: &Value, limits: JsonLimits, depth: usize) -> Result<(), JsonBoundaryError> {
        if depth > limits.max_depth {
            return Err(JsonBoundaryError::Depth);
        }
        match value {
            Value::Number(number) => {
                if number
                    .as_f64()
                    .is_some_and(|value| value == 0.0 && value.is_sign_negative())
                {
                    return Err(JsonBoundaryError::NegativeZero);
                }
            }
            Value::Array(values) => {
                if values.len() > limits.max_container_children {
                    return Err(JsonBoundaryError::Children);
                }
                for value in values {
                    visit(value, limits, depth + 1)?;
                }
            }
            Value::Object(values) => {
                if values.len() > limits.max_container_children {
                    return Err(JsonBoundaryError::Children);
                }
                for value in values.values() {
                    visit(value, limits, depth + 1)?;
                }
            }
            Value::Null | Value::Bool(_) | Value::String(_) => {}
        }
        Ok(())
    }

    visit(value, limits, 0)
}

pub fn canonicalize(value: &Value, limits: JsonLimits) -> Result<Vec<u8>, JsonBoundaryError> {
    validate_lossless(value, limits)?;
    Ok(serde_jcs::to_vec(value)?)
}
