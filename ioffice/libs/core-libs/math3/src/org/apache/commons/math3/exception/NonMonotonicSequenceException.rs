#![allow(unused_imports, dead_code, non_snake_case)]
// Java: import org.apache.commons.math3.util.MathArrays
use crate::org::apache::commons::math3::util::MathArrays;
// Java: import org.apache.commons.math3.exception.util.LocalizedFormats
use crate::org::apache::commons::math3::exception::util::LocalizedFormats;

// Java field: long serialVersionUID
pub(crate) const SERIAL_VERSION_UID: i64 = 3596849179428944575;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum OrderDirection {
    Increasing,
    Decreasing,
}

// Java: public class NonMonotonicSequenceException extends MathIllegalNumberException
#[derive(Debug)]
pub struct NonMonotonicSequenceException {
    // Java field: MathArrays.OrderDirection direction
    direction: OrderDirection,
    // Java field: boolean strict
    strict: bool,
    // Java field: int index
    index: i32,
    // Java field: Number previous
    previous: f64,
}

// Java: public class NonMonotonicSequenceException extends MathIllegalNumberException
impl NonMonotonicSequenceException {
    // Java: NonMonotonicSequenceException()
    pub fn new(wrong: f64, previous: f64, index: i32) -> Self {
        Self::new_full(wrong, previous, index, OrderDirection::Increasing, true)
    }

    // Java: NonMonotonicSequenceException()
    pub fn new_full(_wrong: f64, previous: f64, index: i32, direction: OrderDirection, strict: bool) -> Self {
        NonMonotonicSequenceException { direction, strict, index, previous }
    }

    // Java: MathArrays.OrderDirection getDirection()
    pub fn getDirection(&self) -> OrderDirection {
        // Java: return direction;
        self.direction
    }

    // Java: boolean getStrict()
    pub fn getStrict(&self) -> bool {
        // Java: return strict;
        self.strict
    }

    // Java: int getIndex()
    pub fn getIndex(&self) -> i32 {
        // Java: return index;
        self.index
    }

    // Java: Number getPrevious()
    pub fn getPrevious(&self) -> f64 {
        // Java: return previous;
        self.previous
    }
}

impl std::fmt::Display for NonMonotonicSequenceException {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "NonMonotonicSequenceException at index {}", self.index)
    }
}
impl std::error::Error for NonMonotonicSequenceException {}
