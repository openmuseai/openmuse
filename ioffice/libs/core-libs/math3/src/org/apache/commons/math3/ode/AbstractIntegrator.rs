// Java: import java.util.ArrayList
// Java: import java.util.Collection
// Java: import java.util.Collections
// Java: import java.util.Comparator
// Java: import java.util.Iterator
// Java: import java.util.List
// Java: import java.util.SortedSet
// Java: import java.util.TreeSet
// Java: import org.apache.commons.math3.analysis.solvers.BracketingNthOrderBrentSolver
use crate::org::apache::commons::math3::analysis::solvers::BracketingNthOrderBrentSolver;
// Java: import org.apache.commons.math3.analysis.solvers.UnivariateSolver
use crate::org::apache::commons::math3::analysis::solvers::UnivariateSolver;
// Java: import org.apache.commons.math3.exception.DimensionMismatchException
use crate::org::apache::commons::math3::exception::DimensionMismatchException;
// Java: import org.apache.commons.math3.exception.MaxCountExceededException
use crate::org::apache::commons::math3::exception::MaxCountExceededException;
// Java: import org.apache.commons.math3.exception.NoBracketingException
use crate::org::apache::commons::math3::exception::NoBracketingException;
// Java: import org.apache.commons.math3.exception.NumberIsTooSmallException
use crate::org::apache::commons::math3::exception::NumberIsTooSmallException;
// Java: import org.apache.commons.math3.exception.util.LocalizedFormats
use crate::org::apache::commons::math3::exception::util::LocalizedFormats;
// Java: import org.apache.commons.math3.ode.events.EventHandler
use crate::org::apache::commons::math3::ode::events::EventHandler;
// Java: import org.apache.commons.math3.ode.events.EventState
use crate::org::apache::commons::math3::ode::events::EventState;
// Java: import org.apache.commons.math3.ode.sampling.AbstractStepInterpolator
use crate::org::apache::commons::math3::ode::sampling::AbstractStepInterpolator;
// Java: import org.apache.commons.math3.ode.sampling.StepHandler
use crate::org::apache::commons::math3::ode::sampling::StepHandler;
// Java: import org.apache.commons.math3.util.FastMath
use crate::org::apache::commons::math3::util::FastMath;
// Java: import org.apache.commons.math3.util.IntegerSequence
use crate::org::apache::commons::math3::util::IntegerSequence;
// Java: import org.apache.commons.math3.util.Precision
use crate::org::apache::commons::math3::util::Precision;

// Java: public class managing
pub struct managing {
}

// Java: public class AbstractIntegrator
pub struct AbstractIntegrator {
    // Java field: Collection<StepHandler> stepHandlers
    pub(crate) stepHandlers: Box<dyn std::any::Any>,
    // Java field: double stepStart
    pub(crate) stepStart: f64,
    // Java field: double stepSize
    pub(crate) stepSize: f64,
    // Java field: boolean isLastStep
    pub(crate) isLastStep: bool,
    // Java field: boolean resetOccurred
    pub(crate) resetOccurred: bool,
    // Java field: Collection<EventState> eventsStates
    pub(crate) eventsStates: Box<dyn std::any::Any>,
    // Java field: boolean statesInitialized
    pub(crate) statesInitialized: bool,
    // Java field: String name
    pub(crate) name: Box<dyn std::any::Any>,
    // Java field: IntegerSequence.Incrementor evaluations
    pub(crate) evaluations: Box<dyn std::any::Any>,
    // Java field: ExpandableStatefulODE expandable
    pub(crate) expandable: Box<dyn std::any::Any>,
    // Java field: MaxCountExceededException, NoBracketingException
}

// Java: public class managing
impl managing {
}

// Java: public class AbstractIntegrator
impl AbstractIntegrator {
    // Java: AbstractIntegrator(final String name)
    pub fn AbstractIntegrator(&self) {
        // Java: this.name = name;
        // Java: stepHandlers = new ArrayList<StepHandler>();
        // Java: stepStart = Double.NaN;
        // Java: stepSize  = Double.NaN;
        // Java: eventsStates = new ArrayList<EventState>();
        // Java: statesInitialized = false;
        // Java: evaluations = IntegerSequence.Incrementor.create().withMaximalCount(Integer.MAX_VALUE);
    }

