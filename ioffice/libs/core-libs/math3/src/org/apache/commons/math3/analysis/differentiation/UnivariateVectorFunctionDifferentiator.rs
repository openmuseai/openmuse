// Java: import org.apache.commons.math3.analysis.UnivariateVectorFunction
use crate::org::apache::commons::math3::analysis::UnivariateVectorFunction;

// Java: public interface UnivariateVectorFunctionDifferentiator
pub trait UnivariateVectorFunctionDifferentiator {
    // Java: abstract UnivariateDifferentiableVectorFunction differentiate(UnivariateVectorFunction function)
    fn differentiate(&self) -> Box<dyn std::any::Any>;

}
