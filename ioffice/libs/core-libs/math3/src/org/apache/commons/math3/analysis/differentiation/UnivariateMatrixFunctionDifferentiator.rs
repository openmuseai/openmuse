// Java: import org.apache.commons.math3.analysis.UnivariateMatrixFunction
use crate::org::apache::commons::math3::analysis::UnivariateMatrixFunction;

// Java: public interface UnivariateMatrixFunctionDifferentiator
pub trait UnivariateMatrixFunctionDifferentiator {
    // Java: abstract UnivariateDifferentiableMatrixFunction differentiate(UnivariateMatrixFunction function)
    fn differentiate(&self) -> Box<dyn std::any::Any>;

}
