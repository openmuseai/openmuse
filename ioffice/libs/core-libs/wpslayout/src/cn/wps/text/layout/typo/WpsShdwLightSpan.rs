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

// Java: public class WpsShdwLightSpan extends CharacterStyle
pub struct WpsShdwLightSpan {
    // Java field: float mOutlnWid
    mOutlnWid: f32,
    // Java field: float mShdwRad
    mShdwRad: f32,
    // Java field: boolean mHasFill
    mHasFill: bool,
    // Java field: float mFontsize
    mFontsize: f32,
}

// Java: public class WpsShdwLightSpan extends CharacterStyle
impl WpsShdwLightSpan {
    // Java: WpsShdwLightSpan(float outlnWid, float shdwRad, boolean hasFill, float fontsize)
    pub fn new(&self, _outlnWid: f32, _shdwRad: f32, _hasFill: bool, _fontsize: f32) {
      // Java: super();

      // Java: this.mOutlnWid = outlnWid;

      // Java: this.mShdwRad = shdwRad;

      // Java: this.mHasFill = hasFill;

      // Java: this.mFontsize = fontsize;

    }

    // Java: void updateDrawState(TextPaint paint)
    pub fn updateDrawState(&self, _paint: i32) {
      // Java: this.applyShdwLight(paint);

    }

    // Java: void applyShdwLight(TextPaint paint)
    pub fn applyShdwLight(&self, _paint: i32) {
      // Java: if (this.mHasFill && this.mOutlnWid > 0.0F)

      {
         // Java: paint.setStyle(Style.FILL_AND_STROKE);

         // Java: paint.setStrokeWidth(Math.max(this.mOutlnWid, 0.25F));

      }
      // Java: if (this.mShdwRad > 0.0F)

      {
         // Java: float shdwRad = this.mShdwRad / 4.0F * this.mFontsize / 60.0F;

         // Java: BlurMaskFilter blurFilter = new BlurMaskFilter(shdwRad, Blur.NORMAL);

         // Java: paint.setMaskFilter(blurFilter);

      }
    }

}
