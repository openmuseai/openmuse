#![allow(dead_code, non_snake_case)]
// Java: cn.wps.moffice.writer.layout.base.math.ArrayLayouter
use crate::cn::wps::collections::textrope::ITextRope::ITextRope;
use crate::cn::wps::moffice::writer::cache::TypoDrawing::TypoDrawing;
use crate::cn::wps::moffice::writer::cache::TypoObjBase::TypoObjBase;
use crate::cn::wps::moffice::writer::core::Document::Document;
use crate::cn::wps::moffice::writer::data::MathTool::MathTool;
use crate::cn::wps::moffice::writer::data::PLCMathFunction::{MathNode, MathStartNode};
use crate::cn::wps::moffice::writer::layout::base::env::DocumentEnv::DocumentEnv;
use crate::cn::wps::moffice::writer::layout::base::env::MainState::MainState;
use crate::cn::wps::moffice::writer::layout::base::env::ViewEnv::ViewEnv;
use crate::cn::wps::moffice::writer::layout::base::math::MathLayouterBase::MathLayouterBase;

struct Row {
    drawings: Vec<Option<TypoDrawing>>,
    max_ascent: i32,
    max_descent: i32,
}

// java public class ArrayLayouter extends MathLayouterBase
pub struct ArrayLayouter {
    base: MathLayouterBase,
    row_pos: i32,
    rows: Vec<Row>,
    col_count: i32,
}

impl ArrayLayouter {
    // java public ArrayLayouter(MainState mainState, DocumentEnv documentEnv, ViewEnv viewEnv)
    pub fn new(main: *mut MainState, doc: *mut DocumentEnv, view: *mut ViewEnv) -> Self {
        Self { base: MathLayouterBase::new(main, doc, view), row_pos: 0, rows: Vec::new(), col_count: 0 }
    }

    // java public TypoDrawing layout(Document document, PLCMathFunction.MathStartNode mathNode, boolean isInMathPara)
    pub fn layout(&mut self, document: *mut dyn Document, math_node: &MathStartNode, is_in_math_para: bool) -> Option<TypoDrawing> {
        let doc = unsafe { &*document };
        self.base.init(doc, math_node, is_in_math_para);
        let _ = math_node.getPropertySet();
        let mut drawing = self.base.obtainTypoDrawing();
        let mut last_cp = self.base.start_cp;
        let mut cur_cp = MathTool::getMathSepaCP(doc, last_cp);
        let min_dist = (self.base.font_size as f32 * 0.25) as i32;
        self.row_pos = 0;
        self.rows.clear();
        self.col_count = 0;
        while cur_cp <= self.base.end_cp {
            self.layout_row(document, last_cp + 1, cur_cp, min_dist, &mut drawing);
            last_cp = cur_cp;
            if cur_cp >= self.base.end_cp {
                break;
            }
            cur_cp = MathTool::getMathSepaCP(doc, cur_cp);
        }
        if self.col_count > 0 {
            self.adjust_cols(&mut drawing);
        }
        self.rows.clear();
        drawing.setHeight(self.row_pos - min_dist);
        let half = drawing.height() / 2;
        self.base.calcBaseline(&mut drawing, doc, half, self.base.start_cp);
        Some(drawing)
    }

    fn adjust_cols(&mut self, parent: &mut TypoDrawing) {
        let mut widths = vec![0; self.col_count as usize];
        let row_pointers: Vec<Vec<i32>> = self.rows.iter().map(|row| {
            row.drawings.iter().map(|d| d.as_ref().map(|drawing| drawing.getPointer()).unwrap_or(0)).collect()
        }).collect();
        for pointers in &row_pointers {
            let mut drawings_size = pointers.len() as i32;
            if drawings_size % 2 != 0 {
                drawings_size -= 1;
            }
            for j in 0..drawings_size {
                let pointer = pointers[j as usize];
                if pointer != 0 {
                    let width = TypoDrawing::layoutWidth__int_typosnapshot(pointer, self.base.snapshot());
                    widths[j as usize] = widths[j as usize].max(width);
                }
            }
        }
        for pointers in &row_pointers {
            let drawings_size = pointers.len() as i32;
            if drawings_size % 2 != 0 {
                let col_index = drawings_size - 1;
                let pointer = pointers[col_index as usize];
                if pointer != 0 {
                    let width = TypoDrawing::layoutWidth__int_typosnapshot(pointer, self.base.snapshot());
                    let two_col = widths[col_index as usize] + if col_index < self.col_count - 1 { widths[(col_index + 1) as usize] } else { 0 };
                    let extent = width - two_col;
                    if extent > 0 {
                        if col_index < self.col_count - 1 {
                            let half = extent / 2;
                            widths[col_index as usize] += half;
                            widths[(col_index + 1) as usize] += half;
                        } else {
                            widths[col_index as usize] += extent;
                        }
                    }
                }
            }
        }
        let mut pos = vec![0; self.col_count as usize];
        let mut cur_pos = 0;
        for i in 0..self.col_count {
            pos[i as usize] = cur_pos;
            cur_pos += widths[i as usize];
        }
        for pointers in &row_pointers {
            let drawings_size = pointers.len() as i32;
            let max = if drawings_size % 2 == 0 { drawings_size } else { drawings_size - 1 };
            let mut index = 0;
            while index < max {
                let pointer = pointers[index as usize];
                if pointer != 0 {
                    let layout_width = TypoDrawing::layoutWidth__int_typosnapshot(pointer, self.base.snapshot());
                    let x = if index % 2 == 0 {
                        pos[index as usize] + widths[index as usize] - layout_width
                    } else {
                        pos[index as usize]
                    };
                    TypoObjBase::offsetXTo__int_int_typosnapshot(x, pointer, self.base.snapshot());
                }
                index += 1;
            }
            if index < drawings_size && index % 2 == 0 {
                let pointer = pointers[index as usize];
                if pointer != 0 {
                    let layout_width = TypoDrawing::layoutWidth__int_typosnapshot(pointer, self.base.snapshot());
                    let two_col = widths[index as usize] + if index < self.col_count - 1 { widths[(index + 1) as usize] } else { 0 };
                    TypoObjBase::offsetXTo__int_int_typosnapshot(pos[index as usize] + (two_col - layout_width) / 2, pointer, self.base.snapshot());
                }
            }
        }
        let mut held = Vec::new();
        for row in &mut self.rows {
            for slot in &mut row.drawings {
                if let Some(drawing) = slot.take() {
                    held.push(drawing);
                }
            }
        }
        for drawing in &mut held {
            self.base.recycle_drawing(drawing);
        }
        let last = (self.col_count - 1) as usize;
        parent.setWidth(pos[last] + widths[last]);
    }

