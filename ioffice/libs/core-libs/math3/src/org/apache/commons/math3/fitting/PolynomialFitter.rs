// Java: import org.apache.commons.math3.analysis.polynomials.PolynomialFunction
use crate::org::apache::commons::math3::analysis::polynomials::PolynomialFunction;
// Java: import org.apache.commons.math3.optim.nonlinear.vector.MultivariateVectorOptimizer
use crate::org::apache::commons::math3::optim::nonlinear::vector::MultivariateVectorOptimizer;

// Java: public class PolynomialFitter extends CurveFitter
pub struct PolynomialFitter {
}

// Java: public class PolynomialFitter extends CurveFitter
impl PolynomialFitter {
    // Java: PolynomialFitter(MultivariateVectorOptimizer optimizer)
    pub fn new(_optimizer: MultivariateVectorOptimizer::MultivariateVectorOptimizer) -> Self {
        // Java: super(optimizer);
        PolynomialFitter {}
    }

    // Java: double[] fit(int maxEval, double[] guess)
    pub fn fit_with_max_eval(&self, _max_eval: i32, _guess: &[f64]) -> Vec<f64> {
        // Java: return fit(maxEval, new PolynomialFunction.Parametric(), guess);
        todo!("PolynomialFitter::fit_with_max_eval")
    }

    // Java: double[] fit(double[] guess)
    pub fn fit(&self, _guess: &[f64]) -> Vec<f64> {
        // Java: return fit(new PolynomialFunction.Parametric(), guess);
        todo!("PolynomialFitter::fit")
    }

}
