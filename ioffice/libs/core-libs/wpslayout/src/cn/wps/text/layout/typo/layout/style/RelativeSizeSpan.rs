#![allow(non_snake_case)]
#![allow(unused_variables)]
#![allow(dead_code)]
#![allow(unused_imports)]

// Java: import android.text.TextPaint

// Java: public class RelativeSizeSpan extends MetricAffectingSpan
pub struct RelativeSizeSpan {
    // Java field: float mProportion
    pub(crate) mProportion: f32,
}

// Java: public class RelativeSizeSpan extends MetricAffectingSpan
impl RelativeSizeSpan {
    // Java: RelativeSizeSpan(float proportion)
    pub fn RelativeSizeSpan(proportion: f32) -> Self {
      // Java: super();
      // Java: this.mProportion = proportion;
      RelativeSizeSpan { mProportion: proportion }
    }

    // Java: float getSizeChange()
    pub fn getSizeChange(&self) -> f32 {
      // Java: return this.mProportion;
      self.mProportion
    }

    // Java: void updateDrawState(TextPaint ds)
    pub fn updateDrawState(&self, ds: i32) {
      // Java: ds.setTextSize((float)((int)(ds.getTextSize() * this.mProportion)));
    }

    // Java: void updateMeasureState(TextPaint ds)
    pub fn updateMeasureState(&self, ds: i32) {
      // Java: ds.setTextSize((float)((int)(ds.getTextSize() * this.mProportion)));
    }

}
