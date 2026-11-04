#![allow(dead_code, unused_variables, non_snake_case)]

// Java: import android.graphics.BlurMaskFilter
struct BlurMaskFilter;
// Java: import android.graphics.BlurMaskFilter.Blur
struct Blur;
// Java: import android.graphics.Paint.Style
struct Style;
// Java: import android.text.TextPaint
struct TextPaint;
// Java: import cn.wps.text.layout.typo.layout.style.CharacterStyle
struct CharacterStyle;

// Java: public class WpsTextBlurSpan extends CharacterStyle
pub struct WpsTextBlurSpan {
    // Java field: float mBlurSize
    pub(crate) mBlurSize: f32,
    // Java field: int mBlurColor
    mBlurColor: i32,
}

// Java: public class WpsTextBlurSpan extends CharacterStyle
impl WpsTextBlurSpan {
    // Java: WpsTextBlurSpan(float blurSize, int blurClr)
    pub fn new(&self, _blurSize: f32, _blurClr: i32) {
      // Java: super();

      // Java: this.mBlurSize = blurSize / 2.0F;

      // Java: this.mBlurColor = blurClr;

    }

    // Java: void updateDrawState(TextPaint paint)
    pub fn updateDrawState(&self, _paint: i32) {
      // Java: this.applyTextBlur(paint);

    }

    // Java: void applyTextBlur(TextPaint paint)
    pub fn applyTextBlur(&self, _paint: i32) {
      // Java: if (this.mBlurSize > 0.0F)

      {
         // Java: paint.setStyle(Style.STROKE);

         // Java: paint.setStrokeWidth(this.mBlurSize);

         // Java: paint.setColor(this.mBlurColor);

         // Java: BlurMaskFilter blurFilter = new BlurMaskFilter(this.mBlurSize, Blur.NORMAL);

         // Java: paint.setMaskFilter(blurFilter);

      }
    }

    // Java: void applyTextBlur4(TextPaint paint)
    pub fn applyTextBlur4(&self, _paint: i32) {
      // Java: if (this.mBlurSize > 0.0F)

      {
         // Java: paint.setStyle(Style.FILL);

         // Java: paint.setStrokeWidth(this.mBlurSize);

         // Java: paint.setColor(this.mBlurColor | -16777216);

         // Java: BlurMaskFilter blurFilter = new BlurMaskFilter(this.mBlurSize, Blur.NORMAL);

         // Java: paint.setMaskFilter(blurFilter);

      }
    }

    // Java: float getActualBlurRad()
    pub fn getActualBlurRad(&self) -> f32 {
      // Java: return this.mBlurSize;

      0.0
    }

}
