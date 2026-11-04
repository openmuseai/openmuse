use super::Normalizer::Normalizer;
use super::RangeNormalizer::RangeNormalizer;

// Java field: float LOW_PRESSURE
pub(crate) const LOW_PRESSURE: f32 = 0.1;
// Java field: float HIGH_PRESSURE
pub(crate) const HIGH_PRESSURE: f32 = 0.8;
// Java field: float LOW_SPEED
pub(crate) const LOW_SPEED: f32 = 300.0;
// Java field: float HIGH_SPEED
pub(crate) const HIGH_SPEED: f32 = 2000.0;

// Java: public class StrokeNormalizer
pub struct StrokeNormalizer {
    // Java field: Normalizer mPressureNormalizer
    mPressureNormalizer: Option<RangeNormalizer>,
    // Java field: Normalizer mSpeedNormalizer
    mSpeedNormalizer: Option<RangeNormalizer>,
}

// Java: public class StrokeNormalizer
impl StrokeNormalizer {
    // Java: StrokeNormalizer()
    pub fn StrokeNormalizer() -> Self {
      // Java: super();
      Self {
          mPressureNormalizer: None,
          mSpeedNormalizer: None,
      }
    }

    // Java: float getAdjustedWidth(boolean isNormalizeWithSpeed, float pressure, float speed)
    pub fn getAdjustedWidth(&mut self, is_normalize_with_speed: bool, pressure: f32, speed: f32) -> f32 {
      // Java: return isNormalizeWithSpeed ? this.getWidthBySpeed(speed) : this.getWidthByPressure(pressure);
      if is_normalize_with_speed {
          self.getWidthBySpeed(speed)
      } else {
          self.getWidthByPressure(pressure)
      }
    }

    // Java: float getWidthBySpeed(float speed)
    pub fn getWidthBySpeed(&mut self, speed: f32) -> f32 {
      // Java: if (null == this.mSpeedNormalizer)
      if self.mSpeedNormalizer.is_none()
      {
         // Java: this.mSpeedNormalizer = new RangeNormalizer(300.0F, 2000.0F, true);
         self.mSpeedNormalizer = Some(RangeNormalizer::RangeNormalizer(300.0, 2000.0, true));
      }
      // Java: return this.mSpeedNormalizer.normalize(speed);
      self.mSpeedNormalizer.as_ref().unwrap().normalize(speed)
    }

    // Java: float getWidthByPressure(float pressure)
    pub fn getWidthByPressure(&mut self, pressure: f32) -> f32 {
      // Java: if (null == this.mPressureNormalizer)
      if self.mPressureNormalizer.is_none()
      {
         // Java: this.mPressureNormalizer = new RangeNormalizer(0.1F, 0.8F, false);
         self.mPressureNormalizer = Some(RangeNormalizer::RangeNormalizer(0.1, 0.8, false));
      }
      // Java: return this.mPressureNormalizer.normalize(pressure);
      self.mPressureNormalizer.as_ref().unwrap().normalize(pressure)
    }

}
