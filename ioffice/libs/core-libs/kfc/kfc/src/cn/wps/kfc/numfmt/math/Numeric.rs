// Java: import java.math.BigDecimal
// Java: import java.math.MathContext
// Java: import java.math.RoundingMode

#[derive(Clone, Copy, PartialEq)]
pub(crate) enum RoundingMode {
    HalfUp,
    HalfEven,
    Floor,
}

#[derive(Clone)]
pub(crate) struct MathContext {
    pub(crate) precision: i32,
    pub(crate) rounding_mode: RoundingMode,
}

impl MathContext {
    pub(crate) fn new(precision: i32, rounding_mode: RoundingMode) -> Self {
        MathContext { precision, rounding_mode }
    }
}

// Rust JDK BigDecimal adapter: decimal coefficient and scale are retained
// across both MathContext rounds. No binary floating point rounding shortcut.
#[derive(Clone)]
pub(crate) struct BigDecimal {coefficient:num_bigint::BigInt,scale:i32}
impl BigDecimal {
    pub(crate) fn zero()->Self{Self{coefficient:0.into(),scale:0}}
    pub(crate) fn valueOf(value:f64)->Self{assert!(value.is_finite(),"NumberFormatException");Self::parse(&value.to_string())}
    fn parse(s:&str)->Self{
        let (mantissa,exponent)=s.split_once(['e','E']).map_or((s,0),|(m,e)|(m,e.parse::<i32>().expect("decimal exponent")));
        let scale=mantissa.split_once('.').map_or(0,|(_,f)|f.len()) as i32-exponent;
        Self{coefficient:mantissa.replace('.',"").parse().expect("NumberFormatException"),scale}
    }
    pub(crate) fn from_str_with_context(s:&str,ctx:&MathContext)->Self{Self::parse(s).round(ctx)}
    pub(crate) fn round(&self,ctx:&MathContext)->Self{
        use num_traits::{Signed,Zero};
        assert!(ctx.precision>=0,"Digits < 0");if ctx.precision==0{return self.clone();}
        let precision=self.coefficient.to_string().trim_start_matches('-').len() as i32;
        let shift=precision-ctx.precision;if shift<=0{return self.clone();}
        let divisor=num_bigint::BigInt::from(10u8).pow(shift as u32);let mut coefficient=&self.coefficient/&divisor;
        let remainder=(&self.coefficient%&divisor).abs();let halfway=&remainder*2;
        let increment=match ctx.rounding_mode{
            RoundingMode::HalfUp=>halfway>=divisor,
            RoundingMode::HalfEven=>halfway>divisor||(halfway==divisor&&!(&coefficient%2u8).is_zero()),
            RoundingMode::Floor=>self.coefficient.is_negative()&&!remainder.is_zero(),
        };
        if increment{coefficient+=if self.coefficient.is_negative(){-1}else{1};}
        let mut scale=self.scale-shift;
        // MathContext's carry can increase precision; strip that carry zero.
        if coefficient.to_string().trim_start_matches('-').len() as i32>ctx.precision{coefficient/=10u8;scale-=1;}
        Self{coefficient,scale}
    }
    pub(crate) fn doubleValue(&self)->f64{format!("{}e{}",self.coefficient,-self.scale).parse().expect("decimal double")}
    pub(crate) fn toPlainString(&self)->String{
        use num_traits::{Signed,Zero};
        let mut digits=self.coefficient.abs().to_string();
        if self.scale<=0{if !self.coefficient.is_zero(){digits.push_str(&"0".repeat((-self.scale) as usize));}}
        else{let scale=self.scale as usize;if digits.len()<=scale{digits="0".repeat(scale+1-digits.len())+&digits;}let split=digits.len()-scale;digits.insert(split,'.');}
        if self.coefficient.is_negative(){digits.insert(0,'-');}digits
    }
}

