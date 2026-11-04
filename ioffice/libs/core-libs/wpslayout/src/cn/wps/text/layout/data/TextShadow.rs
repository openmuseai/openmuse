// Java: import android.graphics.PointF
#[derive(Clone)]
pub struct PointF {
    pub x: f32,
    pub y: f32,
}

// Java: public enum ShadowType
#[derive(Clone, Copy, PartialEq, Eq)]
pub enum ShadowType {
    NONE,
    OUTER,
    INNER,
}

// Java: public enum ShadowAlgn
#[derive(Clone, Copy, PartialEq, Eq)]
pub enum ShadowAlgn {
    B,
    BL,
    BR,
    CTR,
    L,
    R,
    T,
    TL,
    TR,
}

// Java: public class TextShadow
#[derive(Clone)]
pub struct TextShadow {
    // Java field: int mShadowColor
    pub(crate) mShadowColor: i32,
    // Java field: float mShadowAlpha
    pub(crate) mShadowAlpha: f32,
    // Java field: float mShadowDis
    pub(crate) mShadowDis: f32,
    // Java field: float mShadowRad
    pub(crate) mShadowRad: f32,
    // Java field: float mShadowAngle
    pub(crate) mShadowAngle: f32,
    // Java field: float xPct
    pub(crate) xPct: f32,
    // Java field: float yPct
    pub(crate) yPct: f32,
    // Java field: float xSkew
    pub(crate) xSkew: f32,
    // Java field: float ySkew
    pub(crate) ySkew: f32,
    // Java field: int algn
    pub(crate) algn: i32,
    // Java field: ShadowType mShdwType
    pub(crate) mShdwType: Option<ShadowType>,
    // Java field: int mPrstShdwType
    pub(crate) mPrstShdwType: i32,
    // Java field: float mShdwOffsetX
    pub(crate) mShdwOffsetX: f32,
    // Java field: float mShdwOffsetY
    pub(crate) mShdwOffsetY: f32,
    // Java field: boolean mIsDoc
    pub(crate) mIsDoc: bool,
    // Java field: PointF mOrigin
    pub(crate) mOrigin: Option<PointF>,
}

// Java: public class TextShadow
impl TextShadow {
    // Java: TextShadow()
    pub fn new() -> Self {
      // Java: super();
      TextShadow {
          mShadowColor: 0,
          mShadowAlpha: 0.0,
          mShadowDis: 0.0,
          mShadowRad: 0.0,
          mShadowAngle: 0.0,
          xPct: 1.0,
          yPct: 1.0,
          xSkew: 0.0,
          ySkew: 0.0,
          algn: 0,
          mShdwType: None,
          mPrstShdwType: 0,
          mShdwOffsetX: 0.0,
          mShdwOffsetY: 0.0,
          mIsDoc: false,
          mOrigin: None,
      }
    }

    // Java: void setOuterShdw(int shdwClr, float shdwAlpha, float shdwDis, float shdwRad, float shdwAngle, float xPct, float yPct, float xSkew, float ySkew, int algn)
    pub fn setOuterShdw(&mut self, shdwClr: i32, shdwAlpha: f32, shdwDis: f32, shdwRad: f32, shdwAngle: f32, xPct: f32, yPct: f32, xSkew: f32, ySkew: f32, algn: i32) {
      // Java: this.mShadowColor = shdwClr;
      self.mShadowColor = shdwClr;
      // Java: this.mShadowAlpha = shdwAlpha;
      self.mShadowAlpha = shdwAlpha;
      // Java: this.mShadowDis = shdwDis;
      self.mShadowDis = shdwDis;
      // Java: this.mShadowRad = shdwRad;
      self.mShadowRad = shdwRad;
      // Java: this.mShadowAngle = shdwAngle;
      self.mShadowAngle = shdwAngle;
      // Java: this.xPct = xPct;
      self.xPct = xPct;
      // Java: this.yPct = yPct;
      self.yPct = yPct;
      // Java: this.xSkew = xSkew;
      self.xSkew = xSkew;
      // Java: this.ySkew = ySkew;
      self.ySkew = ySkew;
      // Java: this.algn = algn;
      self.algn = algn;
      // Java: this.mShdwType = TextShadow.ShadowType.OUTER;
      self.mShdwType = Some(ShadowType::OUTER);
    }

