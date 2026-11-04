#![allow(dead_code, non_snake_case)]
// Java: cn.wps.moffice.writer.layout.base.math.BarLayouter
use stubs::java::lang::Object::Object;
use util::graphics::RectF::RectF;
use crate::cn::wps::moffice::writer::cache::TypoDrawing::TypoDrawing;
use crate::cn::wps::moffice::writer::cache::TypoObjBase::TypoObjBase;
use crate::cn::wps::moffice::writer::core::Document::Document;
use crate::cn::wps::moffice::writer::core::shape::ShapeMetrics::ShapeMetrics;
use crate::cn::wps::moffice::writer::data::math::BarProp::BarProp;
use crate::cn::wps::moffice::writer::data::PLCMathFunction::MathStartNode;
use crate::cn::wps::moffice::writer::layout::base::env::DocumentEnv::DocumentEnv;
use crate::cn::wps::moffice::writer::layout::base::env::MainState::MainState;
use crate::cn::wps::moffice::writer::layout::base::env::ViewEnv::ViewEnv;
use crate::cn::wps::moffice::writer::layout::base::math::MathLayouterBase::MathLayouterBase;
use crate::cn::wps::moffice::writer::layout::base::math::SepShapeFactory::SepShapeFactory;

// java public class BarLayouter extends MathLayouterBase
pub struct BarLayouter {
    base: MathLayouterBase,
}

impl BarLayouter {
    // java public BarLayouter(MainState mainState, DocumentEnv documentEnv, ViewEnv viewEnv)
    pub fn new(main: *mut MainState, doc: *mut DocumentEnv, view: *mut ViewEnv) -> Self {
        Self { base: MathLayouterBase::new(main, doc, view) }
    }

    // java public TypoDrawing layout(Document document, PLCMathFunction.MathStartNode mathNode, boolean isInMathPara)
    pub fn layout(&mut self, document: *mut dyn Document, math_node: &MathStartNode, is_in_math_para: bool) -> Option<TypoDrawing> {
        let doc = unsafe { &*document };
        self.base.init(doc, math_node, is_in_math_para);
        let pos = math_node.getPropertySet().and_then(bar_pos).unwrap_or(1);
        let top_bar = pos == 0;
        let mut drawing = self.base.obtainTypoDrawing();
        let element = self.base.layout_range(document, self.base.start_cp + 1, self.base.end_cp, true)?;
        let mut bar = self.base.obtainTypoDrawing();
        let bar_height = (self.base.font_size as f32 * 0.2) as i32;
        let rect = RectF::new_with(
            0.0,
            0.0,
            ShapeMetrics::transFromLayout__float(element.width() as f32),
            ShapeMetrics::transFromLayout__float(self.base.font_size as f32 * 0.2),
        );
        bar.set(0, 0, element.width(), bar_height);
        let shape = SepShapeFactory::getShape(
            5,
            &rect,
            ShapeMetrics::transFromLayout__float(self.base.font_size as f32 * 0.05),
            self.base.getColor(doc, self.base.start_cp),
            ShapeMetrics::transFromLayout__float(self.base.font_size as f32),
        );
        bar.setCloneShape(shape);
        if top_bar {
            drawing.addChildDrawing(bar.getPointer());
            drawing.addChildDrawing(element.getPointer());
        } else {
            drawing.addChildDrawing(element.getPointer());
            drawing.addChildDrawing(bar.getPointer());
        }
        let dist = if self.base.start_cp + 1 >= self.base.end_cp { bar_height } else { 0 };
        MathLayouterBase::adjustDrawingVert(&mut drawing, self.base.snapshot(), dist, 2);
        let align_type = TypoDrawing::getAlignType__int_typosnapshot(element.getPointer(), self.base.snapshot());
        let top = TypoObjBase::getTop__int_typosnapshot(element.getPointer(), self.base.snapshot());
        let align = TypoDrawing::getAlignLine__int_typosnapshot(element.getPointer(), self.base.snapshot());
        drawing.setAlignType(align_type);
        drawing.setAlignLine(top + align);
        let mut element = element;
        self.base.recycle_drawing(&mut bar);
        self.base.recycle_drawing(&mut element);
        Some(drawing)
    }
}

fn bar_pos(property: &util::moffice::property::PropertySet::PropertySet) -> Option<i32> {
    let value = property.getValue(737)?;
    Some(value.as_ref().as_any().downcast_ref::<BarProp>()?.pos)
}
