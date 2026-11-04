#![allow(unused_imports, dead_code, non_snake_case, unused_variables, unused_mut, unused_assignments)]

// Java: import org.apache.commons.math3.analysis.MultivariateVectorFunction
use crate::org::apache::commons::math3::analysis::MultivariateVectorFunction;
// Java: import org.apache.commons.math3.optim.OptimizationData
use crate::org::apache::commons::math3::optim::OptimizationData;

// Java: public class ObjectiveFunctionGradient implements OptimizationData
pub struct ObjectiveFunctionGradient {
    // Java field: MultivariateVectorFunction gradient
    pub(crate) gradient: Box<dyn std::any::Any>,
}

// Java: public class ObjectiveFunctionGradient implements OptimizationData
impl ObjectiveFunctionGradient {
    // Java: ObjectiveFunctionGradient(MultivariateVectorFunction g)
    pub fn new(g: Box<dyn std::any::Any>) -> Self {
        // Java: gradient = g;
        ObjectiveFunctionGradient { gradient: g }
    }

    // Java: MultivariateVectorFunction getObjectiveFunctionGradient()
    pub fn getObjectiveFunctionGradient(&self) -> &dyn std::any::Any {
        // Java: return gradient;
        self.gradient.as_ref()
    }

}
