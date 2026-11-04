#![allow(dead_code, non_snake_case)]
// Java: cn.wps.moffice.writer.layout.base.equation.EquationALayouter
use util::moffice::util::UnitUtil::UnitUtil;
use crate::cn::wps::moffice::writer::cache::TypoDrawing::TypoDrawing;
use crate::cn::wps::moffice::writer::cache::TypoObjBase::TypoObjBase;
use crate::cn::wps::moffice::writer::core::Document::Document;
use crate::cn::wps::moffice::writer::layout::base::env::DocumentEnv::DocumentEnv;
use crate::cn::wps::moffice::writer::layout::base::env::MainState::MainState;
use crate::cn::wps::moffice::writer::layout::base::env::ViewEnv::ViewEnv;
use crate::cn::wps::moffice::writer::layout::base::equation::EquationAlignment::EquationAlignment;
use crate::cn::wps::moffice::writer::layout::base::equation::EquationLayouter::{EquationException, EquationLayouter};
use crate::cn::wps::moffice::writer::layout::base::equation::EquationResult::EquationResult;
use crate::cn::wps::moffice::writer::layout::base::equation::IEquationLayouter::IEquationLayouter;

struct Row {
    pos: i32,
    drawings: Vec<i32>,
    eq_alignment: EquationAlignment,
}

struct Col {
    pos: i32,
    max_width: i32,
}

// java public class EquationALayouter extends IEquationLayouter
pub struct EquationALayouter;

impl EquationALayouter {
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
        let mut align = 1;
        let mut cols_n = 1;
        let mut v_space = 0;
        let mut h_space = 0;
        let op_size = eq_result.getOPSize();
        for i in 1..op_size {
            let op = eq_result.getOP(i);
            if op == "al" {
                align = 0;
            } else if op == "ar" {
                align = 2;
            } else if op.starts_with("co") {
                cols_n = IEquationLayouter::getInt(&op, 2).max(1);
            } else if op.starts_with("vs") {
                v_space = UnitUtil::PT2TWIPS(IEquationLayouter::getFloat(&op, 2));
            } else if op.starts_with("hs") {
                h_space = UnitUtil::PT2TWIPS(IEquationLayouter::getFloat(&op, 2));
            }
        }
        let mut cols = Vec::with_capacity(cols_n as usize);
        for _ in 0..cols_n {
            cols.push(Col { pos: 0, max_width: 0 });
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
        let mut rows: Vec<Row> = Vec::new();
        let mut cur_col_index = 0i32;
        let mut cur_drawing_height = 0;
        let half_of_v_space = v_space / 2;
        let values_size = eq_result.getValuesSize();
        for value_index in 0..values_size {
            if cur_col_index == 0 {
                rows.push(Row {
                    pos: cur_drawing_height + half_of_v_space,
                    drawings: Vec::new(),
                    eq_alignment: EquationAlignment::EquationAlignment(),
                });
            }
            let sub = EquationLayouter::getEquation(
                main_state,
                document_env,
                view_env,
                Some(document),
                eq_result.getStart(value_index),
                eq_result.getEnd(value_index),
                enable_revision,
                revision_final,
                false,
            )?;
            let row = rows.last_mut().expect("row");
            if sub != 0 {
                drawing.addChildDrawing(sub);
                row.drawings.push(sub);
                row.eq_alignment.onAddDrawing(sub, &main_state.snapshot);
                let width = TypoDrawing::width__int(sub, &main_state.snapshot);
                let col = &mut cols[cur_col_index as usize];
                col.max_width = col.max_width.max(width);
            } else {
                row.drawings.push(0);
            }
            cur_col_index += 1;
            if cur_col_index >= cols_n {
                cur_col_index = 0;
                let row = rows.last_mut().expect("row");
                row.eq_alignment.calcAlignLine();
                cur_drawing_height = row.pos + row.eq_alignment.getLineHeight() + half_of_v_space;
            }
        }
        if cur_col_index != 0 {
            let row = rows.last_mut().expect("row");
            row.eq_alignment.calcAlignLine();
            cur_drawing_height = row.pos + row.eq_alignment.getLineHeight() + half_of_v_space;
        }
        let width = match align {
            0 => do_align_left(h_space, &mut cols, &mut rows, &mut main_state.snapshot),
            2 => do_align_right(h_space, &mut cols, &mut rows, &mut main_state.snapshot),
            _ => do_align_center(h_space, &mut cols, &mut rows, &mut main_state.snapshot),
        };
        drawing.setWidth(width);
        drawing.setHeight(cur_drawing_height);
        IEquationLayouter::calcBaseline(&mut drawing, unsafe { &*document }, view_env, cur_drawing_height / 2, eq_result.getStartCP() + 1);
        let pointer = drawing.getPointer();
        main_state.snapshot.getTypoTools().recycle_drawing(&mut drawing);
        Ok(pointer)
    }
}

fn do_align_center(h_space: i32, cols: &mut [Col], rows: &mut [Row], snapshot: &mut crate::cn::wps::moffice::writer::cache::TypoSnapshot::TypoSnapshot) -> i32 {
    let half = h_space / 2;
    let mut cur = 0;
    for col in cols.iter_mut() {
        col.pos = cur + half;
        cur = col.pos + col.max_width + half;
    }
    place_cells(cols, rows, snapshot, |col, width| col.pos + (col.max_width - width) / 2);
    cur
}

fn do_align_right(h_space: i32, cols: &mut [Col], rows: &mut [Row], snapshot: &mut crate::cn::wps::moffice::writer::cache::TypoSnapshot::TypoSnapshot) -> i32 {
    let mut cur = 0;
    for col in cols.iter_mut() {
        col.pos = cur + h_space;
        cur = col.pos + col.max_width;
    }
    place_cells(cols, rows, snapshot, |col, width| col.pos + (col.max_width - width));
    cur
}

fn do_align_left(h_space: i32, cols: &mut [Col], rows: &mut [Row], snapshot: &mut crate::cn::wps::moffice::writer::cache::TypoSnapshot::TypoSnapshot) -> i32 {
    let mut cur = 0;
    for col in cols.iter_mut() {
        col.pos = cur;
        cur = col.pos + col.max_width + h_space;
    }
    place_cells(cols, rows, snapshot, |col, _width| col.pos);
    cur
}

fn place_cells(cols: &[Col], rows: &[Row], snapshot: &mut crate::cn::wps::moffice::writer::cache::TypoSnapshot::TypoSnapshot, left_of: impl Fn(&Col, i32) -> i32) {
    for row in rows {
        for (col_index, &cell) in row.drawings.iter().enumerate() {
            if cell == 0 {
                continue;
            }
            let col = &cols[col_index];
            let width = TypoDrawing::width__int(cell, snapshot);
            let top = row.pos + row.eq_alignment.getTop(cell, snapshot);
            TypoObjBase::offsetTo__int_int_int_typosnapshot(left_of(col, width), top, cell, snapshot);
        }
    }
}
