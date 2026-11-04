#![allow(dead_code, non_snake_case)]
// Java: cn.wps.moffice.writer.layout.base.math.DelimiterLayouter
use stubs::java::lang::Object::Object;
use util::moffice::property::PropertySet::PropertySet;
use crate::cn::wps::moffice::writer::cache::TypoDrawing::TypoDrawing;
use crate::cn::wps::moffice::writer::cache::TypoObjBase::TypoObjBase;
use crate::cn::wps::moffice::writer::core::Document::Document;
use crate::cn::wps::moffice::writer::data::math::DelimiterProp::DelimiterProp;
use crate::cn::wps::moffice::writer::data::MathTool::MathTool;
use crate::cn::wps::moffice::writer::data::PLCMathFunction::MathStartNode;
use crate::cn::wps::moffice::writer::layout::base::env::DocumentEnv::DocumentEnv;
use crate::cn::wps::moffice::writer::layout::base::env::MainState::MainState;
use crate::cn::wps::moffice::writer::layout::base::env::ViewEnv::ViewEnv;
use crate::cn::wps::moffice::writer::layout::base::math::MathLayouterBase::MathLayouterBase;

// java public class DelimiterLayouter extends MathLayouterBase
pub struct DelimiterLayouter {
    base: MathLayouterBase,
}

impl DelimiterLayouter {
    // java public DelimiterLayouter(MainState mainState, DocumentEnv documentEnv, ViewEnv viewEnv)
    pub fn new(main: *mut MainState, doc: *mut DocumentEnv, view: *mut ViewEnv) -> Self {
        Self { base: MathLayouterBase::new(main, doc, view) }
    }

    // java public TypoDrawing layout(Document document, PLCMathFunction.MathStartNode mathNode, boolean isInMathPara)
    pub fn layout(&mut self, document: *mut dyn Document, math_node: &MathStartNode, is_in_math_para: bool) -> Option<TypoDrawing> {
        self.base.init(unsafe { &*document }, math_node, is_in_math_para);
        let mut drawing = self.base.obtainTypoDrawing();
        let property = math_node.getPropertySet();
        if let Some((prop, dpr)) = property.and_then(|prop| delimiter_prop(prop).map(|dpr| (prop, dpr))) {
            let mut parts = Vec::new();
            let mut last_cp = self.base.start_cp;
            let mut cur_cp = MathTool::getMathSepaCP(unsafe { &*document }, last_cp);
            let mut max_ascent = 0;
            let mut max_descent = 0;
            while cur_cp <= self.base.end_cp {
                if let Some(part) = self.base.layout_range(document, last_cp + 1, cur_cp, true) {
                    self.base.removeStrikeThroughs(&part);
                    let ascent = TypoDrawing::getAlignLine__int_typosnapshot(part.getPointer(), self.base.snapshot());
                    let height = TypoDrawing::height__int(part.getPointer(), self.base.snapshot());
                    max_ascent = max_ascent.max(ascent);
                    max_descent = max_descent.max(height - ascent);
                    parts.push(part);
                }
                last_cp = cur_cp;
                if cur_cp >= self.base.end_cp {
                    break;
                }
                cur_cp = MathTool::getMathSepaCP(unsafe { &*document }, cur_cp);
            }
            let d_height = max_ascent + max_descent;
            let mut seps = Vec::new();
            if dpr.begChr != '\0' {
                if let Some(sep) = self.sep_drawing(document, dpr.begChr, prop) {
                    drawing.addChildDrawing(sep.getPointer());
                    seps.push(sep);
                }
            }
            if dpr.sepChr != '\0' {
                let size = parts.len();
                for (i, mut part) in parts.into_iter().enumerate() {
                    drawing.addChildDrawing(part.getPointer());
                    self.base.recycle_drawing(&mut part);
                    if i + 1 < size {
                        if let Some(sep) = self.sep_drawing(document, dpr.sepChr, prop) {
                            drawing.addChildDrawing(sep.getPointer());
                            seps.push(sep);
                        }
                    }
                }
            }
            if dpr.endChr != '\0' {
                if let Some(sep) = self.sep_drawing(document, dpr.endChr, prop) {
                    drawing.addChildDrawing(sep.getPointer());
                    seps.push(sep);
                }
            }
            self.base.adjustDrawingHori(&mut drawing, 0, 2);
            let height = TypoDrawing::layoutHeight__int_typosnapshot(drawing.getPointer(), self.base.snapshot());
            if dpr.grow {
                for sep in &mut seps {
                    self.set_sep_zoom(sep, height);
                    self.base.recycle_drawing(sep);
                }
            }
        } else if let Some(mut child) = self.base.layout_range(document, self.base.start_cp + 1, self.base.end_cp, true) {
            drawing.addChildDrawing(child.getPointer());
            self.base.recycle_drawing(&mut child);
            self.base.adjustDrawingHori(&mut drawing, 0, 2);
        }
        Some(drawing)
    }

    fn sep_drawing(&mut self, document: *mut dyn Document, sep_chr: char, property: &PropertySet) -> Option<TypoDrawing> {
        let old_ignore = self.base.ignore_leading;
        let old_math = self.base.math_content;
        self.base.ignore_leading = true;
        self.base.math_content = false;
        self.base.set_sep_char(sep_chr, true);
        let start = self.base.start_cp;
        let drawing = self.base.getTypoDrawing(document, start, start + 1, Some(property));
        self.base.set_sep_char('\0', false);
        self.base.math_content = old_math;
        self.base.ignore_leading = old_ignore;
        if let Some(ref drawing) = drawing {
            self.base.removeStrikeThroughs(drawing);
        }
        drawing
    }

    fn set_sep_zoom(&mut self, drawing: &mut TypoDrawing, height: i32) {
        let d_height = drawing.height();
        if d_height > 0 {
            let zoom = height as f32 / drawing.height() as f32;
            if zoom > 1.0 {
                self.base.setMathScale(drawing, zoom, 2);
                let left = drawing.getLeft();
                let pointer = drawing.getPointer();
                TypoObjBase::offsetTo__int_int_int_typosnapshot(left, 0, pointer, self.base.snapshot());
            }
        }
    }
}

fn delimiter_prop(property: &PropertySet) -> Option<DelimiterProp> {
    let value = property.getValue(737)?;
    value.as_ref().as_any().downcast_ref::<DelimiterProp>().cloned()
}
