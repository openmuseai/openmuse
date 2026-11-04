// Java: public class Target implements OptimizationData
pub struct Target {
    // Java field: double[] target
    pub(crate) target: Box<dyn std::any::Any>,
}

// Java: public class Target implements OptimizationData
impl Target {
    // Java: Target(double[] observations)
    pub fn Target(&self) {
        // Java: target = observations.clone();
    }

    // Java: double[] getTarget()
    pub fn getTarget(&self) -> Vec<f64> {
        // Java: return target.clone();
        todo!()
    }

}

