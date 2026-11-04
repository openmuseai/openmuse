#![allow(dead_code, non_snake_case)]
// Java: cn.wps.moffice.writer.layout.base.equation.EquationFLayouter
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

// java public class EquationFLayouter extends IEquationLayouter
pub struct EquationFLayouter;

impl EquationFLayouter {
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
        if eq_result.getValuesSize() != 2 {
            return Err(EquationException);
        }
        let numerator = EquationLayouter::getEquation(
            main_state, document_env, view_env, Some(document),
            eq_result.getStart(0), eq_result.getEnd(0), enable_revision, revision_final, true,
        )?;
        let denominator = EquationLayouter::getEquation(
            main_state, document_env, view_env, Some(document),
            eq_result.getStart(1), eq_result.getEnd(1), enable_revision, revision_final, true,
        )?;
        if numerator == 0 && denominator == 0 {
            return Ok(0);
        }
        let mut drawing = obtain_drawing(main_state);
        let mut sep_width = 0;
        let mut drawing_height = 0;
        if numerator != 0 {
            drawing.addChildDrawing(numerator);
            sep_width = TypoDrawing::width__int(numerator, &main_state.snapshot);
            drawing_height = TypoDrawing::height__int(numerator, &main_state.snapshot);
        }
        if denominator != 0 {
            drawing.addChildDrawing(denominator);
            sep_width = sep_width.max(TypoDrawing::width__int(denominator, &main_state.snapshot));
            drawing_height += TypoDrawing::height__int(denominator, &main_state.snapshot);
        }
        let color = IEquationLayouter::finalRunColor(unsafe { &*document }, revision_final, eq_result.getEnd(1));
        let mut sep = sep_drawing(main_state, sep_width, color);
        drawing_height += sep.height() + 88;
        if numerator != 0 {
            TypoObjBase::offsetTo__int_int_int_typosnapshot(
                (sep_width - TypoDrawing::width__int(numerator, &main_state.snapshot)) / 2,
                0,
                numerator,
                &mut main_state.snapshot,
            );
            sep.offsetTo(0, TypoObjBase::getBottom__int_typosnapshot(numerator, &main_state.snapshot) + 44);
        } else {
            sep.offsetTo(0, 44);
        }
        if denominator != 0 {
            TypoObjBase::offsetTo__int_int_int_typosnapshot(
                (sep_width - TypoDrawing::width__int(denominator, &main_state.snapshot)) / 2,
                sep.getBottom() + 44,
                denominator,
                &mut main_state.snapshot,
            );
        }
        drawing.addChildDrawing(sep.getPointer());
        drawing.setWidth(sep_width);
        drawing.setHeight(drawing_height);
        IEquationLayouter::calcBaseline(&mut drawing, unsafe { &*document }, view_env, (sep.getTop() + sep.getBottom()) / 2, eq_result.getStartCP() + 1);
        let pointer = drawing.getPointer();
        main_state.snapshot.getTypoTools().recycle_drawing(&mut sep);
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

fn sep_drawing(main_state: &mut MainState, sep_width: i32, color: i32) -> TypoDrawing {
    let mut drawing = obtain_drawing(main_state);
    drawing.set(0, 0, sep_width, 22);
    let rect = RectF::new_with(0.0, 0.0, ShapeMetrics::transFromLayout__float(sep_width as f32), ShapeMetrics::transFromLayout__float(22.0));
    let shape = SepShapeFactory::getShape(1, &rect, ShapeMetrics::transFromLayout__float(11.0), color, ShapeMetrics::transFromLayout__float(220.0));
    drawing.setCloneShape(shape);
    drawing
}
