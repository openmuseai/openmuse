// Java: import org.apache.commons.math3.analysis.MultivariateVectorFunction
use crate::org::apache::commons::math3::analysis::MultivariateVectorFunction;
// Java: import org.apache.commons.math3.optim.OptimizationData
use crate::org::apache::commons::math3::optim::OptimizationData;

// Java: public class ModelFunction implements OptimizationData
pub struct ModelFunction {
    // Java field: MultivariateVectorFunction model
    pub(crate) model: Box<dyn std::any::Any>,
}

// Java: public class ModelFunction implements OptimizationData
impl ModelFunction {
    // Java: ModelFunction(MultivariateVectorFunction m)
    pub fn new(m: Box<dyn std::any::Any>) -> Self {
        // Java: model = m;
        ModelFunction { model: m }
    }

    // Java: MultivariateVectorFunction getModelFunction()
    pub fn getModelFunction(&self) -> &dyn std::any::Any {
        // Java: return model;
        &*self.model
    }

}
