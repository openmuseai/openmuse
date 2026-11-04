// Java: public class InitialGuess implements OptimizationData
pub struct InitialGuess {
    // Java field: double[] init
    pub(crate) init: Vec<f64>,
}

// Java: public class InitialGuess implements OptimizationData
impl InitialGuess {
    // Java: InitialGuess(double[] startPoint)
    pub fn new(start_point: Vec<f64>) -> Self {
        // Java: init = startPoint.clone();
        InitialGuess { init: start_point }
    }

    // Java: double[] getInitialGuess()
    pub fn getInitialGuess(&self) -> Vec<f64> {
        // Java: return init.clone();
        self.init.clone()
    }

}
