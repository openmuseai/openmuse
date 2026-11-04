#![allow(
    non_snake_case,
    non_camel_case_types,
    unused_variables,
    unused_imports,
    dead_code,
    unreachable_code,
    unused_mut
)]

// Java: import cn.wps.kfc.numfmt.math.Numeric
use crate::cn::wps::kfc::numfmt::math::Numeric;

// Java field: int MAX_SIGNIFICANT_DIGIT
pub(crate) const MAX_SIGNIFICANT_DIGIT: i32 = 15;

// Java: public class NumberRound
pub struct NumberRound {}

// Java: public class NumberRound
impl NumberRound {
    // Java: NumberRound()

    // rust: source koffice/core-libs/kfc/kfc/src/cn/wps/kfc/numfmt/util/NumberRound.java:8
    // java public NumberRound() {
    // java super();
    // java }
    pub fn new() -> Self {
        // Java: super();
        NumberRound {}
    }

    // Java: static StringBuffer round(double value, int decimalDigits)

    // rust: source koffice/core-libs/kfc/kfc/src/cn/wps/kfc/numfmt/util/NumberRound.java:12
    // java public static StringBuffer round(double value, int decimalDigits) {
    // java StringBuffer result = new StringBuffer();
    // java int exponent = (int)Math.log10(value);
    // java int integerCount = exponent <= 0 && (exponent != 0 || (long)value == 0L) ? 0 : exponent + 1;
    // java if (integerCount >= 15) {
    // java return dealBigNumber(value, integerCount);
    // java } else if (decimalDigits <= 0) {
    // java value += (double)0.5F;
    // java return result.append((long)value);
    // java } else if (Numeric.isWholeNumber(value)) {
    // java return result.append((long)value);
    // java } else {
    // java if (integerCount + decimalDigits > 15) {
    // java decimalDigits = 15 - integerCount;
    // java }
    // java
    // java int dropDigits = decimalDigits + 1;
    // java long ten_powers = (long)Math.pow((double)10.0F, (double)dropDigits);
    // java long vl = (long)(value * (double)ten_powers);
    // java long integerVL = (long)value * ten_powers;
    // java int mantissa = (int)(vl % 10L);
    // java int raise = mantissa > 4 && integerCount + decimalDigits < 15 ? 10 : 0;
    // java long trunc = ((long)raise + vl - (long)mantissa) / ten_powers;
    // java long decimalpartVL = (long)raise + vl - (long)mantissa - integerVL;
    // java StringBuffer decimalStr = new StringBuffer();
    // java if (decimalpartVL - ten_powers >= 0L) {
    // java decimalStr.append('0');
    // java } else {
    // java decimalStr.append(decimalpartVL / 10L);
    // java
    // java for(int len = decimalStr.length(); len < decimalDigits && exponent < 0; ++len) {
    // java decimalStr.insert(0, '0');
    // java }
    // java }
    // java
    // java result.append(trunc);
    // java result.append(".");
    // java result.append(decimalStr);
    // java NumberToStr.trimDecimalZero(result);
    // java return result;
    // java }
    // java }
    pub fn round(mut value: f64, mut decimal_digits: i32) -> String {
        let exponent = value.log10() as i32;
        let integer_count = if exponent <= 0 && (exponent != 0 || value as i64 == 0) {
            0
        } else {
            exponent.wrapping_add(1)
        };
        if integer_count >= 15 {
            return Self::dealBigNumber(value, integer_count);
        }
        if decimal_digits <= 0 {
            value += 0.5;
            return (value as i64).to_string();
        }
        if Numeric::Numeric::isWholeNumber(value) {
            return (value as i64).to_string();
        }
        if integer_count + decimal_digits > 15 {
            decimal_digits = 15 - integer_count;
        }
        let drop_digits = decimal_digits + 1;
        let powers = 10.0f64.powf(drop_digits as f64) as i64;
        let vl = (value * powers as f64) as i64;
        let integer_vl = (value as i64).wrapping_mul(powers);
        let mantissa = (vl % 10) as i32;
        let raise = if mantissa > 4 && integer_count + decimal_digits < 15 {
            10i64
        } else {
            0
        };
        let whole = raise.wrapping_add(vl).wrapping_sub(mantissa as i64);
        let trunc = whole / powers;
        let decimal_part = whole.wrapping_sub(integer_vl);
        let mut decimal = if decimal_part.wrapping_sub(powers) >= 0 {
            "0".into()
        } else {
            (decimal_part / 10).to_string()
        };
        if decimal_part.wrapping_sub(powers) < 0 {
            while decimal.len() < (decimal_digits as usize) && exponent < 0 {
                decimal.insert(0, '0');
            }
        }
        let mut result = format!("{trunc}.{decimal}");
        super::NumberToStr::NumberToStr::trimDecimalZero_buffer(&mut result);
        result
    }

    // Java: static StringBuffer dealBigNumber(double value, int integerCount)

    // rust: source koffice/core-libs/kfc/kfc/src/cn/wps/kfc/numfmt/util/NumberRound.java:55
    // java private static StringBuffer dealBigNumber(double value, int integerCount) {
    // java StringBuffer result = new StringBuffer();
    // java int drop = integerCount - 15;
    // java long valueInt = (long)(value / Math.pow((double)10.0F, (double)drop));
    // java result.append(valueInt);
    // java
    // java while(drop > 0) {
    // java result.append('0');
    // java --drop;
    // java }
    // java
    // java return result;
    // java }
    pub fn dealBigNumber(value: f64, integer_count: i32) -> String {
        let mut drop = integer_count - 15;
        let value_int = (value / 10.0f64.powf(drop as f64)) as i64;
        let mut result = value_int.to_string();
        while drop > 0 {
            result.push('0');
            drop -= 1;
        }
        result
    }
}
