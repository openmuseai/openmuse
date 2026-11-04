#![allow(non_snake_case, non_camel_case_types, unused_variables, unused_imports, dead_code, unreachable_code, unused_mut, unused_assignments)]

use std::sync::OnceLock;

use crate::cn::wps::kfc::numfmt::math::Fraction::Fraction;
use crate::cn::wps::kfc::numfmt::math::Scientificn::Scientificn;
use crate::cn::wps::kfc::numfmt::math::Numeric::Numeric;

static POWERS_OF_TEN_DOUBLE_INIT: OnceLock<Vec<f64>> = OnceLock::new();
static POWERS_OF_TEN_LONG_INIT: OnceLock<Vec<i64>> = OnceLock::new();

fn get_powers_of_ten_double() -> &'static Vec<f64> {
    POWERS_OF_TEN_DOUBLE_INIT.get_or_init(|| {
        let mut v = vec![0.0_f64; 30];
        for i in 0..30 {
            v[i] = format!("1e{}", i).parse::<f64>().unwrap();
        }
        v
    })
}

fn get_powers_of_ten_long() -> &'static Vec<i64> {
    POWERS_OF_TEN_LONG_INIT.get_or_init(|| {
        let mut v = vec![0_i64; 19];
        v[0] = 1;
        for i in 1..19 {
            v[i] = v[i - 1] * 10;
        }
        v
    })
}

// Java: public class NumberToStr
pub struct NumberToStr;

impl NumberToStr {
    // Java field: Fraction FRACTION
    // Java field: Scientificn SCI_NOTATION
    // Java field: Formatter numFormatter
    // Java field: double WHOLE_MAX_15_VALUE
    // Java field: long[] POWERS_OF_TEN_LONG
    // Java field: double[] POWERS_OF_TEN_DOUBLE

    // Java: NumberToStr()
    pub fn new() -> Self {
        // Java: super();
        NumberToStr
    }

    // Java: static void double2String(double value, int decimalDigits, StringBuffer result)
    pub fn double2String_to_buffer(value: f64, decimal_digits: i32, result: &mut String) {
        // Java: result.setLength(0);
        result.clear();
        // Java: formatDoubleFast(value, decimalDigits, result);
        Self::formatDoubleFast(value, decimal_digits, result);
    }

    // Java: static void formatDoubleFast(double value, int decimals, StringBuffer target)
    fn formatDoubleFast(value: f64, decimals: i32, target: &mut String) {
        // Java: if (!Double.isNaN(value) && !Double.isInfinite(value))
        if !value.is_nan() && !value.is_infinite() {
            let mut value = value;
            // Java: if (value < (double)0.0F)
            if value < 0.0 {
                // Java: target.append('-');
                target.push('-');
                // Java: value = -value;
                value = -value;
            }
            // Java: int scale = decimals < 0 ? 0 : decimals;
            let mut scale = if decimals < 0 { 0 } else { decimals };
            // Java: long intPart = (long)Math.floor(value);
            let mut int_part = value.floor() as i64;
            // Java: double tenScale = tenPowDouble(scale);
            let ten_scale = Self::tenPowDouble(scale);
            // Java: double fracUnroundedPart = (value - (double)intPart) * tenScale;
            let frac_unrounded = (value - int_part as f64) * ten_scale;
            // Java: long fracPart = Math.round(fracUnroundedPart);
            let mut frac_part = frac_unrounded.round() as i64;
            // Java: if ((double)fracPart >= tenScale)
            if (frac_part as f64) >= ten_scale {
                // Java: ++intPart;
                int_part += 1;
                // Java: fracPart = Math.round((double)fracPart - tenScale);
                frac_part = ((frac_part as f64) - ten_scale).round() as i64;
            }
            // Java: if (fracPart != 0L)
            if frac_part != 0 {
                // Java: while(fracPart % 10L == 0L)
                while frac_part % 10 == 0 {
                    // Java: fracPart /= 10L;
                    frac_part /= 10;
                    // Java: --scale;
                    scale -= 1;
                }
            }
            // Java: if (intPart == 0L && fracPart == 0L)
            if int_part == 0 && frac_part == 0 {
                // Java: target.append('0');
                target.push('0');
            }
            // Java: else
            else {
                // Java: target.append(intPart);
                target.push_str(&int_part.to_string());
                // Java: if (fracPart != 0L)
                if frac_part != 0 {
                    // Java: target.append('.');
                    target.push('.');
                    // Java: while(scale > 0)
                    while scale > 0 {
                        // Java: double var10000 = (double)fracPart;
                        let var10000 = frac_part as f64;
                        // Java: --scale;
                        scale -= 1;
                        // Java: if (!(var10000 < tenPowDouble(scale)))
                        if !(var10000 < Self::tenPowDouble(scale)) {
                            // Java: break;
                            break;
                        }
                        // Java: target.append('0');
                        target.push('0');
                    }
                    // Java: target.append(fracPart);
                    target.push_str(&frac_part.to_string());
                }
            }
        }
        // Java: else
        else {
            // Java: target.append(Double.toString(value));
            target.push_str(&value.to_string());
        }
    }

