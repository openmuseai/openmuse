#![allow(dead_code, non_snake_case)]
// Java: cn.wps.moffice.writer.layout.base.math.SupSubBaseLayouter
use crate::cn::wps::moffice::writer::cache::TypoDrawing::TypoDrawing;
use crate::cn::wps::moffice::writer::cache::TypoObjBase::TypoObjBase;
use crate::cn::wps::moffice::writer::core::Document::Document;
use crate::cn::wps::moffice::writer::data::MathTool::MathTool;
use crate::cn::wps::moffice::writer::data::PLCMathFunction::MathStartNode;
use crate::cn::wps::moffice::writer::layout::base::env::DocumentEnv::DocumentEnv;
use crate::cn::wps::moffice::writer::layout::base::env::MainState::MainState;
use crate::cn::wps::moffice::writer::layout::base::env::ViewEnv::ViewEnv;
use crate::cn::wps::moffice::writer::layout::base::math::MathLayouterBase::{offset_child, offset_drawing_to, MathLayouterBase};

pub const SUBSUPSCALSE: f32 = 0.6;

// java public class SupSubBaseLayouter extends MathLayouterBase
pub struct SupSubBaseLayouter {
    pub base: MathLayouterBase,
    pub hide_sup: bool,
    pub hide_sub: bool,
    pub sup: Option<TypoDrawing>,
    pub sub: Option<TypoDrawing>,
    pub main_drawing: Option<TypoDrawing>,
}

impl SupSubBaseLayouter {
    pub fn new(main: *mut MainState, doc: *mut DocumentEnv, view: *mut ViewEnv) -> Self {
        Self {
            base: MathLayouterBase::new(main, doc, view),
            hide_sup: false,
            hide_sub: false,
            sup: None,
            sub: None,
            main_drawing: None,
        }
    }

    pub fn is_sub(&self, document: &dyn Document, cp: i32) -> bool {
        self.base.getChar(document, cp) == '_'
    }

    pub fn is_sup(&self, document: &dyn Document, cp: i32) -> bool {
        self.base.getChar(document, cp) == '^'
    }

    fn place(&mut self, document: *mut dyn Document, start: i32, end: i32) {
        let doc = unsafe { &*document };
        if self.is_sub(doc, start) {
            if !self.hide_sub {
                self.sub = self.base.layout_span(document, start + 1, end);
            }
        } else if self.is_sup(doc, start) {
            if !self.hide_sup {
                self.sup = self.base.layout_span(document, start + 1, end);
            }
        } else {
            self.base.math_content = true;
            self.main_drawing = self.base.layout_span(document, start + 1, end);
            self.base.math_content = false;
        }
    }

    // java protected boolean init(...)
    pub fn init(&mut self, document: *mut dyn Document, math_node: &MathStartNode, is_in_math_para: bool) -> bool {
        let doc = unsafe { &*document };
        self.base.init(doc, math_node, is_in_math_para);
        let mut sep_cp = MathTool::getMathSepaCP(doc, self.base.start_cp);
        if sep_cp < 0 {
            return false;
        }
        self.sup = None;
        self.sub = None;
        self.main_drawing = None;
        let mut cp = self.base.start_cp;
        while sep_cp < self.base.end_cp && sep_cp > 0 {
            self.place(document, cp, sep_cp);
            cp = sep_cp;
            sep_cp = MathTool::getMathSepaCP(doc, sep_cp);
        }
        if cp < self.base.end_cp {
            self.place(document, cp, self.base.end_cp);
        }
        if let Some(main) = self.main_drawing.as_mut() {
            self.base.removeStrikeThroughs(main);
        }
        let scale = self.base.getScale() * SUBSUPSCALSE;
        if let Some(sub) = self.sub.as_mut() {
            self.base.scaleMath(sub, scale);
        }
        if let Some(sup) = self.sup.as_mut() {
            self.base.scaleMath(sup, scale);
        }
        true
    }

    // java protected TypoDrawing layoutSSubSup(TypoDrawing s, TypoDrawing sub, TypoDrawing sup, int fontSize, Document document)
    pub fn layout_s_sub_sup(&mut self, s_ptr: i32, sub_ptr: i32, sup_ptr: i32, font_size: i32, document: &dyn Document) -> TypoDrawing {
        let start_cp = self.base.start_cp;
        let mut drawing = self.base.obtainTypoDrawing();
        let (s_right, s_height) = if s_ptr != 0 {
            drawing.addChildDrawing(s_ptr);
            let snap = self.base.snapshot();
            (TypoDrawing::layoutWidth__int_typosnapshot(s_ptr, snap), TypoDrawing::layoutHeight__int_typosnapshot(s_ptr, snap))
        } else {
            (0, 0)
        };
        let mut sup_top = 0;
        if sup_ptr != 0 {
            drawing.addChildDrawing(sup_ptr);
            let sup_h = TypoDrawing::layoutHeight__int_typosnapshot(sup_ptr, self.base.snapshot());
            sup_top = sup_offset(sup_h, s_height, font_size);
            offset_drawing_to(sup_ptr, s_right, sup_top, self.base.snapshot());
            if sup_top > 0 {
                sup_top = 0;
            }
        }
        if sub_ptr != 0 {
            drawing.addChildDrawing(sub_ptr);
            let sub_h = TypoDrawing::layoutHeight__int_typosnapshot(sub_ptr, self.base.snapshot());
            let sub_top = sub_offset(sub_h, s_height, font_size);
            offset_drawing_to(sub_ptr, s_right, sub_top, self.base.snapshot());
        }
        offset_child(&mut drawing, self.base.snapshot(), 0, -sup_top, true);
        if s_ptr != 0 {
            let snap = self.base.snapshot();
            let align_type = TypoDrawing::getAlignType__int_typosnapshot(s_ptr, snap);
            let top = TypoObjBase::getTop__int_typosnapshot(s_ptr, snap);
            let line = if align_type == 1 {
                TypoDrawing::getAlignLine__int_typosnapshot(s_ptr, snap)
            } else {
                TypoDrawing::layoutHeight__int_typosnapshot(s_ptr, snap) / 2
            };
            drawing.setAlignLine(top + line);
            drawing.setAlignType(align_type);
        } else {
            let height = TypoDrawing::layoutHeight__int_typosnapshot(drawing.getPointer(), self.base.snapshot());
            self.base.calcBaseline(&mut drawing, document, height / 2, start_cp);
        }
        drawing
    }

    pub fn recycle_parts(&mut self) {
        if let Some(mut drawing) = self.main_drawing.take() {
            self.base.recycle_drawing(&mut drawing);
        }
        if let Some(mut drawing) = self.sub.take() {
            self.base.recycle_drawing(&mut drawing);
        }
        if let Some(mut drawing) = self.sup.take() {
            self.base.recycle_drawing(&mut drawing);
        }
    }
}

pub(crate) fn sup_offset(sup_height: i32, main_height: i32, font_size: i32) -> i32 {
    let top = if sup_height > main_height {
        main_height as f32 * 0.7 - sup_height as f32
    } else {
        font_size as f32 * 0.2 - (sup_height / 2) as f32
    };
    top as i32
}

pub(crate) fn sub_offset(sub_height: i32, main_height: i32, font_size: i32) -> i32 {
    let top = if sub_height > main_height {
        (main_height / 2) as f32
    } else {
        main_height as f32 - font_size as f32 * 0.1 - (sub_height / 2) as f32
    };
    top as i32
}
