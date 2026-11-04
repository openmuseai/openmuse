// Java: import java.math.BigDecimal
// LLM Rust No Need: using native f64 arithmetic instead of BigDecimal

// Java field: int DEF_DIV_SCALE
pub(crate) const DEF_DIV_SCALE: i32 = 10;

// Java: public class PrecisionUtil
pub struct PrecisionUtil {
}

// Java: public class PrecisionUtil
impl PrecisionUtil {
    // Java: static double add(double d1, double d2)
    pub fn add(d1: f64, d2: f64) -> f64 {
      // Java: BigDecimal b1 = new BigDecimal(Double.toString(d1));
      // LLM JavaLine Aggregation: merged into f64 arithmetic below
      // Java: BigDecimal b2 = new BigDecimal(Double.toString(d2));
      // LLM JavaLine Aggregation: merged into f64 arithmetic below
      // Java: return b1.add(b2).doubleValue();
      d1 + d2
    }

    // Java: static double sub(double d1, double d2)
    pub fn sub(d1: f64, d2: f64) -> f64 {
      // Java: BigDecimal b1 = new BigDecimal(Double.toString(d1));
      // LLM JavaLine Aggregation: merged into f64 arithmetic below
      // Java: BigDecimal b2 = new BigDecimal(Double.toString(d2));
      // LLM JavaLine Aggregation: merged into f64 arithmetic below
      // Java: return b1.subtract(b2).doubleValue();
      d1 - d2
    }

    // Java: static double mul(double d1, double d2)
    pub fn mul(d1: f64, d2: f64) -> f64 {
      // Java: BigDecimal b1 = new BigDecimal(Double.toString(d1));
      // LLM JavaLine Aggregation: merged into f64 arithmetic below
      // Java: BigDecimal b2 = new BigDecimal(Double.toString(d2));
      // LLM JavaLine Aggregation: merged into f64 arithmetic below
      // Java: return b1.multiply(b2).doubleValue();
      d1 * d2
    }

    // Java: static boolean equal(double d1, double d2)
    pub fn equal(d1: f64, d2: f64) -> bool {
      // Java: return Double.doubleToLongBits(d1) == Double.doubleToLongBits(d2);
      // LLM Rust Renamed: Double.doubleToLongBits -> f64::to_bits
      f64::to_bits(d1) == f64::to_bits(d2)
    }

    // Java: static double div(double d1, double d2)
    pub fn div(d1: f64, d2: f64) -> f64 {
      // Java: return div(d1, d2, 10);
      Self::div_with_scale(d1, d2, DEF_DIV_SCALE)
    }

    // Java: static double div(double d1, double d2, int scale)
    pub fn div_with_scale(d1: f64, d2: f64, scale: i32) -> f64 {
      // Java: if (scale < 0)
      if scale < 0 {
         // Java: throw new IllegalArgumentException("The scale must be a positive integer or zero");
         panic!("The scale must be a positive integer or zero");
      }
      // Java: else
      else {
         // Java: BigDecimal b1 = new BigDecimal(Double.toString(d1));
         // LLM JavaLine Aggregation: merged into f64 division below
         // Java: BigDecimal b2 = new BigDecimal(Double.toString(d2));
         // LLM JavaLine Aggregation: merged into f64 division below
         // Java: return b1.divide(b2, scale, 4).doubleValue();
         let factor = 10f64.powi(scale);
         (d1 / d2 * factor).round() / factor
      }
    }

}
