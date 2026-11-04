// Java: import android.text.TextPaint
use super::CharacterStyle::{TextPaint, CharacterStyleTrait};
use super::UpdateLayout::UpdateLayout;

// Java: public class MetricAffectingSpan extends CharacterStyle implements UpdateLayout
pub trait MetricAffectingSpanTrait: CharacterStyleTrait {
    // Java: abstract void updateMeasureState(TextPaint var1)
    fn updateMeasureState(&self, var1: &mut TextPaint);
}

// Java: public class MetricAffectingSpan extends CharacterStyle implements UpdateLayout
pub struct MetricAffectingSpan {
}

impl MetricAffectingSpan {
    // Java: MetricAffectingSpan()
    pub fn new() -> Self {
      // Java: super();
      MetricAffectingSpan {}
    }

    // Java: abstract void updateMeasureState(TextPaint var1)

    // Java: MetricAffectingSpan getUnderlying()
    pub fn getUnderlying(&self) -> &MetricAffectingSpan {
      // Java: return this;
      self
    }
}

// Java: public class Passthrough
pub struct Passthrough {
    // Java field: MetricAffectingSpan mStyle
    pub(crate) mStyle: Box<dyn MetricAffectingSpanTrait>,
}

// Java: public class Passthrough
impl Passthrough {
    // Java: void updateDrawState(TextPaint tp)
    pub fn updateDrawState(&self, tp: &mut TextPaint) {
         // Java: this.mStyle.updateDrawState(tp);
         self.mStyle.updateDrawState(tp);
    }

    // Java: void updateMeasureState(TextPaint tp)
    pub fn updateMeasureState(&self, tp: &mut TextPaint) {
         // Java: this.mStyle.updateMeasureState(tp);
         self.mStyle.updateMeasureState(tp);
    }

    // Java: MetricAffectingSpan getUnderlying()
    pub fn getUnderlying(&self) -> &dyn CharacterStyleTrait {
         // Java: return this.mStyle.getUnderlying();
         self.mStyle.getUnderlying()
    }

}

impl UpdateLayout for MetricAffectingSpan {
}
