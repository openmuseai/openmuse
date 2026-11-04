// Java: import org.apache.commons.math3.exception.MathIllegalArgumentException
use crate::org::apache::commons::math3::exception::MathIllegalArgumentException;
// Java: import org.apache.commons.math3.exception.MaxCountExceededException
use crate::org::apache::commons::math3::exception::MaxCountExceededException;
// Java: import org.apache.commons.math3.exception.NotStrictlyPositiveException
use crate::org::apache::commons::math3::exception::NotStrictlyPositiveException;
// Java: import org.apache.commons.math3.exception.NumberIsTooSmallException
use crate::org::apache::commons::math3::exception::NumberIsTooSmallException;
// Java: import org.apache.commons.math3.exception.TooManyEvaluationsException
use crate::org::apache::commons::math3::exception::TooManyEvaluationsException;
// Java: import org.apache.commons.math3.exception.util.LocalizedFormats
use crate::org::apache::commons::math3::exception::util::LocalizedFormats;
// Java: import org.apache.commons.math3.util.FastMath
use crate::org::apache::commons::math3::util::FastMath;

// Java: public class LegendreGaussIntegrator extends BaseAbstractUnivariateIntegrator
pub struct LegendreGaussIntegrator {
    // Java field: double[] abscissas
    abscissas: Vec<f64>,
    // Java field: double[] weights
    weights: Vec<f64>,
}

// Java: public class LegendreGaussIntegrator extends BaseAbstractUnivariateIntegrator
impl LegendreGaussIntegrator {
    // Java: LegendreGaussIntegrator()
    pub fn LegendreGaussIntegrator(&self) {
    }

    // Java: LegendreGaussIntegrator()
    pub fn LegendreGaussIntegrator_2(&self) {
    }

    // Java: LegendreGaussIntegrator()
    pub fn LegendreGaussIntegrator_3(&self) {
    }

    // Java: double doIntegrate()
    pub fn doIntegrate(&self) -> f64 {
        todo!()
    }

    // Java: double stage(final int n)
    pub fn stage(&self) -> f64 {
        todo!()
    }

}
