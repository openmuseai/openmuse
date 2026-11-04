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

// Java: public class WpsOuterShdwSpan extends CharacterStyle
pub struct WpsOuterShdwSpan {
    // Java field: int mShadowColor
    mShadowColor: i32,
    // Java field: float mShadowRad
    mShadowRad: f32,
    // Java field: float mOutlnWid
    mOutlnWid: f32,
    // Java field: boolean mHasFill
    mHasFill: bool,
    // Java field: boolean mIsCompress
    mIsCompress: bool,
}

// Java: public class WpsOuterShdwSpan extends CharacterStyle
impl WpsOuterShdwSpan {
    // Java: WpsOuterShdwSpan(int color, float shadowRad, float outlnWid, boolean hasFill, boolean isCompress)
    pub fn new(&self, _color: i32, _shadowRad: f32, _outlnWid: f32, _hasFill: bool, _isCompress: bool) {
      // Java: super();

      // Java: this.mShadowColor = color;

      // Java: this.mShadowRad = shadowRad;

      // Java: this.mOutlnWid = outlnWid;

      // Java: this.mHasFill = hasFill;

      // Java: this.mIsCompress = isCompress;

    }

    // Java: void updateDrawState(TextPaint paint)
    pub fn updateDrawState(&self, _paint: i32) {
      // Java: this.applyShadow(paint);

    }

    // Java: void applyShadow(TextPaint paint)
    pub fn applyShadow(&self, _paint: i32) {
      // Java: paint.setColor(this.mShadowColor);

      // Java: if (this.mHasFill && this.mOutlnWid == 0.0F)

      {
         // Java: paint.setStyle(Style.FILL);

      }
      // Java: else if (this.mHasFill && this.mOutlnWid > 0.0F)

      {
         // Java: paint.setStyle(Style.FILL_AND_STROKE);

         // Java: float widPt = this.mOutlnWid > 0.0F && (double)this.mOutlnWid < (double)0.25F ? 0.25F : this.mOutlnWid;

         // Java: paint.setStrokeWidth(widPt);

      }
      // Java: else if (!this.mHasFill && this.mOutlnWid > 0.0F)

      {
         // Java: paint.setStyle(Style.STROKE);

         // Java: float widPt = this.mOutlnWid > 0.0F && (double)this.mOutlnWid < (double)0.25F ? 0.25F : this.mOutlnWid;

         // Java: paint.setStrokeWidth(widPt);

      }
      // Java: else if (!this.mHasFill && this.mOutlnWid == 0.0F)

      {
         // Java: paint.setColor(0);

         // Java: return;

      }
      // Java: if (this.mShadowRad > 0.0F)

      {
         // Java: float shdwRad = this.mShadowRad;

         // Java: if (!this.mIsCompress)

         {
            // Java: shdwRad /= 2.0F;

         }
         // Java: BlurMaskFilter blurFilter = new BlurMaskFilter(shdwRad, Blur.NORMAL);

         // Java: paint.setMaskFilter(blurFilter);

      }
    }

}
