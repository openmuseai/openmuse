#![allow(dead_code, non_snake_case)]
// Java: cn.wps.moffice.writer.layout.base.math.MatrixLayouter
use util::moffice::util::UnitUtil::UnitUtil;
use crate::cn::wps::moffice::writer::cache::TypoDrawing::TypoDrawing;
use crate::cn::wps::moffice::writer::core::Document::Document;
use crate::cn::wps::moffice::writer::data::MathTool::MathTool;
use crate::cn::wps::moffice::writer::data::PLCMathFunction::MathStartNode;
use crate::cn::wps::moffice::writer::layout::base::env::DocumentEnv::DocumentEnv;
use crate::cn::wps::moffice::writer::layout::base::env::MainState::MainState;
use crate::cn::wps::moffice::writer::layout::base::env::ViewEnv::ViewEnv;
use crate::cn::wps::moffice::writer::layout::base::math::MathLayouterBase::{offset_drawing_to, MathLayouterBase};

struct Row {
    drawings: Vec<TypoDrawing>,
    max_ascent: i32,
    max_descent: i32,
}

impl Row {
    fn new() -> Self {
        Self { drawings: Vec::new(), max_ascent: i32::MIN, max_descent: i32::MIN }
    }
}

// java public class MatrixLayouter extends MathLayouterBase
pub struct MatrixLayouter {
    base: MathLayouterBase,
    rows: Vec<Row>,
    widths: Vec<i32>,
    dist_col: i32,
    dist_row: i32,
}

impl MatrixLayouter {
    pub fn new(main: *mut MainState, doc: *mut DocumentEnv, view: *mut ViewEnv) -> Self {
        Self { base: MathLayouterBase::new(main, doc, view), rows: Vec::new(), widths: Vec::new(), dist_col: 0, dist_row: 0 }
    }

    // java public TypoDrawing layout(...)
    pub fn layout(&mut self, document: *mut dyn Document, math_node: &MathStartNode, is_in_math_para: bool) -> Option<TypoDrawing> {
        let doc = unsafe { &*document };
        self.base.init(doc, math_node, is_in_math_para);
        let mut sep_cp = MathTool::getMathSepaCP(doc, self.base.start_cp);
        if sep_cp < 0 {
            return None;
        }
        self.rows.clear();
        self.widths.clear();
        let mut drawing = self.base.obtainTypoDrawing();
        self.cal_dist(doc, self.base.start_cp);
        let mut row = Row::new();
        let mut col = 0;
        let mut cp = self.base.start_cp + 1;
        while sep_cp < self.base.end_cp {
            self.add_to_row(document, cp, sep_cp, &mut row, col, &mut drawing);
            col += 1;
            if self.base.getChar(doc, sep_cp) == '@' {
                self.rows.push(row);
                row = Row::new();
                col = 0;
            }
            cp = sep_cp + 1;
            sep_cp = MathTool::getMathSepaCP(doc, sep_cp);
            if sep_cp < 0 || sep_cp >= self.base.end_cp {
                self.add_to_row(document, cp, self.base.end_cp, &mut row, col, &mut drawing);
                self.rows.push(row);
                break;
            }
        }
        self.adjust(&mut drawing, doc);
        Some(drawing)
    }

    fn add_to_row(&mut self, document: *mut dyn Document, start: i32, end: i32, row: &mut Row, col: usize, parent: &mut TypoDrawing) {
        let doc = unsafe { &*document };
        let mut cell = if end > start { self.base.layout_span(document, start, end) } else { None };
        if cell.is_none() {
            cell = Some(self.base.getEmptyDrawing(doc, UnitUtil::PT2TWIPS(1.0)));
        }
        let cell = cell.unwrap();
        let align = TypoDrawing::getAlignLine__int_typosnapshot(cell.getPointer(), self.base.snapshot());
        let height = cell.height();
        row.max_ascent = row.max_ascent.max(align);
        row.max_descent = row.max_descent.max(height - align);
        let cell_width = cell.width();
        if col >= self.widths.len() {
            self.widths.push(cell_width);
        } else {
            self.widths[col] = self.widths[col].max(cell_width);
        }
        parent.addChildDrawing(cell.getPointer());
        row.drawings.push(cell);
    }

    fn cal_dist(&mut self, document: &dyn Document, cp: i32) {
        let size = self.base.getFontSize(document, cp);
        self.dist_col = size;
        self.dist_row = (0.4 * size as f32) as i32;
    }

    fn adjust(&mut self, drawing: &mut TypoDrawing, document: &dyn Document) {
        let mut top = -self.dist_row;
        let mut placed: Vec<(i32, i32, i32)> = Vec::new();
        for row in &self.rows {
            top += self.dist_row;
            let baseline = top + row.max_ascent;
            let mut x = -self.dist_col;
            for (j, cell) in row.drawings.iter().enumerate() {
                x += self.dist_col;
                let cell_width = self.widths.get(j).copied().unwrap_or(0);
                let drawing_x = x + (cell_width - cell.width()) / 2;
                let align = TypoDrawing::getAlignLine__int_typosnapshot(cell.getPointer(), self.base.snapshot());
                placed.push((cell.getPointer(), drawing_x, baseline - align));
                x += cell_width;
            }
            top += row.max_ascent + row.max_descent;
        }
        for (pointer, x, y) in placed {
            offset_drawing_to(pointer, x, y, self.base.snapshot());
        }
        let mut width = 0;
        for cell_width in &self.widths {
            width += *cell_width;
        }
        width += self.dist_col * (self.widths.len() as i32 - 1);
        drawing.set(0, 0, width, top);
        let start = self.base.start_cp;
        self.base.calcBaseline(drawing, document, top / 2, start);
        let rows = std::mem::take(&mut self.rows);
        for mut row in rows {
            for cell in &mut row.drawings {
                self.base.recycle_drawing(cell);
            }
        }
    }
}
