// Java: import org.apache.commons.math3.RealFieldElement
use crate::org::apache::commons::math3::RealFieldElement;
// Java: import org.apache.commons.math3.ode.FieldODEStateAndDerivative
use crate::org::apache::commons::math3::ode::FieldODEStateAndDerivative;

// Java: public interface represents
pub trait represents {
}

// Java: public interface as
pub trait r#as {
}

// Java: public interface FieldFixedStepHandler
pub trait FieldFixedStepHandler {
    // Java: abstract void init(FieldODEStateAndDerivative<T> initialState, T finalTime)
    fn init(&self);

    // Java: abstract void handleStep(FieldODEStateAndDerivative<T> state, boolean isLast)
    fn handleStep(&self);

}
