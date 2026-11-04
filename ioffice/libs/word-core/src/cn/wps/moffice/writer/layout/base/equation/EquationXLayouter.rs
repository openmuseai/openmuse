#![allow(dead_code, non_snake_case)]
// Java: cn.wps.moffice.writer.layout.base.equation.EquationXLayouter
use util::moffice::util::UnitUtil::UnitUtil;
use crate::cn::wps::moffice::writer::cache::TypoDrawing::TypoDrawing;
use crate::cn::wps::moffice::writer::cache::TypoList::TypoList;
use crate::cn::wps::moffice::writer::cache::TypoObjBase::TypoObjBase;
use crate::cn::wps::moffice::writer::core::Document::Document;
use crate::cn::wps::moffice::writer::layout::base::env::DocumentEnv::DocumentEnv;
use crate::cn::wps::moffice::writer::layout::base::env::MainState::MainState;
use crate::cn::wps::moffice::writer::layout::base::env::ViewEnv::ViewEnv;
use crate::cn::wps::moffice::writer::layout::base::equation::EquationLayouter::{EquationException, EquationLayouter};
use crate::cn::wps::moffice::writer::layout::base::equation::EquationResult::EquationResult;
use crate::cn::wps::moffice::writer::layout::base::equation::IEquationLayouter::IEquationLayouter;
use crate::cn::wps::moffice::writer::data::value::BRC::BRC;

// java public class EquationXLayouter extends IEquationLayouter
pub struct EquationXLayouter;

impl EquationXLayouter {
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
        let mut text_box = {
            let snapshot = &mut main_state.snapshot;
            let p_text_box = TypoDrawing::getTextbox__int_typosnapshot(drawing, snapshot);
            if p_text_box == 0 {
                let mut text_box = snapshot.getTypoTools().obtainTypoTextBox();
                text_box.alloc(snapshot);
                let pointer = text_box.getPointer();
                TypoDrawing::setTextbox__int_int_typosnapshot(pointer, drawing, snapshot);
                snapshot.copy__int_int_int(drawing + 3, pointer + 3, 4);
                text_box
            } else {
                snapshot.getTypoTools().obtainTypoTextBox__int(p_text_box)
            }
        };
        let mut borders = main_state.snapshot.getTypoTools().obtainTypoBRCs();
        borders.alloc(&mut main_state.snapshot);
        {
            let snapshot = &mut main_state.snapshot;
            snapshot.copy__int_int_int(drawing + 3, borders.getPointer() + 2, 4);
        }
        let op_size = eq_result.getOPSize();
        let mut extend_top = 0;
        let mut extend_left = 0;
        let mut extend_right = 0;
        let mut extend_bottom = 0;
        let mut brc = view_env.getDefaultEQXBRC();
        let def_color = IEquationLayouter::finalRunColor(unsafe { &*document }, revision_final, eq_result.getEndCP() - 1);
        if def_color != -16777216 {
            brc = BRC::new__float_int_int_float_boolean_boolean(brc.getLineWidth(), brc.getBorderType(), def_color, brc.getSpace(), false, false);
        }
        if op_size <= 1 {
            borders.setBRCs(&mut brc);
            let extend = UnitUtil::PT2TWIPS(brc.getLineWidth() * 2.0);
            extend_left = extend;
            extend_right = extend;
            extend_top = extend;
            extend_bottom = extend;
        } else {
            for i in 1..op_size {
                let op = eq_result.getOP(i).to_ascii_lowercase();
                let extend = UnitUtil::PT2TWIPS(brc.getLineWidth() * 2.0);
                if op == "le" {
                    borders.setBrcLeft(brc.clone());
                    extend_left = extend;
                } else if op == "to" {
                    borders.setBrcTop(brc.clone());
                    extend_top = extend;
                } else if op == "bo" {
                    borders.setBrcBottom(brc.clone());
                    extend_bottom = extend;
                } else if op == "ri" {
                    borders.setBrcRight(brc.clone());
                    extend_right = extend;
                } else {
                    main_state.snapshot.getTypoTools().recycle_textbox(&mut text_box);
                    return Err(EquationException);
                }
            }
        }
        let mut child_offset_x = 0;
        let mut child_offset_y = 0;
        if extend_top > 0 {
            borders.setTop(-extend_top);
            extend_top *= 3;
            text_box.setMarginTop(extend_top);
            child_offset_y = extend_top;
        }
        if extend_left > 0 {
            borders.setLeft(-extend_left);
            extend_left *= 3;
            text_box.setMarginLeft(extend_left);
            child_offset_x = extend_left;
        }
        if child_offset_x != 0 || child_offset_y != 0 {
            let snapshot = &mut main_state.snapshot;
            let children = TypoDrawing::getChildDrawings__int_typosnapshot(drawing, snapshot);
            let size = TypoList::size__int_typosnapshot(children, snapshot);
            for i in 0..size {
                let child = TypoList::quickGet__int_int_typosnapshot(i, children, snapshot);
                TypoObjBase::offset__int_int_int_typosnapshot(child_offset_x, child_offset_y, child, snapshot);
            }
            let align = TypoDrawing::getAlignLine__int_typosnapshot(drawing, snapshot);
            if child_offset_y != 0 && align != 0 {
                TypoDrawing::setAlignLine__int_int_typosnapshot(align + child_offset_y, drawing, snapshot);
            }
        }
        if extend_right > 0 {
            borders.setRight(borders.getRight() + extend_right);
            extend_right *= 3;
            text_box.setMarginRight(extend_right);
        }
        if extend_bottom > 0 {
            borders.setBottom(borders.getBottom() + extend_bottom);
            extend_bottom *= 3;
            text_box.setMarginBottom(extend_bottom);
        }
        borders.setLeft(borders.getLeft() + extend_left);
        borders.setTop(borders.getTop() + extend_top);
        borders.setRight(borders.getRight() + extend_left);
        borders.setBottom(borders.getBottom() + extend_top);
        let total_h = extend_left + extend_right;
        if total_h > 0 {
            text_box.setWidth(text_box.width() + total_h);
            let width = text_box.width();
            TypoObjBase::setWidth__int_int_typosnapshot(width, drawing, &mut main_state.snapshot);
        }
        let total_v = extend_top + extend_bottom;
        if total_v > 0 {
            text_box.setHeight(text_box.height() + total_v);
            let height = text_box.height();
            TypoObjBase::setHeight__int_int_typosnapshot(height, drawing, &mut main_state.snapshot);
        }
        text_box.setPageBRCs(borders.getPointer());
        main_state.snapshot.getTypoTools().recycle_brcs(&mut borders);
        main_state.snapshot.getTypoTools().recycle_textbox(&mut text_box);
        Ok(drawing)
    }
}
