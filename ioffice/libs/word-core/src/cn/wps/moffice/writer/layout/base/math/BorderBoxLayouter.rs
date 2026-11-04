#![allow(dead_code, non_snake_case)]
// Java: cn.wps.moffice.writer.layout.base.math.BorderBoxLayouter
use util::graphics::RectF::RectF;
use crate::cn::wps::moffice::writer::cache::TypoDrawing::TypoDrawing;
use crate::cn::wps::moffice::writer::cache::TypoObjBase::TypoObjBase;
use crate::cn::wps::moffice::writer::core::Document::Document;
use crate::cn::wps::moffice::writer::core::shape::ShapeMetrics::ShapeMetrics;
use crate::cn::wps::moffice::writer::data::PLCMathFunction::MathStartNode;
use crate::cn::wps::moffice::writer::layout::base::env::DocumentEnv::DocumentEnv;
use crate::cn::wps::moffice::writer::layout::base::env::MainState::MainState;
use crate::cn::wps::moffice::writer::layout::base::env::ViewEnv::ViewEnv;
use crate::cn::wps::moffice::writer::layout::base::math::MathLayouterBase::MathLayouterBase;
use crate::cn::wps::moffice::writer::layout::base::math::SepShapeFactory::SepShapeFactory;

// java public class BorderBoxLayouter extends MathLayouterBase
pub struct BorderBoxLayouter {
    base: MathLayouterBase,
    color: i32,
}

impl BorderBoxLayouter {
    // java public BorderBoxLayouter(MainState mainState, DocumentEnv documentEnv, ViewEnv viewEnv)
    pub fn new(main: *mut MainState, doc: *mut DocumentEnv, view: *mut ViewEnv) -> Self {
        Self { base: MathLayouterBase::new(main, doc, view), color: 0 }
    }

    // java public TypoDrawing layout(Document document, PLCMathFunction.MathStartNode mathNode, boolean isInMathPara)
    pub fn layout(&mut self, document: *mut dyn Document, math_node: &MathStartNode, is_in_math_para: bool) -> Option<TypoDrawing> {
        let doc = unsafe { &*document };
        self.base.init(doc, math_node, is_in_math_para);
        self.color = self.base.getColor(doc, self.base.start_cp);
        let inner = self.base.layout_range(document, self.base.start_cp + 1, self.base.end_cp, false)
            .unwrap_or_else(|| self.base.getEmptyDrawing(doc, (self.base.font_size as f32 * 0.5) as i32));
        Some(self.flow_box(inner))
    }

    fn flow_box(&mut self, mut inner: TypoDrawing) -> TypoDrawing {
        let mut drawing = self.base.obtainTypoDrawing();
        let dist = (self.base.font_size as f32 * 0.18) as i32;
        let rect = RectF::new_with(
            0.0,
            0.0,
            ShapeMetrics::transFromLayout__float((inner.width() + dist * 2) as f32),
            ShapeMetrics::transFromLayout__float((inner.height() + dist * 2) as f32),
        );
        drawing.set(0, 0, inner.width() + dist * 2, inner.height() + dist * 2);
        TypoObjBase::offsetTo__int_int_int_typosnapshot(dist, dist, inner.getPointer(), self.base.snapshot());
        drawing.addChildDrawing(inner.getPointer());
        let shape = SepShapeFactory::getShape(
            3,
            &rect,
            ShapeMetrics::transFromLayout__float(self.base.font_size as f32 * 0.05),
            self.color,
            ShapeMetrics::transFromLayout__float(self.base.font_size as f32),
        );
        drawing.setCloneShape(shape);
        self.base.recycle_drawing(&mut inner);
        drawing
    }
}
