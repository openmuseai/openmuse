// Java: import org.apache.commons.math3.optim.OptimizationData
use crate::org::apache::commons::math3::optim::OptimizationData;

// Java: public class Target implements OptimizationData
pub struct Target {
    // Java field: double[] target
    pub(crate) target: Vec<f64>,
}

// Java: public class Target implements OptimizationData
impl Target {
    // Java: Target(double[] observations)
    pub fn new(observations: Vec<f64>) -> Self {
        // Java: target = observations.clone();
        Target { target: observations }
    }

    // Java: double[] getTarget()
    pub fn getTarget(&self) -> Vec<f64> {
        // Java: return target.clone();
        self.target.clone()
    }

}
