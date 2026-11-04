#![allow(dead_code, non_snake_case)]
// Java: cn.wps.moffice.writer.layout.base.math.LimLowerLayouter
use crate::cn::wps::moffice::writer::cache::TypoDrawing::TypoDrawing;
use crate::cn::wps::moffice::writer::core::Document::Document;
use crate::cn::wps::moffice::writer::data::PLCMathFunction::MathStartNode;
use crate::cn::wps::moffice::writer::layout::base::env::DocumentEnv::DocumentEnv;
use crate::cn::wps::moffice::writer::layout::base::env::MainState::MainState;
use crate::cn::wps::moffice::writer::layout::base::env::ViewEnv::ViewEnv;
use crate::cn::wps::moffice::writer::layout::base::math::LimLayouter::LimLayouter;

// java public class LimLowerLayouter extends LimLayouter
pub struct LimLowerLayouter {
    base: LimLayouter,
}

impl LimLowerLayouter {
    pub fn new(main: *mut MainState, doc: *mut DocumentEnv, view: *mut ViewEnv) -> Self {
        Self { base: LimLayouter::new(main, doc, view) }
    }

    pub fn layout(&mut self, document: *mut dyn Document, math_node: &MathStartNode, is_in_math_para: bool) -> Option<TypoDrawing> {
        self.base.init(document, math_node, is_in_math_para);
        let drawing = self.base.stack(false);
        self.base.recycle_parts();
        Some(drawing)
    }
}
