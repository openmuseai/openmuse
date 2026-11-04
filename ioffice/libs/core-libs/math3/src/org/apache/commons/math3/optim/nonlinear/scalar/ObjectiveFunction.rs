#![allow(unused_imports, dead_code, non_snake_case, unused_variables, unused_mut, unused_assignments)]

// Java: import org.apache.commons.math3.analysis.MultivariateFunction
use crate::org::apache::commons::math3::analysis::MultivariateFunction;
// Java: import org.apache.commons.math3.optim.OptimizationData
use crate::org::apache::commons::math3::optim::OptimizationData;

// Java: public class ObjectiveFunction implements OptimizationData
pub struct ObjectiveFunction {
    // Java field: MultivariateFunction function
    pub(crate) function: Box<dyn std::any::Any>,
}

// Java: public class ObjectiveFunction implements OptimizationData
impl ObjectiveFunction {
    // Java: ObjectiveFunction(MultivariateFunction f)
    pub fn new(f: Box<dyn std::any::Any>) -> Self {
        // Java: function = f;
        ObjectiveFunction { function: f }
    }

    // Java: MultivariateFunction getObjectiveFunction()
    pub fn getObjectiveFunction(&self) -> &dyn std::any::Any {
        // Java: return function;
        self.function.as_ref()
    }

}
