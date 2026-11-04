#![allow(dead_code, non_snake_case)]
// Java: cn.wps.moffice.writer.layout.base.equation.EquationRLayouter
use util::graphics::RectF::RectF;
use crate::cn::wps::moffice::writer::cache::TypoDrawing::TypoDrawing;
use crate::cn::wps::moffice::writer::cache::TypoObjBase::TypoObjBase;
use crate::cn::wps::moffice::writer::core::Document::Document;
use crate::cn::wps::moffice::writer::core::shape::ShapeMetrics::ShapeMetrics;
use crate::cn::wps::moffice::writer::layout::base::env::DocumentEnv::DocumentEnv;
use crate::cn::wps::moffice::writer::layout::base::env::MainState::MainState;
use crate::cn::wps::moffice::writer::layout::base::env::ViewEnv::ViewEnv;
use crate::cn::wps::moffice::writer::layout::base::equation::EquationLayouter::{EquationException, EquationLayouter};
use crate::cn::wps::moffice::writer::layout::base::equation::EquationResult::EquationResult;
use crate::cn::wps::moffice::writer::layout::base::equation::IEquationLayouter::IEquationLayouter;
use crate::cn::wps::moffice::writer::layout::base::math::SepShapeFactory::SepShapeFactory;

// java public class EquationRLayouter extends IEquationLayouter
pub struct EquationRLayouter;

impl EquationRLayouter {
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
        let _ = view_env;
        if eq_result.getValuesSize() > 2 {
            return Err(EquationException);
        }
        let (rad_out, rad_in) = if eq_result.getValuesSize() == 1 {
            let rad_in = EquationLayouter::getEquation(
                main_state, document_env, view_env, Some(document),
                eq_result.getStart(0), eq_result.getEnd(0), enable_revision, revision_final, false,
            )?;
            (0, rad_in)
        } else {
            let rad_out = EquationLayouter::getEquation(
                main_state, document_env, view_env, Some(document),
                eq_result.getStart(0), eq_result.getEnd(0), enable_revision, revision_final, false,
            )?;
            let rad_in = EquationLayouter::getEquation(
                main_state, document_env, view_env, Some(document),
                eq_result.getStart(1), eq_result.getEnd(1), enable_revision, revision_final, false,
            )?;
            (rad_out, rad_in)
        };
        if rad_in == 0 {
            return Ok(rad_out);
        }
        let mut drawing = obtain_drawing(main_state);
        drawing.addChildDrawing(rad_in);
        if rad_out != 0 {
            drawing.addChildDrawing(rad_out);
        }
        let color = IEquationLayouter::finalRunColor(unsafe { &*document }, revision_final, eq_result.getStartCP() + 1);
        let mut rad = get_rad(main_state, rad_in, rad_out, color);
        drawing.addChildDrawing(rad.getPointer());
        drawing.setWidth(rad.getRight());
        drawing.setHeight(rad.getBottom());
        drawing.setAlignType(TypoDrawing::getAlignType__int_typosnapshot(rad_in, &main_state.snapshot));
        drawing.setAlignLine(TypoObjBase::getTop__int_typosnapshot(rad_in, &main_state.snapshot) + TypoDrawing::getAlignLine__int_typosnapshot(rad_in, &main_state.snapshot));
        let pointer = drawing.getPointer();
        main_state.snapshot.getTypoTools().recycle_drawing(&mut rad);
        main_state.snapshot.getTypoTools().recycle_drawing(&mut drawing);
        Ok(pointer)
    }
}

fn obtain_drawing(main_state: &mut MainState) -> TypoDrawing {
    let snapshot = &mut main_state.snapshot;
    let mut drawing = snapshot.getTypoTools().obtainTypoDrawing();
    drawing.alloc(snapshot);
    drawing.setDrawingType(15);
    drawing.setIsRotateWithPage(true);
    drawing.setEmbed(true);
    drawing
}

fn get_rad(main_state: &mut MainState, rad_in: i32, rad_out: i32, color: i32) -> TypoDrawing {
    let rad_in_height = TypoDrawing::height__int(rad_in, &main_state.snapshot);
    let rad_in_width = TypoDrawing::width__int(rad_in, &main_state.snapshot);
    let rad_top_offset = (0.2 * rad_in_height as f32) as i32;
    let rad_bottom_offset = (0.1 * rad_in_height as f32) as i32;
    let rad_height = rad_in_height + rad_top_offset + rad_bottom_offset;
    let rad_left_offset = (0.55 * rad_height as f32) as i32;
    let rad_right_offset = (0.05 * rad_height as f32) as i32;
    let rad_width = rad_in_width + rad_left_offset + rad_right_offset;
    let rect = RectF::new_with(0.0, 0.0, ShapeMetrics::transFromLayout__float(rad_width as f32), ShapeMetrics::transFromLayout__float(rad_height as f32));
    let shape = SepShapeFactory::getShape(6, &rect, ShapeMetrics::transFromLayout__float(13.2), color, ShapeMetrics::transFromLayout__float(220.0));
    let mut typo_drawing = obtain_drawing(main_state);
    typo_drawing.setCloneShape(shape);
    typo_drawing.set(0, 0, rad_width, rad_height);
    TypoObjBase::offsetTo__int_int_int_typosnapshot(rad_left_offset, rad_top_offset, rad_in, &mut main_state.snapshot);
    if rad_out != 0 {
        let mut rad_out_left = (rad_in_height as f32 * 0.3 - TypoDrawing::width__int(rad_out, &main_state.snapshot) as f32) as i32;
        let mut rad_out_top = (rad_height as f32 * 0.4 - TypoDrawing::height__int(rad_out, &main_state.snapshot) as f32) as i32;
        if rad_out_left < 0 || rad_out_top < 0 {
            let mut offset_x = 0;
            let mut offset_y = 0;
            if rad_out_left < 0 {
                offset_x = -rad_out_left;
                rad_out_left = 0;
            }
            if rad_out_top < 0 {
                offset_y = -rad_out_top;
                rad_out_top = 0;
            }
            TypoObjBase::offset__int_int_int_typosnapshot(offset_x, offset_y, rad_in, &mut main_state.snapshot);
            TypoObjBase::offset__int_int_int_typosnapshot(offset_x, offset_y, typo_drawing.getPointer(), &mut main_state.snapshot);
        }
        TypoObjBase::offsetTo__int_int_int_typosnapshot(rad_out_left, rad_out_top, rad_out, &mut main_state.snapshot);
    }
    typo_drawing
}
