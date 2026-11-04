// Java: import java.util.ArrayList
// Java: import java.util.Collection
// Java: import java.util.Collections
// Java: import java.util.Comparator
// Java: import java.util.Iterator
// Java: import java.util.List
// Java: import java.util.SortedSet
// Java: import java.util.TreeSet
// Java: import org.apache.commons.math3.Field
use crate::org::apache::commons::math3::Field;
// Java: import org.apache.commons.math3.RealFieldElement
use crate::org::apache::commons::math3::RealFieldElement;
// Java: import org.apache.commons.math3.analysis.solvers.BracketedRealFieldUnivariateSolver
use crate::org::apache::commons::math3::analysis::solvers::BracketedRealFieldUnivariateSolver;
// Java: import org.apache.commons.math3.analysis.solvers.FieldBracketingNthOrderBrentSolver
use crate::org::apache::commons::math3::analysis::solvers::FieldBracketingNthOrderBrentSolver;
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
// Java: import org.apache.commons.math3.ode.events.FieldEventHandler
use crate::org::apache::commons::math3::ode::events::FieldEventHandler;
// Java: import org.apache.commons.math3.ode.events.FieldEventState
use crate::org::apache::commons::math3::ode::events::FieldEventState;
// Java: import org.apache.commons.math3.ode.sampling.AbstractFieldStepInterpolator
use crate::org::apache::commons::math3::ode::sampling::AbstractFieldStepInterpolator;
// Java: import org.apache.commons.math3.ode.sampling.FieldStepHandler
use crate::org::apache::commons::math3::ode::sampling::FieldStepHandler;
// Java: import org.apache.commons.math3.util.FastMath
use crate::org::apache::commons::math3::util::FastMath;
// Java: import org.apache.commons.math3.util.IntegerSequence
use crate::org::apache::commons::math3::util::IntegerSequence;

// Java field: double DEFAULT_RELATIVE_ACCURACY
pub(crate) const DEFAULT_RELATIVE_ACCURACY: f64 = 0.0;
// Java field: double DEFAULT_FUNCTION_VALUE_ACCURACY
pub(crate) const DEFAULT_FUNCTION_VALUE_ACCURACY: f64 = 0.0;

// Java: public class managing
pub struct managing {
}

// Java: public class AbstractFieldIntegrator
pub struct AbstractFieldIntegrator {
    // Java field: Collection<FieldStepHandler<T>> stepHandlers
    pub(crate) stepHandlers: Box<dyn std::any::Any>,
    // Java field: FieldODEStateAndDerivative<T> stepStart
    pub(crate) stepStart: Box<dyn std::any::Any>,
    // Java field: T stepSize
    pub(crate) stepSize: Box<dyn std::any::Any>,
    // Java field: boolean isLastStep
    pub(crate) isLastStep: bool,
    // Java field: boolean resetOccurred
    pub(crate) resetOccurred: bool,
    // Java field: Field<T> field
    pub(crate) field: Box<dyn std::any::Any>,
    // Java field: Collection<FieldEventState<T>> eventsStates
    pub(crate) eventsStates: Box<dyn std::any::Any>,
    // Java field: boolean statesInitialized
    pub(crate) statesInitialized: bool,
    // Java field: String name
    pub(crate) name: Box<dyn std::any::Any>,
    // Java field: IntegerSequence.Incrementor evaluations
    pub(crate) evaluations: Box<dyn std::any::Any>,
    // Java field: FieldExpandableODE<T> equations
    pub(crate) equations: Box<dyn std::any::Any>,
}

// Java: public class managing
impl managing {
}

// Java: public class AbstractFieldIntegrator
impl AbstractFieldIntegrator {
    // Java: AbstractFieldIntegrator(final Field<T> field, final String name)
    pub fn AbstractFieldIntegrator(&self) {
        // Java: this.field        = field;
        // Java: this.name         = name;
        // Java: stepHandlers      = new ArrayList<FieldStepHandler<T>>();
        // Java: stepStart         = null;
        // Java: stepSize          = null;
        // Java: eventsStates      = new ArrayList<FieldEventState<T>>();
        // Java: statesInitialized = false;
        // Java: evaluations       = IntegerSequence.Incrementor.create().withMaximalCount(Integer.MAX_VALUE);
    }

    // Java: Field<T> getField()
    pub fn getField(&self) -> Box<dyn std::any::Any> {
        // Java: return field;
        todo!()
    }

    // Java: String getName()
    pub fn getName(&self) -> String {
        // Java: return name;
        todo!()
    }

    // Java: void addStepHandler(final FieldStepHandler<T> handler)
    pub fn addStepHandler(&self) {
        // Java: stepHandlers.add(handler);
    }

    // Java: Collection<FieldStepHandler<T>> getStepHandlers()
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

    // Java: Collection<FieldEventHandler<T>> getEventHandlers()
    pub fn getEventHandlers(&self) -> Vec<Box<dyn std::any::Any>> {
        // Java: final List<FieldEventHandler<T>> list = new ArrayList<FieldEventHandler<T>>(eventsStates.size());
        // Java: for (FieldEventState<T> state : eventsStates)
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

    // Java: FieldODEStateAndDerivative<T> getCurrentStepStart()
    pub fn getCurrentStepStart(&self) -> Box<dyn std::any::Any> {
        // Java: return stepStart;
        todo!()
    }

    // Java: T getCurrentSignedStepsize()
    pub fn getCurrentSignedStepsize(&self) -> Box<dyn std::any::Any> {
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

    // Java: FieldODEStateAndDerivative<T> initIntegration()
    pub fn initIntegration(&self) -> Box<dyn std::any::Any> {
        todo!()
    }

    // Java: FieldExpandableODE<T> getEquations()
    pub fn getEquations(&self) -> Box<dyn std::any::Any> {
        // Java: return equations;
        todo!()
    }

    // Java: T[] computeDerivatives(final T t, final T[] y)
    pub fn computeDerivatives(&self) -> Box<dyn std::any::Any> {
        todo!()
    }

    // Java: void setStateInitialized(final boolean stateInitialized)
    pub fn setStateInitialized(&self) {
        // Java: this.statesInitialized = stateInitialized;
    }

    // Java: FieldODEStateAndDerivative<T> acceptStep()
    pub fn acceptStep(&self) -> Box<dyn std::any::Any> {
        todo!()
    }

    // Java: void sanityChecks(final FieldODEState<T> eqn, final T t)
    pub fn sanityChecks(&self) {
    }

    // Java: boolean resetOccurred()
    pub fn resetOccurred(&self) -> bool {
        // Java: return resetOccurred;
        todo!()
    }

    // Java: void setStepSize(final T stepSize)
    pub fn setStepSize(&self) {
        // Java: this.stepSize = stepSize;
    }

    // Java: T getStepSize()
    pub fn getStepSize(&self) -> Box<dyn std::any::Any> {
        // Java: return stepSize;
        todo!()
    }

    // Java: void setStepStart(final FieldODEStateAndDerivative<T> stepStart)
    pub fn setStepStart(&self) {
        // Java: this.stepStart = stepStart;
    }

    // Java: FieldODEStateAndDerivative<T> getStepStart()
    pub fn getStepStart(&self) -> Box<dyn std::any::Any> {
        // Java: return stepStart;
        todo!()
    }

    // Java: void setIsLastStep(final boolean isLastStep)
    pub fn setIsLastStep(&self) {
        // Java: this.isLastStep = isLastStep;
    }

    // Java: boolean isLastStep()
    pub fn isLastStep(&self) -> bool {
        // Java: return isLastStep;
        todo!()
    }

}