    // Java: static double tenPowDouble(int n)
    fn tenPowDouble(n: i32) -> f64 {
        // Java: assert n >= 0;
        debug_assert!(n >= 0);
        let powers = get_powers_of_ten_double();
        // Java: return n < POWERS_OF_TEN_DOUBLE.length ? POWERS_OF_TEN_DOUBLE[n] : Math.pow((double)10.0F, (double)n);
        if (n as usize) < powers.len() {
            powers[n as usize]
        } else {
            10_f64.powi(n)
        }
    }

    // Java: static void double2String(double value, int decimalDigits, boolean isPercent, StringBuffer result)
    pub fn double2String_percent_buffer(value: f64, decimal_digits: i32, is_percent: bool, result: &mut String) {
        // Java: result.setLength(0);
        result.clear();
        let mut value = value;
        // Java: if (isPercent)
        if is_percent {
            // Java: value *= (double)100.0F;
            value *= 100.0;
        }
        // Java: value = Math.abs(value);
        value = value.abs();
        let mut decimal_digits = decimal_digits;
        // Java: if (decimalDigits <= 0)
        if decimal_digits <= 0 {
            // Java: decimalDigits = 0;
            decimal_digits = 0;
        }
        // Java: if (decimalDigits == 0 && Numeric.isWholeNumber(value) && value <= 1.0E15)
        if decimal_digits == 0 && Numeric::isWholeNumber(value) && value <= 1.0E15 {
            // Java: result.append((long)value);
            result.push_str(&format!("{}", value as i64));
        }
        // Java: else
        else {
            // Java: BigDecimal bigDecimal = Numeric.round2Big(value, decimalDigits);
            let big_decimal = Numeric::round2Big(value, decimal_digits);
            // Java: result.append(bigDecimal.toPlainString());
            result.push_str(&big_decimal.toPlainString());
            // Java: trimDecimalZero(result);
            Self::trimDecimalZero_buffer(result);
        }
    }

    // Java: static String double2String(double value, int decimalDigits, boolean isPercent)
    pub fn double2String_percent(value: f64, decimal_digits: i32, is_percent: bool) -> String {
        let mut value = value;
        // Java: if (isPercent)
        if is_percent {
            // Java: value *= (double)100.0F;
            value *= 100.0;
        }
        // Java: value = Math.abs(value);
        value = value.abs();
        let mut decimal_digits = decimal_digits;
        // Java: if (decimalDigits <= 0)
        if decimal_digits <= 0 {
            // Java: decimalDigits = 0;
            decimal_digits = 0;
        }
        // Java: if (decimalDigits == 0 && Numeric.isWholeNumber(value) && value <= 1.0E15)
        if decimal_digits == 0 && Numeric::isWholeNumber(value) && value <= 1.0E15 {
            // Java: return Long.toString((long)value);
            return format!("{}", value as i64);
        }
        // Java: else
        else {
            // Java: BigDecimal bigDecimal = Numeric.round2Big(value, decimalDigits);
            let big_decimal = Numeric::round2Big(value, decimal_digits);
            // Java: String result = bigDecimal.toPlainString();
            let result = big_decimal.toPlainString();
            // Java: result = trimDecimalZero(result);
            let result = Self::trimDecimalZero_str(&result);
            // Java: return result;
            return result;
        }
    }

