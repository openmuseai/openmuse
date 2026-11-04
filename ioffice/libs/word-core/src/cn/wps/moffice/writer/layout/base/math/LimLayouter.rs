#![allow(dead_code, non_snake_case)]
// Java: cn.wps.moffice.writer.layout.base.math.LimLayouter
use crate::cn::wps::moffice::writer::cache::TypoDrawing::TypoDrawing;
use crate::cn::wps::moffice::writer::cache::TypoObjBase::TypoObjBase;
use crate::cn::wps::moffice::writer::core::Document::Document;
use crate::cn::wps::moffice::writer::data::MathTool::MathTool;
use crate::cn::wps::moffice::writer::data::PLCMathFunction::MathStartNode;
use crate::cn::wps::moffice::writer::layout::base::env::DocumentEnv::DocumentEnv;
use crate::cn::wps::moffice::writer::layout::base::env::MainState::MainState;
use crate::cn::wps::moffice::writer::layout::base::env::ViewEnv::ViewEnv;
use crate::cn::wps::moffice::writer::layout::base::math::MathLayouterBase::MathLayouterBase;

pub const LIMSCALSE: f32 = 0.7;

// java public class LimLayouter extends MathLayouterBase
pub struct LimLayouter {
    pub base: MathLayouterBase,
    pub element: Option<TypoDrawing>,
    pub lim: Option<TypoDrawing>,
}

impl LimLayouter {
    pub fn new(main: *mut MainState, doc: *mut DocumentEnv, view: *mut ViewEnv) -> Self {
        Self { base: MathLayouterBase::new(main, doc, view), element: None, lim: None }
    }

    // java protected boolean init(...)
    pub fn init(&mut self, document: *mut dyn Document, math_node: &MathStartNode, is_in_math_para: bool) -> bool {
        let doc = unsafe { &*document };
        self.base.init(doc, math_node, is_in_math_para);
        let sep_cp = MathTool::getMathSepaCP(doc, self.base.start_cp);
        self.element = self.base.layout_range(document, self.base.start_cp + 1, sep_cp, true);
        self.lim = self.base.layout_range(document, sep_cp + 1, self.base.end_cp, true);
        if let Some(lim) = self.lim.as_mut() {
            self.base.scaleMath(lim, LIMSCALSE);
        }
        true
    }

    pub fn distance(&self) -> i32 {
        (self.base.font_size as f32 * 0.1) as i32
    }

    pub fn stack(&mut self, lim_first: bool) -> TypoDrawing {
        let mut drawing = self.base.obtainTypoDrawing();
        let e_ptr = self.element.as_ref().map(|d| d.getPointer()).unwrap_or(0);
        let lim_ptr = self.lim.as_ref().map(|d| d.getPointer()).unwrap_or(0);
        if lim_first {
            if lim_ptr != 0 { drawing.addChildDrawing(lim_ptr); }
            if e_ptr != 0 { drawing.addChildDrawing(e_ptr); }
        } else {
            if e_ptr != 0 { drawing.addChildDrawing(e_ptr); }
            if lim_ptr != 0 { drawing.addChildDrawing(lim_ptr); }
        }
        let dist = self.distance();
        MathLayouterBase::adjustDrawingVert(&mut drawing, self.base.snapshot(), dist, 2);
        if e_ptr != 0 {
            let snap = self.base.snapshot();
            let top = TypoObjBase::getTop__int_typosnapshot(e_ptr, snap);
            let align = TypoDrawing::getAlignLine__int_typosnapshot(e_ptr, snap);
            let align_type = TypoDrawing::getAlignType__int_typosnapshot(e_ptr, snap);
            drawing.setAlignLine(top + align);
            drawing.setAlignType(align_type);
        }
        drawing
    }

    pub fn recycle_parts(&mut self) {
        if let Some(mut drawing) = self.element.take() {
            self.base.recycle_drawing(&mut drawing);
        }
        if let Some(mut drawing) = self.lim.take() {
            self.base.recycle_drawing(&mut drawing);
        }
    }
}
