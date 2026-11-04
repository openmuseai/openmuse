// Java: import android.graphics.Canvas
use thirdparty_kInk::com::hp::hpl::inkml::Canvas;
// Java: import android.graphics.Paint
use crate::android::graphics::Paint;
// Java: import android.text.TextPaint
use crate::android::text::TextPaint;

// Java: public class ReplacementSpan extends MetricAffectingSpan
pub struct ReplacementSpan {
}

// Java: public class ReplacementSpan extends MetricAffectingSpan
impl ReplacementSpan {
    // Java: ReplacementSpan()
    pub fn ReplacementSpan(&self) {
      // Java: super();
      // TODO_RUST_CODE
    }

    // Java: abstract float getSize(Paint var1, CharSequence var2, int var3, int var4, Paint.FontMetrics var5)
    pub fn getSize(&self, Paint var1, CharSequence var2, int var3, int var4, Paint.FontMetrics var5) -> f32 {
    // TODO_RUST_CODE
    }

    // Java: abstract void draw(Canvas var1, CharSequence var2, int var3, int var4, float var5, float var6, float var7, float var8, Paint var9)
    pub fn draw(&self, Canvas var1, CharSequence var2, int var3, int var4, float var5, float var6, float var7, float var8, Paint var9) {
    // TODO_RUST_CODE
    }

    // Java: void updateMeasureState(TextPaint p)
    pub fn updateMeasureState(&self, TextPaint p) {
    // TODO_RUST_CODE
    }

    // Java: void updateDrawState(TextPaint ds)
    pub fn updateDrawState(&self, TextPaint ds) {
    // TODO_RUST_CODE
    }

}
