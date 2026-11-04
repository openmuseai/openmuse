#![allow(dead_code, non_snake_case, unused_variables)]
// Java: cn.wps.moffice.writer.layout.base.equation.EquationDLayouter
use crate::cn::wps::moffice::writer::core::Document::Document;
use crate::cn::wps::moffice::writer::layout::base::equation::EquationResult::EquationResult;

// java public class EquationDLayouter extends IEquationLayouter
pub struct EquationDLayouter;

impl EquationDLayouter {
    // java public int flow(...) { return 0; }
    pub fn flow(_document: *mut dyn Document, _eq_result: &EquationResult, _end: i32) -> i32 {
        0
    }
}