    fn layout_row(&mut self, document: *mut dyn Document, start: i32, end: i32, min_dist: i32, parent: &mut TypoDrawing) {
        self.rows.push(Row { drawings: Vec::new(), max_ascent: 0, max_descent: 0 });
        if start >= end {
            if let Some(mut empty) = self.add_drawing(document, start, end, parent) {
                let left = empty.getLeft();
                TypoObjBase::offsetTo__int_int_int_typosnapshot(left, self.row_pos, empty.getPointer(), self.base.snapshot());
                let row = self.rows.last().unwrap();
                self.row_pos += row.max_ascent + row.max_descent + min_dist;
                let _ = empty;
            }
            self.col_count = self.col_count.max(1);
            return;
        }
        let doc = unsafe { &*document };
        let mut last_control = start - 1;
        if let Some(plc) = doc.getPLCMathFunction().filter(|p| !p.is_null()) {
            let plc = unsafe { &*plc };
            let mut cur = start;
            while cur < end {
                let node = plc.findUpper(cur);
                let mut cur_max = end;
                if !plc.isHead(node) && node.getCP() < end {
                    cur_max = node.getCP();
                }
                if cur_max > cur {
                    let mut buf = vec!['\0'; (cur_max - cur) as usize];
                    if let Some(rope) = doc.getTextRope().filter(|p| !p.is_null()) {
                        unsafe { (*rope).getText(cur, cur_max, &mut buf, 0) };
                    }
                    for (i, ch) in buf.iter().enumerate() {
                        if *ch == '&' {
                            let cur_control = i as i32 + cur;
                            if last_control + 1 < cur_control {
                                self.add_drawing(document, last_control + 1, cur_control, parent);
                            } else {
                                self.rows.last_mut().unwrap().drawings.push(None);
                            }
                            last_control = cur_control;
                        }
                    }
                }
                if cur_max == end {
                    break;
                }
                let math = MathNode { super_: node, mMathChar: '\u{001c}' };
                let next = math.getEndNode().map(|n| n.getCP() + 1).unwrap_or(end);
                if next <= cur {
                    break;
                }
                cur = next;
            }
        }
        if last_control + 1 < end {
            self.add_drawing(document, last_control + 1, end, parent);
        }
        let col = self.rows.last().map(|r| r.drawings.len() as i32).unwrap_or(0);
        self.col_count = self.col_count.max(col);
        let row_pos = self.row_pos;
        let max_ascent = self.rows.last().map(|r| r.max_ascent).unwrap_or(0);
        let pointers: Vec<i32> = self.rows.last().map(|row| row.drawings.iter().flatten().map(|d| d.getPointer()).collect()).unwrap_or_default();
        for pointer in pointers {
            let align = TypoDrawing::getAlignLine__int_typosnapshot(pointer, self.base.snapshot());
            let left = TypoObjBase::getLeft__int_typosnapshot(pointer, self.base.snapshot());
            TypoObjBase::offsetTo__int_int_int_typosnapshot(left, row_pos + max_ascent - align, pointer, self.base.snapshot());
        }
        let max_descent = self.rows.last().map(|r| r.max_descent).unwrap_or(0);
        self.row_pos += max_ascent + max_descent + min_dist;
    }

    fn add_drawing(&mut self, document: *mut dyn Document, start: i32, end: i32, parent: &mut TypoDrawing) -> Option<TypoDrawing> {
        let drawing = self.base.layout_range(document, start, end, true)?;
        let pointer = drawing.getPointer();
        let max_ascent = TypoDrawing::getAlignLine__int_typosnapshot(pointer, self.base.snapshot());
        let max_descent = TypoDrawing::layoutHeight__int_typosnapshot(pointer, self.base.snapshot()) - max_ascent;
        if let Some(row) = self.rows.last_mut() {
            row.max_ascent = row.max_ascent.max(max_ascent);
            row.max_descent = row.max_descent.max(max_descent);
            row.drawings.push(Some(drawing.clone()));
        }
        parent.addChildDrawing(pointer);
        Some(drawing)
    }
}
