use super::CharacterStyle::TextPaint;

// Java: public class ScaleXSpan extends MetricAffectingSpan
pub struct ScaleXSpan {
    // Java field: float mProportion
    pub(crate) mProportion: f32,
}

// Java: public class ScaleXSpan extends MetricAffectingSpan
impl ScaleXSpan {
    // Java: ScaleXSpan(float proportion)
    pub fn new(proportion: f32) -> Self {
      // Java: super();
      // Java: this.mProportion = proportion;
      ScaleXSpan { mProportion: proportion }
    }

    // Java: float getScaleX()
    pub fn getScaleX(&self) -> f32 {
      // Java: return this.mProportion;
      self.mProportion
    }

    // Java: void updateDrawState(TextPaint ds)
    pub fn updateDrawState(&self, ds: &mut TextPaint) {
      // Java: ds.setTextScaleX(ds.getTextScaleX() * this.mProportion);
    }

    // Java: void updateMeasureState(TextPaint ds)
    pub fn updateMeasureState(&self, ds: &mut TextPaint) {
      // Java: ds.setTextScaleX(ds.getTextScaleX() * this.mProportion);
    }

}
