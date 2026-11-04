// Java: import cn.wps.base.converter.UnitConverter
use crate::cn::wps::base::converter::UnitConverter::UnitConverter;

// java  class EmuConverter extends UnitConverter
pub struct EmuConverter {
    // java UnitConverter super_
    pub(crate) super_: UnitConverter,
}

// java  class EmuConverter extends UnitConverter
impl EmuConverter {
    // java  float fromPixelX(float px)
    pub fn fromPixelX(&self, px: f32) -> f32 {
      // java return px * 914400.0F / this.mMetrics.xdpi;
      return px * 914400.0 / self.mMetrics.xdpi;
    }

    // java  float fromPixelY(float px)
    pub fn fromPixelY(&self, px: f32) -> f32 {
      // java return px * 914400.0F / this.mMetrics.ydpi;
      return px * 914400.0 / self.mMetrics.ydpi;
    }

    // java  float fromPointX(float pt)
    pub fn fromPointX(&self, pt: f32) -> f32 {
      // java return pt * 12700.0F;
      return pt * 12700.0;
    }

    // java  float fromPointY(float pt)
    pub fn fromPointY(&self, pt: f32) -> f32 {
      // java return pt * 12700.0F;
      return pt * 12700.0;
    }

    // java  float fromDipX(float dip)
    pub fn fromDipX(&self, dip: f32) -> f32 {
      // java return this.fromPixelX(dip * this.mMetrics.density);
      return self.fromPixelX(dip * self.mMetrics.density);
    }

    // java  float fromDipY(float dip)
    pub fn fromDipY(&self, dip: f32) -> f32 {
      // java return this.fromPixelY(dip * this.mMetrics.density);
      return self.fromPixelY(dip * self.mMetrics.density);
    }

    // java  float fromTwipX(float twip)
    pub fn fromTwipX(&self, twip: f32) -> f32 {
      // java return twip * 635.0F;
      return twip * 635.0;
    }

    // java  float fromTwipY(float twip)
    pub fn fromTwipY(&self, twip: f32) -> f32 {
      // java return twip * 635.0F;
      return twip * 635.0;
    }

}

impl std::ops::Deref for EmuConverter {
    type Target = UnitConverter;

    fn deref(&self) -> &Self::Target {
        &self.super_
    }
}

impl std::ops::DerefMut for EmuConverter {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.super_
    }
}
