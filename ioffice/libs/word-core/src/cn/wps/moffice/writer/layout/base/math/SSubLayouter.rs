#![allow(dead_code, non_snake_case)]
// Java: cn.wps.moffice.writer.layout.base.math.SSubLayouter
use crate::cn::wps::moffice::writer::cache::TypoDrawing::TypoDrawing;
use crate::cn::wps::moffice::writer::core::Document::Document;
use crate::cn::wps::moffice::writer::data::PLCMathFunction::MathStartNode;
use crate::cn::wps::moffice::writer::layout::base::env::DocumentEnv::DocumentEnv;
use crate::cn::wps::moffice::writer::layout::base::env::MainState::MainState;
use crate::cn::wps::moffice::writer::layout::base::env::ViewEnv::ViewEnv;
use crate::cn::wps::moffice::writer::layout::base::math::SSubSupLayouter::SSubSupLayouter;

// java public class SSubLayouter extends SSubSupLayouter
pub struct SSubLayouter {
    base: SSubSupLayouter,
}

impl SSubLayouter {
    pub fn new(main: *mut MainState, doc: *mut DocumentEnv, view: *mut ViewEnv) -> Self {
        Self { base: SSubSupLayouter::new(main, doc, view) }
    }

    // java public TypoDrawing layout(...)
    pub fn layout(&mut self, document: *mut dyn Document, math_node: &MathStartNode, is_in_math_para: bool) -> Option<TypoDrawing> {
        // java this.mHideSup = true;
        self.base.base.hide_sup = true;
        self.base.layout(document, math_node, is_in_math_para)
    }
}
