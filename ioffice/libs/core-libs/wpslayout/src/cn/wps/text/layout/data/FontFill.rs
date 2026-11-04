// Java: import android.graphics.Bitmap
#[derive(Clone)]
pub struct Bitmap;

#[derive(Clone)]
pub struct GradientAttr;

impl GradientAttr {
    pub fn equals(&self, _other: &GradientAttr) -> bool { false }
    pub fn gradType(&self) -> GradType { GradType::LINEDOWN }
}

#[derive(Clone)]
pub struct PatternAttr;

impl PatternAttr {
    pub fn new_with_type(_fore_color: i32, _bg_color: i32, _patt_type: i32) -> Self { PatternAttr }
    pub fn new_with_bmp(_fore_color: i32, _bg_color: i32, _patt_bmp: Bitmap) -> Self { PatternAttr }
    pub fn equals(&self, _other: &PatternAttr) -> bool { false }
}

// Java: public enum FontFillType
#[derive(Clone, Copy, PartialEq, Eq)]
pub enum FontFillType {
    NONE,
    COLORFILL,
    GRADFILL,
    BLIPFILL,
    PATTERNFILL,
}

// Java: public enum GradType
#[derive(Clone, Copy, PartialEq, Eq)]
pub enum GradType {
    LEFTUP_TO_RIGHTDOWN,
    RIGHTUP_TO_LEFTDOWN,
    LEFTDOWN_TO_RIGHTUP,
    RIGHTDOWN_TO_LEFTUP,
    LINEDOWN,
    LINEUP,
    LINERIGHT,
    LINELEFT,
    LOWER_RIGHT_CORNER,
    LOWER_LEFT_CORNER,
    TOP_RIGHT_CORNER,
    TOP_LEFT_CORNER,
    CIRCLE,
    RECT,
    LINEAR,
    SHAPE,
}

// Java: public class FontFill
#[derive(Clone)]
pub struct FontFill {
    // Java field: FontFillType mFillType
    pub mFillType: FontFillType,
    // Java field: BlipFill mBlipFill
    pub mBlipFill: Option<BlipFill>,
    // Java field: GradFill mGradFill
    pub mGradFill: Option<GradFill>,
    // Java field: PatternFill mPattFill
    pub mPattFill: Option<PatternFill>,
    // Java field: int color
    pub color: i32,
}

// Java: public class BlipFill
#[derive(Clone)]
pub struct BlipFill {
    // Java field: int mMediaId
    pub(crate) mMediaId: i32,
    // Java field: Bitmap mBmp
    pub(crate) mBmp: Option<Bitmap>,
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
#[derive(Clone)]
pub struct GradFill {
    // Java field: GradientAttr gradAttr
    pub(crate) gradAttr: GradientAttr,
}

// Java: public class PatternFill
#[derive(Clone)]
pub struct PatternFill {
    // Java field: PatternAttr patternAttr
    pub(crate) patternAttr: PatternAttr,
}

// Java: public class FontFill
impl FontFill {
    // Java: FontFill()
    pub fn new() -> Self {
      // Java: super();
      // Java: this.mFillType = FontFill.FontFillType.COLORFILL;
      FontFill {
          mFillType: FontFillType::COLORFILL,
          mBlipFill: None,
          mGradFill: None,
          mPattFill: None,
          color: 0,
      }
    }

    // Java: void setGradFill(GradientAttr gradAttr)
    pub fn setGradFill(&mut self, gradAttr: GradientAttr) {
      // Java: this.mFillType = FontFill.FontFillType.GRADFILL;
      self.mFillType = FontFillType::GRADFILL;
      // Java: this.mGradFill = new GradFill(gradAttr);
      self.mGradFill = Some(GradFill::new(gradAttr));
    }

    // Java: void setBlipFill(Bitmap bitmap, float alpha)
    pub fn setBlipFill_bitmap(&mut self, bitmap: Bitmap, alpha: f32) {
      // Java: this.mFillType = FontFill.FontFillType.BLIPFILL;
      self.mFillType = FontFillType::BLIPFILL;
      // Java: this.mBlipFill = new BlipFill(bitmap, alpha);
      self.mBlipFill = Some(BlipFill::new_with_bitmap(bitmap, alpha));
    }

