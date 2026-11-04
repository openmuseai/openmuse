// Java: import java.io.Externalizable
// Java: import org.apache.commons.math3.exception.MaxCountExceededException
use crate::org::apache::commons::math3::exception::MaxCountExceededException;

// Java: public interface represents
pub trait represents {
}

// Java: public interface StepInterpolator extends Externalizable
pub trait StepInterpolator {
    // Java: abstract double getPreviousTime()
    fn getPreviousTime(&self) -> f64;

    // Java: abstract double getCurrentTime()
    fn getCurrentTime(&self) -> f64;

    // Java: abstract double getInterpolatedTime()
    fn getInterpolatedTime(&self) -> f64;

    // Java: abstract void setInterpolatedTime(double time)
    fn setInterpolatedTime(&self);

    // Java: abstract double[] getInterpolatedState()
    fn getInterpolatedState(&self) -> Vec<f64>;

    // Java: abstract double[] getInterpolatedDerivatives()
    fn getInterpolatedDerivatives(&self) -> Box<dyn std::any::Any>;

    // Java: abstract double[] getInterpolatedSecondaryState(int index)
    fn getInterpolatedSecondaryState(&self) -> Vec<f64>;

    // Java: abstract double[] getInterpolatedSecondaryDerivatives(int index)
    fn getInterpolatedSecondaryDerivatives(&self) -> Box<dyn std::any::Any>;

    // Java: abstract boolean isForward()
    fn isForward(&self) -> bool;

    // Java: abstract StepInterpolator copy()
    fn copy(&self) -> Box<dyn std::any::Any>;

}
