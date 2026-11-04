// Java: import org.apache.commons.math3.RealFieldElement
use crate::org::apache::commons::math3::RealFieldElement;
// Java: import org.apache.commons.math3.analysis.RealFieldUnivariateFunction
use crate::org::apache::commons::math3::analysis::RealFieldUnivariateFunction;
// Java: import org.apache.commons.math3.analysis.solvers.AllowedSolution
use crate::org::apache::commons::math3::analysis::solvers::AllowedSolution;
// Java: import org.apache.commons.math3.analysis.solvers.BracketedRealFieldUnivariateSolver
use crate::org::apache::commons::math3::analysis::solvers::BracketedRealFieldUnivariateSolver;
// Java: import org.apache.commons.math3.exception.MaxCountExceededException
use crate::org::apache::commons::math3::exception::MaxCountExceededException;
// Java: import org.apache.commons.math3.exception.NoBracketingException
use crate::org::apache::commons::math3::exception::NoBracketingException;
// Java: import org.apache.commons.math3.ode.FieldODEState
use crate::org::apache::commons::math3::ode::FieldODEState;
// Java: import org.apache.commons.math3.ode.FieldODEStateAndDerivative
use crate::org::apache::commons::math3::ode::FieldODEStateAndDerivative;
// Java: import org.apache.commons.math3.ode.sampling.FieldStepInterpolator
use crate::org::apache::commons::math3::ode::sampling::FieldStepInterpolator;
// Java: import org.apache.commons.math3.util.FastMath
use crate::org::apache::commons::math3::util::FastMath;

// Java: public class handles
pub struct handles {
}

// Java: public class handles
pub struct handles_2 {
}

// Java: public class FieldEventState
pub struct FieldEventState {
    // Java field: FieldEventHandler<T> handler
    pub(crate) handler: Box<dyn std::any::Any>,
    // Java field: double maxCheckInterval
    pub(crate) maxCheckInterval: f64,
    // Java field: T convergence
    pub(crate) convergence: Box<dyn std::any::Any>,
    // Java field: int maxIterationCount
    pub(crate) maxIterationCount: i32,
    // Java field: T t0
    pub(crate) t0: Box<dyn std::any::Any>,
    // Java field: T g0
    pub(crate) g0: Box<dyn std::any::Any>,
    // Java field: boolean g0Positive
    pub(crate) g0Positive: bool,
    // Java field: boolean pendingEvent
    pub(crate) pendingEvent: bool,
    // Java field: T pendingEventTime
    pub(crate) pendingEventTime: Box<dyn std::any::Any>,
    // Java field: T previousEventTime
    pub(crate) previousEventTime: Box<dyn std::any::Any>,
    // Java field: boolean forward
    pub(crate) forward: bool,
    // Java field: boolean increasing
    pub(crate) increasing: bool,
    // Java field: Action nextAction
    pub(crate) nextAction: Box<dyn std::any::Any>,
    // Java field: BracketedRealFieldUnivariateSolver<T> solver
    solver: Box<dyn std::any::Any>,
}

// Java: public class handles
impl handles {
}

// Java: public class handles
impl handles_2 {
}

// Java: public class FieldEventState
impl FieldEventState {
    // Java: FieldEventState()
    pub fn FieldEventState(&self) {
    }

    // Java: FieldEventHandler<T> getEventHandler()
    pub fn getEventHandler(&self) -> Box<dyn std::any::Any> {
        // Java: return handler;
        todo!()
    }

    // Java: double getMaxCheckInterval()
    pub fn getMaxCheckInterval(&self) -> f64 {
        // Java: return maxCheckInterval;
        todo!()
    }

    // Java: T getConvergence()
    pub fn getConvergence(&self) -> Box<dyn std::any::Any> {
        // Java: return convergence;
        todo!()
    }

    // Java: int getMaxIterationCount()
    pub fn getMaxIterationCount(&self) -> i32 {
        // Java: return maxIterationCount;
        todo!()
    }

    // Java: void reinitializeBegin(final FieldStepInterpolator<T> interpolator)
    pub fn reinitializeBegin(&self) {
    }

    // Java: boolean evaluateStep(final FieldStepInterpolator<T> interpolator)
    pub fn evaluateStep(&self) -> bool {
        todo!()
    }

    // Java: T getEventTime()
    pub fn getEventTime(&self) -> Box<dyn std::any::Any> {
        // Java: return pendingEvent ?
               // Java: pendingEventTime :
               // Java: t0.getField().getZero().add(forward ? Double.POSITIVE_INFINITY : Double.NEGATIVE_INFINITY);
        todo!()
    }

    // Java: void stepAccepted(final FieldODEStateAndDerivative<T> state)
    pub fn stepAccepted(&self) {
        // Java: t0 = state.getTime();
        // Java: g0 = handler.g(state);
        // Java: if (pendingEvent && pendingEventTime.subtract(state.getTime()).abs().subtract(convergence).getReal() <= 0)
        {
            // Java: // force the sign to its value "just after the event"
            // Java: previousEventTime = state.getTime();
            // Java: g0Positive        = increasing;
            // Java: nextAction        = handler.eventOccurred(state, !(increasing ^ forward));
        }
        // Java: else
        {
            // Java: g0Positive = g0.getReal() >= 0;
            // Java: nextAction = Action.CONTINUE;
        }
    }

    // Java: boolean stop()
    pub fn stop(&self) -> bool {
        // Java: return nextAction == Action.STOP;
        todo!()
    }

    // Java: FieldODEState<T> reset(final FieldODEStateAndDerivative<T> state)
    pub fn reset(&self) -> Box<dyn std::any::Any> {
        // Java: if (!(pendingEvent && pendingEventTime.subtract(state.getTime()).abs().subtract(convergence).getReal() <= 0))
        {
            // Java: return null;
        }
        // Java: final FieldODEState<T> newState;
        // Java: if (nextAction == Action.RESET_STATE)
        {
            // Java: newState = handler.resetState(state);
        }
        // Java: else if (nextAction == Action.RESET_DERIVATIVES)
        {
            // Java: newState = state;
        }
        // Java: else
        {
            // Java: newState = null;
        }
        // Java: pendingEvent      = false;
        // Java: pendingEventTime  = null;
        // Java: return newState;
        todo!()
    }

}