    // Java: AbstractIntegrator()
    pub fn AbstractIntegrator_2(&self) {
        // Java: this(null);
    }

    // Java: String getName()
    pub fn getName(&self) -> String {
        // Java: return name;
        todo!()
    }

    // Java: void addStepHandler(final StepHandler handler)
    pub fn addStepHandler(&self) {
        // Java: stepHandlers.add(handler);
    }

    // Java: Collection<StepHandler> getStepHandlers()
    pub fn getStepHandlers(&self) -> Vec<Box<dyn std::any::Any>> {
        // Java: return Collections.unmodifiableCollection(stepHandlers);
        todo!()
    }

    // Java: void clearStepHandlers()
    pub fn clearStepHandlers(&self) {
        // Java: stepHandlers.clear();
    }

    // Java: void addEventHandler()
    pub fn addEventHandler(&self) {
    }

    // Java: void addEventHandler()
    pub fn addEventHandler_2(&self) {
    }

    // Java: Collection<EventHandler> getEventHandlers()
    pub fn getEventHandlers(&self) -> Vec<Box<dyn std::any::Any>> {
        // Java: final List<EventHandler> list = new ArrayList<EventHandler>(eventsStates.size());
        // Java: for (EventState state : eventsStates)
        {
            // Java: list.add(state.getEventHandler());
        }
        // Java: return Collections.unmodifiableCollection(list);
        todo!()
    }

    // Java: void clearEventHandlers()
    pub fn clearEventHandlers(&self) {
        // Java: eventsStates.clear();
    }

    // Java: double getCurrentStepStart()
    pub fn getCurrentStepStart(&self) -> f64 {
        // Java: return stepStart;
        todo!()
    }

    // Java: double getCurrentSignedStepsize()
    pub fn getCurrentSignedStepsize(&self) -> f64 {
        // Java: return stepSize;
        todo!()
    }

    // Java: void setMaxEvaluations(int maxEvaluations)
    pub fn setMaxEvaluations(&self) {
        // Java: evaluations = evaluations.withMaximalCount((maxEvaluations < 0) ? Integer.MAX_VALUE : maxEvaluations);
    }

    // Java: int getMaxEvaluations()
    pub fn getMaxEvaluations(&self) -> i32 {
        // Java: return evaluations.getMaximalCount();
        todo!()
    }

    // Java: int getEvaluations()
    pub fn getEvaluations(&self) -> i32 {
        // Java: return evaluations.getCount();
        todo!()
    }

    // Java: void initIntegration(final double t0, final double[] y0, final double t)
    pub fn initIntegration(&self) {
        // Java: evaluations = evaluations.withStart(0);
        // Java: for (final EventState state : eventsStates)
        {
            // Java: state.setExpandable(expandable);
            // Java: state.getEventHandler().init(t0, y0, t);
        }
        // Java: for (StepHandler handler : stepHandlers)
        {
            // Java: handler.init(t0, y0, t);
        }
        // Java: setStateInitialized(false);
    }

    // Java: void setEquations(final ExpandableStatefulODE equations)
    pub fn setEquations(&self) {
        // Java: this.expandable = equations;
    }

    // Java: ExpandableStatefulODE getExpandable()
    pub fn getExpandable(&self) -> Box<dyn std::any::Any> {
        // Java: return expandable;
        todo!()
    }

    // Java: double integrate()
    pub fn integrate(&self) -> f64 {
        todo!()
    }

    // Java: abstract void integrate(ExpandableStatefulODE equations, double t)
    pub fn integrate_2(&self) {
    }

    // Java: void computeDerivatives(final double t, final double[] y, final double[] yDot)
    pub fn computeDerivatives(&self) {
    }

    // Java: void setStateInitialized(final boolean stateInitialized)
    pub fn setStateInitialized(&self) {
        // Java: this.statesInitialized = stateInitialized;
    }

    // Java: double acceptStep()
    pub fn acceptStep(&self) -> f64 {
        todo!()
    }

    // Java: void sanityChecks(final ExpandableStatefulODE equations, final double t)
    pub fn sanityChecks(&self) {
    }

}