    // Java: void setBlipFill(Bitmap bitmap, float alpha, float offsetX, float offsetY, float scaleX, float scaleY, int algn, int flipType)
    pub fn setBlipFill_bitmap_full(&mut self, bitmap: Bitmap, alpha: f32, offsetX: f32, offsetY: f32, scaleX: f32, scaleY: f32, algn: i32, flipType: i32) {
      // Java: this.mFillType = FontFill.FontFillType.BLIPFILL;
      self.mFillType = FontFillType::BLIPFILL;
      // Java: this.mBlipFill = new BlipFill(bitmap, alpha, offsetX, offsetY, scaleX, scaleY, algn, flipType);
      self.mBlipFill = Some(BlipFill::new_with_bitmap_full(bitmap, alpha, offsetX, offsetY, scaleX, scaleY, algn, flipType));
    }

    // Java: void setBlipFill(int mediaId, float alpha)
    pub fn setBlipFill_mediaId(&mut self, mediaId: i32, alpha: f32) {
      // Java: this.mFillType = FontFill.FontFillType.BLIPFILL;
      self.mFillType = FontFillType::BLIPFILL;
      // Java: this.mBlipFill = new BlipFill(mediaId, alpha);
      self.mBlipFill = Some(BlipFill::new_with_mediaId(mediaId, alpha));
    }

    // Java: void setBlipFill(int mediaId, float alpha, float offsetX, float offsetY, float scaleX, float scaleY, int algn, int flipType)
    pub fn setBlipFill_mediaId_full(&mut self, mediaId: i32, alpha: f32, offsetX: f32, offsetY: f32, scaleX: f32, scaleY: f32, algn: i32, flipType: i32) {
      // Java: this.mFillType = FontFill.FontFillType.BLIPFILL;
      self.mFillType = FontFillType::BLIPFILL;
      // Java: this.mBlipFill = new BlipFill(mediaId, alpha, offsetX, offsetY, scaleX, scaleY, algn, flipType);
      self.mBlipFill = Some(BlipFill::new_with_mediaId_full(mediaId, alpha, offsetX, offsetY, scaleX, scaleY, algn, flipType));
    }

    // Java: void setPattFill(int foreColor, int bgColor, int pattType)
    pub fn setPattFill_type(&mut self, foreColor: i32, bgColor: i32, pattType: i32) {
      // Java: this.mFillType = FontFill.FontFillType.PATTERNFILL;
      self.mFillType = FontFillType::PATTERNFILL;
      // Java: PatternAttr patternAttr = new PatternAttr(foreColor, bgColor, pattType);
      let patternAttr = PatternAttr::new_with_type(foreColor, bgColor, pattType);
      // Java: this.mPattFill = new PatternFill(patternAttr);
      self.mPattFill = Some(PatternFill::new(patternAttr));
    }

    // Java: void setPattFill(int foreColor, int bgColor, Bitmap pattBmp)
    pub fn setPattFill_bmp(&mut self, foreColor: i32, bgColor: i32, pattBmp: Bitmap) {
      // Java: this.mFillType = FontFill.FontFillType.PATTERNFILL;
      self.mFillType = FontFillType::PATTERNFILL;
      // Java: PatternAttr patternAttr = new PatternAttr(foreColor, bgColor, pattBmp);
      let patternAttr = PatternAttr::new_with_bmp(foreColor, bgColor, pattBmp);
      // Java: this.mPattFill = new PatternFill(patternAttr);
      self.mPattFill = Some(PatternFill::new(patternAttr));
    }

    // Java: FontFillType type()
    pub fn fill_type(&self) -> FontFillType {
      // Java: return this.mFillType;
      self.mFillType
    }

    // Java: void setType(FontFillType filltype)
    pub fn setType(&mut self, filltype: FontFillType) {
      // Java: this.mFillType = filltype;
      self.mFillType = filltype;
    }

