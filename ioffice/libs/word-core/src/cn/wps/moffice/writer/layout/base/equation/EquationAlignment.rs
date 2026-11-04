#![allow(dead_code, non_snake_case)]
// Java: cn.wps.moffice.writer.layout.base.equation.EquationAlignment
use crate::cn::wps::moffice::writer::cache::TypoDrawing::TypoDrawing;
use crate::cn::wps::moffice::writer::cache::TypoSnapshot::TypoSnapshot;

// java public class EquationAlignment
pub struct EquationAlignment {
    // java int maxAscentForBaseLine = 0;
    maxAscentForBaseLine: i32,
    // java int maxDescentForBaseLine = 0;
    maxDescentForBaseLine: i32,
    // java int lineHeight = 0;
    lineHeight: i32,
    // java private int alignType;
    alignType: i32,
    // java private int alignBaseLine;
    alignBaseLine: i32,
}

impl EquationAlignment {
    pub fn EquationAlignment() -> Self {
        Self {
            maxAscentForBaseLine: 0,
            maxDescentForBaseLine: 0,
            lineHeight: 0,
            alignType: 0,
            alignBaseLine: 0,
        }
    }

    // java public void reset()
    pub fn reset(&mut self) {
        self.maxAscentForBaseLine = 0;
        self.maxDescentForBaseLine = 0;
        self.lineHeight = 0;
    }

    // java public void onAddDrawing(int drawing, TypoSnapshot snapshot)
    pub fn onAddDrawing(&mut self, drawing: i32, snapshot: &TypoSnapshot) {
        if TypoDrawing::getAlignType__int_typosnapshot(drawing, snapshot) == 1 {
            let align_line = TypoDrawing::getAlignLine__int_typosnapshot(drawing, snapshot);
            self.maxAscentForBaseLine = self.maxAscentForBaseLine.max(align_line);
            let descent = TypoDrawing::height__int(drawing, snapshot) - align_line;
            self.maxDescentForBaseLine = self.maxDescentForBaseLine.max(descent);
        } else {
            self.maxAscentForBaseLine = self.maxAscentForBaseLine.max(TypoDrawing::height__int(drawing, snapshot));
        }
    }

    // java public void calcAlignLine()
    pub fn calcAlignLine(&mut self) {
        self.alignBaseLine = self.maxAscentForBaseLine;
        self.lineHeight = self.maxAscentForBaseLine + self.maxDescentForBaseLine;
    }

    // java public int getAlignType()
    pub fn getAlignType(&self) -> i32 {
        let _ = self.alignType;
        1
    }

    // java public int getAlignLine()
    pub fn getAlignLine(&self) -> i32 {
        self.alignBaseLine
    }

    // java public int getTop(int drawing, TypoSnapshot snapshot)
    pub fn getTop(&self, drawing: i32, snapshot: &TypoSnapshot) -> i32 {
        if TypoDrawing::getAlignType__int_typosnapshot(drawing, snapshot) == 1 {
            self.alignBaseLine - TypoDrawing::getAlignLine__int_typosnapshot(drawing, snapshot)
        } else {
            self.alignBaseLine - TypoDrawing::height__int(drawing, snapshot)
        }
    }

    // java public int getLineHeight()
    pub fn getLineHeight(&self) -> i32 {
        self.lineHeight
    }
}
