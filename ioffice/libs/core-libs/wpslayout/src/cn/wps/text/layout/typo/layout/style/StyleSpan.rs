#![allow(dead_code, unused_variables, non_snake_case)]

// Java: import android.text.TextPaint
struct TextPaint;

// Java: public class StyleSpan extends MetricAffectingSpan
pub struct StyleSpan {
    // Java field: int mStyle
    pub(crate) mStyle: i32,
}

// Java: public class StyleSpan extends MetricAffectingSpan
impl StyleSpan {
    // Java: StyleSpan(int style)
    pub fn new(&self, _style: i32) {
      // Java: super();

      // Java: this.mStyle = style;

    }

    // Java: int getStyle()
    pub fn getStyle(&self) -> i32 {
      // Java: return this.mStyle;

      0
    }

    // Java: void updateDrawState(TextPaint ds)
    pub fn updateDrawState(&self, _ds: i32) {
      // Java: apply(ds, this.mStyle);

    }

    // Java: void updateMeasureState(TextPaint paint)
    pub fn updateMeasureState(&self, _paint: i32) {
      // Java: apply(paint, this.mStyle);

    }

    // Java: static void apply(TextPaint paint, int style)
    pub fn apply(&self, _paint: i32, _style: i32) {

    }

}