    // Java: boolean equals(FontFill fontFill)
    pub fn equals(&self, fontFill: Option<&FontFill>) -> bool {
      // Java: if (fontFill == null)
      let fontFill = match fontFill {
          None =>
      {
         // Java: return false;
         return false;
      }
      // Java: else
          Some(f) => f,
      };
      {
         // Java: boolean isEquals = false;
         let is_equals: bool;
         // Java: switch (this.mFillType)
         match self.mFillType
         {
            // Java: case NONE:
            // Java: case COLORFILL:
            FontFillType::NONE | FontFillType::COLORFILL => {
               // Java: isEquals = fontFill.mFillType.equals(this.mFillType);
               is_equals = fontFill.mFillType == self.mFillType;
               // Java: break;
            }
            // Java: case GRADFILL:
            FontFillType::GRADFILL => {
               // Java: isEquals = fontFill.mFillType.equals(FontFill.FontFillType.GRADFILL) && (this.mGradFill == null && fontFill.mGradFill == null || this.mGradFill != null && this.mGradFill.equals(fontFill.mGradFill));
               is_equals = fontFill.mFillType == FontFillType::GRADFILL && match (&self.mGradFill, &fontFill.mGradFill) {
                   (None, None) => true,
                   (Some(a), Some(b)) => a.equals(Some(b)),
                   _ => false,
               };
               // Java: break;
            }
            // Java: case BLIPFILL:
            FontFillType::BLIPFILL => {
               // Java: isEquals = fontFill.mFillType.equals(FontFill.FontFillType.BLIPFILL) && (this.mBlipFill == null && fontFill.mBlipFill == null || this.mBlipFill != null && this.mBlipFill.equals(fontFill.mBlipFill));
               is_equals = fontFill.mFillType == FontFillType::BLIPFILL && match (&self.mBlipFill, &fontFill.mBlipFill) {
                   (None, None) => true,
                   (Some(a), Some(b)) => a.equals(Some(b)),
                   _ => false,
               };
               // Java: break;
            }
            // Java: case PATTERNFILL:
            FontFillType::PATTERNFILL => {
               // Java: isEquals = fontFill.mFillType.equals(FontFill.FontFillType.PATTERNFILL) && (this.mPattFill == null && fontFill.mFillType == null || this.mPattFill != null && this.mPattFill.equals(fontFill.mPattFill));
               is_equals = fontFill.mFillType == FontFillType::PATTERNFILL && match (&self.mPattFill, &fontFill.mPattFill) {
                   (None, None) => true,
                   (Some(a), Some(b)) => a.equals(Some(b)),
                   _ => false,
               };
            }
         }
         // Java: return isEquals;
         is_equals
      }
    }

}

// Java: public enum FontFillType
impl FontFillType {
    // Java: FontFillType()
}

// Java: public class BlipFill
impl BlipFill {
    // Java: BlipFill(Bitmap bmp, float alpha)
    pub fn new_with_bitmap(bmp: Bitmap, alpha: f32) -> Self {
         // Java: super();
         // Java: this.mFillMode = 1;
         // Java: this.mBmp = bmp;
         // Java: this.mAlpha = alpha;
         BlipFill {
             mMediaId: -1,
             mBmp: Some(bmp),
             mAlpha: alpha,
             mFillMode: 1,
             mOffsetX: 0.0,
             mOffsetY: 0.0,
             mScaleX: 1.0,
             mScaleY: 1.0,
             mAlgn: 0,
             mFlipType: 0,
         }
    }

    // Java: BlipFill(int mediaId, float alpha)
    pub fn new_with_mediaId(mediaId: i32, alpha: f32) -> Self {
         // Java: super();
         // Java: this.mFillMode = 1;
         // Java: this.mMediaId = mediaId;
         // Java: this.mAlpha = alpha;
         BlipFill {
             mMediaId: mediaId,
             mBmp: None,
             mAlpha: alpha,
             mFillMode: 1,
             mOffsetX: 0.0,
             mOffsetY: 0.0,
             mScaleX: 1.0,
             mScaleY: 1.0,
             mAlgn: 0,
             mFlipType: 0,
         }
    }

