// Java: import java.util.Collection
// Java: import org.apache.commons.math3.analysis.solvers.UnivariateSolver
use crate::org::apache::commons::math3::analysis::solvers::UnivariateSolver;
// Java: import org.apache.commons.math3.ode.events.EventHandler
use crate::org::apache::commons::math3::ode::events::EventHandler;
// Java: import org.apache.commons.math3.ode.sampling.StepHandler
use crate::org::apache::commons::math3::ode::sampling::StepHandler;

// Java: public interface defines
pub trait defines {
}

// Java: public interface ODEIntegrator
pub trait ODEIntegrator {
    // Java: abstract String getName()
    fn getName(&self) -> String;

    // Java: abstract void addStepHandler(StepHandler handler)
    fn addStepHandler(&self);

    // Java: abstract Collection<StepHandler> getStepHandlers()
    fn getStepHandlers(&self) -> Vec<Box<dyn std::any::Any>>;

    // Java: abstract void clearStepHandlers()
    fn clearStepHandlers(&self);

    // Java: abstract void addEventHandler()
    fn addEventHandler(&self);

    // Java: abstract void addEventHandler()
    fn addEventHandler_2(&self);

    // Java: abstract Collection<EventHandler> getEventHandlers()
    fn getEventHandlers(&self) -> Vec<Box<dyn std::any::Any>>;

    // Java: abstract void clearEventHandlers()
    fn clearEventHandlers(&self);

    // Java: abstract double getCurrentStepStart()
    fn getCurrentStepStart(&self) -> f64;

    // Java: abstract double getCurrentSignedStepsize()
    fn getCurrentSignedStepsize(&self) -> f64;

    // Java: abstract void setMaxEvaluations(int maxEvaluations)
    fn setMaxEvaluations(&self);

    // Java: abstract int getMaxEvaluations()
    fn getMaxEvaluations(&self) -> i32;

    // Java: abstract int getEvaluations()
    fn getEvaluations(&self) -> i32;

}
