// Java: import android.graphics.PointF

#[derive(Clone)]
pub struct PointF { pub x: f32, pub y: f32 }
impl PointF {
    pub fn new() -> Self { PointF { x: 0.0, y: 0.0 } }
}

// Java: public class ShadowRenderData
#[derive(Clone)]
pub struct ShadowRenderData {
    // Java field: PointF mStart
    pub(crate) mStart: PointF,
    // Java field: PointF mOffset
    pub(crate) mOffset: PointF,
    // Java field: PointF mDistance
    pub(crate) mDistance: PointF,
    // Java field: PointF mBeforeTrans
    pub(crate) mBeforeTrans: PointF,
    // Java field: PointF mAfterTrans
    pub(crate) mAfterTrans: PointF,
    // Java field: PointF mScale
    pub(crate) mScale: PointF,
    // Java field: PointF mSkew
    pub(crate) mSkew: PointF,
    // Java field: float mLineHeight
    pub(crate) mLineHeight: f32,
    // Java field: float mLineBase
    pub(crate) mLineBase: f32,
}

// Java: public class ShadowRenderData
impl ShadowRenderData {
    // Java: ShadowRenderData()
    pub fn new() -> Self {
      // Java: super();
      ShadowRenderData {
          mStart: PointF::new(),
          mOffset: PointF::new(),
          mDistance: PointF::new(),
          mBeforeTrans: PointF::new(),
          mAfterTrans: PointF::new(),
          mScale: PointF::new(),
          mSkew: PointF::new(),
          mLineHeight: 0.0,
          mLineBase: 0.0,
      }
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

    // Java: float getLineBase()
    pub fn getLineBase(&self) -> f32 {
      // Java: return this.mLineBase;
      self.mLineBase
    }

    // Java: void setLineBase(float mLineBase)
    pub fn setLineBase(&mut self, mLineBase: f32) {
      // Java: this.mLineBase = mLineBase;
      self.mLineBase = mLineBase;
    }

    // Java: float getLineHeight()
    pub fn getLineHeight(&self) -> f32 {
      // Java: return this.mLineHeight;
      self.mLineHeight
    }

    // Java: void setLineHeight(float mLineHeight)
    pub fn setLineHeight(&mut self, mLineHeight: f32) {
      // Java: this.mLineHeight = mLineHeight;
      self.mLineHeight = mLineHeight;
    }

    // Java: PointF getStart()
    pub fn getStart(&self) -> &PointF {
      // Java: return this.mStart;
      &self.mStart
    }

    // Java: void setStart(PointF mStart)
    pub fn setStart(&mut self, mStart: PointF) {
      // Java: this.mStart = mStart;
      self.mStart = mStart;
    }

    // Java: PointF getOffset()
    pub fn getOffset(&self) -> &PointF {
      // Java: return this.mOffset;
      &self.mOffset
    }

    // Java: void setOffset(PointF mOffset)
    pub fn setOffset(&mut self, mOffset: PointF) {
      // Java: this.mOffset = mOffset;
      self.mOffset = mOffset;
    }

    // Java: PointF getDistance()
    pub fn getDistance(&self) -> &PointF {
      // Java: return this.mDistance;
      &self.mDistance
    }

    // Java: void setDistance(PointF mDistance)
    pub fn setDistance(&mut self, mDistance: PointF) {
      // Java: this.mDistance = mDistance;
      self.mDistance = mDistance;
    }

    // Java: PointF getBeforeTrans()
    pub fn getBeforeTrans(&self) -> &PointF {
      // Java: return this.mBeforeTrans;
      &self.mBeforeTrans
    }

    // Java: void setBeforeTrans(PointF mBeforeTrans)
    pub fn setBeforeTrans(&mut self, mBeforeTrans: PointF) {
      // Java: this.mBeforeTrans = mBeforeTrans;
      self.mBeforeTrans = mBeforeTrans;
    }

    // Java: PointF getAfterTrans()
    pub fn getAfterTrans(&self) -> &PointF {
      // Java: return this.mAfterTrans;
      &self.mAfterTrans
    }

    // Java: void setAfterTrans(PointF mAfterTrans)
    pub fn setAfterTrans(&mut self, mAfterTrans: PointF) {
      // Java: this.mAfterTrans = mAfterTrans;
      self.mAfterTrans = mAfterTrans;
    }

}
