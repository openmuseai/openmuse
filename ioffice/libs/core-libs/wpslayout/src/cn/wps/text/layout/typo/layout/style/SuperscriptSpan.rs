use super::CharacterStyle::TextPaint;

// Java: public class SuperscriptSpan extends MetricAffectingSpan
pub struct SuperscriptSpan {
}

// Java: public class SuperscriptSpan extends MetricAffectingSpan
impl SuperscriptSpan {
    // Java: SuperscriptSpan()
    pub fn new() -> Self {
      // Java: super();
      SuperscriptSpan {}
    }

    // Java: void updateDrawState(TextPaint textPaint)
    pub fn updateDrawState(&self, textPaint: &mut TextPaint) {
      // Java: textPaint.baselineShift = (int)(textPaint.ascent() / 2.0F);
    }

    // Java: void updateMeasureState(TextPaint textPaint)
    pub fn updateMeasureState(&self, textPaint: &mut TextPaint) {
      // Java: textPaint.baselineShift = (int)(textPaint.ascent() / 2.0F);
    }

}