// Java field: int MAX_SIGNIFICANT_DIGIT
pub(crate) const MAX_SIGNIFICANT_DIGIT: i32 = 15;
// Java field: MathContext MAX_PRECISION
fn get_MAX_PRECISION() -> &'static MathContext {
    static MAX_PRECISION: std::sync::OnceLock<MathContext> = std::sync::OnceLock::new();
    MAX_PRECISION.get_or_init(|| MathContext::new(15, RoundingMode::HalfUp))
}
// Java field: MathContext EXCEL_MAX_PRECISION
fn get_EXCEL_MAX_PRECISION() -> &'static MathContext {
    static EXCEL_MAX_PRECISION: std::sync::OnceLock<MathContext> = std::sync::OnceLock::new();
    EXCEL_MAX_PRECISION.get_or_init(|| MathContext::new(15, RoundingMode::HalfEven))
}

// Java: public class Numeric
pub struct Numeric {
}

// Java: public class Numeric
impl Numeric {
    // Java: Numeric()
    pub fn new() -> Self {
      // Java: super();
      Numeric {}
    }

    // Java: static boolean isWholeNumber(double d)
    pub fn isWholeNumber(d: f64) -> bool {
      // Java: return d == Math.floor(d);
      let d = d;
      d == d.floor()
    }

    // Java: static double round(double value, int decimalDigits)
    pub fn round(value: f64, decimal_digits: i32) -> f64 {
      // Java: return value == (double)0.0F ? value : round2Big(value, decimalDigits).doubleValue();
      let value = value;
      if value == 0.0 { value } else { Self::round2Big(value, decimal_digits).doubleValue() }
    }

    // Java: static BigDecimal round2Big(double value, int decimalDigits)
    pub(crate) fn round2Big(value: f64, decimal_digits: i32) -> BigDecimal {
      // Java: assert decimalDigits >= 0;
      debug_assert!(decimal_digits >= 0);
      // Java: double log = Math.log10(value);
      let log = value.log10();
      // Java: int exponent = (int)log;
      let exponent = log as i32;
      // Java: int integerCount = exponent <= 0 && (exponent != 0 || (long)value == 0L) ? 0 : exponent + 1;
      let integerCount = if exponent <= 0 && (exponent != 0 || value as i64 == 0) { 0 } else { exponent + 1 };
      // Java: int precision = integerCount + (log < (double)-1.0F ? decimalDigits + exponent : decimalDigits);
      let mut precision = integerCount + if log < -1.0 { decimal_digits + exponent } else { decimal_digits };
        // Java: if (precision == 0 && log < (double)-1.0F && log - (double)exponent == (double)0.0F)
      if precision == 0 && log < -1.0 && log - exponent as f64 == 0.0 {
         // Java: ++precision;
         precision += 1;
      }
      // Java: RoundingMode roundMode = RoundingMode.HALF_UP;
      let mut roundMode = RoundingMode::HalfUp;
        // Java: if (precision < 0)
      if precision < 0 {
         // Java: return BigDecimal.ZERO;
         return BigDecimal::zero();
      }
        // Java: else
      else {
            // Java: if (precision == 0)
         if precision == 0 {
            // Java: precision = 1;
            precision = 1;
            // Java: roundMode = RoundingMode.FLOOR;
            roundMode = RoundingMode::Floor;
            // Java: long powValue = (long)(value * Math.pow((double)10.0F, (double)(1 - exponent)));
            let powValue = (value * 10_f64.powi(1 - exponent)) as i64;
                // Java: if (powValue <= 4L)
            if powValue <= 4 {
               // Java: return BigDecimal.ZERO;
               return BigDecimal::zero();
            }
            // Java: value += Math.pow((double)10.0F, (double)exponent);
            let value = value + 10_f64.powi(exponent);
            // Java: char[] dchars = Double.toString(value).toCharArray();
            let dchars = value.to_string();
            // Java: BigDecimal bigDecimal = (new BigDecimal(dchars, 0, dchars.length, MAX_PRECISION)).round(new MathContext(precision == 0 ? 1 : precision, roundMode));
            let bigDecimal = BigDecimal::from_str_with_context(&dchars, get_MAX_PRECISION())
                .round(&MathContext::new(if precision == 0 { 1 } else { precision }, roundMode));
            // Java: return bigDecimal;
            return bigDecimal;
         }
         // Java: char[] dchars = Double.toString(value).toCharArray();
         let dchars = value.to_string();
         // Java: BigDecimal bigDecimal = (new BigDecimal(dchars, 0, dchars.length, MAX_PRECISION)).round(new MathContext(precision == 0 ? 1 : precision, roundMode));
         let bigDecimal = BigDecimal::from_str_with_context(&dchars, get_MAX_PRECISION())
             .round(&MathContext::new(if precision == 0 { 1 } else { precision }, roundMode));
         // Java: return bigDecimal;
         return bigDecimal;
      }
    }

