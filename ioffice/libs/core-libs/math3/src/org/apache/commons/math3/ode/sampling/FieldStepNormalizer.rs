// Java: import org.apache.commons.math3.RealFieldElement
use crate::org::apache::commons::math3::RealFieldElement;
// Java: import org.apache.commons.math3.exception.MaxCountExceededException
use crate::org::apache::commons::math3::exception::MaxCountExceededException;
// Java: import org.apache.commons.math3.ode.FieldODEStateAndDerivative
use crate::org::apache::commons::math3::ode::FieldODEStateAndDerivative;
// Java: import org.apache.commons.math3.util.FastMath
use crate::org::apache::commons::math3::util::FastMath;
// Java: import org.apache.commons.math3.util.Precision
use crate::org::apache::commons::math3::util::Precision;

// Java: public class wraps
pub struct wraps {
}

// Java: public class FieldStepNormalizer extends RealFieldElement implements FieldStepHandler
pub struct FieldStepNormalizer {
    // Java field: double h
    h: f64,
    // Java field: FieldFixedStepHandler<T> handler
    handler: Option<Box<dyn std::any::Any>>,
    // Java field: FieldODEStateAndDerivative<T> first
    first: Option<Box<dyn std::any::Any>>,
    // Java field: FieldODEStateAndDerivative<T> last
    last: Option<Box<dyn std::any::Any>>,
    // Java field: boolean forward
    forward: bool,
    // Java field: StepNormalizerBounds bounds
    bounds: Option<Box<dyn std::any::Any>>,
    // Java field: StepNormalizerMode mode
    mode: Option<Box<dyn std::any::Any>>,
}

// Java: public class wraps
impl wraps {
}

// Java: public class FieldStepNormalizer extends RealFieldElement implements FieldStepHandler
impl FieldStepNormalizer {
    // Java: FieldStepNormalizer(final double h, final FieldFixedStepHandler<T> handler)
    pub fn FieldStepNormalizer(&self) {
        // Java: this(h, handler, StepNormalizerMode.INCREMENT,
             // Java: StepNormalizerBounds.FIRST);
    }

    // Java: FieldStepNormalizer()
    pub fn FieldStepNormalizer_2(&self) {
    }

    // Java: FieldStepNormalizer()
    pub fn FieldStepNormalizer_3(&self) {
    }

    // Java: FieldStepNormalizer()
    pub fn FieldStepNormalizer_4(&self) {
    }

    // Java: void init(final FieldODEStateAndDerivative<T> initialState, final T finalTime)
    pub fn init(&self) {
        // Java: first   = null;
        // Java: last    = null;
        // Java: forward = true;
        // Java: // initialize the underlying handler
        // Java: handler.init(initialState, finalTime);
    }

    // Java: void handleStep(final FieldStepInterpolator<T> interpolator, final boolean isLast)
    pub fn handleStep(&self) {
    }

    // Java: boolean isNextInStep(final T nextTime, final FieldStepInterpolator<T> interpolator)
    pub fn isNextInStep(&self) -> bool {
        // Java: return forward ?
               // Java: nextTime.getReal() <= interpolator.getCurrentState().getTime().getReal() :
               // Java: nextTime.getReal() >= interpolator.getCurrentState().getTime().getReal();
        todo!()
    }

    // Java: void doNormalizedStep(final boolean isLast)
    pub fn doNormalizedStep(&self) {
        // Java: if (!bounds.firstIncluded() && first.getTime().getReal() == last.getTime().getReal())
        {
            // Java: return;
        }
        // Java: handler.handleStep(last, isLast);
    }

}

