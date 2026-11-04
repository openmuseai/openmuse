#![allow(dead_code, non_snake_case)]
// Java: cn.wps.moffice.writer.layout.base.math.RadLayouter
use util::graphics::RectF::RectF;
use crate::cn::wps::moffice::writer::cache::TypoDrawing::TypoDrawing;
use crate::cn::wps::moffice::writer::core::Document::Document;
use crate::cn::wps::moffice::writer::core::shape::ShapeMetrics::ShapeMetrics;
use crate::cn::wps::moffice::writer::data::MathTool::MathTool;
use crate::cn::wps::moffice::writer::data::PLCMathFunction::MathStartNode;
use crate::cn::wps::moffice::writer::layout::base::env::DocumentEnv::DocumentEnv;
use crate::cn::wps::moffice::writer::layout::base::env::MainState::MainState;
use crate::cn::wps::moffice::writer::layout::base::env::ViewEnv::ViewEnv;
use crate::cn::wps::moffice::writer::layout::base::math::MathLayouterBase::MathLayouterBase;
use crate::cn::wps::moffice::writer::layout::base::math::SepShapeFactory::SepShapeFactory;

// java public class RadLayouter extends MathLayouterBase
pub struct RadLayouter {
    base: MathLayouterBase,
    color: i32,
}

impl RadLayouter {
    pub fn new(main: *mut MainState, doc: *mut DocumentEnv, view: *mut ViewEnv) -> Self {
        Self { base: MathLayouterBase::new(main, doc, view), color: 0 }
    }

    // java public TypoDrawing layout(...)
    pub fn layout(&mut self, document: *mut dyn Document, math_node: &MathStartNode, is_in_math_para: bool) -> Option<TypoDrawing> {
        let doc = unsafe { &*document };
        self.base.init(doc, math_node, is_in_math_para);
        let sep_cp = MathTool::getMathSepaCP(doc, self.base.start_cp);
        self.color = self.base.getColor(doc, self.base.start_cp);
        let mut drawing = self.base.obtainTypoDrawing();
        let mut rad_out = self.base.layout_span(document, self.base.start_cp + 1, sep_cp);
        let mut rad_in = self.base.layout_span(document, sep_cp + 1, self.base.end_cp);
        if rad_in.is_none() {
            rad_in = Some(self.base.getEmptyDrawing(doc, self.base.font_size / 2));
        }
        let mut rad_in = rad_in.unwrap();
        drawing.addChildDrawing(rad_in.getPointer());
        if rad_out.is_none() {
            rad_out = Some(self.base.getEmptyDrawing(doc, self.base.font_size / 2));
        }
        let mut rad_out = rad_out.unwrap();
        rad_out.setZoom(0.5);
        drawing.addChildDrawing(rad_out.getPointer());
        if let Some(mut rad) = self.flow_rad(&mut rad_in, &mut rad_out) {
            drawing.addChildDrawing(rad.getPointer());
            self.base.recycle_drawing(&mut rad);
        }
        MathLayouterBase::adjustDrawingSize(&mut drawing, self.base.snapshot());
        let align = {
            let snap = self.base.snapshot();
            crate::cn::wps::moffice::writer::cache::TypoObjBase::TypoObjBase::getTop__int_typosnapshot(rad_in.getPointer(), snap)
                + self.base.content_base_line()
        };
        drawing.setAlignLine(align);
        drawing.setAlignType(1);
        self.base.recycle_drawing(&mut rad_in);
        self.base.recycle_drawing(&mut rad_out);
        Some(drawing)
    }

    fn flow_rad(&mut self, rad_in: &mut TypoDrawing, rad_out: &mut TypoDrawing) -> Option<TypoDrawing> {
        let mut drawing = self.base.obtainTypoDrawing();
        let font = self.base.font_size;
        let rad_left = (0.55 * font as f32) as i32;
        let rad_top = (0.2 * font as f32) as i32;
        let rad_bottom = (0.1 * font as f32) as i32;
        let rad_right = (0.05 * font as f32) as i32;
        let inout = (0.3 * font as f32) as i32;
        let out_right = -inout;
        let out_bottom = ((rad_in.height() + rad_top + rad_bottom) as f32 * 0.4) as i32;
        let zoom = TypoDrawing::getZoom__int_typosnapshot(rad_out.getPointer(), self.base.snapshot());
        let dx = (rad_out.width() as f32 * zoom) as i32 - out_right;
        let dy = (rad_out.height() as f32 * zoom) as i32 - out_bottom;
        if dy > 0 {
            rad_in.offsetTo(dx, dy + rad_top);
        } else {
            rad_in.offsetTo(dx, rad_top);
            rad_out.offsetTo(0, -dy);
        }
        let rect = RectF::new_with(
            0.0,
            ShapeMetrics::transFromLayout__float(font as f32 * 0.03),
            ShapeMetrics::transFromLayout__float((rad_in.width() + rad_left + rad_right) as f32),
            ShapeMetrics::transFromLayout__float((rad_in.height() + rad_top + rad_bottom) as f32),
        );
        let shape = SepShapeFactory::getShape(2, &rect, ShapeMetrics::transFromLayout__float(font as f32 * 0.06), self.color, ShapeMetrics::transFromLayout__float(font as f32));
        drawing.setCloneShape(shape);
        let left = rad_in.getLeft() - rad_left;
        let top = rad_in.getTop() - rad_top;
        let right = rad_in.getRight() + rad_right;
        let bottom = rad_in.getBottom() + rad_bottom;
        drawing.set(left, top, right, bottom);
        Some(drawing)
    }
}
