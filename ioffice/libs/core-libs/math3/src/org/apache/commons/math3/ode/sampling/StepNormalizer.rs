// Java: import org.apache.commons.math3.exception.MaxCountExceededException
use crate::org::apache::commons::math3::exception::MaxCountExceededException;
// Java: import org.apache.commons.math3.util.FastMath
use crate::org::apache::commons::math3::util::FastMath;
// Java: import org.apache.commons.math3.util.Precision
use crate::org::apache::commons::math3::util::Precision;

// Java: public class wraps
pub struct wraps {
}

// Java: public class StepNormalizer implements StepHandler
pub struct StepNormalizer {
    // Java field: double h
    h: f64,
    // Java field: FixedStepHandler handler
    handler: Option<Box<dyn std::any::Any>>,
    // Java field: double firstTime
    firstTime: f64,
    // Java field: double lastTime
    lastTime: f64,
    // Java field: double[] lastState
    lastState: Option<Vec<f64>>,
    // Java field: double[] lastDerivatives
    lastDerivatives: Option<Vec<f64>>,
    // Java field: boolean forward
    forward: bool,
    // Java field: StepNormalizerBounds bounds
    bounds: Option<Box<dyn std::any::Any>>,
    // Java field: StepNormalizerMode mode
    mode: Option<Box<dyn std::any::Any>>,
}

// Java: public class wraps
impl wraps {
}

// Java: public class StepNormalizer implements StepHandler
impl StepNormalizer {
    // Java: StepNormalizer(final double h, final FixedStepHandler handler)
    pub fn StepNormalizer(&self) {
        // Java: this(h, handler, StepNormalizerMode.INCREMENT,
             // Java: StepNormalizerBounds.FIRST);
    }

    // Java: StepNormalizer()
    pub fn StepNormalizer_2(&self) {
    }

    // Java: StepNormalizer()
    pub fn StepNormalizer_3(&self) {
    }

    // Java: StepNormalizer()
    pub fn StepNormalizer_4(&self) {
    }

    // Java: void init(double t0, double[] y0, double t)
    pub fn init(&self) {
        // Java: firstTime       = Double.NaN;
        // Java: lastTime        = Double.NaN;
        // Java: lastState       = null;
        // Java: lastDerivatives = null;
        // Java: forward         = true;
        // Java: // initialize the underlying handler
        // Java: handler.init(t0, y0, t);
    }

    // Java: void handleStep(final StepInterpolator interpolator, final boolean isLast)
    pub fn handleStep(&self) {
    }

    // Java: boolean isNextInStep()
    pub fn isNextInStep(&self) -> bool {
        todo!()
    }

    // Java: void doNormalizedStep(boolean isLast)
    pub fn doNormalizedStep(&self) {
        // Java: if (!bounds.firstIncluded() && firstTime == lastTime)
        {
            // Java: return;
        }
        // Java: handler.handleStep(lastTime, lastState, lastDerivatives, isLast);
    }

    // Java: void storeStep(StepInterpolator interpolator, double t)
    pub fn storeStep(&self) {
    }

}

