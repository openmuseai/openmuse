// Java: import android.graphics.Bitmap
use util::graphics::Bitmap;

// Java: public class FontFill
pub struct FontFill {
    // Java field: FontFillType mFillType
    pub(crate) mFillType: Arc<Mutex<FontFillType>>,
    // Java field: BlipFill mBlipFill
    pub(crate) mBlipFill: Arc<Mutex<BlipFill>>,
    // Java field: GradFill mGradFill
    pub(crate) mGradFill: Arc<Mutex<GradFill>>,
    // Java field: PatternFill mPattFill
    pub(crate) mPattFill: Arc<Mutex<PatternFill>>,
    // Java field: int color
    pub(crate) color: i32,
}

// Java: public enum FontFillType
pub struct FontFillType {
}

// Java: public class BlipFill
pub struct BlipFill {
    // Java field: int mMediaId
    pub(crate) mMediaId: i32,
    // Java field: Bitmap mBmp
    pub(crate) mBmp: Arc<Mutex<Bitmap>>,
    // Java field: float mAlpha
    pub(crate) mAlpha: f32,
    // Java field: int mFillMode
    pub(crate) mFillMode: i32,
    // Java field: float mOffsetX
    pub(crate) mOffsetX: f32,
    // Java field: float mOffsetY
    pub(crate) mOffsetY: f32,
    // Java field: float mScaleX
    pub(crate) mScaleX: f32,
    // Java field: float mScaleY
    pub(crate) mScaleY: f32,
    // Java field: int mAlgn
    pub(crate) mAlgn: i32,
    // Java field: int mFlipType
    pub(crate) mFlipType: i32,
}

// Java: public class GradFill
pub struct GradFill {
    // Java field: GradientAttr gradAttr
    pub(crate) gradAttr: Arc<Mutex<GradientAttr>>,
}

// Java: public enum GradType
pub struct GradType {
}

// Java: public class PatternFill
pub struct PatternFill {
    // Java field: PatternAttr patternAttr
    pub(crate) patternAttr: Arc<Mutex<PatternAttr>>,
}

// Java: public class FontFill
impl FontFill {
    // Java: FontFill()
    pub fn FontFill(&self) {
      // Java: super();
      // TODO_RUST_CODE
      // Java: this.mFillType = FontFill.FontFillType.COLORFILL;
      // TODO_RUST_CODE
    }

    // Java: void setGradFill(GradientAttr gradAttr)
    pub fn setGradFill(&self, GradientAttr gradAttr) {
      // Java: this.mFillType = FontFill.FontFillType.GRADFILL;
      // TODO_RUST_CODE
      // Java: this.mGradFill = new GradFill(gradAttr);
      // TODO_RUST_CODE
    }

    // Java: void setBlipFill(Bitmap bitmap, float alpha)
    pub fn setBlipFill(&self, Bitmap bitmap, float alpha) {
      // Java: this.mFillType = FontFill.FontFillType.BLIPFILL;
      // TODO_RUST_CODE
      // Java: this.mBlipFill = new BlipFill(bitmap, alpha);
      // TODO_RUST_CODE
    }

    // Java: void setBlipFill(Bitmap bitmap, float alpha, float offsetX, float offsetY, float scaleX, float scaleY, int algn, int flipType)
    pub fn setBlipFill(&self, Bitmap bitmap, float alpha, float offsetX, float offsetY, float scaleX, float scaleY, int algn, int flipType) {
      // Java: this.mFillType = FontFill.FontFillType.BLIPFILL;
      // TODO_RUST_CODE
      // Java: this.mBlipFill = new BlipFill(bitmap, alpha, offsetX, offsetY, scaleX, scaleY, algn, flipType);
      // TODO_RUST_CODE
    }

    // Java: void setBlipFill(int mediaId, float alpha)
    pub fn setBlipFill(&self, int mediaId, float alpha) {
      // Java: this.mFillType = FontFill.FontFillType.BLIPFILL;
      // TODO_RUST_CODE
      // Java: this.mBlipFill = new BlipFill(mediaId, alpha);
      // TODO_RUST_CODE
    }

