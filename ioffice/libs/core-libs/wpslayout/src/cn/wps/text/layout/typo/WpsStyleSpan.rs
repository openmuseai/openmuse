#![allow(dead_code, unused_variables, non_snake_case)]

// Java: import android.graphics.Paint
struct Paint;
// Java: import android.text.TextPaint
struct TextPaint;
// Java: import cn.wps.text.layout.typo.layout.style.StyleSpan
struct StyleSpan;

// Java: public class WpsStyleSpan extends StyleSpan
pub struct WpsStyleSpan {
    // Java field: int newStyle
    pub(crate) newStyle: i32,
}

// Java: public class WpsStyleSpan extends StyleSpan
impl WpsStyleSpan {
    // Java: WpsStyleSpan(int style)
    pub fn new(&self, _style: i32) {
      // Java: super(style);

      // Java: this.newStyle = style;

    }

    // Java: int getNewStyle()
    pub fn getNewStyle(&self) -> i32 {
      // Java: return this.newStyle;

      0
    }

    // Java: void updateDrawState(TextPaint paint)
    pub fn updateDrawState(&self, _paint: i32) {
      // Java: applyStyle(paint, this.newStyle);

    }

    // Java: void updateMeasureState(TextPaint paint)
    pub fn updateMeasureState(&self, _paint: i32) {
      // Java: applyStyle(paint, this.newStyle);

    }

    // Java: static void applyStyle(Paint paint, int style)
    pub fn applyStyle(&self, _paint: i32, _style: i32) {
      // Java: if ((style & 1) != 0)

      {
         // Java: paint.setFakeBoldText(true);

      }
      // Java: if ((style & 2) != 0)

      {
         // Java: paint.setTextSkewX(-0.25F);

      }
    }

}
