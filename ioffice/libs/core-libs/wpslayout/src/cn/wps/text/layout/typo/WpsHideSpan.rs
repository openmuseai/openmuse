#![allow(dead_code, unused_variables, non_snake_case)]

// Java: import android.graphics.Canvas
struct Canvas;
// Java: import android.graphics.Paint
struct Paint;
// Java: import cn.wps.text.layout.typo.layout.style.CharacterStyle
struct CharacterStyle;
// Java: import cn.wps.text.layout.typo.layout.style.ReplacementSpan
struct ReplacementSpan;

// Java: public class WpsHideSpan extends ReplacementSpan
pub struct WpsHideSpan {
    // Java field: float[] widths
    widths: Option<Vec<f32>>,
}

// Java: public class Factory
pub struct Factory {
}

// Java: public class WpsHideSpan extends ReplacementSpan
impl WpsHideSpan {
    // Java: WpsHideSpan()
    pub fn new(&self) {
      // Java: super();

    }

    // Java: void setWidths(float[] widths)
    pub fn setWidths(&self, _widths: Vec<f32>) {
      // Java: this.widths = widths;

    }

    // Java: float getSize(Paint paint, CharSequence text, int start, int end, Paint.FontMetrics fm)
    pub fn getSize(&self, _paint: i32, _text: i32, _start: i32, _end: i32, _fm: i32) -> f32 {
      // Java: if (this.widths != null && this.widths.length != 0)

      {
         // Java: float width = 0.0F;

         // Java: for(int i = start; i < end; ++i)

         {
            // Java: width += this.widths[i];

         }
         // Java: return width;

      }
      // Java: else

      {
         // Java: return 0.0F;

      }
      0.0
    }

    // Java: void draw(Canvas canvas, CharSequence text, int start, int end, float x, float top, float y, float bottom, Paint paint)
    pub fn draw(&self, _canvas: i32, _text: i32, _start: i32, _end: i32, _x: f32, _top: f32, _y: f32, _bottom: f32, _paint: i32) {

    }

}

// Java: public class Factory
impl Factory {
    // Java: Factory()
    pub fn new(&self) {
         // Java: super();

    }

    // Java: static CharacterStyle create()
    pub fn create(&self) -> i32 {
         // Java: return new WpsHideSpan();

         0
    }

}
