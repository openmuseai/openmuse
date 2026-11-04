// Java: import org.apache.commons.math3.analysis.MultivariateFunction
#[allow(unused_imports)]
use crate::org::apache::commons::math3::analysis::MultivariateFunction;
// Java: import org.apache.commons.math3.exception.MathIllegalArgumentException
#[allow(unused_imports)]
use crate::org::apache::commons::math3::exception::MathIllegalArgumentException;
use super::DerivativeStructure::DerivativeStructure;

// Java field: throws MathIllegalArgumentException

// Java: public interface MultivariateDifferentiableFunction extends MultivariateFunction
pub trait MultivariateDifferentiableFunction {
    // Java: abstract DerivativeStructure value(DerivativeStructure[] point)
    fn value_ds(&self, point: &[DerivativeStructure]) -> DerivativeStructure;

}
