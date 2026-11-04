// Java: import android.graphics.Canvas
// Java: import android.graphics.Paint
// Java: import android.text.TextPaint
use super::CharacterStyle::TextPaint;

pub struct Paint;
pub struct Canvas;

// Java: public class ReplacementSpan extends MetricAffectingSpan
pub struct ReplacementSpan {
}

// Java: public class ReplacementSpan extends MetricAffectingSpan
impl ReplacementSpan {
    // Java: ReplacementSpan()
    pub fn new() -> Self {
      // Java: super();
      ReplacementSpan {}
    }

    // Java: abstract float getSize(Paint var1, CharSequence var2, int var3, int var4, Paint.FontMetrics var5)

    // Java: abstract void draw(Canvas var1, CharSequence var2, int var3, int var4, float var5, float var6, float var7, float var8, Paint var9)

    // Java: void updateMeasureState(TextPaint p)
    pub fn updateMeasureState(&self, _p: &mut TextPaint) {
    }

    // Java: void updateDrawState(TextPaint ds)
    pub fn updateDrawState(&self, _ds: &mut TextPaint) {
    }

}
