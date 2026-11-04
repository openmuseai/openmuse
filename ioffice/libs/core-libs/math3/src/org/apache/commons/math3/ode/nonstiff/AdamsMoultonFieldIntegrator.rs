// Java: import java.util.Arrays
// Java: import org.apache.commons.math3.Field
use crate::org::apache::commons::math3::Field;
// Java: import org.apache.commons.math3.RealFieldElement
use crate::org::apache::commons::math3::RealFieldElement;
// Java: import org.apache.commons.math3.exception.DimensionMismatchException
use crate::org::apache::commons::math3::exception::DimensionMismatchException;
// Java: import org.apache.commons.math3.exception.MaxCountExceededException
use crate::org::apache::commons::math3::exception::MaxCountExceededException;
// Java: import org.apache.commons.math3.exception.NoBracketingException
use crate::org::apache::commons::math3::exception::NoBracketingException;
// Java: import org.apache.commons.math3.exception.NumberIsTooSmallException
use crate::org::apache::commons::math3::exception::NumberIsTooSmallException;
// Java: import org.apache.commons.math3.linear.Array2DRowFieldMatrix
use crate::org::apache::commons::math3::linear::Array2DRowFieldMatrix;
// Java: import org.apache.commons.math3.linear.FieldMatrixPreservingVisitor
use crate::org::apache::commons::math3::linear::FieldMatrixPreservingVisitor;
// Java: import org.apache.commons.math3.ode.FieldExpandableODE
use crate::org::apache::commons::math3::ode::FieldExpandableODE;
// Java: import org.apache.commons.math3.ode.FieldODEState
use crate::org::apache::commons::math3::ode::FieldODEState;
// Java: import org.apache.commons.math3.ode.FieldODEStateAndDerivative
use crate::org::apache::commons::math3::ode::FieldODEStateAndDerivative;
// Java: import org.apache.commons.math3.util.MathArrays
use crate::org::apache::commons::math3::util::MathArrays;
// Java: import org.apache.commons.math3.util.MathUtils
use crate::org::apache::commons::math3::util::MathUtils;

// Java field: String METHOD_NAME
pub(crate) const METHOD_NAME: &str = "";

// Java: public class implements implements implicit Adams
pub struct implements {
}

// Java: public class AdamsMoultonFieldIntegrator
pub struct AdamsMoultonFieldIntegrator {
}

// Java: public class Corrector
pub struct Corrector {
    // Java field: T[] previous
    pub(crate) previous: Box<dyn std::any::Any>,
    // Java field: T[] scaled
    pub(crate) scaled: Box<dyn std::any::Any>,
    // Java field: T[] before
    pub(crate) before: Box<dyn std::any::Any>,
    // Java field: T[] after
    pub(crate) after: Box<dyn std::any::Any>,
}

// Java: public class implements implements implicit Adams
impl implements {
}

// Java: public class AdamsMoultonFieldIntegrator
impl AdamsMoultonFieldIntegrator {
    // Java: AdamsMoultonFieldIntegrator()
    pub fn AdamsMoultonFieldIntegrator(&self) {
    }

    // Java: AdamsMoultonFieldIntegrator()
    pub fn AdamsMoultonFieldIntegrator_2(&self) {
    }

    // Java: FieldODEStateAndDerivative<T> integrate()
    pub fn integrate(&self) -> Box<dyn std::any::Any> {
        todo!()
    }

}

// Java: public class Corrector
impl Corrector {
    // Java: void start()
    pub fn start(&self) {
    }

    // Java: void visit(int row, int column, T value)
    pub fn visit(&self) {
            // Java: if ((row & 0x1) == 0)
            {
                // Java: after[column] = after[column].subtract(value);
            }
            // Java: else
            {
                // Java: after[column] = after[column].add(value);
            }
    }

    // Java: T end()
    pub fn end(&self) -> Box<dyn std::any::Any> {
            // Java: T error = getField().getZero();
            // Java: for (int i = 0; i < after.length; ++i)
            {
                // Java: after[i] = after[i].add(previous[i].add(scaled[i]));
                // Java: if (i < mainSetDimension)
                {
                    // Java: final T yScale = MathUtils.max(previous[i].abs(), after[i].abs());
                    // Java: final T tol = (vecAbsoluteTolerance == null) ?
                                  // Java: yScale.multiply(scalRelativeTolerance).add(scalAbsoluteTolerance) :
                                  // Java: yScale.multiply(vecRelativeTolerance[i]).add(vecAbsoluteTolerance[i]);
                    // Java: final T ratio  = after[i].subtract(before[i]).divide(tol); // (corrected-predicted)/tol
                    // Java: error = error.add(ratio.multiply(ratio));
                }
            }
            // Java: return error.divide(mainSetDimension).sqrt();
        todo!()
    }

}

impl r#implements {
}
