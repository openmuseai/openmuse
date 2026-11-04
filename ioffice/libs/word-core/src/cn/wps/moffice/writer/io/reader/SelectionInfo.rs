#![allow(non_snake_case, dead_code, unused_variables, unused_imports)]
//! Soft SelectionInfo until SelectionType/KRange wiring is restored in core/mod.

use crate::cn::wps::moffice::writer::core::KRange::KRange;
use crate::cn::wps::moffice::writer::data::DrawingContainer::SoftShape;
use std::collections::{HashMap, HashSet};

#[derive(Default, Clone, Copy, Debug, PartialEq, Eq)]
pub enum SelectionType {
    #[default]
    None,
    Text,
    Shape,
}

#[derive(Default)]
pub struct SelectionInfo {
    mCellsSelected: Option<HashMap<i32, HashSet<i32>>>,
    mKRange: Option<KRange>,
    mSelectionType: Option<SelectionType>,
    mSelectWholeTable: bool,
    mShapes: Option<Vec<SoftShape>>,
}

impl SelectionInfo {
    pub fn new() -> Self {
        Self::default()
    }
}
