use super::CharacterStyle::TextPaint;

// Java: public class SubscriptSpan extends MetricAffectingSpan
pub struct SubscriptSpan {
}

// Java: public class SubscriptSpan extends MetricAffectingSpan
impl SubscriptSpan {
    // Java: SubscriptSpan()
    pub fn new() -> Self {
      // Java: super();
      SubscriptSpan {}
    }

    // Java: void updateDrawState(TextPaint textPaint)
    pub fn updateDrawState(&self, textPaint: &mut TextPaint) {
      // Java: textPaint.baselineShift = -((int)(textPaint.ascent() / 2.0F));
    }

    // Java: void updateMeasureState(TextPaint textPaint)
    pub fn updateMeasureState(&self, textPaint: &mut TextPaint) {
      // Java: textPaint.baselineShift = -((int)(textPaint.ascent() / 2.0F));
    }

}