    // Java: static BigDecimal round2Big2(double value, int precision)
    pub(crate) fn round2Big2(value: f64, precision: i32) -> BigDecimal {
      // Java: BigDecimal bigDecimal = null;
      let bigDecimal;
        // Java: if (precision >= 15)
      if precision >= 15 {
         // Java: bigDecimal = BigDecimal.valueOf(value).round(MAX_PRECISION);
         bigDecimal = BigDecimal::valueOf(value).round(get_MAX_PRECISION());
      }
        // Java: else
      else {
         // Java: bigDecimal = BigDecimal.valueOf(value).round(new MathContext(precision));
         bigDecimal = BigDecimal::valueOf(value).round(&MathContext::new(precision, RoundingMode::HalfUp));
      }
      // Java: return bigDecimal;
      bigDecimal
    }

    // Java: static double roundExcel(double value)
    pub fn roundExcel(value: f64) -> f64 {
      // Java: return !Double.isInfinite(value) && !Double.isNaN(value) ? getExcelDecimal(value).doubleValue() : value;
      if !value.is_infinite() && !value.is_nan() { Self::getExcelDecimal(value).doubleValue() } else { value }
    }

    // Java: static BigDecimal getExcelDecimal(double value)
    pub(crate) fn getExcelDecimal(value: f64) -> BigDecimal {
      // Java: assert !Double.isInfinite(value) && !Double.isNaN(value);
      debug_assert!(!value.is_infinite() && !value.is_nan());
      // Java: char[] dchars = Double.toString(value).toCharArray();
      let dchars = value.to_string();
      // Java: return new BigDecimal(dchars, 0, dchars.length, EXCEL_MAX_PRECISION);
      BigDecimal::from_str_with_context(&dchars, get_EXCEL_MAX_PRECISION())
    }

}
// Rust exact JDK BigDecimal operations used by NumFmtScience.formatValue.
impl BigDecimal {
    pub(crate) fn multiply(&self,rhs:&Self)->Self {Self{coefficient:&self.coefficient*&rhs.coefficient,scale:self.scale+rhs.scale}}
    pub(crate) fn divide(&self,rhs:&Self)->Self {
        use num_traits::{Signed,Zero,One};
        assert!(!rhs.coefficient.is_zero(),"ArithmeticException: division by zero");
        let mut n=self.coefficient.clone();let mut d=rhs.coefficient.clone();if d.is_negative(){n=-n;d=-d;}
        let (mut a,mut b)=(n.abs(),d.clone());while !b.is_zero(){let next=&a%&b;a=b;b=next;}n/=&a;d/=&a;
        let mut twos=0;let mut fives=0;while (&d%2u8).is_zero(){d/=2u8;twos+=1;}while (&d%5u8).is_zero(){d/=5u8;fives+=1;}
        assert!(d.is_one(),"ArithmeticException: Non-terminating decimal expansion");let digits=twos.max(fives);n*=num_bigint::BigInt::from(2u8).pow(digits-twos);n*=num_bigint::BigInt::from(5u8).pow(digits-fives);
        Self{coefficient:n,scale:self.scale-rhs.scale+digits as i32}
    }
}
