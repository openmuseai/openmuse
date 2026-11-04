// Java: import cn.wps.base.converter.UnitConverter
use crate::cn::wps::base::converter::UnitConverter::UnitConverter;

// java  class PixelConverter extends UnitConverter
pub struct PixelConverter {
    // java UnitConverter super_
    pub(crate) super_: UnitConverter,
}

// java  class PixelConverter extends UnitConverter
impl PixelConverter {
    // java  float fromEmuX(float emu)
    pub fn fromEmuX(&self, emu: f32) -> f32 {
      // java return emu * this.mMetrics.xdpi / 914400.0F;
      return emu * self.mMetrics.xdpi / 914400.0;
    }

    // java  float fromEmuY(float emu)
    pub fn fromEmuY(&self, emu: f32) -> f32 {
      // java return emu * this.mMetrics.ydpi / 914400.0F;
      return emu * self.mMetrics.ydpi / 914400.0;
    }

    // java  float fromPointX(float pt)
    pub fn fromPointX(&self, pt: f32) -> f32 {
      // java return pt * this.mMetrics.xdpi / 72.0F;
      return pt * self.mMetrics.xdpi / 72.0;
    }

    // java  float fromPointY(float pt)
    pub fn fromPointY(&self, pt: f32) -> f32 {
      // java return pt * this.mMetrics.ydpi / 72.0F;
      return pt * self.mMetrics.ydpi / 72.0;
    }

    // java  float fromDipX(float dip)
    pub fn fromDipX(&self, dip: f32) -> f32 {
      // java return dip * this.mMetrics.density;
      return dip * self.mMetrics.density;
    }

    // java  float fromDipY(float dip)
    pub fn fromDipY(&self, dip: f32) -> f32 {
      // java return dip * this.mMetrics.density;
      return dip * self.mMetrics.density;
    }

    // java  float fromTwipX(float twip)
    pub fn fromTwipX(&self, twip: f32) -> f32 {
      // java return this.fromPointX(twip / 20.0F);
      return self.fromPointX(twip / 20.0);
    }

    // java  float fromTwipY(float twip)
    pub fn fromTwipY(&self, twip: f32) -> f32 {
      // java return this.fromPointY(twip / 20.0F);
      return self.fromPointY(twip / 20.0);
    }

}

impl std::ops::Deref for PixelConverter {
    type Target = UnitConverter;

    fn deref(&self) -> &Self::Target {
        &self.super_
    }
}

impl std::ops::DerefMut for PixelConverter {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.super_
    }
}
