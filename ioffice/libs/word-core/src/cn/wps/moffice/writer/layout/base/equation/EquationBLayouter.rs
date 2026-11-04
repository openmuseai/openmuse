#![allow(dead_code, non_snake_case)]
// Java: cn.wps.moffice.writer.layout.base.equation.EquationBLayouter
use crate::cn::wps::moffice::writer::cache::TypoDrawing::TypoDrawing;
use crate::cn::wps::moffice::writer::cache::TypoObjBase::TypoObjBase;
use crate::cn::wps::moffice::writer::core::Document::Document;
use crate::cn::wps::moffice::writer::layout::base::env::DocumentEnv::DocumentEnv;
use crate::cn::wps::moffice::writer::layout::base::env::MainState::MainState;
use crate::cn::wps::moffice::writer::layout::base::env::ViewEnv::ViewEnv;
use crate::cn::wps::moffice::writer::layout::base::equation::EquationContentLayouter::EquationContentLayouter;
use crate::cn::wps::moffice::writer::layout::base::equation::EquationLayouter::{EquationException, EquationLayouter};
use crate::cn::wps::moffice::writer::layout::base::equation::EquationResult::EquationResult;
use crate::cn::wps::moffice::writer::layout::base::equation::IEquationLayouter::IEquationLayouter;

const LEFT_PUNCTS: &str = "([{";
const RIGHT_PUNCTS: &str = ")]}";
const OTHER_PUNCTS: &str = "|";

// java public class EquationBLayouter extends IEquationLayouter
pub struct EquationBLayouter;

impl EquationBLayouter {
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
            return Err(EquationException);
        }
        let sub = EquationLayouter::getEquation(
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
        if sub == 0 {
            return Ok(0);
        }
        let mut drawing = obtain_drawing(main_state);
        drawing.addChildDrawing(sub);
        let def_color = IEquationLayouter::finalRunColor(unsafe { &*document }, revision_final, eq_result.getStartCP() + 1);
        let mut left_char = '(';
        let mut left_scale_v = true;
        let mut left_color = def_color;
        let mut right_char = ')';
        let mut right_scale_v = true;
        let mut right_color = def_color;
        let op_size = eq_result.getOPSize();
        if op_size <= 2 {
            if op_size == 2 {
                let op = eq_result.getOP(1).to_ascii_lowercase();
                if op != "lc" && op != "rc" && op != "bc" {
                    main_state.snapshot.getTypoTools().recycle_drawing(&mut drawing);
                    return Err(EquationException);
                }
            }
        } else {
            left_char = '\0';
            right_char = '\0';
            let mut i = 1;
            while i < op_size {
                let op = eq_result.getOP(i).to_ascii_lowercase();
                if op == "lc" {
                    i += 1;
                    if i < op_size {
                        let value = eq_result.getOP(i);
                        left_color = IEquationLayouter::finalRunColor(unsafe { &*document }, revision_final, eq_result.getOPStartCP(i));
                        left_char = value.chars().next().unwrap_or('\0');
                        left_scale_v = just_scale_v(left_char);
                        i += 1;
                        continue;
                    }
                }
                if op == "rc" {
                    i += 1;
                    if i < op_size {
                        let value = eq_result.getOP(i);
                        right_color = IEquationLayouter::finalRunColor(unsafe { &*document }, revision_final, eq_result.getOPStartCP(i));
                        right_char = value.chars().next().unwrap_or('\0');
                        right_scale_v = just_scale_v(right_char);
                        i += 1;
                        continue;
                    }
                }
                if op != "bc" {
                    main_state.snapshot.getTypoTools().recycle_drawing(&mut drawing);
                    return Err(EquationException);
                }
                i += 1;
                if i >= op_size {
                    main_state.snapshot.getTypoTools().recycle_drawing(&mut drawing);
                    return Err(EquationException);
                }
                let value = eq_result.getOP(i);
                left_char = value.chars().next().unwrap_or('\0');
                let color = IEquationLayouter::finalRunColor(unsafe { &*document }, revision_final, eq_result.getOPStartCP(i));
                left_color = color;
                right_color = color;
                right_char = match LEFT_PUNCTS.chars().position(|c| c == left_char) {
                    Some(index) => RIGHT_PUNCTS.chars().nth(index).unwrap_or(left_char),
                    None => left_char,
                };
                left_scale_v = just_scale_v(left_char);
                right_scale_v = left_scale_v;
                i += 1;
            }
        }
        let left = char_drawing(main_state, document_env, view_env, document, left_char, left_color);
        let right = char_drawing(main_state, document_env, view_env, document, right_char, right_color);
        let mut drawing_width = 0;
        if left != 0 {
            drawing.addChildDrawing(left);
            let mut left_drawing = main_state.snapshot.getTypoTools().obtainTypoDrawing__int(left);
            let sub_h = TypoDrawing::height__int(sub, &main_state.snapshot);
            let left_h = left_drawing.height();
            let scale = if left_h == 0 { f32::INFINITY } else { sub_h as f32 / left_h as f32 };
            IEquationLayouter::setMathScale(&mut left_drawing, &mut main_state.snapshot, scale, if left_scale_v { 2 } else { 0 });
            left_drawing.offsetTo(0, 0);
            drawing_width += TypoDrawing::layoutWidth__int_typosnapshot(left, &main_state.snapshot);
            main_state.snapshot.getTypoTools().recycle_drawing(&mut left_drawing);
        }
        TypoObjBase::offsetTo__int_int_int_typosnapshot(drawing_width, 0, sub, &mut main_state.snapshot);
        drawing_width += TypoDrawing::width__int(sub, &main_state.snapshot);
        if right != 0 {
            drawing.addChildDrawing(right);
            let mut right_drawing = main_state.snapshot.getTypoTools().obtainTypoDrawing__int(right);
            let sub_h = TypoDrawing::height__int(sub, &main_state.snapshot);
            let right_h = right_drawing.height();
            let scale = if right_h == 0 { f32::INFINITY } else { sub_h as f32 / right_h as f32 };
            IEquationLayouter::setMathScale(&mut right_drawing, &mut main_state.snapshot, scale, if right_scale_v { 2 } else { 0 });
            right_drawing.offsetTo(drawing_width, 0);
            drawing_width += TypoDrawing::layoutWidth__int_typosnapshot(right, &main_state.snapshot);
            main_state.snapshot.getTypoTools().recycle_drawing(&mut right_drawing);
        }
        drawing.setAlignLine(TypoDrawing::getAlignLine__int_typosnapshot(sub, &main_state.snapshot));
        drawing.setAlignType(TypoDrawing::getAlignType__int_typosnapshot(sub, &main_state.snapshot));
        drawing.setWidth(drawing_width);
        drawing.setHeight(TypoDrawing::height__int(sub, &main_state.snapshot));
        let pointer = drawing.getPointer();
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

fn char_drawing(main_state: &mut MainState, document_env: &mut DocumentEnv, view_env: &mut ViewEnv, document: *mut dyn Document, ch: char, color: i32) -> i32 {
    if ch == '\0' {
        return 0;
    }
    EquationContentLayouter::get_char_drawing(
        main_state,
        document_env,
        view_env,
        document,
        ch,
        color,
        ch == '*',
        true,
    )
}

fn just_scale_v(ch: char) -> bool {
    LEFT_PUNCTS.contains(ch) || RIGHT_PUNCTS.contains(ch) || OTHER_PUNCTS.contains(ch)
}
