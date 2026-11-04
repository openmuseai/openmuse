// Java: import java.util.Arrays
// Java: import org.apache.commons.math3.exception.DimensionMismatchException
use crate::org::apache::commons::math3::exception::DimensionMismatchException;
// Java: import org.apache.commons.math3.exception.MaxCountExceededException
use crate::org::apache::commons::math3::exception::MaxCountExceededException;
// Java: import org.apache.commons.math3.exception.NoBracketingException
use crate::org::apache::commons::math3::exception::NoBracketingException;
// Java: import org.apache.commons.math3.exception.NumberIsTooSmallException
use crate::org::apache::commons::math3::exception::NumberIsTooSmallException;
// Java: import org.apache.commons.math3.linear.Array2DRowRealMatrix
use crate::org::apache::commons::math3::linear::Array2DRowRealMatrix;
// Java: import org.apache.commons.math3.linear.RealMatrixPreservingVisitor
use crate::org::apache::commons::math3::linear::RealMatrixPreservingVisitor;
// Java: import org.apache.commons.math3.ode.EquationsMapper
use crate::org::apache::commons::math3::ode::EquationsMapper;
// Java: import org.apache.commons.math3.ode.ExpandableStatefulODE
use crate::org::apache::commons::math3::ode::ExpandableStatefulODE;
// Java: import org.apache.commons.math3.ode.sampling.NordsieckStepInterpolator
use crate::org::apache::commons::math3::ode::sampling::NordsieckStepInterpolator;
// Java: import org.apache.commons.math3.util.FastMath
use crate::org::apache::commons::math3::util::FastMath;

// Java field: String METHOD_NAME
pub(crate) const METHOD_NAME: &str = "";

// Java: public class implements implements implicit Adams
pub struct implements {
}

// Java: public class AdamsMoultonIntegrator
pub struct AdamsMoultonIntegrator {
}

// Java: public class Corrector
pub struct Corrector {
    // Java field: double[] previous
    pub(crate) previous: Box<dyn std::any::Any>,
    // Java field: double[] scaled
    pub(crate) scaled: Box<dyn std::any::Any>,
    // Java field: double[] before
    pub(crate) before: Box<dyn std::any::Any>,
    // Java field: double[] after
    pub(crate) after: Box<dyn std::any::Any>,
}

// Java: public class implements implements implicit Adams
impl implements {
}

// Java: public class AdamsMoultonIntegrator
impl AdamsMoultonIntegrator {
    // Java: AdamsMoultonIntegrator()
    pub fn AdamsMoultonIntegrator(&self) {
    }

    // Java: AdamsMoultonIntegrator()
    pub fn AdamsMoultonIntegrator_2(&self) {
    }

    // Java: void integrate(final ExpandableStatefulODE equations, final double t)
    pub fn integrate(&self) {
    }

}

// Java: public class Corrector
impl Corrector {
    // Java: void start()
    pub fn start(&self) {
    }

    // Java: void visit(int row, int column, double value)
    pub fn visit(&self) {
            // Java: if ((row & 0x1) == 0)
            {
                // Java: after[column] -= value;
            }
            // Java: else
            {
                // Java: after[column] += value;
            }
    }

    // Java: double end()
    pub fn end(&self) -> f64 {
            // Java: double error = 0;
            // Java: for (int i = 0; i < after.length; ++i)
            {
                // Java: after[i] += previous[i] + scaled[i];
                // Java: if (i < mainSetDimension)
                {
                    // Java: final double yScale = FastMath.max(FastMath.abs(previous[i]), FastMath.abs(after[i]));
                    // Java: final double tol    = (vecAbsoluteTolerance == null) ?
                                          // Java: (scalAbsoluteTolerance + scalRelativeTolerance * yScale) :
                                          // Java: (vecAbsoluteTolerance[i] + vecRelativeTolerance[i] * yScale);
                    // Java: final double ratio  = (after[i] - before[i]) / tol; // (corrected-predicted)/tol
                    // Java: error += ratio * ratio;
                }
            }
            // Java: return FastMath.sqrt(error / mainSetDimension);
        todo!()
    }

}

impl r#implements {
}