    // Java: static void trimDecimalZero(StringBuffer result)
    pub fn trimDecimalZero_buffer(result: &mut String) {
        // Java: if (result.indexOf(".") != -1)
        if result.find('.').is_some() {
            // Java: int index = -1;
            let mut index: i32 = -1;
            let bytes = result.as_bytes();
            // Java: for(int i = result.length() - 1; i > 0; index = i--)
            let mut i = result.len() as i32 - 1;
            while i > 0 {
                // Java: char c = result.charAt(i);
                let c = bytes[i as usize] as char;
                // Java: if (c != '0')
                if c != '0' {
                    // Java: if (c == '.')
                    if c == '.' {
                        // Java: index = i;
                        index = i;
                    }
                    // Java: break;
                    break;
                }
                index = i;
                i -= 1;
            }
            // Java: if (index > -1)
            if index > -1 {
                // Java: result.delete(index, result.length());
                result.truncate(index as usize);
            }
        }
    }

    // Java: static String trimDecimalZero(String result)
    pub fn trimDecimalZero_str(input: &str) -> String {
        // Java: if (result.indexOf(".") == -1)
        if input.find('.').is_none() {
            // Java: return result;
            return input.to_string();
        }
        // Java: else
        {
            // Java: int index = -1;
            let mut index: i32 = -1;
            let bytes = input.as_bytes();
            // Java: for(int i = result.length() - 1; i > 0; index = i--)
            let mut i = input.len() as i32 - 1;
            while i > 0 {
                // Java: char c = result.charAt(i);
                let c = bytes[i as usize] as char;
                // Java: if (c != '0')
                if c != '0' {
                    // Java: if (c == '.')
                    if c == '.' {
                        // Java: index = i;
                        index = i;
                    }
                    // Java: break;
                    break;
                }
                index = i;
                i -= 1;
            }
            // Java: return index > -1 ? result.substring(0, index) : result;
            if index > -1 {
                input[..index as usize].to_string()
            } else {
                input.to_string()
            }
        }
    }

    // Java: static String double2String(double value, int precision)
    pub fn double2String_precision(value: f64, precision: i32) -> String {
        // Java: BigDecimal bigDecimal = Numeric.round2Big2(value, precision);
        let big_decimal = Numeric::round2Big2(value, precision);
        // Java: String result = bigDecimal.toPlainString();
        let result = big_decimal.toPlainString();
        // Java: return trimDecimalZero(result);
        Self::trimDecimalZero_str(&result)
    }

