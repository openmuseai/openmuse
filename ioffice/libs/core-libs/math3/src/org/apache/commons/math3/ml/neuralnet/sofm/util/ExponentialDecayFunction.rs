#![allow(unused_imports, dead_code, non_snake_case, unused_variables, unused_mut, unused_assignments)]

// Java: import org.apache.commons.math3.exception.NotStrictlyPositiveException
use crate::org::apache::commons::math3::exception::NotStrictlyPositiveException;
// Java: import org.apache.commons.math3.exception.NumberIsTooLargeException
use crate::org::apache::commons::math3::exception::NumberIsTooLargeException;
// Java: import org.apache.commons.math3.util.FastMath
use crate::org::apache::commons::math3::util::FastMath::FastMath;

// Java: public class ExponentialDecayFunction
pub struct ExponentialDecayFunction {
    // Java field: double a
    pub(crate) a: f64,
    // Java field: double oneOverB
    pub(crate) oneOverB: f64,
}

// Java: public class ExponentialDecayFunction
impl ExponentialDecayFunction {
    // Java: ExponentialDecayFunction()
    pub fn new(initValue: f64, numCall: i64) -> Self {
        if initValue <= 0.0 {
            panic!("NotStrictlyPositiveException: {}", initValue);
        }
        if numCall <= 0 {
            panic!("NotStrictlyPositiveException: {}", numCall);
        }
        ExponentialDecayFunction {
            a: initValue,
            oneOverB: -1.0 / numCall as f64,
        }
    }

    // Java: double value(long numCall)
    pub fn value(&self, numCall: i64) -> f64 {
        // Java: return a * FastMath.exp(-numCall * oneOverB);
        self.a * FastMath::exp(-numCall as f64 * self.oneOverB)
    }

}