    // Java: void setBlipFill(int mediaId, float alpha, float offsetX, float offsetY, float scaleX, float scaleY, int algn, int flipType)
    pub fn setBlipFill(&self, int mediaId, float alpha, float offsetX, float offsetY, float scaleX, float scaleY, int algn, int flipType) {
      // Java: this.mFillType = FontFill.FontFillType.BLIPFILL;
      // TODO_RUST_CODE
      // Java: this.mBlipFill = new BlipFill(mediaId, alpha, offsetX, offsetY, scaleX, scaleY, algn, flipType);
      // TODO_RUST_CODE
    }

    // Java: void setPattFill(int foreColor, int bgColor, int pattType)
    pub fn setPattFill(&self, int foreColor, int bgColor, int pattType) {
      // Java: this.mFillType = FontFill.FontFillType.PATTERNFILL;
      // TODO_RUST_CODE
      // Java: PatternAttr patternAttr = new PatternAttr(foreColor, bgColor, pattType);
      // TODO_RUST_CODE
      // Java: this.mPattFill = new PatternFill(patternAttr);
      // TODO_RUST_CODE
    }

    // Java: void setPattFill(int foreColor, int bgColor, Bitmap pattBmp)
    pub fn setPattFill(&self, int foreColor, int bgColor, Bitmap pattBmp) {
      // Java: this.mFillType = FontFill.FontFillType.PATTERNFILL;
      // TODO_RUST_CODE
      // Java: PatternAttr patternAttr = new PatternAttr(foreColor, bgColor, pattBmp);
      // TODO_RUST_CODE
      // Java: this.mPattFill = new PatternFill(patternAttr);
      // TODO_RUST_CODE
    }

    // Java: FontFillType type()
    pub fn type(&self) -> FontFillType {
      // Java: return this.mFillType;
      // TODO_RUST_CODE
    }

    // Java: void setType(FontFillType filltype)
    pub fn setType(&self, FontFillType filltype) {
      // Java: this.mFillType = filltype;
      // TODO_RUST_CODE
    }

    // Java: boolean equals(FontFill fontFill)
    pub fn equals(&self, FontFill fontFill) -> bool {
      // Java: if (fontFill == null)
        // TODO_RUST_CODE
      {
         // Java: return false;
         // TODO_RUST_CODE
      }
      // Java: else
        // TODO_RUST_CODE
      {
         // Java: boolean isEquals = false;
         // TODO_RUST_CODE
         // Java: switch (this.mFillType)
            // TODO_RUST_CODE
         {
            // Java: case NONE:
            // TODO_RUST_CODE
            // Java: case COLORFILL:
            // TODO_RUST_CODE
               // Java: isEquals = fontFill.mFillType.equals(this.mFillType);
               // TODO_RUST_CODE
               // Java: break;
               // TODO_RUST_CODE
            // Java: case GRADFILL:
            // TODO_RUST_CODE
               // Java: isEquals = fontFill.mFillType.equals(FontFill.FontFillType.GRADFILL) && (this.mGradFill == null && fontFill.mGradFill == null || this.mGradFill != null && this.mGradFill.equals(fontFill.mGradFill));
               // TODO_RUST_CODE
               // Java: break;
               // TODO_RUST_CODE
            // Java: case BLIPFILL:
            // TODO_RUST_CODE
               // Java: isEquals = fontFill.mFillType.equals(FontFill.FontFillType.BLIPFILL) && (this.mBlipFill == null && fontFill.mBlipFill == null || this.mBlipFill != null && this.mBlipFill.equals(fontFill.mBlipFill));
               // TODO_RUST_CODE
               // Java: break;
               // TODO_RUST_CODE
            // Java: case PATTERNFILL:
            // TODO_RUST_CODE
               // Java: isEquals = fontFill.mFillType.equals(FontFill.FontFillType.PATTERNFILL) && (this.mPattFill == null && fontFill.mFillType == null || this.mPattFill != null && this.mPattFill.equals(fontFill.mPattFill));
               // TODO_RUST_CODE
         }
         // Java: return isEquals;
         // TODO_RUST_CODE
      }
    }

}

// Java: public enum FontFillType
impl FontFillType {
    // Java: FontFillType()
    pub fn FontFillType(&self) {
    // TODO_RUST_CODE
    }

}

