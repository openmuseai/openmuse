#![allow(non_snake_case, dead_code, unused_variables)]

// Java field: int MIN_VALUE
pub(crate) const MIN_VALUE: i32 = i32::MIN;
// Java field: int MAX_POWER_OF_TWO
pub(crate) const MAX_POWER_OF_TWO: i32 = 1073741824;

// Java: public class Ints
pub struct Ints;

// Java: public class Ints
impl Ints {
    // Java: Ints()
    // Java: super();

    // Java: static int saturatedCast(long value)
    pub fn saturatedCast(value: i64) -> i32 {
        // Java: if (value > 2147483647L)
        if value > i32::MAX as i64 {
            // Java: return Integer.MAX_VALUE;
            return i32::MAX;
        }
        // Java: else
        {
            // Java: return value < -2147483648L ? Integer.MIN_VALUE : (int)value;
            if value < i32::MIN as i64 { i32::MIN } else { value as i32 }
        }
    }
}
