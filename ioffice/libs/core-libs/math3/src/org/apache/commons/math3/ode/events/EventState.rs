// Java: import org.apache.commons.math3.analysis.UnivariateFunction
use crate::org::apache::commons::math3::analysis::UnivariateFunction;
// Java: import org.apache.commons.math3.analysis.solvers.AllowedSolution
use crate::org::apache::commons::math3::analysis::solvers::AllowedSolution;
// Java: import org.apache.commons.math3.analysis.solvers.BracketedUnivariateSolver
use crate::org::apache::commons::math3::analysis::solvers::BracketedUnivariateSolver;
// Java: import org.apache.commons.math3.analysis.solvers.PegasusSolver
use crate::org::apache::commons::math3::analysis::solvers::PegasusSolver;
// Java: import org.apache.commons.math3.analysis.solvers.UnivariateSolver
use crate::org::apache::commons::math3::analysis::solvers::UnivariateSolver;
// Java: import org.apache.commons.math3.analysis.solvers.UnivariateSolverUtils
use crate::org::apache::commons::math3::analysis::solvers::UnivariateSolverUtils;
// Java: import org.apache.commons.math3.exception.MaxCountExceededException
use crate::org::apache::commons::math3::exception::MaxCountExceededException;
// Java: import org.apache.commons.math3.exception.NoBracketingException
use crate::org::apache::commons::math3::exception::NoBracketingException;
// Java: import org.apache.commons.math3.ode.EquationsMapper
use crate::org::apache::commons::math3::ode::EquationsMapper;
// Java: import org.apache.commons.math3.ode.ExpandableStatefulODE
use crate::org::apache::commons::math3::ode::ExpandableStatefulODE;
// Java: import org.apache.commons.math3.ode.sampling.StepInterpolator
use crate::org::apache::commons::math3::ode::sampling::StepInterpolator;
// Java: import org.apache.commons.math3.util.FastMath
use crate::org::apache::commons::math3::util::FastMath;

// Java field: long serialVersionUID

// Java: public class handles
pub struct handles {
}

// Java: public class handles
pub struct handles_2 {
}

// Java: public class EventState
pub struct EventState {
    // Java field: EventHandler handler
    pub(crate) handler: Box<dyn std::any::Any>,
    // Java field: double maxCheckInterval
    pub(crate) maxCheckInterval: f64,
    // Java field: double convergence
    pub(crate) convergence: f64,
    // Java field: int maxIterationCount
    pub(crate) maxIterationCount: i32,
    // Java field: ExpandableStatefulODE expandable
    pub(crate) expandable: Box<dyn std::any::Any>,
    // Java field: double t0
    pub(crate) t0: f64,
    // Java field: double g0
    pub(crate) g0: f64,
    // Java field: boolean g0Positive
    pub(crate) g0Positive: bool,
    // Java field: boolean pendingEvent
    pub(crate) pendingEvent: bool,
    // Java field: double pendingEventTime
    pub(crate) pendingEventTime: f64,
    // Java field: double previousEventTime
    pub(crate) previousEventTime: f64,
    // Java field: boolean forward
    pub(crate) forward: bool,
    // Java field: boolean increasing
    pub(crate) increasing: bool,
    // Java field: EventHandler.Action nextAction
    pub(crate) nextAction: Box<dyn std::any::Any>,
    // Java field: UnivariateSolver solver
    solver: Box<dyn std::any::Any>,
}

// Java: public class LocalMaxCountExceededException
pub struct LocalMaxCountExceededException {
    // Java field: MaxCountExceededException wrapped
    pub(crate) wrapped: Box<dyn std::any::Any>,
}

// Java: public class handles
impl handles {
}

// Java: public class handles
impl handles_2 {
}

// Java: public class EventState
impl EventState {
    // Java: EventState()
    pub fn EventState(&self) {
    }

    // Java: EventHandler getEventHandler()
    pub fn getEventHandler(&self) -> Box<dyn std::any::Any> {
        // Java: return handler;
        todo!()
    }

    // Java: void setExpandable(final ExpandableStatefulODE expandable)
    pub fn setExpandable(&self) {
        // Java: this.expandable = expandable;
    }

    // Java: double getMaxCheckInterval()
    pub fn getMaxCheckInterval(&self) -> f64 {
        // Java: return maxCheckInterval;
        todo!()
    }

    // Java: double getConvergence()
    pub fn getConvergence(&self) -> f64 {
        // Java: return convergence;
        todo!()
    }

    // Java: int getMaxIterationCount()
    pub fn getMaxIterationCount(&self) -> i32 {
        // Java: return maxIterationCount;
        todo!()
    }

    // Java: void reinitializeBegin(final StepInterpolator interpolator)
    pub fn reinitializeBegin(&self) {
    }

    // Java: double[] getCompleteState(final StepInterpolator interpolator)
    pub fn getCompleteState(&self) -> Vec<f64> {
        // Java: final double[] complete = new double[expandable.getTotalDimension()];
        // Java: expandable.getPrimaryMapper().insertEquationData(interpolator.getInterpolatedState(),
                                                         // Java: complete);
        // Java: int index = 0;
        // Java: for (EquationsMapper secondary : expandable.getSecondaryMappers())
        {
            // Java: secondary.insertEquationData(interpolator.getInterpolatedSecondaryState(index++),
                                         // Java: complete);
        }
        // Java: return complete;
        todo!()
    }

    // Java: boolean evaluateStep(final StepInterpolator interpolator)
    pub fn evaluateStep(&self) -> bool {
        todo!()
    }

    // Java: double getEventTime()
    pub fn getEventTime(&self) -> f64 {
        // Java: return pendingEvent ?
               // Java: pendingEventTime :
               // Java: (forward ? Double.POSITIVE_INFINITY : Double.NEGATIVE_INFINITY);
        todo!()
    }

    // Java: void stepAccepted(final double t, final double[] y)
    pub fn stepAccepted(&self) {
        // Java: t0 = t;
        // Java: g0 = handler.g(t, y);
        // Java: if (pendingEvent && (FastMath.abs(pendingEventTime - t) <= convergence))
        {
            // Java: // force the sign to its value "just after the event"
            // Java: previousEventTime = t;
            // Java: g0Positive        = increasing;
            // Java: nextAction        = handler.eventOccurred(t, y, !(increasing ^ forward));
        }
        // Java: else
        {
            // Java: g0Positive = g0 >= 0;
            // Java: nextAction = EventHandler.Action.CONTINUE;
        }
    }

    // Java: boolean stop()
    pub fn stop(&self) -> bool {
        // Java: return nextAction == EventHandler.Action.STOP;
        todo!()
    }

    // Java: boolean reset(final double t, final double[] y)
    pub fn reset(&self) -> bool {
        // Java: if (!(pendingEvent && (FastMath.abs(pendingEventTime - t) <= convergence)))
        {
            // Java: return false;
        }
        // Java: if (nextAction == EventHandler.Action.RESET_STATE)
        {
            // Java: handler.resetState(t, y);
        }
        // Java: pendingEvent      = false;
        // Java: pendingEventTime  = Double.NaN;
        // Java: return (nextAction == EventHandler.Action.RESET_STATE) ||
               // Java: (nextAction == EventHandler.Action.RESET_DERIVATIVES);
        todo!()
    }

}

// Java: public class LocalMaxCountExceededException
impl LocalMaxCountExceededException {
    // Java: MaxCountExceededException getException()
    pub fn getException(&self) -> Box<dyn std::any::Any> {
            // Java: return wrapped;
        todo!()
    }

}
