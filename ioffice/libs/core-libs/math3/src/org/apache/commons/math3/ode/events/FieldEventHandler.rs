// Java: import org.apache.commons.math3.RealFieldElement
use crate::org::apache::commons::math3::RealFieldElement;
// Java: import org.apache.commons.math3.ode.FieldODEState
use crate::org::apache::commons::math3::ode::FieldODEState;
// Java: import org.apache.commons.math3.ode.FieldODEStateAndDerivative
use crate::org::apache::commons::math3::ode::FieldODEStateAndDerivative;

// Java: public interface represents
pub trait represents {
}

// Java: public interface FieldEventHandler
pub trait FieldEventHandler {
    // Java: abstract void init(FieldODEStateAndDerivative<T> initialState, T finalTime)
    fn init(&self);

    // Java: abstract T g(FieldODEStateAndDerivative<T> state)
    fn g(&self) -> Box<dyn std::any::Any>;

    // Java: abstract Action eventOccurred(FieldODEStateAndDerivative<T> state, boolean increasing)
    fn eventOccurred(&self) -> Box<dyn std::any::Any>;

    // Java: abstract FieldODEState<T> resetState(FieldODEStateAndDerivative<T> state)
    fn resetState(&self) -> Box<dyn std::any::Any>;

}
