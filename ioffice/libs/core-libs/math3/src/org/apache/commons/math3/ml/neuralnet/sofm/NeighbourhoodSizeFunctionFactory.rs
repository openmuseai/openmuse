#![allow(unused_imports, dead_code, non_snake_case, unused_variables, unused_mut, unused_assignments)]

// Java: import org.apache.commons.math3.ml.neuralnet.sofm.util.ExponentialDecayFunction
use crate::org::apache::commons::math3::ml::neuralnet::sofm::util::ExponentialDecayFunction;
// Java: import org.apache.commons.math3.ml.neuralnet.sofm.util.QuasiSigmoidDecayFunction
use crate::org::apache::commons::math3::ml::neuralnet::sofm::util::QuasiSigmoidDecayFunction;
// Java: import org.apache.commons.math3.util.FastMath
use crate::org::apache::commons::math3::util::FastMath::FastMath;

use super::NeighbourhoodSizeFunction::NeighbourhoodSizeFunction;

// Java: public class NeighbourhoodSizeFunctionFactory
pub struct NeighbourhoodSizeFunctionFactory {
}

// Java: public class NeighbourhoodSizeFunctionFactory
impl NeighbourhoodSizeFunctionFactory {
    // Java: NeighbourhoodSizeFunctionFactory()
    fn new() -> Self {
        NeighbourhoodSizeFunctionFactory {}
    }

    // Java: static NeighbourhoodSizeFunction exponentialDecay()
    pub fn exponentialDecay(initValue: f64, numCall: i64) -> Box<dyn NeighbourhoodSizeFunction> {
        todo!()
    }

    // Java: static NeighbourhoodSizeFunction quasiSigmoidDecay()
    pub fn quasiSigmoidDecay(initValue: f64, slope: f64, numCall: i64) -> Box<dyn NeighbourhoodSizeFunction> {
        todo!()
    }

}
