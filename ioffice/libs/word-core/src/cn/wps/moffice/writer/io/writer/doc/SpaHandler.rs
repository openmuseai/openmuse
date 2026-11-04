#![allow(non_snake_case, dead_code, unused_variables, unused_imports)]
//! Soft SpaHandler stub (prior auto-stub had invalid Java-style types).

use crate::cn::wps::moffice::writer::data::DrawingContainer::SoftShape;

pub struct SpaHandler;

impl SpaHandler {
    pub fn new() -> Self {
        Self
    }
    pub fn getDiagramPath(_sp: &SoftShape) -> Option<String> {
        None
    }
}
