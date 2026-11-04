// Java: import org.apache.commons.math3.exception.MaxCountExceededException
use crate::org::apache::commons::math3::exception::MaxCountExceededException;

// Java: public interface represents
pub trait represents {
}

// Java field: throws MaxCountExceededException
// Java: public interface StepHandler
pub trait StepHandler {
    // Java: abstract void init(double t0, double[] y0, double t)
    fn init(&self);

    // Java: abstract void handleStep(StepInterpolator interpolator, boolean isLast)
    fn handleStep(&self);

}
