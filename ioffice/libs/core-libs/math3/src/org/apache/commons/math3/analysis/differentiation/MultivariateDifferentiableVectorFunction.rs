// Java: import org.apache.commons.math3.analysis.MultivariateVectorFunction
#[allow(unused_imports)]
use crate::org::apache::commons::math3::analysis::MultivariateVectorFunction;
// Java: import org.apache.commons.math3.exception.MathIllegalArgumentException
#[allow(unused_imports)]
use crate::org::apache::commons::math3::exception::MathIllegalArgumentException;
use super::DerivativeStructure::DerivativeStructure;

// Java field: throws MathIllegalArgumentException

// Java: public interface MultivariateDifferentiableVectorFunction
pub trait MultivariateDifferentiableVectorFunction {
    // Java: abstract DerivativeStructure[] value(DerivativeStructure[] point)
    fn value_ds(&self, point: &[DerivativeStructure]) -> Vec<DerivativeStructure>;
}