// Java: public class BlipFill
impl BlipFill {
    // Java: BlipFill(Bitmap bmp, float alpha)
    pub fn BlipFill(&self, Bitmap bmp, float alpha) {
         // Java: super();
         // TODO_RUST_CODE
         // Java: this.mFillMode = 1;
         // TODO_RUST_CODE
         // Java: this.mBmp = bmp;
         // TODO_RUST_CODE
         // Java: this.mAlpha = alpha;
         // TODO_RUST_CODE
    }

    // Java: BlipFill(int mediaId, float alpha)
    pub fn BlipFill(&self, int mediaId, float alpha) {
         // Java: super();
         // TODO_RUST_CODE
         // Java: this.mFillMode = 1;
         // TODO_RUST_CODE
         // Java: this.mMediaId = mediaId;
         // TODO_RUST_CODE
         // Java: this.mAlpha = alpha;
         // TODO_RUST_CODE
    }

    // Java: BlipFill(Bitmap bmp, float alpha, float offsetX, float offsetY, float scaleX, float scaleY, int algn, int flipType)
    pub fn BlipFill(&self, Bitmap bmp, float alpha, float offsetX, float offsetY, float scaleX, float scaleY, int algn, int flipType) {
         // Java: super();
         // TODO_RUST_CODE
         // Java: this.mFillMode = 0;
         // TODO_RUST_CODE
         // Java: this.mBmp = bmp;
         // TODO_RUST_CODE
         // Java: this.mAlpha = alpha;
         // TODO_RUST_CODE
         // Java: this.mOffsetX = offsetX;
         // TODO_RUST_CODE
         // Java: this.mOffsetY = offsetY;
         // TODO_RUST_CODE
         // Java: this.mScaleX = scaleX;
         // TODO_RUST_CODE
         // Java: this.mScaleY = scaleY;
         // TODO_RUST_CODE
         // Java: this.mAlgn = algn;
         // TODO_RUST_CODE
         // Java: this.mFlipType = flipType;
         // TODO_RUST_CODE
    }

    // Java: BlipFill(int mediaId, float alpha, float offsetX, float offsetY, float scaleX, float scaleY, int algn, int flipType)
    pub fn BlipFill(&self, int mediaId, float alpha, float offsetX, float offsetY, float scaleX, float scaleY, int algn, int flipType) {
         // Java: super();
         // TODO_RUST_CODE
         // Java: this.mFillMode = 0;
         // TODO_RUST_CODE
         // Java: this.mMediaId = mediaId;
         // TODO_RUST_CODE
         // Java: this.mAlpha = alpha;
         // TODO_RUST_CODE
         // Java: this.mOffsetX = offsetX;
         // TODO_RUST_CODE
         // Java: this.mOffsetY = offsetY;
         // TODO_RUST_CODE
         // Java: this.mScaleX = scaleX;
         // TODO_RUST_CODE
         // Java: this.mScaleY = scaleY;
         // TODO_RUST_CODE
         // Java: this.mAlgn = algn;
         // TODO_RUST_CODE
         // Java: this.mFlipType = flipType;
         // TODO_RUST_CODE
    }

    // Java: Bitmap getBitmap()
    pub fn getBitmap(&self) -> Bitmap {
         // Java: return this.mBmp;
         // TODO_RUST_CODE
    }

    // Java: void setBitmap(Bitmap bmp)
    pub fn setBitmap(&self, Bitmap bmp) {
         // Java: this.mBmp = bmp;
         // TODO_RUST_CODE
    }

    // Java: int getMediaId()
    pub fn getMediaId(&self) -> i32 {
         // Java: return this.mMediaId;
         // TODO_RUST_CODE
    }

    // Java: float getAlpha()
    pub fn getAlpha(&self) -> f32 {
         // Java: return this.mAlpha;
         // TODO_RUST_CODE
    }

    // Java: int getFillMode()
    pub fn getFillMode(&self) -> i32 {
         // Java: return this.mFillMode;
         // TODO_RUST_CODE
    }

    // Java: float getOffsetX()
    pub fn getOffsetX(&self) -> f32 {
         // Java: return this.mOffsetX;
         // TODO_RUST_CODE
    }

