#![allow(dead_code, unused_variables, non_snake_case)]

// Java: import android.graphics.Bitmap
struct Bitmap;
// Java: import android.graphics.BitmapShader
struct BitmapShader;
// Java: import android.graphics.EmbossMaskFilter
struct EmbossMaskFilter;
// Java: import android.graphics.Matrix
struct Matrix;
// Java: import android.graphics.Paint.Join
struct Join;
// Java: import android.graphics.Paint.Style
struct Style;
// Java: import android.graphics.Shader.TileMode
struct TileMode;
// Java: import android.text.TextPaint
struct TextPaint;
// Java: import cn.wps.text.layout.typo.layout.style.CharacterStyle
struct CharacterStyle;

// Java field: int INNERSHDW_BMP_SCALE
pub(crate) const INNERSHDW_BMP_SCALE: i32 = 4;

// Java: public class WpsInnerShdwSpan extends CharacterStyle
pub struct WpsInnerShdwSpan {
    // Java field: Bitmap mBmp
    mBmp: Option<i32>,
    // Java field: int mDScale
    mDScale: i32,
    // Java field: float mOutlnWid
    mOutlnWid: f32,
    // Java field: int mOutlnClr
    mOutlnClr: i32,
    // Java field: boolean mHasFill
    mHasFill: bool,
    // Java field: float mAlpha
    mAlpha: f32,
}

// Java: public class WpsInnerShdwSpan extends CharacterStyle
impl WpsInnerShdwSpan {
    // Java: WpsInnerShdwSpan(Bitmap bmp, int recursionIdx, float outlnWid, float alpha)
    pub fn new(&self, _bmp: i32, _recursionIdx: i32, _outlnWid: f32, _alpha: f32) {
      // Java: super();

      // Java: this.mBmp = bmp;

      // Java: this.mDScale = recursionIdx;

      // Java: this.mOutlnWid = outlnWid;

      // Java: this.mAlpha = alpha;

    }

    // Java: WpsInnerShdwSpan(float outlnWid, int outlnClr, boolean hasFill)
    pub fn new_2(&self, _outlnWid: f32, _outlnClr: i32, _hasFill: bool) {
      // Java: super();

      // Java: this.mOutlnWid = outlnWid;

      // Java: this.mOutlnClr = outlnClr;

      // Java: this.mHasFill = hasFill;

    }

    // Java: void updateDrawState(TextPaint textPaint)
    pub fn updateDrawState(&self, _textPaint: i32) {
      // Java: if (this.mBmp != null)

      {
         // Java: this.applyInnerShdw(textPaint);

      }
      // Java: else

      {
         // Java: this.applySimilarEffect(textPaint);

      }
    }

    // Java: void applyInnerShdw(TextPaint paint)
    pub fn applyInnerShdw(&self, _paint: i32) {
      // Java: paint.setAlpha((int)(255.0F * this.mAlpha));

      // Java: if (this.mOutlnWid > 0.0F)

      {
         // Java: paint.setStyle(Style.FILL_AND_STROKE);

         // Java: paint.setStrokeWidth(Math.max(this.mOutlnWid, 0.25F));

         // Java: paint.setStrokeJoin(Join.ROUND);

      }
      // Java: Matrix matrix = new Matrix();

      // Java: matrix.setScale(1.0F / (float)(4 - this.mDScale), 1.0F / (float)(4 - this.mDScale));

      // Java: BitmapShader bmpShader = new BitmapShader(this.mBmp, TileMode.CLAMP, TileMode.CLAMP);

      // Java: bmpShader.setLocalMatrix(matrix);

      // Java: paint.setShader(bmpShader);

    }

    // Java: void applySimilarEffect(TextPaint paint)
    pub fn applySimilarEffect(&self, _paint: i32) {
      // Java: if (this.mOutlnWid > 0.0F && !this.mHasFill)

      {
         // Java: paint.setColor(this.mOutlnClr);

         // Java: paint.setStyle(Style.STROKE);

         // Java: paint.setStrokeWidth(Math.max(this.mOutlnWid, 0.25F));

         // Java: paint.setStrokeJoin(Join.ROUND);

      }
      // Java: else

      {
         // Java: paint.setStyle(Style.FILL);

      }
      // Java: float[] direction = new float[]{10.0F, 10.0F, 8.0F};

      // Java: EmbossMaskFilter filter = new EmbossMaskFilter(direction, 0.4F, 1.0F, 2.0F);

      // Java: paint.setMaskFilter(filter);

    }

}
