#![allow(non_snake_case, non_camel_case_types, unused_variables, unused_imports, dead_code, unreachable_code)]

// Java: public class Scientificn
pub struct Scientificn {
    // Java field: String POSITIVE_SIGN
    // Java field: String NEGATIVE_SING
    // Java field: double m_significand
    pub m_significand: f64,
    // Java field: int m_sign
    pub m_sign: i32,
    // Java field: long m_exponent
    pub m_exponent: i64,
    // Java field: int mIntPrecision
    mIntPrecision: i32,
    // Java field: StringBuffer mBuffer
    mBuffer: String,
}

// Java field: String POSITIVE_SIGN
pub const POSITIVE_SIGN: &str = "+";
// Java field: String NEGATIVE_SING
pub const NEGATIVE_SING: &str = "-";

impl Scientificn {
    // Java: Scientificn()
    pub fn new() -> Self {
        // Java: super();
        // Java: this.m_significand = (double)0.0F;
        // Java: this.m_sign = 0;
        // Java: this.m_exponent = 0L;
        // Java: this.mIntPrecision = 1;
        // Java: this.mBuffer = new StringBuffer();
        Scientificn {
            m_significand: 0.0,
            m_sign: 0,
            m_exponent: 0,
            mIntPrecision: 1,
            mBuffer: String::new(),
        }
    }

    // Java: Scientificn(double value, int intPrecision)
    pub fn new_with_value(value: f64, int_precision: i32) -> Self {
        // Java: super();
        let mut s = Scientificn::new();
        // Java: this.calcute(value, intPrecision);
        s.calcute(value, int_precision);
        s
    }

    // Java: void calcute(double value, int intPrecision)
    pub fn calcute(&mut self, value: f64, int_precision: i32) {
        // Java: assert intPrecision >= 0;
        debug_assert!(int_precision >= 0);
        // Java: this.mIntPrecision = intPrecision;
        self.mIntPrecision = int_precision;
        // Java: this.m_significand = (double)0.0F;
        self.m_significand = 0.0;
        // Java: this.m_exponent = 0L;
        self.m_exponent = 0;
        // Java: if (value != (double)0.0F)
        if value != 0.0 {
            // Java: double absValue = Math.abs(value);
            let abs_value = value.abs();
            // Java: long integer = (long)absValue;
            let integer = abs_value as i64;
            // Java: this.m_significand = absValue;
            self.m_significand = abs_value;
            // Java: long maxSign = intPrecision > 0 ? (long)Math.pow((double)10.0F, (double)intPrecision) : 0L;
            let mut max_sign: i64 = if int_precision > 0 { 10_f64.powi(int_precision) as i64 } else { 0 };
            // Java: if (integer <= 0L)
            if integer <= 0 {
                // Java: if (maxSign == 0L)
                if max_sign == 0 {
                    // Java: maxSign = 1L;
                    max_sign = 1;
                }
                // Java: while((long)this.m_significand < maxSign)
                while (self.m_significand as i64) < max_sign {
                    // Java: this.m_significand *= (double)10.0F;
                    self.m_significand *= 10.0;
                    // Java: --this.m_exponent;
                    self.m_exponent -= 1;
                }
                // Java: while((long)this.m_significand >= maxSign)
                while (self.m_significand as i64) >= max_sign {
                    // Java: this.m_significand *= 0.1;
                    self.m_significand *= 0.1;
                    // Java: ++this.m_exponent;
                    self.m_exponent += 1;
                }
            }
            // Java: else
            else {
                // Java: while((long)this.m_significand > maxSign)
                while (self.m_significand as i64) > max_sign {
                    // Java: this.m_significand *= 0.1;
                    self.m_significand *= 0.1;
                    // Java: ++this.m_exponent;
                    self.m_exponent += 1;
                }
                // Java: if (maxSign > 0L && (long)this.m_significand == maxSign)
                if max_sign > 0 && (self.m_significand as i64) == max_sign {
                    // Java: this.m_significand *= 0.1;
                    self.m_significand *= 0.1;
                    // Java: ++this.m_exponent;
                    self.m_exponent += 1;
                }
            }
            // Java: this.m_sign = value < (double)0.0F ? 1 : 0;
            self.m_sign = if value < 0.0 { 1 } else { 0 };
        }
    }

