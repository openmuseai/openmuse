// Java: import org.apache.commons.math3.analysis.DifferentiableUnivariateFunction
use crate::org::apache::commons::math3::analysis::DifferentiableUnivariateFunction;
// Java: import org.apache.commons.math3.analysis.FunctionUtils
use crate::org::apache::commons::math3::analysis::FunctionUtils;
// Java: import org.apache.commons.math3.analysis.UnivariateFunction
use crate::org::apache::commons::math3::analysis::UnivariateFunction;
// Java: import org.apache.commons.math3.analysis.differentiation.DerivativeStructure
use crate::org::apache::commons::math3::analysis::differentiation::DerivativeStructure;
// Java: import org.apache.commons.math3.analysis.differentiation.UnivariateDifferentiableFunction
use crate::org::apache::commons::math3::analysis::differentiation::UnivariateDifferentiableFunction;
// Java: import org.apache.commons.math3.exception.DimensionMismatchException
use crate::org::apache::commons::math3::exception::DimensionMismatchException;
// Java: import org.apache.commons.math3.util.FastMath
use crate::org::apache::commons::math3::util::FastMath;

// Java field: double SHORTCUT
pub(crate) const SHORTCUT: f64 = 0.0;

// Java: public class Sinc implements UnivariateDifferentiableFunction, DifferentiableUnivariateFunction
pub struct Sinc {
    // Java field: boolean normalized
    normalized: bool,
}

// Java: public class Sinc implements UnivariateDifferentiableFunction, DifferentiableUnivariateFunction
impl Sinc {
    // Java: Sinc()
    pub fn Sinc(&self) {
        // Java: this(false);
    }

    // Java: Sinc(boolean normalized)
    pub fn Sinc_2(&self) {
        // Java: this.normalized = normalized;
    }

    // Java: double value(final double x)
    pub fn value(&self) -> f64 {
        // Java: final double scaledX = normalized ? FastMath.PI * x : x;
        // Java: if (FastMath.abs(scaledX) <= SHORTCUT)
        {
            // Java: // use Taylor series
            // Java: final double scaledX2 = scaledX * scaledX;
            // Java: return ((scaledX2 - 20) * scaledX2 + 120) / 120;
        }
        // Java: else
        {
            // Java: // use definition expression
            // Java: return FastMath.sin(scaledX) / scaledX;
        }
        todo!()
    }

    // Java: UnivariateFunction derivative()
    pub fn derivative(&self) -> Box<dyn std::any::Any> {
        // Java: return FunctionUtils.toDifferentiableUnivariateFunction(this).derivative();
        todo!()
    }

    // Java: DerivativeStructure value(final DerivativeStructure t)
    pub fn value_2(&self) -> Box<dyn std::any::Any> {
        todo!()
    }

}


