#![allow(dead_code, non_snake_case)]
// Java: cn.wps.moffice.writer.layout.base.math.SSubSupLayouter
use crate::cn::wps::moffice::writer::cache::TypoDrawing::TypoDrawing;
use crate::cn::wps::moffice::writer::core::Document::Document;
use crate::cn::wps::moffice::writer::data::PLCMathFunction::MathStartNode;
use crate::cn::wps::moffice::writer::layout::base::env::DocumentEnv::DocumentEnv;
use crate::cn::wps::moffice::writer::layout::base::env::MainState::MainState;
use crate::cn::wps::moffice::writer::layout::base::env::ViewEnv::ViewEnv;
use crate::cn::wps::moffice::writer::layout::base::math::SupSubBaseLayouter::SupSubBaseLayouter;

// java public class SSubSupLayouter extends SupSubBaseLayouter
pub struct SSubSupLayouter {
    pub base: SupSubBaseLayouter,
}

impl SSubSupLayouter {
    pub fn new(main: *mut MainState, doc: *mut DocumentEnv, view: *mut ViewEnv) -> Self {
        Self { base: SupSubBaseLayouter::new(main, doc, view) }
    }

    // java public TypoDrawing layout(...)
    pub fn layout(&mut self, document: *mut dyn Document, math_node: &MathStartNode, is_in_math_para: bool) -> Option<TypoDrawing> {
        if !self.base.init(document, math_node, is_in_math_para) {
            return None;
        }
        let font_size = self.base.base.font_size;
        let s = self.base.main_drawing.as_ref().map(|d| d.getPointer()).unwrap_or(0);
        let sub = self.base.sub.as_ref().map(|d| d.getPointer()).unwrap_or(0);
        let sup = self.base.sup.as_ref().map(|d| d.getPointer()).unwrap_or(0);
        let drawing = self.base.layout_s_sub_sup(s, sub, sup, font_size, unsafe { &*document });
        self.base.recycle_parts();
        Some(drawing)
    }
}