    // Java: BlipFill(Bitmap bmp, float alpha, float offsetX, float offsetY, float scaleX, float scaleY, int algn, int flipType)
    pub fn new_with_bitmap_full(bmp: Bitmap, alpha: f32, offsetX: f32, offsetY: f32, scaleX: f32, scaleY: f32, algn: i32, flipType: i32) -> Self {
         // Java: super();
         // Java: this.mFillMode = 0;
         // Java: this.mBmp = bmp;
         // Java: this.mAlpha = alpha;
         // Java: this.mOffsetX = offsetX;
         // Java: this.mOffsetY = offsetY;
         // Java: this.mScaleX = scaleX;
         // Java: this.mScaleY = scaleY;
         // Java: this.mAlgn = algn;
         // Java: this.mFlipType = flipType;
         BlipFill {
             mMediaId: -1,
             mBmp: Some(bmp),
             mAlpha: alpha,
             mFillMode: 0,
             mOffsetX: offsetX,
             mOffsetY: offsetY,
             mScaleX: scaleX,
             mScaleY: scaleY,
             mAlgn: algn,
             mFlipType: flipType,
         }
    }

    // Java: BlipFill(int mediaId, float alpha, float offsetX, float offsetY, float scaleX, float scaleY, int algn, int flipType)
    pub fn new_with_mediaId_full(mediaId: i32, alpha: f32, offsetX: f32, offsetY: f32, scaleX: f32, scaleY: f32, algn: i32, flipType: i32) -> Self {
         // Java: super();
         // Java: this.mFillMode = 0;
         // Java: this.mMediaId = mediaId;
         // Java: this.mAlpha = alpha;
         // Java: this.mOffsetX = offsetX;
         // Java: this.mOffsetY = offsetY;
         // Java: this.mScaleX = scaleX;
         // Java: this.mScaleY = scaleY;
         // Java: this.mAlgn = algn;
         // Java: this.mFlipType = flipType;
         BlipFill {
             mMediaId: mediaId,
             mBmp: None,
             mAlpha: alpha,
             mFillMode: 0,
             mOffsetX: offsetX,
             mOffsetY: offsetY,
             mScaleX: scaleX,
             mScaleY: scaleY,
             mAlgn: algn,
             mFlipType: flipType,
         }
    }

    // Java: Bitmap getBitmap()
    pub fn getBitmap(&self) -> Option<&Bitmap> {
         // Java: return this.mBmp;
         self.mBmp.as_ref()
    }

    // Java: void setBitmap(Bitmap bmp)
    pub fn setBitmap(&mut self, bmp: Bitmap) {
         // Java: this.mBmp = bmp;
         self.mBmp = Some(bmp);
    }

    // Java: int getMediaId()
    pub fn getMediaId(&self) -> i32 {
         // Java: return this.mMediaId;
         self.mMediaId
    }

    // Java: float getAlpha()
    pub fn getAlpha(&self) -> f32 {
         // Java: return this.mAlpha;
         self.mAlpha
    }

    // Java: int getFillMode()
    pub fn getFillMode(&self) -> i32 {
         // Java: return this.mFillMode;
         self.mFillMode
    }

    // Java: float getOffsetX()
    pub fn getOffsetX(&self) -> f32 {
         // Java: return this.mOffsetX;
         self.mOffsetX
    }

    // Java: float getOffsetY()
    pub fn getOffsetY(&self) -> f32 {
         // Java: return this.mOffsetY;
         self.mOffsetY
    }

    // Java: float getScaleX()
    pub fn getScaleX(&self) -> f32 {
         // Java: return this.mScaleX;
         self.mScaleX
    }

    // Java: float getScaleY()
    pub fn getScaleY(&self) -> f32 {
         // Java: return this.mScaleY;
         self.mScaleY
    }

    // Java: int getAlgn()
    pub fn getAlgn(&self) -> i32 {
         // Java: return this.mAlgn;
         self.mAlgn
    }

    // Java: int getFlipType()
    pub fn getFlipType(&self) -> i32 {
         // Java: return this.mFlipType;
         self.mFlipType
    }