    // Java: static void double2Fraction(double value, int denominator, boolean hasInteger, boolean isDigitDenom, String[] fraction)
    pub fn double2Fraction(value: f64, denominator: i32, has_integer: bool, is_digit_denom: bool, fraction: &mut [String; 3]) {
        // Java: Fraction result = FRACTION;
        let mut result = Fraction::new();
        // Java: boolean isValid = false;
        let mut is_valid;
        // Java: if (!isDigitDenom)
        if !is_digit_denom {
            // Java: isValid = Fraction.calcFraction(value, denominator, result);
            is_valid = Fraction::calcFraction(value, denominator, &mut result);
        }
        // Java: else
        else {
            // Java: isValid = Fraction.calcFraction2(value, denominator, result);
            is_valid = Fraction::calcFraction2(value, denominator, &mut result);
        }
        // Java: if (!isValid)
        if !is_valid {
            // Java: if (hasInteger)
            if has_integer {
                // Java: fraction[0] = NumberRound.round(value, 0).toString();
                fraction[0] = super::NumberRound::NumberRound::round(value,0);
            }
            // Java: else
            else {
                // Java: fraction[0] = NumFmtUtil.invalidNumberSignString;
                fraction[0] = "#".repeat(255);
            }
            // Java: fraction[1] = fraction[2] = "";
            fraction[1] = String::new();
            fraction[2] = String::new();
        }
        // Java: else
        else {
            // Java: if (hasInteger)
            if has_integer {
                // Java: if (result.m_numerator > 0L && result.mInteger == 0L)
                if result.m_numerator > 0 && result.mInteger == 0 {
                    // Java: fraction[0] = "";
                    fraction[0] = String::new();
                }
                // Java: else
                else {
                    // Java: fraction[0] = String.valueOf(result.mInteger);
                    fraction[0] = format!("{}", result.mInteger);
                }
                // Java: fraction[1] = result.m_numerator > 0L ? String.valueOf(result.m_numerator) : "";
                fraction[1] = if result.m_numerator > 0 { format!("{}", result.m_numerator) } else { String::new() };
                // Java: fraction[2] = result.m_numerator > 0L ? String.valueOf(result.m_denominator) : "";
                fraction[2] = if result.m_numerator > 0 { format!("{}", result.m_denominator) } else { String::new() };
            }
            // Java: else
            else {
                // Java: double numerator = (double)(result.m_numerator + result.mInteger * result.m_denominator);
                let numerator = result.m_numerator.wrapping_add(result.mInteger.wrapping_mul(result.m_denominator)) as f64;
                // Java: if (numerator < (double)Integer.MAX_VALUE && numerator > (double)Integer.MIN_VALUE)
                if numerator < (i32::MAX as f64) && numerator > (i32::MIN as f64) {
                    // Java: fraction[0] = "";
                    fraction[0] = String::new();
                    // Java: fraction[1] = String.valueOf((long)numerator);
                    fraction[1] = format!("{}", numerator as i64);
                    // Java: fraction[2] = String.valueOf(result.m_denominator);
                    fraction[2] = format!("{}", result.m_denominator);
                }
                // Java: else
                else {
                    // Java: fraction[0] = NumFmtUtil.invalidNumberSignString;
                    fraction[0] = "#".repeat(255);
                }
            }
        }
    }

    // Java: static double double2Science(double value, int decimalDigits, int integerDigits, String[] science)
    pub fn double2Science(value: f64, decimal_digits: i32, integer_digits: i32, science: &mut [String; 4]) -> f64 {
        // Java: SCI_NOTATION.calcute(value, integerDigits);
        let mut sci = Scientificn::new();
        sci.calcute(value, integer_digits);
        // Java: return SCI_NOTATION.etoString(decimalDigits, science);
        sci.etoString(decimal_digits, science)
    }

    // Java: static String double2StandardE(double value, int precision)
    pub fn double2StandardE(value: f64, precision: i32) -> String {
        // Java: precision = precision > 15 ? 15 : precision;
        let precision = if precision > 15 { 15 } else { precision };
        // Java: StringBuilder builder = (StringBuilder)numFormatter.out();
        // Java: builder.setLength(0);
        // Java: numFormatter.format("%." + precision + "E", value);
        assert!(precision>=0,"UnknownFormatConversionException");
        let mut builder = if value.is_nan(){"NAN".into()}else if value==f64::INFINITY{"INFINITY".into()}else if value==f64::NEG_INFINITY{"-INFINITY".into()}else{let raw=format!("{:.prec$E}",value,prec=precision as usize);let (mantissa,exponent)=raw.split_once('E').unwrap();let exponent:i32=exponent.parse().unwrap();format!("{mantissa}E{exponent:+03}")};
        // Java: int index = builder.indexOf(".");
        let dot_index = builder.find('.');
        // Java: int ePos = builder.indexOf("E");
        let e_pos = builder.find('E');
        // Java: int zeroPos;
        if let (Some(index), Some(e_pos)) = (dot_index, e_pos) {
            // Java: for(zeroPos = ePos - 1; zeroPos > index && builder.charAt(zeroPos) == '0'; --zeroPos)
            let mut zero_pos = e_pos - 1;
            while zero_pos > index && builder.as_bytes()[zero_pos] == b'0' {
                zero_pos -= 1;
            }
            // Java: if (zeroPos + 1 != ePos)
            if zero_pos + 1 != e_pos {
                // Java: builder.delete(zeroPos + 1, ePos);
                let before = &builder[..zero_pos + 1];
                let after = &builder[e_pos..];
                builder = format!("{}{}", before, after);
            }
        }
        // Java: return builder.toString();
        builder
    }

