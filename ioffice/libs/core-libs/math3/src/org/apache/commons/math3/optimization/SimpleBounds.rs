// Java: public class SimpleBounds implements OptimizationData
pub struct SimpleBounds {
    // Java field: double[] lower
    pub(crate) lower: Box<dyn std::any::Any>,
    // Java field: double[] upper
    pub(crate) upper: Box<dyn std::any::Any>,
}

// Java: public class SimpleBounds implements OptimizationData
impl SimpleBounds {
    // Java: SimpleBounds()
    pub fn SimpleBounds(&self) {
    }

    // Java: double[] getLower()
    pub fn getLower(&self) -> Vec<f64> {
        // Java: return lower.clone();
        todo!()
    }

    // Java: double[] getUpper()
    pub fn getUpper(&self) -> Vec<f64> {
        // Java: return upper.clone();
        todo!()
    }

}

