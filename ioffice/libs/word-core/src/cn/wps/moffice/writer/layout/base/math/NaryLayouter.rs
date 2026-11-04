#![allow(dead_code, non_snake_case)]
// Java: cn.wps.moffice.writer.layout.base.math.NaryLayouter
use crate::cn::wps::moffice::writer::cache::TypoDrawing::TypoDrawing;
use crate::cn::wps::moffice::writer::cache::TypoObjBase::TypoObjBase;
use crate::cn::wps::moffice::writer::core::Document::Document;
use crate::cn::wps::moffice::writer::data::math::NaryProp::NaryProp;
use crate::cn::wps::moffice::writer::data::PLCMathFunction::MathStartNode;
use crate::cn::wps::moffice::writer::data::value::MathProperty::MathProperty;
use crate::cn::wps::moffice::writer::layout::base::env::DocumentEnv::DocumentEnv;
use crate::cn::wps::moffice::writer::layout::base::env::MainState::MainState;
use crate::cn::wps::moffice::writer::layout::base::env::ViewEnv::ViewEnv;
use crate::cn::wps::moffice::writer::layout::base::math::MathLayouterBase::MathLayouterBase;
use crate::cn::wps::moffice::writer::layout::base::math::SSubSupLayouter::SSubSupLayouter;

pub const NARYSCALSE: f32 = 2.0;

// java public class NaryLayouter extends SSubSupLayouter
pub struct NaryLayouter {
    base: SSubSupLayouter,
}

impl NaryLayouter {
    pub fn new(main: *mut MainState, doc: *mut DocumentEnv, view: *mut ViewEnv) -> Self {
        Self { base: SSubSupLayouter::new(main, doc, view) }
    }

    // java public TypoDrawing layout(...)
    pub fn layout(&mut self, document: *mut dyn Document, math_node: &MathStartNode, is_in_math_para: bool) -> Option<TypoDrawing> {
        let doc = unsafe { &*document };
        self.base.base.init(document, math_node, is_in_math_para);
        let npr = math_node.getPropertySet().map(NaryProp::from_property).unwrap_or_default();
        self.base.base.hide_sub = npr.subHide;
        self.base.base.hide_sup = npr.supHide;
        let mut drawing = self.base.base.base.obtainTypoDrawing();
        let start = self.base.base.base.start_cp;
        self.base.base.base.nary = true;
        let mut nary = self.base.base.base.getTypoDrawing(document, start, start + 1, math_node.getPropertySet())?;
        self.base.base.base.removeStrikeThroughs(&nary);
        if is_in_math_para {
            self.base.base.base.scaleMath(&mut nary, NARYSCALSE);
        }
        let mut element = if let Some(main) = self.base.base.main_drawing.take() {
            main
        } else {
            let size = self.base.base.base.getFontSize(doc, start);
            self.base.base.base.getEmptyDrawing(doc, size)
        };
        let e_height = TypoDrawing::layoutHeight__int_typosnapshot(element.getPointer(), self.base.base.base.snapshot());
        let nary_height = TypoDrawing::layoutHeight__int_typosnapshot(nary.getPointer(), self.base.base.base.snapshot());
        if npr.grow && e_height > nary_height {
            let nary_zoom = e_height as f32 / nary_height as f32;
            self.base.base.base.scaleMath(&mut nary, nary_zoom);
            let align = TypoDrawing::getAlignLine__int_typosnapshot(element.getPointer(), self.base.base.base.snapshot());
            nary.setAlignLine(align);
        }
        let nary_height = TypoDrawing::layoutHeight__int_typosnapshot(nary.getPointer(), self.base.base.base.snapshot());
        self.base.base.base.calcBaseline(&mut nary, doc, nary_height / 2, start);
        let lim_loc = self.lim_loc(doc, &npr, is_in_math_para);
        let font_size = self.base.base.base.font_size;
        let nary_ptr = nary.getPointer();
        let sub_ptr = self.base.base.sub.as_ref().map(|d| d.getPointer()).unwrap_or(0);
        let sup_ptr = self.base.base.sup.as_ref().map(|d| d.getPointer()).unwrap_or(0);
        let mut stacked = if lim_loc == 1 {
            self.layout_und_ovr(nary_ptr, sub_ptr, sup_ptr, font_size, doc)
        } else {
            self.base.base.layout_s_sub_sup(nary_ptr, sub_ptr, sup_ptr, font_size, doc)
        };
        self.base.base.base.recycle_drawing(&mut nary);
        if let Some(mut sub) = self.base.base.sub.take() {
            self.base.base.base.recycle_drawing(&mut sub);
        }
        if let Some(mut sup) = self.base.base.sup.take() {
            self.base.base.base.recycle_drawing(&mut sup);
        }
        drawing.addChildDrawing(stacked.getPointer());
        self.base.base.base.recycle_drawing(&mut stacked);
        drawing.addChildDrawing(element.getPointer());
        self.base.base.base.removeStrikeThroughs(&element);
        self.base.base.base.recycle_drawing(&mut element);
        self.base.base.base.adjustDrawingHori(&mut drawing, 0, 2);
        Some(drawing)
    }

    // java private int getNaryLimLoc(...)
    fn lim_loc(&self, document: &dyn Document, npr: &NaryProp, is_in_math_para: bool) -> i32 {
        if !is_in_math_para {
            return 0;
        }
        if let Some(loc) = npr.limLoc {
            return loc;
        }
        let integral = npr.chr == char::from_u32(8747).unwrap_or('\u{222b}');
        if let Some(text) = document.getTextDocument() {
            let property = text.getProperty();
            if let Some(value) = property.getValue(642) {
                if let Some(math) = value.as_ref().as_any().downcast_ref::<MathProperty>() {
                    if let Some(set) = math.getMathPropertySet() {
                        let key = if integral { 612 } else { 613 };
                        if set.getValue(key).is_some() {
                            return set.getInt(key);
                        }
                    }
                }
            }
        }
        if integral { 0 } else { 1 }
    }

    // java private TypoDrawing layoutUndOvr(...)
    fn layout_und_ovr(&mut self, s: i32, sub: i32, sup: i32, font_size: i32, document: &dyn Document) -> TypoDrawing {
        let mut drawing = self.base.base.base.obtainTypoDrawing();
        if sup != 0 {
            drawing.addChildDrawing(sup);
        }
        if s != 0 {
            drawing.addChildDrawing(s);
        }
        if sub != 0 {
            drawing.addChildDrawing(sub);
        }
        MathLayouterBase::adjustDrawingVert(&mut drawing, self.base.base.base.snapshot(), (font_size as f32 * 0.1) as i32, 2);
        if s != 0 {
            let snap = self.base.base.base.snapshot();
            let top = TypoObjBase::getTop__int_typosnapshot(s, snap);
            let align = TypoDrawing::getAlignLine__int_typosnapshot(s, snap);
            let align_type = TypoDrawing::getAlignType__int_typosnapshot(s, snap);
            drawing.setAlignLine(top + align);
            drawing.setAlignType(align_type);
        } else {
            let height = TypoDrawing::layoutHeight__int_typosnapshot(drawing.getPointer(), self.base.base.base.snapshot());
            self.base.base.base.calcBaseline(&mut drawing, document, height / 2, self.base.base.base.start_cp);
        }
        drawing
    }
}
