#![allow(dead_code, non_snake_case)]
// Java: cn.wps.moffice.writer.layout.base.equation.EquationLinearLayouter
use crate::cn::wps::moffice::writer::cache::TypoDrawing::TypoDrawing;
use crate::cn::wps::moffice::writer::cache::TypoList::TypoList;
use crate::cn::wps::moffice::writer::cache::TypoObjBase::TypoObjBase;
use crate::cn::wps::moffice::writer::core::Document::Document;
use crate::cn::wps::moffice::writer::layout::base::env::DocumentEnv::DocumentEnv;
use crate::cn::wps::moffice::writer::layout::base::env::MainState::MainState;
use crate::cn::wps::moffice::writer::layout::base::env::PropertyTool::PropertyTool;
use crate::cn::wps::moffice::writer::layout::base::env::ViewEnv::ViewEnv;
use crate::cn::wps::moffice::writer::layout::base::equation::EquationAlignment::EquationAlignment;
use crate::cn::wps::moffice::writer::layout::base::equation::EquationLayouter::{EquationException, EquationLayouter};
use crate::cn::wps::moffice::writer::layout::base::equation::EquationParser::EquationParser;
use crate::cn::wps::moffice::writer::layout::base::equation::EquationResult::EquationResult;

// java public class EquationLinearLayouter extends IEquationLayouter
pub struct EquationLinearLayouter;

impl EquationLinearLayouter {
    // java public int flow(Document document, EquationResult eqResult, int end)
    pub fn flow(
        main_state: &mut MainState,
        document_env: &mut DocumentEnv,
        view_env: &mut ViewEnv,
        document: *mut dyn Document,
        mut eq_result: EquationResult,
        end: i32,
        enable_revision: bool,
        revision_final: bool,
        need_calc_graph_height: bool,
    ) -> Result<i32, EquationException> {
        let mut drawing = {
            let snapshot = &mut main_state.snapshot;
            let mut drawing = snapshot.getTypoTools().obtainTypoDrawing();
            drawing.alloc(snapshot);
            drawing.setDrawingType(15);
            drawing.setIsRotateWithPage(true);
            drawing.setEmbed(true);
            drawing
        };
        let mut eq_align = EquationAlignment::EquationAlignment();
        let mut find_error = false;
        loop {
            let cur_end = eq_result.getEndCP();
            let sub = EquationLayouter::flow_child(
                main_state,
                document_env,
                view_env,
                document,
                &eq_result,
                cur_end,
                enable_revision,
                revision_final,
                need_calc_graph_height,
            )?;
            if sub != 0 {
                eq_align.onAddDrawing(sub, &main_state.snapshot);
                drawing.addChildDrawing(sub);
            }
            let cur_cp_end = eq_result.getEndCP();
            if cur_cp_end >= end {
                break;
            }
            let doc = unsafe { &*document };
            let doc_type = doc.getType__on_IDocument();
            let (para_cache, run_cache) = if doc_type == 0 { (64, 128) } else { (4, 4) };
            let mut property_tool = PropertyTool::PropertyTool(doc, para_cache, run_cache);
            match EquationParser::parse(doc, &mut property_tool, cur_cp_end, end, enable_revision, revision_final) {
                None => {
                    find_error = true;
                    break;
                }
                Some(next) if next.isError() => {
                    find_error = true;
                    break;
                }
                Some(next) => eq_result = next,
            }
        }
        if find_error {
            main_state.snapshot.getTypoTools().recycle_drawing(&mut drawing);
            return Err(EquationException);
        }
        eq_align.calcAlignLine();
        let p_children = drawing.getChildDrawings();
        let mut drawing_width = 0;
        {
            let snapshot = &mut main_state.snapshot;
            let size = TypoList::size__int_typosnapshot(p_children, snapshot);
            for i in 0..size {
                let cur = TypoList::quickGet__int_int_typosnapshot(i, p_children, snapshot);
                let top = eq_align.getTop(cur, snapshot);
                TypoObjBase::offsetTo__int_int_int_typosnapshot(drawing_width, top, cur, snapshot);
                drawing_width = TypoObjBase::getRight__int_typosnapshot(cur, snapshot);
            }
        }
        drawing.setWidth(drawing_width);
        drawing.setHeight(eq_align.getLineHeight());
        drawing.setAlignLine(eq_align.getAlignLine());
        drawing.setAlignType(eq_align.getAlignType());
        let pointer = drawing.getPointer();
        main_state.snapshot.getTypoTools().recycle_drawing(&mut drawing);
        Ok(pointer)
    }
}
