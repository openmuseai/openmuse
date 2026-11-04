#![allow(dead_code, non_snake_case)]
// Java: cn.wps.moffice.writer.layout.base.math.SPreLayouter
use crate::cn::wps::moffice::writer::cache::TypoDrawing::TypoDrawing;
use crate::cn::wps::moffice::writer::cache::TypoObjBase::TypoObjBase;
use crate::cn::wps::moffice::writer::core::Document::Document;
use crate::cn::wps::moffice::writer::data::PLCMathFunction::MathStartNode;
use crate::cn::wps::moffice::writer::layout::base::env::DocumentEnv::DocumentEnv;
use crate::cn::wps::moffice::writer::layout::base::env::MainState::MainState;
use crate::cn::wps::moffice::writer::layout::base::env::ViewEnv::ViewEnv;
use crate::cn::wps::moffice::writer::layout::base::math::MathLayouterBase::{offset_child, offset_drawing_to};
use crate::cn::wps::moffice::writer::layout::base::math::SupSubBaseLayouter::{sub_offset, sup_offset, SupSubBaseLayouter};

// java public class SPreLayouter extends SupSubBaseLayouter
pub struct SPreLayouter {
    base: SupSubBaseLayouter,
}

impl SPreLayouter {
    pub fn new(main: *mut MainState, doc: *mut DocumentEnv, view: *mut ViewEnv) -> Self {
        Self { base: SupSubBaseLayouter::new(main, doc, view) }
    }

    // java public TypoDrawing layout(...)
    pub fn layout(&mut self, document: *mut dyn Document, math_node: &MathStartNode, is_in_math_para: bool) -> Option<TypoDrawing> {
        if !self.base.init(document, math_node, is_in_math_para) {
            return None;
        }
        let drawing = self.layout_s_pre(unsafe { &*document });
        self.base.recycle_parts();
        Some(drawing)
    }

    fn layout_s_pre(&mut self, document: &dyn Document) -> TypoDrawing {
        let font_size = self.base.base.font_size;
        let start_cp = self.base.base.start_cp;
        let mut drawing = self.base.base.obtainTypoDrawing();
        let s_ptr = self.base.main_drawing.as_ref().map(|d| d.getPointer()).unwrap_or(0);
        let sup_ptr = self.base.sup.as_ref().map(|d| d.getPointer()).unwrap_or(0);
        let sub_ptr = self.base.sub.as_ref().map(|d| d.getPointer()).unwrap_or(0);
        let s_height = if s_ptr != 0 {
            drawing.addChildDrawing(s_ptr);
            TypoDrawing::layoutHeight__int_typosnapshot(s_ptr, self.base.base.snapshot())
        } else {
            0
        };
        let mut sup_top = 0;
        let mut sup_width = 0;
        if sup_ptr != 0 {
            drawing.addChildDrawing(sup_ptr);
            let sup_h = TypoDrawing::layoutHeight__int_typosnapshot(sup_ptr, self.base.base.snapshot());
            sup_top = sup_offset(sup_h, s_height, font_size);
            sup_width = TypoDrawing::layoutWidth__int_typosnapshot(sup_ptr, self.base.base.snapshot());
            offset_drawing_to(sup_ptr, -sup_width, sup_top, self.base.base.snapshot());
            if sup_top > 0 {
                sup_top = 0;
            }
        }
        let mut sub_width = 0;
        if sub_ptr != 0 {
            drawing.addChildDrawing(sub_ptr);
            let sub_h = TypoDrawing::layoutHeight__int_typosnapshot(sub_ptr, self.base.base.snapshot());
            let sub_top = sub_offset(sub_h, s_height, font_size);
            sub_width = TypoDrawing::layoutWidth__int_typosnapshot(sub_ptr, self.base.base.snapshot());
            offset_drawing_to(sub_ptr, -sub_width, sub_top, self.base.base.snapshot());
        }
        offset_child(&mut drawing, self.base.base.snapshot(), sup_width.max(sub_width), -sup_top, true);
        if s_ptr != 0 {
            let snap = self.base.base.snapshot();
            let top = TypoObjBase::getTop__int_typosnapshot(s_ptr, snap);
            let align = TypoDrawing::getAlignLine__int_typosnapshot(s_ptr, snap);
            let align_type = TypoDrawing::getAlignType__int_typosnapshot(s_ptr, snap);
            drawing.setAlignLine(top + align);
            drawing.setAlignType(align_type);
        } else {
            let height = TypoDrawing::layoutHeight__int_typosnapshot(drawing.getPointer(), self.base.base.snapshot());
            self.base.base.calcBaseline(&mut drawing, document, height / 2, start_cp);
        }
        drawing
    }
}
