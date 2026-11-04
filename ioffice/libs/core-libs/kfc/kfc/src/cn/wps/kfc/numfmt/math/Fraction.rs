// Java field: double DBL_MAX
pub(crate) const DBL_MAX: f64 = f64::MAX;
// Java field: int MAX_DENOMCOUNT
pub(crate) const MAX_DENOMCOUNT: i32 = 7;

// Java: public class Fraction
pub struct Fraction {
    // Java field: long mInteger
    pub(crate) mInteger: i64,
    // Java field: long m_numerator
    pub(crate) m_numerator: i64,
    // Java field: long m_denominator
    pub(crate) m_denominator: i64,
    // Java field: double m_fraction
    m_fraction: f64,
}

// Java: public class Fraction
impl Fraction {
    // Java: Fraction()
    pub fn new() -> Self {
      // Java: super();
      // Java: this.m_numerator = 1L;
      // Java: this.m_denominator = 1L;
      // Java: this.m_fraction = (double)1.0F;
      Fraction {
          mInteger: 0,
          m_numerator: 1,
          m_denominator: 1,
          m_fraction: 1.0,
      }
    }

    // Java: Fraction(long integer, long numerator, long denominator)
    pub fn Fraction_with_int(integer: i64, numerator: i64, denominator: i64) -> Self {
      // Java: this(numerator, denominator);
      let mut f = Self::Fraction_with_num_den(numerator, denominator);
      // Java: this.mInteger = integer;
      f.mInteger = integer;
      f
    }

    // Java: Fraction(long numerator, long denominator)
    pub fn Fraction_with_num_den(numerator: i64, denominator: i64) -> Self {
      // Java: super();
      let mut f = Fraction {
          mInteger: 0,
      // Java: this.m_numerator = numerator;
          m_numerator: numerator,
      // Java: this.m_denominator = denominator;
          m_denominator: denominator,
          m_fraction: 0.0,
      };
      // Java: this.caculate();
      f.caculate();
      f
    }

    // Java: void copy(Fraction src)
    pub fn copy(&mut self, src: &Fraction) {
      // Java: this.m_denominator = src.m_denominator;
      self.m_denominator = src.m_denominator;
      // Java: this.m_numerator = src.m_numerator;
      self.m_numerator = src.m_numerator;
      // Java: this.m_fraction = src.m_fraction;
      self.m_fraction = src.m_fraction;
    }

    // Java: double inc_denominator()
    pub fn inc_denominator(&mut self) -> f64 {
      // Java: ++this.m_denominator;
      self.m_denominator += 1;
      // Java: this.m_fraction = (double)this.m_numerator / (double)this.m_denominator;
      self.m_fraction = self.m_numerator as f64 / self.m_denominator as f64;
      // Java: return (double)this.m_denominator;
      self.m_denominator as f64
    }

    // Java: void increment()
    pub fn increment(&mut self) {
      // Java: ++this.m_numerator;
      self.m_numerator += 1;
      // Java: ++this.m_denominator;
      self.m_denominator += 1;
      // Java: this.m_fraction = (double)this.m_numerator / (double)this.m_denominator;
      self.m_fraction = self.m_numerator as f64 / self.m_denominator as f64;
    }

    // Java: void caculate()
    fn caculate(&mut self) {
        // Java: if (0L == this.m_denominator)
      if 0 == self.m_denominator {
         // Java: this.m_fraction = Double.MAX_VALUE;
         self.m_fraction = f64::MAX;
      }
        // Java: else if (0L == this.m_numerator)
      else if 0 == self.m_numerator {
         // Java: this.m_fraction = (double)0.0F;
         self.m_fraction = 0.0;
      }
        // Java: else
      else {
            // Java: if (this.m_numerator < 0L)
         if self.m_numerator < 0 {
            // Java: this.m_numerator = -this.m_numerator;
            self.m_numerator = -self.m_numerator;
         }
            // Java: if (this.m_denominator < 0L)
         if self.m_denominator < 0 {
            // Java: this.m_denominator = -this.m_denominator;
            self.m_denominator = -self.m_denominator;
         }
            // Java: if (this.m_numerator == this.m_denominator)
         if self.m_numerator == self.m_denominator {
            // Java: this.m_fraction = (double)1.0F;
            self.m_fraction = 1.0;
         }
            // Java: else
         else {
                // Java: if (this.m_numerator > this.m_denominator)
            if self.m_numerator > self.m_denominator {
               // Java: this.m_numerator -= this.m_denominator;
               self.m_numerator -= self.m_denominator;
            }
            // Java: this.m_fraction = (double)this.m_numerator / (double)this.m_denominator;
            self.m_fraction = self.m_numerator as f64 / self.m_denominator as f64;
         }
      }
    }

