// Java: import org.apache.commons.math3.analysis.UnivariateFunction
use crate::org::apache::commons::math3::analysis::UnivariateFunction;

// Java: public interface UnivariateFunctionDifferentiator
pub trait UnivariateFunctionDifferentiator {
    // Java: abstract UnivariateDifferentiableFunction differentiate(UnivariateFunction function)
    fn differentiate(&self) -> Box<dyn std::any::Any>;

}
