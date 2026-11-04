#![allow(non_snake_case, dead_code, unused_imports)]

/// Soft usable DocumentCorrector for Document.getDocumentCorrector().
#[derive(Default)]
pub struct DocumentCorrector {}

impl DocumentCorrector {
    pub fn new() -> Self {
        Self::default()
    }
    pub fn fixAll(&self) {}
}
