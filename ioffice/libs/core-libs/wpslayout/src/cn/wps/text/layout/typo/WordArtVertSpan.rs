#![allow(dead_code, unused_variables, non_snake_case)]

// Java: import android.graphics.Canvas
struct Canvas;
// Java: import android.graphics.Paint
struct Paint;
// Java: import java.util.Arrays

struct FontMetricsInt;

// Java: public class WordArtVertSpan extends LooseReplacementSpan
pub struct WordArtVertSpan {
    // Java field: Paint.FontMetricsInt metrics
    metrics: Option<i32>,
    // Java field: int color
    color: i32,
    // Java field: boolean isSuperScript
    isSuperScript: bool,
    // Java field: boolean isSubScript
    isSubScript: bool,
    // Java field: boolean isLeftToRight
    isLeftToRight: bool,
}

// Java: public class WordArtVertSpan extends LooseReplacementSpan
impl WordArtVertSpan {
    // Java: WordArtVertSpan(int color, boolean isSuperScript, boolean isSubScript, boolean isLeftToRight)
    pub fn new(&self, _color: i32, _isSuperScript: bool, _isSubScript: bool, _isLeftToRight: bool) {
      // Java: super();

      // Java: this.color = color;

      // Java: this.isSuperScript = isSuperScript;

      // Java: this.isSubScript = isSubScript;

      // Java: this.isLeftToRight = isLeftToRight;

    }

    // Java: int getSize(Paint paint, CharSequence text, int start, int end, Paint.FontMetricsInt fontMetricsInt)
    pub fn getSize(&self, _paint: i32, _text: i32, _start: i32, _end: i32, _fontMetricsInt: i32) -> i32 {
      // Java: int singleWidth = this.setMetrics(paint);

      // Java: if (fontMetricsInt != null)

      {
         // Java: fontMetricsInt.top = this.metrics.top;

         // Java: fontMetricsInt.ascent = this.metrics.ascent;

         // Java: fontMetricsInt.descent = this.metrics.descent;

         // Java: fontMetricsInt.bottom = this.metrics.bottom;

         // Java: fontMetricsInt.leading = this.metrics.leading;

      }
      // Java: return singleWidth * (end - start);

      0
    }

    // Java: int[] getSizes(Paint paint, CharSequence text, int start, int end, Paint.FontMetricsInt fontMetricsInt)
    pub fn getSizes(&self, _paint: i32, _text: i32, _start: i32, _end: i32, _fontMetricsInt: i32) -> Vec<i32> {
      // Java: int singleWidth = this.setMetrics(paint);

      // Java: int[] widths = new int[end - start];

      // Java: Arrays.fill(widths, singleWidth);

      // Java: if (fontMetricsInt != null)

      {
         // Java: fontMetricsInt.top = this.metrics.top;

         // Java: fontMetricsInt.ascent = this.metrics.ascent;

         // Java: fontMetricsInt.descent = this.metrics.descent;

         // Java: fontMetricsInt.bottom = this.metrics.bottom;

         // Java: fontMetricsInt.leading = this.metrics.leading;

      }
      // Java: return widths;

      Vec::new()
    }

    // Java: void draw(Canvas canvas, CharSequence text, int start, int end, float x, int top, int y, int bottom, Paint paint)
    pub fn draw(&self, _canvas: i32, _text: i32, _start: i32, _end: i32, _x: f32, _top: i32, _y: i32, _bottom: i32, _paint: i32) {
      // Java: this.setPaint(paint);

      // Java: int singleWidth = this.setMetrics(paint);

      // Java: canvas.save();

      // Java: canvas.translate(x, this.isLeftToRight ? (float)top : (float)bottom);

      // Java: canvas.rotate(this.isLeftToRight ? 90.0F : -90.0F);

      // Java: int length = end - start;

      // Java: float[] charWidths = new float[length];

      // Java: paint.getTextWidths(text, start, end, charWidths);

      // Java: float maxWidth = (float)(bottom - top);

      // Java: float baseLinePos = this.isLeftToRight ? (float)(-this.metrics.bottom) : (float)(-this.metrics.top);

      // Java: for(int i = 0; i < length; ++i)

      {
         // Java: int index = start + i;

         // Java: float charWidth = charWidths[i];

         // Java: canvas.save();

         // Java: if (this.isSuperScript)

         {
            // Java: canvas.translate(maxWidth / 2.0F, baseLinePos);

         }
         // Java: else if (this.isSubScript)

         {
            // Java: canvas.translate(maxWidth / 2.0F - charWidth, baseLinePos);

         }
         // Java: else

         {
            // Java: canvas.translate((maxWidth - charWidth) / 2.0F, baseLinePos);

         }
         // Java: canvas.drawText(text, index, index + 1, 0.0F, 0.0F, paint);

         // Java: canvas.restore();

         // Java: canvas.translate(0.0F, this.isLeftToRight ? (float)(-singleWidth) : (float)singleWidth);

      }
      // Java: canvas.restore();

    }

    // Java: int setMetrics(Paint paint)
    pub fn setMetrics(&self, _paint: i32) -> i32 {
      // Java: paint.getFontMetricsInt(this.metrics);

      // Java: return (int)((float)(this.metrics.bottom - this.metrics.top) * 1.15F);

      0
    }

    // Java: void setPaint(Paint paint)
    pub fn setPaint(&self, _paint: i32) {
      // Java: paint.setColor(this.color);

    }

}
