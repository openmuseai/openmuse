#![allow(non_snake_case, dead_code, unused_imports)]
use crate::cn::wps::moffice::writer::core::Document::Document;
use crate::cn::wps::moffice::writer::data::PLC::PLC;

/// Soft usable PLCDocumentFields (Document.getDocumentFileds).
pub struct PLCDocumentFields {
    pub(crate) super_: PLC,
}

impl PLCDocumentFields {
    pub fn PLCDocumentFields(document: &dyn Document) -> Self {
        Self {
            super_: PLC::new(Some(document)),
        }
    }
    pub fn size(&self) -> i32 {
        self.super_.size()
    }
}

impl std::ops::Deref for PLCDocumentFields {
    type Target = PLC;
    fn deref(&self) -> &Self::Target {
        &self.super_
    }
}