    // Java: void setPrstShdw(int shdwClr, float shdwAlpha, float shdwDis, float shdwAngle, float yPct, float xSkew, int algn, int prstShdwType)
    pub fn setPrstShdw(&mut self, shdwClr: i32, shdwAlpha: f32, shdwDis: f32, shdwAngle: f32, yPct: f32, xSkew: f32, algn: i32, prstShdwType: i32) {
      // Java: this.mShadowColor = shdwClr;
      self.mShadowColor = shdwClr;
      // Java: this.mShadowAlpha = shdwAlpha;
      self.mShadowAlpha = shdwAlpha;
      // Java: this.mShadowDis = shdwDis;
      self.mShadowDis = shdwDis;
      // Java: this.mShadowRad = 0.0F;
      self.mShadowRad = 0.0;
      // Java: this.mShadowAngle = shdwAngle;
      self.mShadowAngle = shdwAngle;
      // Java: this.xPct = 1.0F;
      self.xPct = 1.0;
      // Java: this.yPct = yPct;
      self.yPct = yPct;
      // Java: this.xSkew = xSkew;
      self.xSkew = xSkew;
      // Java: this.ySkew = 0.0F;
      self.ySkew = 0.0;
      // Java: this.algn = algn;
      self.algn = algn;
      // Java: this.mPrstShdwType = prstShdwType;
      self.mPrstShdwType = prstShdwType;
      // Java: this.mShdwType = TextShadow.ShadowType.OUTER;
      self.mShdwType = Some(ShadowType::OUTER);
    }

    // Java: void setPrstShdwDoc(int shdwClr, float shdwAlpha, float shdwOffsetX, float shdwOffsetY, PointF pct, PointF origin, float xSkew, int prstShdwType, ShadowType shdwType)
    pub fn setPrstShdwDoc_with_type(&mut self, shdwClr: i32, shdwAlpha: f32, shdwOffsetX: f32, shdwOffsetY: f32, pct: PointF, origin: PointF, xSkew: f32, prstShdwType: i32, shdwType: ShadowType) {
      // Java: this.mShadowColor = shdwClr;
      self.mShadowColor = shdwClr;
      // Java: this.mShadowAlpha = shdwAlpha;
      self.mShadowAlpha = shdwAlpha;
      // Java: this.mShadowRad = 0.0F;
      self.mShadowRad = 0.0;
      // Java: this.mShdwOffsetX = shdwOffsetX;
      self.mShdwOffsetX = shdwOffsetX;
      // Java: this.mShdwOffsetY = shdwOffsetY;
      self.mShdwOffsetY = shdwOffsetY;
      // Java: this.mIsDoc = true;
      self.mIsDoc = true;
      // Java: this.xPct = pct.x;
      self.xPct = pct.x;
      // Java: this.yPct = pct.y;
      self.yPct = pct.y;
      // Java: this.mOrigin = origin;
      self.mOrigin = Some(origin);
      // Java: this.xSkew = xSkew;
      self.xSkew = xSkew;
      // Java: this.ySkew = 0.0F;
      self.ySkew = 0.0;
      // Java: this.mPrstShdwType = prstShdwType;
      self.mPrstShdwType = prstShdwType;
      // Java: this.mShdwType = shdwType;
      self.mShdwType = Some(shdwType);
    }

    // Java: void setPrstShdwDoc(int shdwClr, float shdwAlpha, float shdwOffsetX, float shdwOffsetY, PointF pct, PointF origin, float xSkew, int prstShdwType)
    pub fn setPrstShdwDoc(&mut self, shdwClr: i32, shdwAlpha: f32, shdwOffsetX: f32, shdwOffsetY: f32, pct: PointF, origin: PointF, xSkew: f32, prstShdwType: i32) {
      // Java: this.setPrstShdwDoc(shdwClr, shdwAlpha, shdwOffsetX, shdwOffsetY, pct, origin, xSkew, prstShdwType, TextShadow.ShadowType.OUTER);
      self.setPrstShdwDoc_with_type(shdwClr, shdwAlpha, shdwOffsetX, shdwOffsetY, pct, origin, xSkew, prstShdwType, ShadowType::OUTER);
    }