    // Java: static String int2Str(int value, int minReserved)
    pub fn int2Str(value: i32, min_reserved: i32) -> String {
        // Java: String digits = Integer.toString(value);
        let digits = value.to_string();
        // Java: int len = digits.length();
        let len = digits.len() as i32;
        // Java: if (len >= minReserved)
        if len >= min_reserved {
            // Java: return digits;
            digits
        }
        // Java: else
        else {
            // Java: StringBuffer buffer = new StringBuffer();
            let mut buffer = String::new();
            // Java: for(int i = 0; i < minReserved - len; ++i)
            for _ in 0..(min_reserved - len) {
                // Java: buffer.insert(i, '0');
                buffer.push('0');
            }
            // Java: buffer.append(digits);
            buffer.push_str(&digits);
            // Java: return buffer.toString();
            buffer
        }
    }

    // Java: static String long2Str(long value, int minReserved)
    pub fn long2Str(value: i64, min_reserved: i32) -> String {
        // Java: String digits = Long.toString(value);
        let digits = value.to_string();
        // Java: int len = digits.length();
        let len = digits.len() as i32;
        // Java: if (len >= minReserved)
        if len >= min_reserved {
            // Java: return digits;
            digits
        }
        // Java: else
        else {
            // Java: StringBuffer buffer = new StringBuffer();
            let mut buffer = String::new();
            // Java: for(int i = 0; i < minReserved - len; ++i)
            for _ in 0..(min_reserved - len) {
                // Java: buffer.insert(i, '0');
                buffer.push('0');
            }
            // Java: buffer.append(digits);
            buffer.push_str(&digits);
            // Java: return buffer.toString();
            buffer
        }
    }

    // Java: static void double2StringEx(double value, int decimalDigits, int integerDigits, boolean isPercent, StringBuffer result)
    #[deprecated]
    pub fn double2StringEx(value: f64, decimal_digits: i32, _integer_digits: i32, is_percent: bool, result: &mut String) {
        // Java: result.setLength(0);
        result.clear();
        let mut value = value;
        // Java: if (isPercent)
        if is_percent {
            // Java: value *= (double)100.0F;
            value *= 100.0;
        }
        let mut decimal_digits = decimal_digits;
        // Java: if (decimalDigits < 0)
        if decimal_digits < 0 {
            // Java: decimalDigits = 0;
            decimal_digits = 0;
        }
        // Java: value = Math.abs(value);
        value = value.abs();
        // Java: boolean isWholeNumber = Numeric.isWholeNumber(value);
        let is_whole = Numeric::isWholeNumber(value);
        // Java: double round = !isWholeNumber ? Numeric.round(value, decimalDigits) : value;
        let mut round = if !is_whole { Numeric::round(value, decimal_digits) } else { value };
        // Java: long integer = (long)round;
        let integer = round as i64;
        // Java: result.append(String.valueOf(integer));
        result.push_str(&integer.to_string());
        // Java: if (decimalDigits != 0 && !Numeric.isWholeNumber(round))
        if decimal_digits != 0 && !Numeric::isWholeNumber(round) {
            // Java: result.append(".");
            result.push('.');
            // Java: for(int i = 0; i < decimalDigits - 1; ++i)
            for _ in 0..(decimal_digits - 1) {
                // Java: round *= (double)10.0F;
                round *= 10.0;
                // Java: result.append(String.valueOf((int)round % 10));
                result.push_str(&format!("{}", (round as i32) % 10));
            }
            // Java: round *= (double)10.0F;
            round *= 10.0;
            // Java: result.append(String.valueOf((int)(round + (double)0.5F) % 10));
            result.push_str(&format!("{}", ((round + 0.5) as i32) % 10));
            // Java: for(int i = result.length() - 1; i >= 0 && result.charAt(i) == '0'; --i)
            while result.ends_with('0') {
                // Java: result.deleteCharAt(i);
                result.pop();
            }
        }
    }
}
