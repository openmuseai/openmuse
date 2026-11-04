// Java: import android.graphics.PointF
// Java: import cn.wps.text.layout.data.TextShadow
use crate::cn::wps::text::layout::data::TextShadow::TextShadow;

#[derive(Clone)]
pub struct PointF { pub x: f32, pub y: f32 }
impl PointF {
    pub fn new() -> Self { PointF { x: 0.0, y: 0.0 } }
}

// Java: public class ShadowImageData
pub struct ShadowImageData {
    // Java field: TextShadow mShadow
    pub(crate) mShadow: Option<TextShadow>,
    // Java field: boolean isDoubleShadow
    pub(crate) isDoubleShadow: bool,
    // Java field: PointF mDoubleBeforeTrans
    pub(crate) mDoubleBeforeTrans: PointF,
    // Java field: PointF mDoubleAfterTrans
    pub(crate) mDoubleAfterTrans: PointF,
    // Java field: PointF mFirstBeforeTrans
    pub(crate) mFirstBeforeTrans: PointF,
    // Java field: PointF mFirstAfterTrans
    pub(crate) mFirstAfterTrans: PointF,
    // Java field: PointF mScale
    pub(crate) mScale: PointF,
    // Java field: PointF mSkew
    pub(crate) mSkew: PointF,
}

// Java: public class ShadowImageData
impl ShadowImageData {
    // Java: ShadowImageData()
    pub fn new() -> Self {
      // Java: super();
      ShadowImageData {
          mShadow: None, isDoubleShadow: false,
          mDoubleBeforeTrans: PointF::new(), mDoubleAfterTrans: PointF::new(),
          mFirstBeforeTrans: PointF::new(), mFirstAfterTrans: PointF::new(),
          mScale: PointF::new(), mSkew: PointF::new(),
      }
    }

    // Java: TextShadow getShadow()
    pub fn getShadow(&self) -> Option<&TextShadow> {
      // Java: return this.mShadow;
      self.mShadow.as_ref()
    }

    // Java: void setShadow(TextShadow shadow)
    pub fn setShadow(&mut self, shadow: TextShadow) {
      // Java: this.mShadow = shadow;
      self.mShadow = Some(shadow);
    }

    // Java: boolean isDoubleShadow()
    pub fn isDoubleShadow(&self) -> bool {
      // Java: return this.isDoubleShadow;
      self.isDoubleShadow
    }

    // Java: void setDoubleShadow(boolean doubleShadow)
    pub fn setDoubleShadow(&mut self, doubleShadow: bool) {
      // Java: this.isDoubleShadow = doubleShadow;
      self.isDoubleShadow = doubleShadow;
    }

    // Java: PointF getDoubleBeforeTrans()
    pub fn getDoubleBeforeTrans(&self) -> &PointF {
      // Java: return this.mDoubleBeforeTrans;
      &self.mDoubleBeforeTrans
    }

    // Java: void setDoubleBeforeTrans(PointF mDoubleBeforeTrans)
    pub fn setDoubleBeforeTrans(&mut self, mDoubleBeforeTrans: PointF) {
      // Java: this.mDoubleBeforeTrans = mDoubleBeforeTrans;
      self.mDoubleBeforeTrans = mDoubleBeforeTrans;
    }

    // Java: PointF getDoubleAfterTrans()
    pub fn getDoubleAfterTrans(&self) -> &PointF {
      // Java: return this.mDoubleAfterTrans;
      &self.mDoubleAfterTrans
    }

    // Java: void setDoubleAfterTrans(PointF mDoubleAfterTrans)
    pub fn setDoubleAfterTrans(&mut self, mDoubleAfterTrans: PointF) {
      // Java: this.mDoubleAfterTrans = mDoubleAfterTrans;
      self.mDoubleAfterTrans = mDoubleAfterTrans;
    }

    // Java: PointF getFirstBeforeTrans()
    pub fn getFirstBeforeTrans(&self) -> &PointF {
      // Java: return this.mFirstBeforeTrans;
      &self.mFirstBeforeTrans
    }

    // Java: void setFirstBeforeTrans(PointF mFirstBeforeTrans)
    pub fn setFirstBeforeTrans(&mut self, mFirstBeforeTrans: PointF) {
      // Java: this.mFirstBeforeTrans = mFirstBeforeTrans;
      self.mFirstBeforeTrans = mFirstBeforeTrans;
    }

    // Java: PointF getFirstAfterTrans()
    pub fn getFirstAfterTrans(&self) -> &PointF {
      // Java: return this.mFirstAfterTrans;
      &self.mFirstAfterTrans
    }

    // Java: void setFirstAfterTrans(PointF mFirstAfterTrans)
    pub fn setFirstAfterTrans(&mut self, mFirstAfterTrans: PointF) {
      // Java: this.mFirstAfterTrans = mFirstAfterTrans;
      self.mFirstAfterTrans = mFirstAfterTrans;
    }

    // Java: PointF getScale()
    pub fn getScale(&self) -> &PointF {
      // Java: return this.mScale;
      &self.mScale
    }

    // Java: void setScale(PointF mScale)
    pub fn setScale(&mut self, mScale: PointF) {
      // Java: this.mScale = mScale;
      self.mScale = mScale;
    }

    // Java: PointF getSkew()
    pub fn getSkew(&self) -> &PointF {
      // Java: return this.mSkew;
      &self.mSkew
    }

    // Java: void setSkew(PointF mSkew)
    pub fn setSkew(&mut self, mSkew: PointF) {
      // Java: this.mSkew = mSkew;
      self.mSkew = mSkew;
    }

}
