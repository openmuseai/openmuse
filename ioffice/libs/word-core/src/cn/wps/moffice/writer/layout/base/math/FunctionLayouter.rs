#![allow(dead_code, non_snake_case)]
// Java: cn.wps.moffice.writer.layout.base.math.FunctionLayouter
use crate::cn::wps::moffice::writer::cache::TypoDrawing::TypoDrawing;
use crate::cn::wps::moffice::writer::core::Document::Document;
use crate::cn::wps::moffice::writer::data::PLCMathFunction::MathStartNode;
use crate::cn::wps::moffice::writer::layout::base::env::DocumentEnv::DocumentEnv;
use crate::cn::wps::moffice::writer::layout::base::env::MainState::MainState;
use crate::cn::wps::moffice::writer::layout::base::env::ViewEnv::ViewEnv;
use crate::cn::wps::moffice::writer::layout::base::math::MathLayouterBase::MathLayouterBase;

// java public class FunctionLayouter extends MathLayouterBase
pub struct FunctionLayouter {
    base: MathLayouterBase,
}

impl FunctionLayouter {
    // java public FunctionLayouter(MainState mainState, DocumentEnv documentEnv, ViewEnv viewEnv)
    pub fn new(main: *mut MainState, doc: *mut DocumentEnv, view: *mut ViewEnv) -> Self {
        Self { base: MathLayouterBase::new(main, doc, view) }
    }

    // java public TypoDrawing layout(Document document, PLCMathFunction.MathStartNode mathNode, boolean isInMathPara)
    pub fn layout(&mut self, document: *mut dyn Document, math_node: &MathStartNode, is_in_math_para: bool) -> Option<TypoDrawing> {
        // java this.mMathContent = true;
        self.base.math_content = true;
        // java readGraphBuff inserts the function-name separator when one exists.
        self.base.function_sep = true;
        // java this.init(document, mathNode, isInMathPara);
        self.base.init(unsafe { &*document }, math_node, is_in_math_para);
        // java TypoDrawing drawing = this.getTypoDrawing(document, this.mStartCp + 1, this.mEndCp);
        let drawing = self.base.getTypoDrawing(document, self.base.start_cp + 1, self.base.end_cp, None);
        // java this.removeStrikeThroughs(drawing);
        if let Some(drawing) = drawing.as_ref() {
            self.base.removeStrikeThroughs(drawing);
        }
        drawing
    }
}
