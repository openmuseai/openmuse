// Java: import android.graphics.Canvas
use thirdparty_kInk::com::hp::hpl::inkml::Canvas;
// Java: import android.graphics.Paint
use crate::android::graphics::Paint;
// Java: import cn.wps.text.layout.typo.layout.style.CharacterStyle
use crate::text::layout::typo::layout::style::CharacterStyle;
// Java: import cn.wps.text.layout.typo.layout.style.ReplacementSpan
use crate::text::layout::typo::layout::style::ReplacementSpan;

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
    pub fn WpsHideSpan(&self) {
      // Java: super();
      // TODO_RUST_CODE
    }

    // Java: void setWidths(float[] widths)
    pub fn setWidths(&self, float[] widths) {
      // Java: this.widths = widths;
      // TODO_RUST_CODE
    }

    // Java: float getSize(Paint paint, CharSequence text, int start, int end, Paint.FontMetrics fm)
    pub fn getSize(&self, Paint paint, CharSequence text, int start, int end, Paint.FontMetrics fm) -> f32 {
      // Java: if (this.widths != null && this.widths.length != 0)
        // TODO_RUST_CODE
      {
         // Java: float width = 0.0F;
         // TODO_RUST_CODE
         // Java: for(int i = start; i < end; ++i)
            // TODO_RUST_CODE
         {
            // Java: width += this.widths[i];
            // TODO_RUST_CODE
         }
         // Java: return width;
         // TODO_RUST_CODE
      }
      // Java: else
        // TODO_RUST_CODE
      {
         // Java: return 0.0F;
         // TODO_RUST_CODE
      }
    }

    // Java: void draw(Canvas canvas, CharSequence text, int start, int end, float x, float top, float y, float bottom, Paint paint)
    pub fn draw(&self, Canvas canvas, CharSequence text, int start, int end, float x, float top, float y, float bottom, Paint paint) {
    // TODO_RUST_CODE
    }

}

// Java: public class Factory
impl Factory {
    // Java: Factory()
    pub fn Factory(&self) {
         // Java: super();
         // TODO_RUST_CODE
    }

    // Java: static CharacterStyle create()
    pub fn create(&self) -> CharacterStyle {
         // Java: return new WpsHideSpan();
         // TODO_RUST_CODE
    }

}
