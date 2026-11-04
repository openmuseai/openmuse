#![allow(non_snake_case, non_camel_case_types, unused_variables, unused_imports, dead_code, unreachable_code, unused_mut)]

use std::sync::OnceLock;

// Java field: String[] EN_NUM_x
static EN_NUM_x: OnceLock<Vec<String>> = OnceLock::new();
// Java field: String[] EN_NUM_1x
static EN_NUM_1x: OnceLock<Vec<String>> = OnceLock::new();
// Java field: String[] EN_NUM_Tens
static EN_NUM_Tens: OnceLock<Vec<String>> = OnceLock::new();
// Java field: char CHAR_BLANK
pub(crate) const CHAR_BLANK: char = ' ';

// Java: public class NumberingUtil
pub struct NumberingUtil {
}

// Java: public class NumberingUtil
impl NumberingUtil {
    // Java: NumberingUtil()
    pub fn new() -> Self {
        // Java: super();
        NumberingUtil {}
    }

    // Java: static void convertEnglishOne(long value, boolean bCapital, StringBuffer result)
    pub fn convertEnglishOne(_value: i64, _b_capital: bool, _result: &mut String) {
        // Java: if (value <= 999999L)
        {
            // Java: if (value == 0L)
            {
                // Java: result.append(EN_NUM_x[0]);
            }
            // Java: else
            {
                // Java: convertAbove1000(value, result);
            }
            // Java: convertCapital(bCapital, result);
        }
        todo!("NumberingUtil::convertEnglishOne")
    }

    // Java: static void convertCapital(boolean bCapital, StringBuffer result)
    pub fn convertCapital(_b_capital: bool, _result: &mut String) {
        // Java: if (bCapital)
        {
            // Java: int dist = 32;
            // Java: result.setCharAt(0, (char)(result.charAt(0) - dist));
        }
        todo!("NumberingUtil::convertCapital")
    }

    // Java: static void convertAbove1000(long value, StringBuffer result)
    pub fn convertAbove1000(_value: i64, _result: &mut String) {
        // Java: if (value < 1000L)
        {
            // Java: convert100to999(value, result);
        }
        // Java: else
        {
            // Java: String THOUSAND = "thousand";
            // Java: convert100to999(value / 1000L, result);
            // Java: result.append(' ').append(THOUSAND);
            // Java: value %= 1000L;
            // Java: if (value != 0L)
            {
                // Java: result.append(' ');
                // Java: convert100to999(value, result);
            }
        }
        todo!("NumberingUtil::convertAbove1000")
    }

    // Java: static void convert100to999(long value, StringBuffer result)
    pub fn convert100to999(_value: i64, _result: &mut String) {
        // Java: if (value < 100L)
        {
            // Java: convert1To99(value, result);
        }
        // Java: else
        {
            // Java: String HUNDRED = "hundred";
            // Java: result.append(EN_NUM_x[(int)(value / 100L)]).append(' ').append(HUNDRED);
            // Java: value %= 100L;
            // Java: if (value != 0L)
            {
                // Java: result.append(' ');
                // Java: convert1To99(value, result);
            }
        }
        todo!("NumberingUtil::convert100to999")
    }

    // Java: static void convert1To99(long value, StringBuffer result)
    pub fn convert1To99(_value: i64, _result: &mut String) {
        // Java: if (value < 10L)
        {
            // Java: result.append(EN_NUM_x[(int)value]);
        }
        // Java: else
        {
            // Java: char CHAR_SUB = '-';
            // Java: if (value / 10L != 1L)
            {
                // Java: result.append(EN_NUM_Tens[(int)(value / 10L)]);
                // Java: value %= 10L;
                // Java: if (value != 0L)
                {
                    // Java: result.append(CHAR_SUB);
                    // Java: result.append(EN_NUM_x[(int)(value % 10L)]);
                }
            }
            // Java: else
            {
                // Java: result.append(EN_NUM_1x[(int)(value % 10L)]);
            }
        }
        todo!("NumberingUtil::convert1To99")
    }
}