    // Java: boolean equals(BlipFill blipFill)
    pub fn equals(&self, blipFill: Option<&BlipFill>) -> bool {
         // Java: if (blipFill == null)
         let blipFill = match blipFill {
             None =>
         {
            // Java: return false;
            return false;
         }
         // Java: else if (this.mFillMode != blipFill.mFillMode)
             Some(f) => f,
         };
         if self.mFillMode != blipFill.mFillMode
         {
            // Java: return false;
            return false;
         }
         // Java: else
         else
         {
            // Java: boolean equals = false;
            let equals: bool;
            // Java: switch (this.mFlipType)
            match self.mFlipType
            {
               // Java: case 0:
               0 => {
                  // Java: equals = (this.mBmp == null && blipFill.mBmp == null || this.mBmp != null && this.mBmp.equals(blipFill.mBmp)) && this.mAlpha == blipFill.mAlpha && this.mOffsetY == blipFill.mOffsetY && this.mOffsetX == blipFill.mOffsetX && this.mScaleX == blipFill.mScaleX && this.mScaleY == blipFill.mScaleY && this.mAlgn == blipFill.mAlgn && this.mFlipType == blipFill.mFlipType;
                  equals = (self.mBmp.is_none() && blipFill.mBmp.is_none() || self.mBmp.is_some())
                      && self.mAlpha == blipFill.mAlpha
                      && self.mOffsetY == blipFill.mOffsetY
                      && self.mOffsetX == blipFill.mOffsetX
                      && self.mScaleX == blipFill.mScaleX
                      && self.mScaleY == blipFill.mScaleY
                      && self.mAlgn == blipFill.mAlgn
                      && self.mFlipType == blipFill.mFlipType;
                  // Java: break;
               }
               // Java: case 1:
               1 => {
                  // Java: equals = (this.mBmp == null && blipFill.mBmp == null || this.mBmp != null && this.mBmp.equals(blipFill.mBmp)) && this.mAlpha == blipFill.mAlpha;
                  equals = (self.mBmp.is_none() && blipFill.mBmp.is_none() || self.mBmp.is_some())
                      && self.mAlpha == blipFill.mAlpha;
               }
               _ => { equals = false; }
            }
            // Java: return equals;
            equals
         }
    }

}

// Java: public class GradFill
impl GradFill {
    // Java: GradFill(GradientAttr gradAttr)
    pub fn new(gradAttr: GradientAttr) -> Self {
         // Java: super();
         // Java: this.gradAttr = gradAttr;
         GradFill { gradAttr }
    }

    // Java: GradientAttr getGradAttr()
    pub fn getGradAttr(&self) -> &GradientAttr {
         // Java: return this.gradAttr;
         &self.gradAttr
    }

    // Java: boolean equals(GradFill gradFill)
    pub fn equals(&self, gradFill: Option<&GradFill>) -> bool {
         // Java: return gradFill == null ? false : this.gradAttr.equals(gradFill.gradAttr);
         match gradFill {
             None => false,
             Some(gf) => self.gradAttr.equals(&gf.gradAttr),
         }
    }

}

// Java: public enum GradType
impl GradType {
    // Java: GradType()
}

// Java: public class PatternFill
impl PatternFill {
    // Java: PatternFill(PatternAttr patternAttr)
    pub fn new(patternAttr: PatternAttr) -> Self {
         // Java: super();
         // Java: this.patternAttr = patternAttr;
         PatternFill { patternAttr }
    }

    // Java: PatternAttr getPatternAttr()
    pub fn getPatternAttr(&self) -> &PatternAttr {
         // Java: return this.patternAttr;
         &self.patternAttr
    }

    // Java: boolean equals(PatternFill patternFill)
    pub fn equals(&self, patternFill: Option<&PatternFill>) -> bool {
         // Java: return patternFill == null ? false : this.patternAttr.equals(patternFill.patternAttr);
         match patternFill {
             None => false,
             Some(pf) => self.patternAttr.equals(&pf.patternAttr),
         }
    }

}
