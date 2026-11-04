// Java: import android.text.TextPaint
use crate::android::text::TextPaint;

// Java: public class StyleSpan extends MetricAffectingSpan
pub struct StyleSpan {
    // Java field: int mStyle
    pub(crate) mStyle: i32,
}

// Java: public class StyleSpan extends MetricAffectingSpan
impl StyleSpan {
    // Java: StyleSpan(int style)
    pub fn StyleSpan(&self, int style) {
      // Java: super();
      // TODO_RUST_CODE
      // Java: this.mStyle = style;
      // TODO_RUST_CODE
    }

    // Java: int getStyle()
    pub fn getStyle(&self) -> i32 {
      // Java: return this.mStyle;
      // TODO_RUST_CODE
    }

    // Java: void updateDrawState(TextPaint ds)
    pub fn updateDrawState(&self, TextPaint ds) {
      // Java: apply(ds, this.mStyle);
      // TODO_RUST_CODE
    }

    // Java: void updateMeasureState(TextPaint paint)
    pub fn updateMeasureState(&self, TextPaint paint) {
      // Java: apply(paint, this.mStyle);
      // TODO_RUST_CODE
    }

    // Java: static void apply(TextPaint paint, int style)
    pub fn apply(&self, TextPaint paint, int style) {
    // TODO_RUST_CODE
    }

}
