// Java: import android.graphics.PointF
// Java: import cn.wps.text.layout.data.TextShadow
use crate::cn::wps::text::layout::data::TextShadow::TextShadow;

#[derive(Clone)]
pub struct PointF { pub x: f32, pub y: f32 }

// Java: public class ShadowTypoData
pub struct ShadowTypoData {
    // Java field: PointF mShadowScale
    pub(crate) mShadowScale: PointF,
    // Java field: PointF mShadowSkew
    pub(crate) mShadowSkew: PointF,
    // Java field: float mShadowAngle
    pub(crate) mShadowAngle: f32,
    // Java field: float mShadowDistance
    pub(crate) mShadowDistance: f32,
    // Java field: int mShadowAlign
    pub(crate) mShadowAlign: i32,
}

// Java: public class ShadowTypoData
impl ShadowTypoData {
    // Java: ShadowTypoData()
    pub fn new() -> Self {
      // Java: super();
      ShadowTypoData {
          mShadowScale: PointF { x: 1.0, y: 1.0 },
          mShadowSkew: PointF { x: 0.0, y: 0.0 },
          mShadowAngle: 0.0,
          mShadowDistance: 0.0,
          mShadowAlign: 0,
      }
    }

    // Java: void setDefault()
    pub fn setDefault(&mut self) {
      // Java: this.mShadowScale = new PointF(1.0F, 1.0F);
      self.mShadowScale = PointF { x: 1.0, y: 1.0 };
      // Java: this.mShadowSkew = new PointF(0.0F, 0.0F);
      self.mShadowSkew = PointF { x: 0.0, y: 0.0 };
      // Java: this.mShadowAngle = 0.0F;
      self.mShadowAngle = 0.0;
      // Java: this.mShadowDistance = 0.0F;
      self.mShadowDistance = 0.0;
      // Java: this.mShadowAlign = TextShadow.ShadowAlgn.B.ordinal();
      self.mShadowAlign = 0;
    }

}