    // Java: static boolean calcFraction2(double value, int denominator, Fraction result)
    pub fn calcFraction2(value: f64, denominator: i32, result: &mut Fraction) -> bool {
      // Java: double absValue = Math.abs(value);
      let abs_value = value.abs();
        // Java: if (!valid(absValue))
      if !Self::valid(abs_value) {
         // Java: return false;
         return false;
      }
        // Java: else
      else {
         // Java: long roundValue = Math.round(absValue);
         let round_value = abs_value.round() as i64;
         // Java: long integer = (long)absValue;
         let mut integer = abs_value as i64;
         // Java: double decimal = absValue - (double)integer;
         let decimal = abs_value - integer as f64;
         // Java: long numerator = 0L;
         let mut numerator: i64 = 0;
            // Java: if (decimal != (double)0.0F)
         if decimal != 0.0 {
            // Java: numerator = Math.round(decimal * (double)denominator);
            numerator = (decimal * denominator as f64).round() as i64;
                // Java: if ((long)denominator == numerator)
            if denominator as i64 == numerator {
               // Java: integer = roundValue > integer ? roundValue : integer;
               integer = if round_value > integer { round_value } else { integer };
               // Java: numerator = 0L;
               numerator = 0;
            }
         }
         // Java: result.mInteger = integer;
         result.mInteger = integer;
         // Java: result.m_numerator = numerator;
         result.m_numerator = numerator;
         // Java: result.m_denominator = (long)denominator;
         result.m_denominator = denominator as i64;
         // Java: return true;
         return true;
      }
    }

