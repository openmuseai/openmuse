#![allow(dead_code, unused_variables, non_snake_case)]

// Java: import android.graphics.BlurMaskFilter
struct BlurMaskFilter;
// Java: import android.graphics.ComposeShader
struct ComposeShader;
// Java: import android.graphics.LinearGradient
struct LinearGradient;
// Java: import android.graphics.Shader
struct Shader;
// Java: import android.graphics.BlurMaskFilter.Blur
struct Blur;
// Java: import android.graphics.PorterDuff.Mode
struct Mode;
// Java: import android.graphics.Shader.TileMode
struct TileMode;
// Java: import android.text.TextPaint
struct TextPaint;
// Java: import cn.wps.text.layout.data.TextImage
struct TextImage;
// Java: import cn.wps.text.layout.typo.layout.style.CharacterStyle
struct CharacterStyle;

// Java: public class WpsImageSpan extends CharacterStyle
pub struct WpsImageSpan {
    // Java field: TextImage mTextImage
    mTextImage: Option<i32>,
    // Java field: float mLineHei
    mLineHei: f32,
    // Java field: boolean mVertical
    mVertical: bool,
}

// Java: public class WpsImageSpan extends CharacterStyle
impl WpsImageSpan {
    // Java: WpsImageSpan(TextImage txtImage, float lineHei)
    pub fn new(&self, _txtImage: i32, _lineHei: f32) {
      // Java: super();

      // Java: this.mTextImage = txtImage;

      // Java: this.mLineHei = lineHei;

    }

    // Java: WpsImageSpan(TextImage txtImage, boolean vertical)
    pub fn new_2(&self, _txtImage: i32, _vertical: bool) {
      // Java: super();

      // Java: this.mTextImage = txtImage;

      // Java: this.mVertical = vertical;

    }

    // Java: void setTextWidth(float textWidth)
    pub fn setTextWidth(&self, _textWidth: f32) {
      // Java: this.mLineHei = textWidth;

    }

    // Java: void setVertical(boolean vertical)
    pub fn setVertical(&self, _vertical: bool) {
      // Java: this.mVertical = vertical;

    }

    // Java: void updateDrawState(TextPaint paint)
    pub fn updateDrawState(&self, _paint: i32) {
      // Java: this.applyImage(paint);

    }

    // Java: void applyImage(TextPaint paint)
    pub fn applyImage(&self, _paint: i32) {
      // Java: float[] positions = new float[]{(float)((double)1.0F - this.mTextImage.getEndPos()), (float)((double)1.0F - this.mTextImage.getStPos())};

      // Java: Shader srcShader = paint.getShader();

      // Java: float x0 = this.mVertical ? 1.2F * this.mLineHei : 0.0F;

      // Java: float y1 = this.mVertical ? 0.0F : this.mLineHei;

      // Java: if (srcShader == null)

      {
         // Java: int color = paint.getColor();

         // Java: int endClr = this.getAlphaColor(color, (int)((double)255.0F * this.mTextImage.getEndA()));

         // Java: int startClr = this.getAlphaColor(color, (int)((double)255.0F * this.mTextImage.getStA()));

         // Java: int[] colors = new int[]{endClr, startClr};

         // Java: LinearGradient lineGrad = new LinearGradient(x0, 0.0F, 0.0F, y1, colors, positions, TileMode.CLAMP);

         // Java: paint.setShader(lineGrad);

      }
      // Java: else

      {
         // Java: int endClr = this.getAlphaColor(-1, (int)((double)255.0F * this.mTextImage.getEndA()));

         // Java: int startClr = this.getAlphaColor(-1, (int)((double)255.0F * this.mTextImage.getStA()));

         // Java: int[] colors = new int[]{endClr, startClr};

         // Java: LinearGradient lineGrad = new LinearGradient(x0, 0.0F, 0.0F, y1, colors, positions, TileMode.CLAMP);

         // Java: ComposeShader composeShader = new ComposeShader(srcShader, lineGrad, Mode.DST_ATOP);

         // Java: paint.setShader(composeShader);

      }
      // Java: if (this.mTextImage.getBlurRad() > 0.0F)

      {
         // Java: BlurMaskFilter blurFilter = new BlurMaskFilter(this.mTextImage.getBlurRad() / 2.0F, Blur.NORMAL);

         // Java: paint.setMaskFilter(blurFilter);

      }
    }

    // Java: int getAlphaColor(int color, int alpha)
    pub fn getAlphaColor(&self, _color: i32, _alpha: i32) -> i32 {
      // Java: return alpha << 24 | 16777215 & color;

      0
    }

}
