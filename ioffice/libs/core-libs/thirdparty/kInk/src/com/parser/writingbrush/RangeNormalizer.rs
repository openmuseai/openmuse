use super::Normalizer::Normalizer;

// Java field: float MIN_NORM
pub(crate) const MIN_NORM: f32 = 0.1;

// Java: public class RangeNormalizer implements Normalizer
pub struct RangeNormalizer {
    // Java field: boolean mReverse
    mReverse: bool,
    // Java field: float mMin
    mMin: f32,
    // Java field: float mMax
    mMax: f32,
}

// Java: public class RangeNormalizer implements Normalizer
impl RangeNormalizer {
    // Java: RangeNormalizer(float min, float max, boolean reverse)
    pub fn RangeNormalizer(min: f32, max: f32, reverse: bool) -> Self {
      // Java: super();
      // Java: this.mMin = min;
      // Java: this.mMax = max;
      // Java: this.mReverse = reverse;
      Self {
          mReverse: reverse,
          mMin: min,
          mMax: max,
      }
    }

    // Java: float normalize(float value)
    pub fn normalize(&self, value: f32) -> f32 {
      // Java: float norm = (value - this.mMin) / (this.mMax - this.mMin);
      let mut norm = (value - self.mMin) / (self.mMax - self.mMin);
      // Java: if (this.mReverse)
      if self.mReverse
      {
         // Java: norm = 1.0F - norm;
         norm = 1.0 - norm;
      }
      // Java: if (norm > 1.0F)
      if norm > 1.0
      {
         // Java: return (float)Math.sqrt((double)norm);
         return (norm as f64).sqrt() as f32;
      }
      // Java: else
      else
      {
         // Java: if (norm < 0.1F)
         if norm < MIN_NORM
         {
            // Java: norm = 0.1F;
            norm = MIN_NORM;
         }
         // Java: return norm * norm;
         norm * norm
      }
    }

}

impl Normalizer for RangeNormalizer {
    fn normalize(&self, var1: f32) -> f32 {
        RangeNormalizer::normalize(self, var1)
    }
}
