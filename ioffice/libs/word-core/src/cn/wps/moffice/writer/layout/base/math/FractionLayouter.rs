#![allow(dead_code, non_snake_case)]
// Java: cn.wps.moffice.writer.layout.base.math.FractionLayouter
use stubs::java::lang::Object::Object;
use util::graphics::RectF::RectF;
use crate::cn::wps::moffice::writer::cache::TypoDrawing::TypoDrawing;
use crate::cn::wps::moffice::writer::cache::TypoList::TypoList;
use crate::cn::wps::moffice::writer::cache::TypoObjBase::TypoObjBase;
use crate::cn::wps::moffice::writer::core::Document::Document;
use crate::cn::wps::moffice::writer::core::shape::ShapeMetrics::ShapeMetrics;
use crate::cn::wps::moffice::writer::data::math::FractionProp::FractionProp;
use crate::cn::wps::moffice::writer::data::MathTool::MathTool;
use crate::cn::wps::moffice::writer::data::PLCMathFunction::MathStartNode;
use crate::cn::wps::moffice::writer::layout::base::env::DocumentEnv::DocumentEnv;
use crate::cn::wps::moffice::writer::layout::base::env::MainState::MainState;
use crate::cn::wps::moffice::writer::layout::base::env::ViewEnv::ViewEnv;
use crate::cn::wps::moffice::writer::layout::base::math::MathLayouterBase::MathLayouterBase;
use crate::cn::wps::moffice::writer::layout::base::math::SepShapeFactory::SepShapeFactory;

// java public class FractionLayouter extends MathLayouterBase
pub struct FractionLayouter {
    base: MathLayouterBase,
    color: i32,
    numerator: Option<TypoDrawing>,
    denominator: Option<TypoDrawing>,
    sep: Option<TypoDrawing>,
}

impl FractionLayouter {
    // java public FractionLayouter(MainState mainState, DocumentEnv documentEnv, ViewEnv viewEnv)
    pub fn new(main: *mut MainState, doc: *mut DocumentEnv, view: *mut ViewEnv) -> Self {
        Self { base: MathLayouterBase::new(main, doc, view), color: 0, numerator: None, denominator: None, sep: None }
    }

    // java public TypoDrawing layout(Document document, PLCMathFunction.MathStartNode mathNode, boolean isInMathPara)
    pub fn layout(&mut self, document: *mut dyn Document, math_node: &MathStartNode, is_in_math_para: bool) -> Option<TypoDrawing> {
        let doc = unsafe { &*document };
        self.base.math_content = true;
        self.base.init(doc, math_node, is_in_math_para);
        let sep_cp = MathTool::getMathSepaCP(doc, self.base.start_cp);
        let fraction_type = fraction_type(math_node);
        self.color = self.base.getColor(doc, self.base.start_cp);
        let mut drawing = self.base.obtainTypoDrawing();
        self.numerator = self.base.layout_range(document, self.base.start_cp + 1, sep_cp, true);
        self.denominator = self.base.layout_range(document, sep_cp + 1, self.base.end_cp, true);
        self.sep = self.flow_math_feature(doc, fraction_type);
        self.adjust(&mut drawing, doc, fraction_type);
        if let Some(mut drawing) = self.numerator.take() {
            self.base.recycle_drawing(&mut drawing);
        }
        if let Some(mut drawing) = self.denominator.take() {
            self.base.recycle_drawing(&mut drawing);
        }
        if let Some(mut drawing) = self.sep.take() {
            self.base.recycle_drawing(&mut drawing);
        }
        Some(drawing)
    }

    fn adjust(&mut self, drawing: &mut TypoDrawing, document: &dyn Document, fraction_type: i32) {
        match fraction_type {
            1 => {
                self.add_child(drawing, true);
                MathLayouterBase::adjustDrawingVert(drawing, self.base.snapshot(), 0, 2);
                drawing.setAlignType(1);
                if let Some(sep) = self.sep.as_ref() {
                    let top = TypoObjBase::getTop__int_typosnapshot(sep.getPointer(), self.base.snapshot());
                    let align = TypoDrawing::getAlignLine__int_typosnapshot(sep.getPointer(), self.base.snapshot());
                    drawing.setAlignLine(top + align);
                }
                let dist = (self.base.font_size as f32 * 0.075) as i32;
                let children = TypoDrawing::getChildDrawings__int_typosnapshot(drawing.getPointer(), self.base.snapshot());
                let size = TypoList::size__int_typosnapshot(children, self.base.snapshot());
                for i in 0..size {
                    let child = TypoList::quickGet__int_int_typosnapshot(i, children, self.base.snapshot());
                    TypoObjBase::offsetX__int_int_typosnapshot(dist, child, self.base.snapshot());
                }
                drawing.setWidth(drawing.width() + dist * 2);
            }
            2 => {
                self.add_child(drawing, true);
                self.base.adjustDrawingHori(drawing, 0, 2);
                drawing.setAlignType(1);
                if let Some(sep) = self.sep.as_ref() {
                    let top = TypoObjBase::getTop__int_typosnapshot(sep.getPointer(), self.base.snapshot());
                    let align = TypoDrawing::getAlignLine__int_typosnapshot(sep.getPointer(), self.base.snapshot());
                    drawing.setAlignLine(top + align);
                }
            }
            3 => {
                self.add_child(drawing, false);
                let dist = (self.base.font_size as f32 * 0.5) as i32;
                MathLayouterBase::adjustDrawingVert(drawing, self.base.snapshot(), dist, 2);
                if let Some(numerator) = self.numerator.as_ref() {
                    let top = TypoObjBase::getTop__int_typosnapshot(numerator.getPointer(), self.base.snapshot());
                    let height = TypoDrawing::layoutHeight__int_typosnapshot(numerator.getPointer(), self.base.snapshot());
                    self.base.calcBaseline(drawing, document, top + height + dist / 2, self.base.start_cp);
                }
            }
            4 => {
                self.add_child(drawing, true);
                self.adjust_skew(drawing);
                if let Some(numerator) = self.numerator.as_ref() {
                    let top = TypoObjBase::getTop__int_typosnapshot(numerator.getPointer(), self.base.snapshot());
                    let height = TypoDrawing::layoutHeight__int_typosnapshot(numerator.getPointer(), self.base.snapshot());
                    self.base.calcBaseline(drawing, document, top + height, self.base.start_cp);
                }
            }
            _ => {}
        }
    }

