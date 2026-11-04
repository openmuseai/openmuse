// Java: import org.apache.commons.math3.analysis.polynomials.PolynomialFunction
use crate::org::apache::commons::math3::analysis::polynomials::PolynomialFunction;
// Java: import org.apache.commons.math3.optimization.DifferentiableMultivariateVectorOptimizer
use crate::org::apache::commons::math3::optimization::DifferentiableMultivariateVectorOptimizer;

// Java: public class PolynomialFitter extends CurveFitter
pub struct PolynomialFitter {
    // Java field: int degree
    degree: i32,
}

// Java: public class PolynomialFitter extends CurveFitter
impl PolynomialFitter {
    // Java: PolynomialFitter(int degree, final DifferentiableMultivariateVectorOptimizer optimizer)
    pub fn PolynomialFitter(&self) {
        // Java: super(optimizer);
        // Java: this.degree = degree;
    }

    // Java: PolynomialFitter(DifferentiableMultivariateVectorOptimizer optimizer)
    pub fn PolynomialFitter_2(&self) {
        // Java: super(optimizer);
        // Java: degree = -1; // To avoid compilation error until the instance variable is removed.
    }

    // Java: double[] fit()
    pub fn fit(&self) -> Vec<f64> {
        // Java: return fit(new PolynomialFunction.Parametric(), new double[degree + 1]);
        todo!()
    }

    // Java: double[] fit(int maxEval, double[] guess)
    pub fn fit_2(&self) -> Vec<f64> {
        // Java: return fit(maxEval, new PolynomialFunction.Parametric(), guess);
        todo!()
    }

    // Java: double[] fit(double[] guess)
    pub fn fit_3(&self) -> Vec<f64> {
        // Java: return fit(new PolynomialFunction.Parametric(), guess);
        todo!()
    }

}