    // Java: static boolean calcFraction(double value, int denomCount, Fraction result)
    pub fn calcFraction(value: f64, mut denom_count: i32, result: &mut Fraction) -> bool {
      // Java: long numerator = 0L;
      let mut numerator: i64;
      // Java: long denominator = 0L;
      let mut denominator: i64;
      // Java: double absValue = Math.abs(value);
      let abs_value = value.abs();
        // Java: if (!valid(absValue))
      if !Self::valid(abs_value) {
         // Java: return false;
         return false;
      }
        // Java: else
      else {
         // Java: long integer = (long)absValue;
         let mut integer = abs_value as i64;
         // Java: double decimal = absValue - (double)integer;
         let decimal = abs_value - integer as f64;
            // Java: if ((double)0.0F == decimal)
         if 0.0 == decimal {
            // Java: numerator = 0L;
            numerator = 0;
            // Java: denominator = 1L;
            denominator = 1;
            // Java: result.mInteger = integer;
            result.mInteger = integer;
            // Java: result.m_numerator = numerator;
            result.m_numerator = numerator;
            // Java: result.m_denominator = denominator;
            result.m_denominator = denominator;
            // Java: return true;
            return true;
         }
            // Java: else
         else {
            // Java: Fraction Result = new Fraction();
            let mut frac_result = Fraction::new();
            // Java: Fraction temp = new Fraction(1L, 2L);
            let mut temp = Fraction::Fraction_with_num_den(1, 2);
            // Java: double abs = (double)1.0F - decimal;
            let mut abs_val = 1.0_f64 - decimal;
            // Java: double absTmp = (double)0.0F;
            let mut abs_tmp: f64;
            // Java: double small = (double)0.0F;
            let mut small: f64 = 0.0;
            // Java: double large = (double)1.0F;
            let mut large: f64 = 1.0;
                // Java: if (denomCount > 7)
            if denom_count > 7 {
               // Java: denomCount = 7;
               denom_count = 7;
               // Java: temp.m_denominator = (long)Math.pow((double)10.0F, (double)(denomCount - 1));
               temp.m_denominator = 10_f64.powi(denom_count - 1) as i64;
               // Java: temp.m_numerator = (long)(decimal * Math.pow((double)10.0F, (double)(denomCount - 1)));
               temp.m_numerator = (decimal * 10_f64.powi(denom_count - 1)) as i64;
            }
            // Java: long maxDen = (long)Math.pow((double)10.0F, (double)denomCount);
            let max_den = 10_f64.powi(denom_count) as i64;
                // Java: while(temp.m_denominator < maxDen)
            while temp.m_denominator < max_den {
                    // Java: if (Math.abs(temp.m_fraction - decimal) < 1.0E-30)
               if (temp.m_fraction - decimal).abs() < 1.0e-30 {
                  // Java: Result.copy(temp);
                  frac_result.copy(&temp);
                  // Java: break;
                  break;
               }
                    // Java: if (temp.m_fraction < small)
               else if temp.m_fraction < small {
                  // Java: temp.increment();
                  temp.increment();
               }
                    // Java: else if (temp.m_fraction > large)
               else if temp.m_fraction > large {
                  // Java: temp.inc_denominator();
                  temp.inc_denominator();
               }
                    // Java: else if (temp.m_fraction > decimal)
               else if temp.m_fraction > decimal {
                  // Java: absTmp = temp.m_fraction - decimal;
                  abs_tmp = temp.m_fraction - decimal;
                        // Java: if (absTmp < abs)
                  if abs_tmp < abs_val {
                     // Java: Result.copy(temp);
                     frac_result.copy(&temp);
                     // Java: large = Result.m_fraction;
                     large = frac_result.m_fraction;
                     // Java: abs = absTmp;
                     abs_val = abs_tmp;
                  }
                  // Java: temp.inc_denominator();
                  temp.inc_denominator();
               }
                    // Java: else
               else {
                  // Java: absTmp = decimal - temp.m_fraction;
                  abs_tmp = decimal - temp.m_fraction;
                        // Java: if (absTmp < abs)
                  if abs_tmp < abs_val {
                     // Java: Result.copy(temp);
                     frac_result.copy(&temp);
                     // Java: small = Result.m_fraction;
                     small = frac_result.m_fraction;
                     // Java: abs = absTmp;
                     abs_val = abs_tmp;
                  }
                  // Java: temp.increment();
                  temp.increment();
               }
            }
                // Java: if (decimal < abs)
            if decimal < abs_val {
               // Java: numerator = 0L;
               numerator = 0;
               // Java: denominator = 1L;
               denominator = 1;
            }
                // Java: else
            else {
               // Java: long lDiv = calcGCD(Result.m_numerator, Result.m_denominator);
               let l_div = Self::calcGCD(frac_result.m_numerator, frac_result.m_denominator);
               // Java: numerator = Result.m_numerator / lDiv;
               numerator = frac_result.m_numerator / l_div;
               // Java: denominator = Result.m_denominator / lDiv;
               denominator = frac_result.m_denominator / l_div;
                    // Java: if (numerator == denominator)
               if numerator == denominator {
                  // Java: numerator = 0L;
                  numerator = 0;
                  // Java: denominator = 1L;
                  denominator = 1;
                  // Java: integer = (long)((float)integer + ((double)0.0F <= value ? 1.0F : -1.0F));
                  integer = (integer as f32 + if 0.0 <= value { 1.0_f32 } else { -1.0_f32 }) as i64;
               }
            }
            // Java: result.mInteger = integer;
            result.mInteger = integer;
            // Java: result.m_numerator = numerator;
            result.m_numerator = numerator;
            // Java: result.m_denominator = denominator;
            result.m_denominator = denominator;
            // Java: return true;
            return true;
         }
      }
    }

    // Java: static boolean valid(double absValue)
    fn valid(abs_value: f64) -> bool {
      // Java: return absValue < 1.0E16;
      abs_value < 1.0e16
    }

    // Java: static long calcGCD(long a, long b)
    fn calcGCD(mut a: i64, mut b: i64) -> i64 {
        // Java: if (a < b)
      if a < b {
         // Java: long t = a;
         let t = a;
         // Java: a = b;
         a = b;
         // Java: b = t;
         b = t;
      }
        // Java: for(long c = a % b; c != 0L; c = a % c)
      let mut c = a % b;
      while c != 0 {
         // Java: a = b;
         a = b;
         // Java: b = c;
         b = c;
         c = a % b;
      }
      // Java: return b;
      b
    }

}
