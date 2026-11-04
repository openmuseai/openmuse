// Java: import org.apache.commons.math3.analysis.UnivariateFunction
#[allow(unused_imports)]
use crate::org::apache::commons::math3::analysis::UnivariateFunction;
// Java: import org.apache.commons.math3.exception.DimensionMismatchException
#[allow(unused_imports)]
use crate::org::apache::commons::math3::exception::DimensionMismatchException;
use super::DerivativeStructure::DerivativeStructure;

// Java: public interface represents
#[allow(non_camel_case_types)]
pub trait represents {
}

// Java field: throws DimensionMismatchException

// Java: public interface UnivariateDifferentiableFunction
pub trait UnivariateDifferentiableFunction {
    // Java: abstract DerivativeStructure value(DerivativeStructure t)
    fn value_ds(&self, t: DerivativeStructure) -> DerivativeStructure;

}