    fn add_child(&mut self, drawing: &mut TypoDrawing, with_sep: bool) {
        if let Some(numerator) = self.numerator.as_ref() {
            drawing.addChildDrawing(numerator.getPointer());
        }
        if with_sep {
            if let Some(sep) = self.sep.as_ref() {
                drawing.addChildDrawing(sep.getPointer());
            }
        }
        if let Some(denominator) = self.denominator.as_ref() {
            drawing.addChildDrawing(denominator.getPointer());
        }
    }

    fn adjust_skew(&mut self, drawing: &mut TypoDrawing) {
        let Some(sep) = self.sep.as_ref() else { return };
        let Some(numerator) = self.numerator.as_ref() else { return };
        let sep_ptr = sep.getPointer();
        let num_ptr = numerator.getPointer();
        let offset_x = (sep.width() as f32 * 0.4) as i32;
        let sep_height = sep.height();
        let num_right = TypoObjBase::getRight__int_typosnapshot(num_ptr, self.base.snapshot());
        TypoObjBase::offsetTo__int_int_int_typosnapshot(num_right - offset_x, (sep_height as f32 * 0.2) as i32, sep_ptr, self.base.snapshot());
        let sep_right = TypoObjBase::getRight__int_typosnapshot(sep_ptr, self.base.snapshot());
        let num_bottom = TypoObjBase::getBottom__int_typosnapshot(num_ptr, self.base.snapshot());
        if let Some(denominator) = self.denominator.as_ref() {
            TypoObjBase::offsetTo__int_int_int_typosnapshot(sep_right - offset_x, num_bottom, denominator.getPointer(), self.base.snapshot());
        }
        MathLayouterBase::adjustDrawingSize(drawing, self.base.snapshot());
    }

    fn flow_math_feature(&mut self, document: &dyn Document, fraction_type: i32) -> Option<TypoDrawing> {
        if fraction_type == 3 {
            return None;
        }
        let mut drawing = self.base.obtainTypoDrawing();
        let rect = self.sep_rect(fraction_type);
        drawing.set(
            ShapeMetrics::transToLayout__float(rect.left),
            ShapeMetrics::transToLayout__float(rect.top),
            ShapeMetrics::transToLayout__float(rect.right),
            ShapeMetrics::transToLayout__float(rect.bottom),
        );
        let sep_kind = if fraction_type == 1 { 1 } else { 4 };
        let shape = SepShapeFactory::getShape(
            sep_kind,
            &rect,
            ShapeMetrics::transFromLayout__float(self.base.font_size as f32 * 0.05),
            self.color,
            ShapeMetrics::transFromLayout__float(self.base.font_size as f32),
        );
        drawing.setCloneShape(shape);
        let mid = (drawing.getTop() + drawing.getBottom()) / 2;
        self.base.calcBaseline(&mut drawing, document, mid, self.base.start_cp);
        Some(drawing)
    }

    fn sep_rect(&mut self, fraction_type: i32) -> RectF {
        if fraction_type == 1 {
            let num_width = self.numerator.as_ref().map(|d| d.width()).unwrap_or(0);
            let den_width = self.denominator.as_ref().map(|d| d.width()).unwrap_or(0);
            let width = num_width.max(den_width);
            RectF::new_with(0.0, 0.0, ShapeMetrics::transFromLayout__float(width as f32), ShapeMetrics::transFromLayout__float(self.base.font_size as f32 * 0.1))
        } else {
            let scale = if fraction_type == 2 { 0.5 } else { 0.6 };
            let num_height = self.numerator.as_ref().map(|d| d.height()).unwrap_or(0);
            let den_height = self.denominator.as_ref().map(|d| d.height()).unwrap_or(0);
            let height = ((num_height + den_height) as f32 * scale) as i32;
            let width = height / 2;
            RectF::new_with(0.0, 0.0, ShapeMetrics::transFromLayout__float(width as f32), ShapeMetrics::transFromLayout__float(height as f32))
        }
    }
}

fn fraction_type(math_node: &MathStartNode) -> i32 {
    let Some(property) = math_node.getPropertySet() else { return 1 };
    let Some(value) = property.getValue(737) else { return 1 };
    value.as_ref().as_any().downcast_ref::<FractionProp>().map(|prop| prop.fType).unwrap_or(1)
}
