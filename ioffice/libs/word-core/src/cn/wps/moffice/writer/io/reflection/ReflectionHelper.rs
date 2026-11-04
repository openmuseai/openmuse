#![allow(non_snake_case, dead_code, unused_variables, unused_imports)]
//! Soft ReflectionHelper — full rewrite deferred (blocked on TextDocument/SubDocument in core/mod).

use crate::cn::wps::moffice::writer::core::Document::{Document, SoftPLCCommentRef};
use crate::cn::wps::moffice::writer::helper::soft_drawing::{
    ICmtCustDatasWriter, SoftCmtCustDatasWriter,
};
use std::collections::HashMap;

pub struct ReflectionHelper;

impl ReflectionHelper {
    pub fn getCmtCustDatasWriter(
        _owner: Option<&()>,
        _mainDoc: Option<&mut dyn Document>,
    ) -> SoftCmtCustDatasWriter {
        SoftCmtCustDatasWriter::new()
    }
}
