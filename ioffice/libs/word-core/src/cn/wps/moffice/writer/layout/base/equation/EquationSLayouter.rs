#![allow(dead_code, non_snake_case)]
// Java: cn.wps.moffice.writer.layout.base.equation.EquationSLayouter
use util::moffice::util::UnitUtil::UnitUtil;
use crate::cn::wps::moffice::writer::cache::TypoDrawing::TypoDrawing;
use crate::cn::wps::moffice::writer::cache::TypoObjBase::TypoObjBase;
use crate::cn::wps::moffice::writer::cache::TypoTextBox::TypoTextBox;
use crate::cn::wps::moffice::writer::core::Document::Document;
use crate::cn::wps::moffice::writer::layout::base::env::DocumentEnv::DocumentEnv;
use crate::cn::wps::moffice::writer::layout::base::env::MainState::MainState;
use crate::cn::wps::moffice::writer::layout::base::env::ViewEnv::ViewEnv;
use crate::cn::wps::moffice::writer::layout::base::equation::EquationLayouter::{EquationException, EquationLayouter};
use crate::cn::wps::moffice::writer::layout::base::equation::EquationResult::EquationResult;

// java public class EquationSLayouter extends IEquationLayouter
pub struct EquationSLayouter;

impl EquationSLayouter {
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
        if eq_result.getValuesSize() != 1 {
            return Ok(0);
        }
        let drawing = EquationLayouter::getEquation(
            main_state,
            document_env,
            view_env,
            Some(document),
            eq_result.getStart(0),
            eq_result.getEnd(0),
            enable_revision,
            revision_final,
            false,
        )?;
        if drawing == 0 {
            return Ok(0);
        }
        let snapshot = &mut main_state.snapshot;
        let text_box = TypoDrawing::getTextbox__int_typosnapshot(drawing, snapshot);
        if text_box != 0 {
            let mut hps_offset = -TypoTextBox::height__int_typosnapshot(text_box, snapshot) / 2;
            let mut extend_line_before = 0;
            let mut extend_line_after = 0;
            let op_size = eq_result.getOPSize();
            for i in 1..op_size {
                let op = eq_result.getOP(i);
                if op.starts_with("ai") {
                    extend_line_before = UnitUtil::PT2TWIPS(get_float(&op, 2));
                } else if op.starts_with("di") {
                    extend_line_after = UnitUtil::PT2TWIPS(get_float(&op, 2));
                } else if op.starts_with("up") {
                    hps_offset = UnitUtil::PT2TWIPS(-get_float(&op, 2));
                } else if op.starts_with("do") {
                    hps_offset = UnitUtil::PT2TWIPS(get_float(&op, 2));
                }
            }
            if hps_offset > 0 {
                TypoObjBase::offset__int_int_int_typosnapshot(0, hps_offset, text_box, snapshot);
                let height = TypoDrawing::height__int(drawing, snapshot) + hps_offset;
                TypoObjBase::setHeight__int_int_typosnapshot(height, drawing, snapshot);
            } else if hps_offset < 0 {
                let align = TypoDrawing::getAlignLine__int_typosnapshot(drawing, snapshot) - hps_offset;
                TypoDrawing::setAlignLine__int_int_typosnapshot(align, drawing, snapshot);
                let height = TypoDrawing::height__int(drawing, snapshot) - hps_offset;
                TypoObjBase::setHeight__int_int_typosnapshot(height, drawing, snapshot);
            }
            if extend_line_before > 0 {
                TypoObjBase::offset__int_int_int_typosnapshot(0, extend_line_before, text_box, snapshot);
                let align = TypoDrawing::getAlignLine__int_typosnapshot(drawing, snapshot) + extend_line_before;
                TypoDrawing::setAlignLine__int_int_typosnapshot(align, drawing, snapshot);
                let height = TypoDrawing::height__int(drawing, snapshot) + extend_line_before;
                TypoObjBase::setHeight__int_int_typosnapshot(height, drawing, snapshot);
            }
            if extend_line_after > 0 {
                let height = TypoDrawing::height__int(drawing, snapshot) + extend_line_after;
                TypoObjBase::setHeight__int_int_typosnapshot(height, drawing, snapshot);
            }
        }
        Ok(drawing)
    }
}

// java IEquationLayouter.getFloat(String op, int start)
fn get_float(op: &str, start: i32) -> f32 {
    let rest: String = op
        .chars()
        .skip(start.max(0) as usize)
        .skip_while(|ch| !(*ch >= '0' && *ch <= '9'))
        .collect();
    if rest.is_empty() {
        0.0
    } else {
        rest.parse::<f32>().unwrap_or(0.0)
    }
}
