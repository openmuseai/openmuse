// Java: import cn.wps.base.converter.UnitConverter
use crate::cn::wps::base::converter::UnitConverter::UnitConverter;

// java  class TwipConverter extends UnitConverter
pub struct TwipConverter {
    // java UnitConverter super_
    pub(crate) super_: UnitConverter,
}

// java  class TwipConverter extends UnitConverter
impl TwipConverter {
    // java  float fromEmuX(float emu)
    pub fn fromEmuX(&self, emu: f32) -> f32 {
      // java return emu / 635.0F;
      return emu / 635.0;
    }

    // java  float fromEmuY(float emu)
    pub fn fromEmuY(&self, emu: f32) -> f32 {
      // java return emu / 635.0F;
      return emu / 635.0;
    }

    // java  float fromPixelX(float px)
    pub fn fromPixelX(&self, px: f32) -> f32 {
      // java return px / this.mMetrics.xdpi * 1440.0F;
      return px / self.mMetrics.xdpi * 1440.0;
    }

    // java  float fromPixelY(float px)
    pub fn fromPixelY(&self, px: f32) -> f32 {
      // java return px / this.mMetrics.ydpi * 1440.0F;
      return px / self.mMetrics.ydpi * 1440.0;
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

    // java  float fromPointX(float pt)
    pub fn fromPointX(&self, pt: f32) -> f32 {
      // java return pt * 20.0F;
      return pt * 20.0;
    }

    // java  float fromPointY(float pt)
    pub fn fromPointY(&self, pt: f32) -> f32 {
      // java return pt * 20.0F;
      return pt * 20.0;
    }

}

impl std::ops::Deref for TwipConverter {
    type Target = UnitConverter;

    fn deref(&self) -> &Self::Target {
        &self.super_
    }
}

impl std::ops::DerefMut for TwipConverter {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.super_
    }
}
