// Java: import org.apache.commons.math3.RealFieldElement
use crate::org::apache::commons::math3::RealFieldElement;
// Java: import org.apache.commons.math3.exception.MaxCountExceededException
use crate::org::apache::commons::math3::exception::MaxCountExceededException;
// Java: import org.apache.commons.math3.ode.FieldODEStateAndDerivative
use crate::org::apache::commons::math3::ode::FieldODEStateAndDerivative;

// Java: public interface represents
pub trait represents {
}

// Java field: throws MaxCountExceededException
// Java: public interface FieldStepHandler
pub trait FieldStepHandler {
    // Java: abstract void init(FieldODEStateAndDerivative<T> initialState, T finalTime)
    fn init(&self);

    // Java: abstract void handleStep(FieldStepInterpolator<T> interpolator, boolean isLast)
    fn handleStep(&self);

}