    // Java: void setInnerShdw(int shdwClr, float shdwAlpha, float shdwDis, float shdwRad, float shdwAngle)
    pub fn setInnerShdw(&mut self, shdwClr: i32, shdwAlpha: f32, shdwDis: f32, shdwRad: f32, shdwAngle: f32) {
      // Java: this.mShadowColor = shdwClr;
      self.mShadowColor = shdwClr;
      // Java: this.mShadowAlpha = shdwAlpha;
      self.mShadowAlpha = shdwAlpha;
      // Java: this.mShadowDis = shdwDis;
      self.mShadowDis = shdwDis;
      // Java: this.mShadowRad = shdwRad;
      self.mShadowRad = shdwRad;
      // Java: this.mShadowAngle = shdwAngle;
      self.mShadowAngle = shdwAngle;
      // Java: this.mShdwType = TextShadow.ShadowType.INNER;
      self.mShdwType = Some(ShadowType::INNER);
    }

    // Java: int getShdwClr()
    pub fn getShdwClr(&self) -> i32 {
      // Java: return this.mShadowColor;
      self.mShadowColor
    }

    // Java: float getShdwAlpha()
    pub fn getShdwAlpha(&self) -> f32 {
      // Java: return this.mShadowAlpha;
      self.mShadowAlpha
    }

    // Java: float getShdwDis()
    pub fn getShdwDis(&self) -> f32 {
      // Java: return this.mShadowDis;
      self.mShadowDis
    }

    // Java: float getShdwRad()
    pub fn getShdwRad(&self) -> f32 {
      // Java: return this.mShadowRad;
      self.mShadowRad
    }

    // Java: float getShdwAngle()
    pub fn getShdwAngle(&self) -> f32 {
      // Java: return this.mShadowAngle;
      self.mShadowAngle
    }

    // Java: float getShdwXPct()
    pub fn getShdwXPct(&self) -> f32 {
      // Java: return this.xPct;
      self.xPct
    }

    // Java: float getShdwYPct()
    pub fn getShdwYPct(&self) -> f32 {
      // Java: return this.yPct;
      self.yPct
    }

    // Java: float getShdwXSkew()
    pub fn getShdwXSkew(&self) -> f32 {
      // Java: return this.xSkew;
      self.xSkew
    }

    // Java: float getShdwYSkew()
    pub fn getShdwYSkew(&self) -> f32 {
      // Java: return this.ySkew;
      self.ySkew
    }

    // Java: int getShdwAlgn()
    pub fn getShdwAlgn(&self) -> i32 {
      // Java: return this.algn;
      self.algn
    }

    // Java: ShadowType getShdwType()
    pub fn getShdwType(&self) -> Option<ShadowType> {
      // Java: return this.mShdwType;
      self.mShdwType
    }

    // Java: int getPrstShdwType()
    pub fn getPrstShdwType(&self) -> i32 {
      // Java: return this.mPrstShdwType;
      self.mPrstShdwType
    }

    // Java: void setShadowColor(int mShadowColor)
    pub fn setShadowColor(&mut self, mShadowColor: i32) {
      // Java: this.mShadowColor = mShadowColor;
      self.mShadowColor = mShadowColor;
    }

    // Java: void setShadowAlpha(int mShadowAlpha)
    pub fn setShadowAlpha(&mut self, mShadowAlpha: i32) {
      // Java: this.mShadowAlpha = (float)mShadowAlpha;
      self.mShadowAlpha = mShadowAlpha as f32;
    }

    // Java: void setShadowDis(float mShadowDis)
    pub fn setShadowDis(&mut self, mShadowDis: f32) {
      // Java: this.mShadowDis = mShadowDis;
      self.mShadowDis = mShadowDis;
    }

    // Java: void setShadowRad(float mShadowRad)
    pub fn setShadowRad(&mut self, mShadowRad: f32) {
      // Java: this.mShadowRad = mShadowRad;
      self.mShadowRad = mShadowRad;
    }

    // Java: void setShadowAngle(float mShadowAngle)
    pub fn setShadowAngle(&mut self, mShadowAngle: f32) {
      // Java: this.mShadowAngle = mShadowAngle;
      self.mShadowAngle = mShadowAngle;
    }

    // Java: void setXPct(float xPct)
    pub fn setXPct(&mut self, xPct: f32) {
      // Java: this.xPct = xPct;
      self.xPct = xPct;
    }

