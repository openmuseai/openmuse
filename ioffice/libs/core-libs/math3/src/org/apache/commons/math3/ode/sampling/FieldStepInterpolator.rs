// Java: import org.apache.commons.math3.RealFieldElement
use crate::org::apache::commons::math3::RealFieldElement;
// Java: import org.apache.commons.math3.ode.FieldODEStateAndDerivative
use crate::org::apache::commons::math3::ode::FieldODEStateAndDerivative;

// Java: public interface represents
pub trait represents {
}

// Java: public interface FieldStepInterpolator
pub trait FieldStepInterpolator {
    // Java: abstract FieldODEStateAndDerivative<T> getPreviousState()
    fn getPreviousState(&self) -> Box<dyn std::any::Any>;

    // Java: abstract FieldODEStateAndDerivative<T> getCurrentState()
    fn getCurrentState(&self) -> Box<dyn std::any::Any>;

    // Java: abstract FieldODEStateAndDerivative<T> getInterpolatedState(T time)
    fn getInterpolatedState(&self) -> Box<dyn std::any::Any>;

    // Java: abstract boolean isForward()
    fn isForward(&self) -> bool;

}
