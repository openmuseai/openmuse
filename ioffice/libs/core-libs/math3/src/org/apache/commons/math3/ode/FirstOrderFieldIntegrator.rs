// Java: import java.util.Collection
// Java: import org.apache.commons.math3.RealFieldElement
use crate::org::apache::commons::math3::RealFieldElement;
// Java: import org.apache.commons.math3.analysis.solvers.BracketedRealFieldUnivariateSolver
use crate::org::apache::commons::math3::analysis::solvers::BracketedRealFieldUnivariateSolver;
// Java: import org.apache.commons.math3.exception.MaxCountExceededException
use crate::org::apache::commons::math3::exception::MaxCountExceededException;
// Java: import org.apache.commons.math3.exception.NoBracketingException
use crate::org::apache::commons::math3::exception::NoBracketingException;
// Java: import org.apache.commons.math3.exception.NumberIsTooSmallException
use crate::org::apache::commons::math3::exception::NumberIsTooSmallException;
// Java: import org.apache.commons.math3.ode.events.FieldEventHandler
use crate::org::apache::commons::math3::ode::events::FieldEventHandler;
// Java: import org.apache.commons.math3.ode.sampling.FieldStepHandler
use crate::org::apache::commons::math3::ode::sampling::FieldStepHandler;

// Java: public interface represents
pub trait represents {
}

// Java field: throws NumberIsTooSmallException, MaxCountExceededException, NoBracketingException
// Java: public interface FirstOrderFieldIntegrator extends RealFieldElement
pub trait FirstOrderFieldIntegrator {
    // Java: abstract String getName()
    fn getName(&self) -> String;

    // Java: abstract void addStepHandler(FieldStepHandler<T> handler)
    fn addStepHandler(&self);

    // Java: abstract Collection<FieldStepHandler<T>> getStepHandlers()
    fn getStepHandlers(&self) -> Vec<Box<dyn std::any::Any>>;

    // Java: abstract void clearStepHandlers()
    fn clearStepHandlers(&self);

    // Java: abstract void addEventHandler()
    fn addEventHandler(&self);

    // Java: abstract void addEventHandler()
    fn addEventHandler_2(&self);

    // Java: abstract Collection<FieldEventHandler<T> > getEventHandlers()
    fn getEventHandlers(&self) -> Vec<Box<dyn std::any::Any>>;

    // Java: abstract void clearEventHandlers()
    fn clearEventHandlers(&self);

    // Java: abstract FieldODEStateAndDerivative<T> getCurrentStepStart()
    fn getCurrentStepStart(&self) -> Box<dyn std::any::Any>;

    // Java: abstract T getCurrentSignedStepsize()
    fn getCurrentSignedStepsize(&self) -> Box<dyn std::any::Any>;

    // Java: abstract void setMaxEvaluations(int maxEvaluations)
    fn setMaxEvaluations(&self);

    // Java: abstract int getMaxEvaluations()
    fn getMaxEvaluations(&self) -> i32;

    // Java: abstract int getEvaluations()
    fn getEvaluations(&self) -> i32;

    // Java: abstract FieldODEStateAndDerivative<T> integrate()
    fn integrate(&self) -> Box<dyn std::any::Any>;

}