    // Java: float getOffsetY()
    pub fn getOffsetY(&self) -> f32 {
         // Java: return this.mOffsetY;
         // TODO_RUST_CODE
    }

    // Java: float getScaleX()
    pub fn getScaleX(&self) -> f32 {
         // Java: return this.mScaleX;
         // TODO_RUST_CODE
    }

    // Java: float getScaleY()
    pub fn getScaleY(&self) -> f32 {
         // Java: return this.mScaleY;
         // TODO_RUST_CODE
    }

    // Java: int getAlgn()
    pub fn getAlgn(&self) -> i32 {
         // Java: return this.mAlgn;
         // TODO_RUST_CODE
    }

    // Java: int getFlipType()
    pub fn getFlipType(&self) -> i32 {
         // Java: return this.mFlipType;
         // TODO_RUST_CODE
    }

    // Java: boolean equals(BlipFill blipFill)
    pub fn equals(&self, BlipFill blipFill) -> bool {
         // Java: if (blipFill == null)
        // TODO_RUST_CODE
         {
            // Java: return false;
            // TODO_RUST_CODE
         }
         // Java: else if (this.mFillMode != blipFill.mFillMode)
        // TODO_RUST_CODE
         {
            // Java: return false;
            // TODO_RUST_CODE
         }
         // Java: else
        // TODO_RUST_CODE
         {
            // Java: boolean equals = false;
            // TODO_RUST_CODE
            // Java: switch (this.mFlipType)
            // TODO_RUST_CODE
            {
               // Java: case 0:
               // TODO_RUST_CODE
                  // Java: equals = (this.mBmp == null && blipFill.mBmp == null || this.mBmp != null && this.mBmp.equals(blipFill.mBmp)) && this.mAlpha == blipFill.mAlpha && this.mOffsetY == blipFill.mOffsetY && this.mOffsetX == blipFill.mOffsetX && this.mScaleX == blipFill.mScaleX && this.mScaleY == blipFill.mScaleY && this.mAlgn == blipFill.mAlgn && this.mFlipType == blipFill.mFlipType;
                  // TODO_RUST_CODE
                  // Java: break;
                  // TODO_RUST_CODE
               // Java: case 1:
               // TODO_RUST_CODE
                  // Java: equals = (this.mBmp == null && blipFill.mBmp == null || this.mBmp != null && this.mBmp.equals(blipFill.mBmp)) && this.mAlpha == blipFill.mAlpha;
                  // TODO_RUST_CODE
            }
            // Java: return equals;
            // TODO_RUST_CODE
         }
    }

}

// Java: public class GradFill
impl GradFill {
    // Java: GradFill(GradientAttr gradAttr)
    pub fn GradFill(&self, GradientAttr gradAttr) {
         // Java: super();
         // TODO_RUST_CODE
         // Java: this.gradAttr = gradAttr;
         // TODO_RUST_CODE
    }

    // Java: GradientAttr getGradAttr()
    pub fn getGradAttr(&self) -> GradientAttr {
         // Java: return this.gradAttr;
         // TODO_RUST_CODE
    }

    // Java: boolean equals(GradFill gradFill)
    pub fn equals(&self, GradFill gradFill) -> bool {
         // Java: return gradFill == null ? false : this.gradAttr.equals(gradFill.gradAttr);
         // TODO_RUST_CODE
    }

}

// Java: public enum GradType
impl GradType {
    // Java: GradType()
    pub fn GradType(&self) {
    // TODO_RUST_CODE
    }

}

// Java: public class PatternFill
impl PatternFill {
    // Java: PatternFill(PatternAttr patternAttr)
    pub fn PatternFill(&self, PatternAttr patternAttr) {
         // Java: super();
         // TODO_RUST_CODE
         // Java: this.patternAttr = patternAttr;
         // TODO_RUST_CODE
    }

    // Java: PatternAttr getPatternAttr()
    pub fn getPatternAttr(&self) -> PatternAttr {
         // Java: return this.patternAttr;
         // TODO_RUST_CODE
    }

    // Java: boolean equals(PatternFill patternFill)
    pub fn equals(&self, PatternFill patternFill) -> bool {
         // Java: return patternFill == null ? false : this.patternAttr.equals(patternFill.patternAttr);
         // TODO_RUST_CODE
    }

}
