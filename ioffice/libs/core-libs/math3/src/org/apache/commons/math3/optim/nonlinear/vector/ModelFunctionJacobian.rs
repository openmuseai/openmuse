// Java: import org.apache.commons.math3.analysis.MultivariateMatrixFunction
use crate::org::apache::commons::math3::analysis::MultivariateMatrixFunction;
// Java: import org.apache.commons.math3.optim.OptimizationData
use crate::org::apache::commons::math3::optim::OptimizationData;

// Java: public class ModelFunctionJacobian implements OptimizationData
pub struct ModelFunctionJacobian {
    // Java field: MultivariateMatrixFunction jacobian
    pub(crate) jacobian: Box<dyn std::any::Any>,
}

// Java: public class ModelFunctionJacobian implements OptimizationData
impl ModelFunctionJacobian {
    // Java: ModelFunctionJacobian(MultivariateMatrixFunction j)
    pub fn new(j: Box<dyn std::any::Any>) -> Self {
        // Java: jacobian = j;
        ModelFunctionJacobian { jacobian: j }
    }

    // Java: MultivariateMatrixFunction getModelFunctionJacobian()
    pub fn getModelFunctionJacobian(&self) -> &dyn std::any::Any {
        // Java: return jacobian;
        &*self.jacobian
    }

}
