#![allow(non_snake_case, dead_code)]

/// JDK java.lang.CloneNotSupportedException
#[derive(Debug, Default, Clone)]
pub struct CloneNotSupportedException {
    pub message: Option<String>,
}

impl CloneNotSupportedException {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn with_message(message: impl Into<String>) -> Self {
        Self {
            message: Some(message.into()),
        }
    }
}
