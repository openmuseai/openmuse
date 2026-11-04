// Java: import cn.wps.graphics.PointF
use util::graphics::PointF::PointF;

// Java: public interface InkMLConverter
pub trait InkMLConverter {
    // Java: abstract void adjustEventXY(float var1, float var2, PointF var3)
    fn adjustEventXY(&self, var1: f32, var2: f32, var3: &mut PointF);

    // Java: abstract float adjustStrokeWidth(float var1)
    fn adjustStrokeWidth(&self, var1: f32) -> f32;

    // Java: abstract void getInkScaleXY(PointF var1)
    fn getInkScaleXY(&self, var1: &mut PointF);

    // Java: abstract void adjustEraserXY(float var1, float var2, PointF var3)
    fn adjustEraserXY(&self, var1: f32, var2: f32, var3: &mut PointF);

    // Java: abstract float getEraserScale()
    fn getEraserScale(&self) -> f32;

    // Java: abstract void getEstimateScaleXY(PointF var1)
    fn getEstimateScaleXY(&self, var1: &mut PointF);

    // Java: abstract void getEstimateOffsetXY(PointF var1)
    fn getEstimateOffsetXY(&self, var1: &mut PointF);

}
