// Java: import android.text.TextPaint
use crate::android::text::TextPaint;

// Java: public class MetricAffectingSpan extends CharacterStyle implements UpdateLayout
pub struct MetricAffectingSpan {
}

// Java: public class Passthrough
pub struct Passthrough {
    // Java field: MetricAffectingSpan mStyle
    pub(crate) mStyle: Arc<Mutex<MetricAffectingSpan>>,
}

// Java: public class MetricAffectingSpan extends CharacterStyle implements UpdateLayout
impl MetricAffectingSpan {
    // Java: MetricAffectingSpan()
    pub fn MetricAffectingSpan(&self) {
      // Java: super();
      // TODO_RUST_CODE
    }

    // Java: abstract void updateMeasureState(TextPaint var1)
    pub fn updateMeasureState(&self, TextPaint var1) {
    // TODO_RUST_CODE
    }

    // Java: MetricAffectingSpan getUnderlying()
    pub fn getUnderlying(&self) -> MetricAffectingSpan {
      // Java: return this;
      // TODO_RUST_CODE
    }

}

// Java: public class Passthrough
impl Passthrough {
    // Java: void updateDrawState(TextPaint tp)
    pub fn updateDrawState(&self, TextPaint tp) {
         // Java: this.mStyle.updateDrawState(tp);
         // TODO_RUST_CODE
    }

    // Java: void updateMeasureState(TextPaint tp)
    pub fn updateMeasureState(&self, TextPaint tp) {
         // Java: this.mStyle.updateMeasureState(tp);
         // TODO_RUST_CODE
    }

    // Java: MetricAffectingSpan getUnderlying()
    pub fn getUnderlying(&self) -> MetricAffectingSpan {
         // Java: return this.mStyle.getUnderlying();
         // TODO_RUST_CODE
    }

}

impl UpdateLayout for MetricAffectingSpan {
}
