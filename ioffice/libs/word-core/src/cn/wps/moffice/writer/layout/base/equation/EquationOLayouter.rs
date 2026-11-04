#![allow(dead_code, non_snake_case)]
// Java: cn.wps.moffice.writer.layout.base.equation.EquationOLayouter
use crate::cn::wps::moffice::writer::cache::TypoDrawing::TypoDrawing;
use crate::cn::wps::moffice::writer::cache::TypoList::TypoList;
use crate::cn::wps::moffice::writer::cache::TypoObjBase::TypoObjBase;
use crate::cn::wps::moffice::writer::core::Document::Document;
use crate::cn::wps::moffice::writer::layout::base::env::DocumentEnv::DocumentEnv;
use crate::cn::wps::moffice::writer::layout::base::env::MainState::MainState;
use crate::cn::wps::moffice::writer::layout::base::env::ViewEnv::ViewEnv;
use crate::cn::wps::moffice::writer::layout::base::equation::EquationAlignment::EquationAlignment;
use crate::cn::wps::moffice::writer::layout::base::equation::EquationLayouter::{EquationException, EquationLayouter};
use crate::cn::wps::moffice::writer::layout::base::equation::EquationResult::EquationResult;

// java public class EquationOLayouter extends IEquationLayouter
pub struct EquationOLayouter;

impl EquationOLayouter {
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
        let values_size = eq_result.getValuesSize();
        if values_size <= 0 {
            return Ok(0);
        }
        if values_size == 1 {
            return EquationLayouter::getEquation(
                main_state,
                document_env,
                view_env,
                Some(document),
                eq_result.getStart(0),
                eq_result.getEnd(0),
                enable_revision,
                revision_final,
                false,
            );
        }
        let mut drawing = {
            let snapshot = &mut main_state.snapshot;
            let mut drawing = snapshot.getTypoTools().obtainTypoDrawing();
            drawing.alloc(snapshot);
            drawing.setDrawingType(15);
            drawing.setIsRotateWithPage(true);
            drawing.setEmbed(true);
            drawing
        };
        let mut max_width = 0;
        let mut eq_alignment = EquationAlignment::EquationAlignment();
        for i in 0..values_size {
            let sub = EquationLayouter::getEquation(
                main_state,
                document_env,
                view_env,
                Some(document),
                eq_result.getStart(i),
                eq_result.getEnd(i),
                enable_revision,
                revision_final,
                false,
            )?;
            if sub != 0 {
                max_width = max_width.max(TypoDrawing::width__int(sub, &main_state.snapshot));
                eq_alignment.onAddDrawing(sub, &main_state.snapshot);
                drawing.addChildDrawing(sub);
            }
        }
        eq_alignment.calcAlignLine();
        drawing.setAlignType(eq_alignment.getAlignType());
        drawing.setAlignLine(eq_alignment.getAlignLine());
        let mut align = 1;
        let op_size = eq_result.getOPSize();
        for i in 1..op_size {
            let op = eq_result.getOP(i);
            if op == "al" {
                align = 0;
            } else if op == "ar" {
                align = 2;
            }
        }
        // Java lists children then walks that list. The decompiled loop reuses the index name.
        let p_children = drawing.getChildDrawings();
        {
            let snapshot = &mut main_state.snapshot;
            let size = TypoList::size__int_typosnapshot(p_children, snapshot);
            for i in 0..size {
                let sub = TypoList::quickGet__int_int_typosnapshot(i, p_children, snapshot);
                let top = eq_alignment.getTop(sub, snapshot);
                let width = TypoDrawing::width__int(sub, snapshot);
                let left = match align {
                    1 => (max_width - width) / 2,
                    2 => max_width - width,
                    _ => 0,
                };
                TypoObjBase::offsetTo__int_int_int_typosnapshot(left, top, sub, snapshot);
            }
        }
        drawing.setHeight(eq_alignment.getLineHeight());
        drawing.setWidth(max_width);
        let pointer = drawing.getPointer();
        main_state.snapshot.getTypoTools().recycle_drawing(&mut drawing);
        Ok(pointer)
    }
}
