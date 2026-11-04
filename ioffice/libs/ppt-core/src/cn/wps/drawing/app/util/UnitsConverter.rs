use super::PrecisionUtil::PrecisionUtil;

// Java field: int INCH_EMU
pub(crate) const INCH_EMU: i32 = 914400;
// Java field: double INCH_CM
pub(crate) const INCH_CM: f64 = 2.54;
// Java field: int PT_EMU
pub(crate) const PT_EMU: i32 = 12700;
// Java field: int DEGREE_ANGLE
pub(crate) const DEGREE_ANGLE: i32 = 60000;
// Java field: double PERCENT
pub(crate) const PERCENT: f64 = 100.0;

// Java: public class UnitsConverter
pub struct UnitsConverter {
}

// Java: public class UnitsConverter
impl UnitsConverter {
    // Java: static double decimals2percent(double decimals)
    pub fn decimals2percent(decimals: f64) -> f64 {
      // Java: return PrecisionUtil.mul(decimals, (double)100.0F);
      PrecisionUtil::mul(decimals, 100.0)
    }

    // Java: static double percent2decimals(double percent)
    pub fn percent2decimals(percent: f64) -> f64 {
      // Java: return PrecisionUtil.div(percent, (double)100.0F);
      PrecisionUtil::div(percent, 100.0)
    }

    // Java: static double emu2inch(double emu)
    pub fn emu2inch(emu: f64) -> f64 {
      // Java: return emu / (double)914400.0F;
      emu / 914400.0
    }

    // Java: static double inch2emu(double inch)
    pub fn inch2emu(inch: f64) -> f64 {
      // Java: return inch * (double)914400.0F;
      inch * 914400.0
    }

    // Java: static double emu2cm(double emu)
    pub fn emu2cm(emu: f64) -> f64 {
      // Java: return emu2inch(emu) * 2.54;
      Self::emu2inch(emu) * 2.54
    }

    // Java: static double cm2emu(double cm)
    pub fn cm2emu(cm: f64) -> f64 {
      // Java: return inch2emu(cm / 2.54);
      Self::inch2emu(cm / 2.54)
    }

    // Java: static double emu2pt(double emu)
    pub fn emu2pt(emu: f64) -> f64 {
      // Java: return PrecisionUtil.div(emu, (double)12700.0F, 2);
      // LLM Rust Renamed: PrecisionUtil.div(3-arg) -> PrecisionUtil::div_with_scale
      PrecisionUtil::div_with_scale(emu, 12700.0, 2)
    }

    // Java: static double emu2pt(double emu, int precision)
    pub fn emu2pt_with_precision(emu: f64, precision: i32) -> f64 {
      // Java: return PrecisionUtil.div(emu, (double)12700.0F, precision);
      // LLM Rust Renamed: PrecisionUtil.div(3-arg) -> PrecisionUtil::div_with_scale
      PrecisionUtil::div_with_scale(emu, 12700.0, precision)
    }

    // Java: static double pt2emu(double pt)
    pub fn pt2emu(pt: f64) -> f64 {
      // Java: return pt * (double)12700.0F;
      pt * 12700.0
    }

    // Java: static int degree2Angle(double degree)
    pub fn degree2Angle(degree: f64) -> i32 {
      // Java: return (int)(degree * (double)60000.0F);
      (degree * 60000.0) as i32
    }

    // Java: static double angle2Degree(double angle)
    pub fn angle2Degree(angle: f64) -> f64 {
      // Java: return angle / (double)60000.0F;
      angle / 60000.0
    }

    // Java: static double angle2Degree(double angle, int precision)
    pub fn angle2Degree_with_precision(angle: f64, precision: i32) -> f64 {
      // Java: return PrecisionUtil.div(angle, (double)60000.0F, precision);
      // LLM Rust Renamed: PrecisionUtil.div(3-arg) -> PrecisionUtil::div_with_scale
      PrecisionUtil::div_with_scale(angle, 60000.0, precision)
    }

}
