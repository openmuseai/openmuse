#![allow(non_snake_case, non_upper_case_globals, dead_code, unused_variables, unused_imports, unused_mut)]

// Java: import cn.wps.core.runtime.Platform
use util_adapter::cn::wps::core::runtime::Platform::Platform;
// Java: import cn.wps.graphics.KDisplayMetrics
use util::cn::wps::graphics::KDisplayMetrics::KDisplayMetrics;

// Java field: float MIN_DPI
pub(crate) const MIN_DPI: f32 = 64.0;
// Java field: float DEFAULT_DPI
pub(crate) const DEFAULT_DPI: f32 = 96.0;
// Java field: float POINT_TWIP
pub(crate) const POINT_TWIP: f32 = 20.0;
// Java field: float INCH_POINTS
pub(crate) const INCH_POINTS: f32 = 72.0;
// Java field: float INCH_TWIPS
pub(crate) const INCH_TWIPS: f32 = 1440.0;
// Java field: float POINT_EMU
pub(crate) const POINT_EMU: f32 = 12700.0;
// Java field: float TWIP_EMU
pub(crate) const TWIP_EMU: f32 = 635.0;
// Java field: float POINT_INCHS
pub(crate) const POINT_INCHS: f32 = 0.013888889;
// Java field: float INCH_EMU
pub(crate) const INCH_EMU: f32 = 914400.0;

// Java: public class UnitConverter
pub struct UnitConverter {
    // Java field: KDisplayMetrics mMetrics
    pub(crate) mMetrics: KDisplayMetrics,
}

// Java: public class UnitConverter
impl UnitConverter {
    // Java: UnitConverter()
    pub fn new() -> Self {
      // Java: this(Platform.getDisplayMetrics());
      let default_metrics = KDisplayMetrics::new();
      if let Some(metrics) = Platform::getDisplayMetrics() {
          Self::new_with_metrics(metrics.as_ref())
      } else {
          Self::new_with_metrics(&default_metrics)
      }
    }

    pub fn UnitConverter() -> Self {
      Self::new()
    }

    pub fn UnitConverter__metrics(metrics: &KDisplayMetrics) -> Self {
      Self::new_with_metrics(metrics)
    }

    // Java: UnitConverter(KDisplayMetrics metrics)
    pub fn new_with_metrics(metrics: &KDisplayMetrics) -> Self {
      // Java: this.mMetrics = new KDisplayMetrics();
      let mut mMetrics = KDisplayMetrics::new();
      // Java: this.mMetrics.setTo(metrics);
      mMetrics.setTo(metrics);
      // Java: float xdpi = this.mMetrics.xdpi > 64.0F ? this.mMetrics.xdpi : 96.0F;
      let mut xdpi: f32 = if mMetrics.xdpi > 64.0 { mMetrics.xdpi } else { 96.0 };
      // Java: float ydpi = this.mMetrics.ydpi > 64.0F ? this.mMetrics.ydpi : 96.0F;
      let mut ydpi: f32 = if mMetrics.ydpi > 64.0 { mMetrics.ydpi } else { 96.0 };
      // Java: if (Math.abs(xdpi - ydpi) / xdpi >= 0.2F)
      if (xdpi - ydpi).abs() / xdpi >= 0.2 {
         // Java: ydpi = xdpi;
         ydpi = xdpi;
      }
      // Java: this.mMetrics.xdpi = xdpi;
      mMetrics.xdpi = xdpi;
      // Java: this.mMetrics.ydpi = ydpi;
      mMetrics.ydpi = ydpi;
      UnitConverter { mMetrics }
    }

    // Java: float fromEmuX(float emu)
    pub fn fromEmuX(&self, emu: f32) -> f32 {
      // Java: return emu;
      emu
    }

    // Java: float fromEmuY(float emu)
    pub fn fromEmuY(&self, emu: f32) -> f32 {
      // Java: return emu;
      emu
    }

    // Java: float fromPixelX(float px)
    pub fn fromPixelX(&self, px: f32) -> f32 {
      // Java: return px;
      px
    }

    // Java: float fromPixelY(float px)
    pub fn fromPixelY(&self, px: f32) -> f32 {
      // Java: return px;
      px
    }

    // Java: float fromPointX(float pt)
    pub fn fromPointX(&self, pt: f32) -> f32 {
      // Java: return pt;
      pt
    }

    // Java: float fromPointY(float pt)
    pub fn fromPointY(&self, pt: f32) -> f32 {
      // Java: return pt;
      pt
    }

    // Java: float fromTwipX(float twip)
    pub fn fromTwipX(&self, twip: f32) -> f32 {
      // Java: return twip;
      twip
    }

    // Java: float fromTwipY(float twip)
    pub fn fromTwipY(&self, twip: f32) -> f32 {
      // Java: return twip;
      twip
    }

    // Java: float fromDipX(float dip)
    pub fn fromDipX(&self, dip: f32) -> f32 {
      // Java: return dip;
      dip
    }

    // Java: float fromDipY(float dip)
    pub fn fromDipY(&self, dip: f32) -> f32 {
      // Java: return dip;
      dip
    }

    // Java: float dipToPixel(float dip)
    pub fn dipToPixel(&self, dip: f32) -> f32 {
      // Java: return dip * this.mMetrics.density;
      dip * self.mMetrics.density
    }

    // Java: float pixelToDip(float pixel)
    pub fn pixelToDip(&self, pixel: f32) -> f32 {
      // Java: return pixel / this.mMetrics.density;
      pixel / self.mMetrics.density
    }

}
