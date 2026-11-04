// java package org.apache.poi.ss.util;
use super::ExpandedDouble::ExpandedDouble;
// java public final class NumberToTextConverter
pub struct NumberToTextConverter;
impl NumberToTextConverter {
    // java rawDoubleBitsToText(Double.doubleToLongBits(value),decimalSeparator);
    pub fn toText(value: f64, separator: char) -> String {
        Self::rawDoubleBitsToText(
            if value.is_nan() {
                0x7ff8000000000000
            } else {
                value.to_bits()
            },
            separator,
        )
    }
    fn rawDoubleBitsToText(bits: u64, separator: char) -> String {
        let mut negative = (bits as i64) < 0;
        let raw = if negative {
            bits & 0x7fffffffffffffff
        } else {
            bits
        };
        if raw == 0 {
            return if negative { "-0" } else { "0" }.into();
        }
        let ed = ExpandedDouble::ExpandedDouble(raw);
        if ed.getBinaryExponent() < -1022 {
            return if negative { "-0" } else { "0" }.into();
        }
        if ed.getBinaryExponent() == 1024 {
            // java compares rawBits after clearing its sign to the original negative EXCEL_NAN_BITS.
            if raw as i64 == -276939487313920 {
                return format!("3{separator}484840871308E+308");
            }
            negative = false;
        }
        let rnd = ed.normaliseBaseTen().roundUnits();
        let mut exp = rnd.getDecimalExponent();
        let digits = if exp.abs() > 98 {
            let d = rnd.getSignificantDecimalDigitsLastDigitRounded();
            if d.len() == 16 {
                exp += 1;
            }
            d
        } else {
            rnd.getSignificantDecimalDigits()
        };
        // java countSignifantDigits strips only trailing zeroes.
        let count = digits.trim_end_matches('0').len();
        assert!(count > 0, "No non-zero digits found");
        let mut out = String::new();
        if negative {
            out.push('-');
        }
        if exp < 0 {
            let leading = (-exp - 1) as usize;
            if 2 + leading + count > 20 {
                out.push(digits.as_bytes()[0] as char);
                // Java intentionally uses '.' here even when decimalSeparator is another character.
                if count > 1 {
                    out.push('.');
                    out.push_str(&digits[1..count]);
                }
                out.push_str("E-");
                Self::appendExp(&mut out, -exp);
            } else {
                out.push('0');
                out.push(separator);
                out.extend(std::iter::repeat('0').take(leading));
                out.push_str(&digits[..count]);
            }
        } else if exp > 19 {
            out.push(digits.as_bytes()[0] as char);
            if count > 1 {
                out.push(separator);
                out.push_str(&digits[1..count]);
            }
            out.push_str("E+");
            Self::appendExp(&mut out, exp);
        } else {
            let fraction = count as i32 - exp - 1;
            if fraction > 0 {
                let point = exp as usize + 1;
                out.push_str(&digits[..point]);
                out.push(separator);
                out.push_str(&digits[point..count]);
            } else {
                out.push_str(&digits[..count]);
                out.extend(std::iter::repeat('0').take((-fraction) as usize));
            }
        }
        out
    }
    // java if(val<10) sb.append('0'); sb.append(val);
    fn appendExp(out: &mut String, val: i32) {
        if val < 10 {
            out.push('0');
        }
        out.push_str(&val.to_string());
    }
}