    // Java: void setYPct(float yPct)
    pub fn setYPct(&mut self, yPct: f32) {
      // Java: this.yPct = yPct;
      self.yPct = yPct;
    }

    // Java: void setXSkew(float xSkew)
    pub fn setXSkew(&mut self, xSkew: f32) {
      // Java: this.xSkew = xSkew;
      self.xSkew = xSkew;
    }

    // Java: void setYSkew(float ySkew)
    pub fn setYSkew(&mut self, ySkew: f32) {
      // Java: this.ySkew = ySkew;
      self.ySkew = ySkew;
    }

    // Java: void setAlgn(int algn)
    pub fn setAlgn(&mut self, algn: i32) {
      // Java: this.algn = algn;
      self.algn = algn;
    }

    // Java: void setShdwType(ShadowType mShdwType)
    pub fn setShdwType(&mut self, mShdwType: ShadowType) {
      // Java: this.mShdwType = mShdwType;
      self.mShdwType = Some(mShdwType);
    }

    // Java: float getShdwOffsetX()
    pub fn getShdwOffsetX(&self) -> f32 {
      // Java: return this.mShdwOffsetX;
      self.mShdwOffsetX
    }

    // Java: void setShdwOffsetX(float mShdwOffsetX)
    pub fn setShdwOffsetX(&mut self, mShdwOffsetX: f32) {
      // Java: this.mShdwOffsetX = mShdwOffsetX;
      self.mShdwOffsetX = mShdwOffsetX;
    }

    // Java: float getShdwOffsetY()
    pub fn getShdwOffsetY(&self) -> f32 {
      // Java: return this.mShdwOffsetY;
      self.mShdwOffsetY
    }

    // Java: void setShdwOffsetY(float mShdwOffsetY)
    pub fn setShdwOffsetY(&mut self, mShdwOffsetY: f32) {
      // Java: this.mShdwOffsetY = mShdwOffsetY;
      self.mShdwOffsetY = mShdwOffsetY;
    }

    // Java: boolean getIsDoc()
    pub fn getIsDoc(&self) -> bool {
      // Java: return this.mIsDoc;
      self.mIsDoc
    }

    // Java: PointF getOrigin()
    pub fn getOrigin(&self) -> Option<&PointF> {
      // Java: return this.mOrigin;
      self.mOrigin.as_ref()
    }

    // Java: boolean equals(TextShadow txtShadow)
    pub fn equals(&self, txtShadow: Option<&TextShadow>) -> bool {
      // Java: if (txtShadow == null)
      let txtShadow = match txtShadow {
          None =>
      {
         // Java: return false;
         return false;
      }
      // Java: else
          Some(t) => t,
      };
      {
         // Java: return this.mShadowColor == txtShadow.mShadowColor && this.mShadowAlpha == txtShadow.mShadowAlpha && this.mShadowDis == txtShadow.mShadowDis && this.mShadowRad == txtShadow.mShadowRad && this.mShadowAngle == txtShadow.mShadowAngle && this.xPct == txtShadow.xPct && this.yPct == txtShadow.yPct && this.xSkew == txtShadow.xSkew && this.ySkew == txtShadow.ySkew && this.algn == txtShadow.algn && (this.mShdwType == null && txtShadow.mShdwType == null || this.mShdwType != null && this.mShdwType.equals(txtShadow.mShdwType)) && this.mPrstShdwType == txtShadow.mPrstShdwType;
         self.mShadowColor == txtShadow.mShadowColor
             && self.mShadowAlpha == txtShadow.mShadowAlpha
             && self.mShadowDis == txtShadow.mShadowDis
             && self.mShadowRad == txtShadow.mShadowRad
             && self.mShadowAngle == txtShadow.mShadowAngle
             && self.xPct == txtShadow.xPct
             && self.yPct == txtShadow.yPct
             && self.xSkew == txtShadow.xSkew
             && self.ySkew == txtShadow.ySkew
             && self.algn == txtShadow.algn
             && self.mShdwType == txtShadow.mShdwType
             && self.mPrstShdwType == txtShadow.mPrstShdwType
      }
    }

}

// Java: public enum ShadowType
impl ShadowType {
    // Java: ShadowType()
}

// Java: public enum ShadowAlgn
impl ShadowAlgn {
    // Java: ShadowAlgn()
}