    // Java: double etoString(int decPrecision, String[] science)
    pub fn etoString(&mut self, dec_precision: i32, science: &mut [String; 4]) -> f64 {
        // Java: double value = this.m_significand;
        let mut value = self.m_significand;
        // Java: this.mBuffer.setLength(0);
        self.mBuffer.clear();
        let mut dec_precision = dec_precision;
        // Java: if (decPrecision <= 0)
        if dec_precision <= 0 {
            // Java: decPrecision = 0;
            dec_precision = 0;
            // Java: value += (double)0.5F;
            value += 0.5;
            // Java: int exponent = (int)Math.log10(value) + 1;
            let exponent = (value.log10() as i32) + 1;
            // Java: if (exponent > this.mIntPrecision)
            if exponent > self.mIntPrecision {
                // Java: this.m_significand = value * Math.pow((double)10.0F, (double)(this.mIntPrecision - exponent));
                self.m_significand = value * 10_f64.powi(self.mIntPrecision - exponent);
                // Java: this.m_exponent += (long)(exponent - this.mIntPrecision);
                self.m_exponent += (exponent - self.mIntPrecision) as i64;
                // Java: value = this.m_significand;
                value = self.m_significand;
            }
        }

        // Java: if (decPrecision != 0 && !Numeric.isWholeNumber(value))
        if dec_precision != 0 && !super::Numeric::Numeric::isWholeNumber(value) {
            // Java: BigDecimal bigDecimal = Numeric.round2Big(this.m_significand, decPrecision);
            let mut big_decimal = super::Numeric::Numeric::round2Big(self.m_significand, dec_precision);
            // Java: value = bigDecimal.doubleValue();
            value = big_decimal.doubleValue();
            // Java: int exponent = (int)Math.log10(value) + 1;
            let exponent = (value.log10() as i32) + 1;
            // Java: if (exponent > this.mIntPrecision && this.mIntPrecision > 0)
            if exponent > self.mIntPrecision && self.mIntPrecision > 0 {
                // Java: this.m_significand *= Math.pow((double)10.0F, (double)(this.mIntPrecision - exponent));
                self.m_significand *= 10_f64.powi(self.mIntPrecision - exponent);
                // Java: this.m_exponent += (long)(exponent - this.mIntPrecision);
                self.m_exponent += (exponent - self.mIntPrecision) as i64;
                // Java: bigDecimal = Numeric.round2Big(this.m_significand, decPrecision);
                big_decimal = super::Numeric::Numeric::round2Big(self.m_significand, dec_precision);
            }
            // Java: this.mBuffer.append(bigDecimal.toPlainString());
            self.mBuffer.push_str(&big_decimal.toPlainString());
            // Java: NumberToStr.trimDecimalZero(this.mBuffer);
            crate::cn::wps::kfc::numfmt::util::NumberToStr::NumberToStr::trimDecimalZero_buffer(&mut self.mBuffer);
        }
        // Java: else
        else {
            // Java: this.mBuffer.append(String.valueOf((long)value));
            self.mBuffer.push_str(&format!("{}", value as i64));
        }

        // Java: int index = this.mBuffer.indexOf(".");
        let index = self.mBuffer.find('.').map(|i| i as i32).unwrap_or(-1);
        // Java: science[0] = this.mBuffer.substring(0, index > -1 ? index : this.mBuffer.length());
        science[0] = if index > -1 { self.mBuffer[..index as usize].to_string() } else { self.mBuffer.clone() };
        // Java: science[1] = index > -1 ? this.mBuffer.substring(index + 1) : "";
        science[1] = if index > -1 { self.mBuffer[(index as usize + 1)..].to_string() } else { String::new() };
        // Java: science[2] = this.m_exponent >= 0L ? "+" : "-";
        science[2] = if self.m_exponent >= 0 { "+".to_string() } else { "-".to_string() };
        // Java: science[3] = String.valueOf(this.m_exponent > 0L ? this.m_exponent : -this.m_exponent);
        science[3] = format!("{}", self.m_exponent.abs());
        // Java: return value;
        value
    }
}
