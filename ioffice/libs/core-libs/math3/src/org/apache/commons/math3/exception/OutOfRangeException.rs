// Java: import org.apache.commons.math3.exception.util.LocalizedFormats
#[allow(unused_imports)]
use crate::org::apache::commons::math3::exception::util::LocalizedFormats;
// Java: import org.apache.commons.math3.exception.util.Localizable
#[allow(unused_imports)]
use crate::org::apache::commons::math3::exception::util::Localizable;

// Java field: long serialVersionUID
#[allow(dead_code)]
pub(crate) const serialVersionUID: i64 = 111601815794403609;

// Java: public class OutOfRangeException extends MathIllegalNumberException
#[allow(non_snake_case)]
pub struct OutOfRangeException {
    // Java field: Number lo
    pub(crate) lo: f64,
    // Java field: Number hi
    pub(crate) hi: f64,
}

// Java: public class OutOfRangeException extends MathIllegalNumberException
#[allow(non_snake_case)]
impl OutOfRangeException {
    // Java: OutOfRangeException()
    pub fn new(_value: f64, lo: f64, hi: f64) -> Self {
        OutOfRangeException { lo, hi }
    }

    // Java: OutOfRangeException()
    pub fn new_with_specific(_specific: &str, _value: f64, lo: f64, hi: f64) -> Self {
        OutOfRangeException { lo, hi }
    }

    // Java: Number getLo()
    pub fn getLo(&self) -> f64 {
        // Java: return lo;
        self.lo
    }

    // Java: Number getHi()
    pub fn getHi(&self) -> f64 {
        // Java: return hi;
        self.hi
    }

}
