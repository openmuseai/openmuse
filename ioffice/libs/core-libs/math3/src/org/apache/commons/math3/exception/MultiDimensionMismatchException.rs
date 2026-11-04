// Java: import org.apache.commons.math3.exception.util.Localizable
use crate::org::apache::commons::math3::exception::util::Localizable;
// Java: import org.apache.commons.math3.exception.util.LocalizedFormats
use crate::org::apache::commons::math3::exception::util::LocalizedFormats;

// Java field: long serialVersionUID
pub(crate) const serialVersionUID: i64 = 0;

// Java: public class MultiDimensionMismatchException extends MathIllegalArgumentException
pub struct MultiDimensionMismatchException {
    // Java field: Integer[] wrong
    pub(crate) wrong: Vec<i32>,
    // Java field: Integer[] expected
    pub(crate) expected: Vec<i32>,
}

// Java: public class MultiDimensionMismatchException extends MathIllegalArgumentException
impl MultiDimensionMismatchException {
    // Java: MultiDimensionMismatchException()
    pub fn MultiDimensionMismatchException(&self) {
    }

    // Java: MultiDimensionMismatchException()
    pub fn MultiDimensionMismatchException_1(&self) {
    }

    // Java: Integer[] getWrongDimensions()
    pub fn getWrongDimensions(&self) -> Vec<i32> {
        // Java: return wrong.clone();
        todo!();
    }

    // Java: Integer[] getExpectedDimensions()
    pub fn getExpectedDimensions(&self) -> Vec<i32> {
        // Java: return expected.clone();
        todo!();
    }

    // Java: int getWrongDimension(int index)
    pub fn getWrongDimension(&self, index: i32) -> i32 {
        // Java: return wrong[index].intValue();
        todo!();
    }

    // Java: int getExpectedDimension(int index)
    pub fn getExpectedDimension(&self, index: i32) -> i32 {
        // Java: return expected[index].intValue();
        todo!();
    }

}
