#![allow(dead_code, non_snake_case)]
// Java: cn.wps.moffice.writer.layout.base.equation.EquationLLayouter
use crate::cn::wps::moffice::writer::core::Document::Document;
use crate::cn::wps::moffice::writer::layout::base::env::DocumentEnv::DocumentEnv;
use crate::cn::wps::moffice::writer::layout::base::env::MainState::MainState;
use crate::cn::wps::moffice::writer::layout::base::env::ViewEnv::ViewEnv;
use crate::cn::wps::moffice::writer::layout::base::equation::EquationLayouter::{EquationException, EquationLayouter};
use crate::cn::wps::moffice::writer::layout::base::equation::EquationResult::EquationResult;

// java public class EquationLLayouter extends IEquationLayouter
pub struct EquationLLayouter;

impl EquationLLayouter {
    // java public int flow(Document document, EquationResult eqResult, int end)
    pub fn flow(
        main_state: &mut MainState,
        document_env: &mut DocumentEnv,
        view_env: &mut ViewEnv,
        document: *mut dyn Document,
        eq_result: &EquationResult,
        _end: i32,
        enable_revision: bool,
        revision_final: bool,
    ) -> Result<i32, EquationException> {
        let size = eq_result.getValuesSize();
        if size == 0 {
            return Ok(0);
        }
        // java getEquation(start(0), end(valuesSize - 1)) — the overload without needCalcGraphHeight
        EquationLayouter::getEquation(
            main_state,
            document_env,
            view_env,
            Some(document),
            eq_result.getStart(0),
            eq_result.getEnd(size - 1),
            enable_revision,
            revision_final,
            false,
        )
    }
}
