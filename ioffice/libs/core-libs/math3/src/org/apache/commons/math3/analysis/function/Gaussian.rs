// Java: import java.util.Arrays
// Java: import org.apache.commons.math3.analysis.FunctionUtils
use crate::org::apache::commons::math3::analysis::FunctionUtils;
// Java: import org.apache.commons.math3.analysis.UnivariateFunction
use crate::org::apache::commons::math3::analysis::UnivariateFunction;
// Java: import org.apache.commons.math3.analysis.DifferentiableUnivariateFunction
use crate::org::apache::commons::math3::analysis::DifferentiableUnivariateFunction;
// Java: import org.apache.commons.math3.analysis.ParametricUnivariateFunction
use crate::org::apache::commons::math3::analysis::ParametricUnivariateFunction;
// Java: import org.apache.commons.math3.analysis.differentiation.DerivativeStructure
use crate::org::apache::commons::math3::analysis::differentiation::DerivativeStructure;
// Java: import org.apache.commons.math3.analysis.differentiation.UnivariateDifferentiableFunction
use crate::org::apache::commons::math3::analysis::differentiation::UnivariateDifferentiableFunction;
// Java: import org.apache.commons.math3.exception.NotStrictlyPositiveException
use crate::org::apache::commons::math3::exception::NotStrictlyPositiveException;
// Java: import org.apache.commons.math3.exception.NullArgumentException
use crate::org::apache::commons::math3::exception::NullArgumentException;
// Java: import org.apache.commons.math3.exception.DimensionMismatchException
use crate::org::apache::commons::math3::exception::DimensionMismatchException;
// Java: import org.apache.commons.math3.util.FastMath
use crate::org::apache::commons::math3::util::FastMath;
// Java: import org.apache.commons.math3.util.Precision
use crate::org::apache::commons::math3::util::Precision;

// Java: public class Gaussian implements UnivariateDifferentiableFunction, DifferentiableUnivariateFunction
pub struct Gaussian {
    // Java field: double mean
    mean: f64,
    // Java field: double is
    is: f64,
    // Java field: double i2s2
    i2s2: f64,
    // Java field: double norm
    norm: f64,
}

// Java: public class Parametric
pub struct Parametric {
}

// Java: public class Gaussian implements UnivariateDifferentiableFunction, DifferentiableUnivariateFunction
impl Gaussian {
    // Java: Gaussian()
    pub fn new_with_norm_mean_sigma(norm: f64, mean: f64, sigma: f64) -> Self {
        if sigma <= 0.0 {
            panic!("NotStrictlyPositiveException: {}", sigma);
        }
        let is = 1.0 / sigma;
        let i2s2 = 0.5 * is * is;
        Gaussian { mean, is, i2s2, norm }
    }

    // Java: Gaussian()
    pub fn new_with_mean_sigma(mean: f64, sigma: f64) -> Self {
        Self::new_with_norm_mean_sigma(1.0 / (sigma * (2.0 * std::f64::consts::PI).sqrt()), mean, sigma)
    }

    // Java: Gaussian()
    pub fn new() -> Self {
        // Java: this(0, 1);
        Self::new_with_mean_sigma(0.0, 1.0)
    }

    // Java: double value(double x)
    pub fn value(&self, x: f64) -> f64 {
        // Java: return value(x - mean, norm, i2s2);
        Self::value_static(x - self.mean, self.norm, self.i2s2)
    }

    // Java: UnivariateFunction derivative()
    pub fn derivative(&self) -> Box<dyn std::any::Any> {
        // Java: return FunctionUtils.toDifferentiableUnivariateFunction(this).derivative();
        todo!("Gaussian::derivative")
    }

    // Java: static double value()
    pub fn value_static(x_minus_mean: f64, norm: f64, i2s2: f64) -> f64 {
        norm * (-x_minus_mean * x_minus_mean * i2s2).exp()
    }

    // Java: DerivativeStructure value(final DerivativeStructure t)
    pub fn value_ds(&self, _t: &dyn std::any::Any) -> Box<dyn std::any::Any> {
        todo!("Gaussian::value_ds")
    }

}

// Java: public class Parametric
impl Parametric {
    // Java: double value(double x, double ... param)
    pub fn value(&self, _x: f64, _param: &[f64]) -> f64 {
        todo!("Parametric::value")
    }

    // Java: double[] gradient(double x, double ... param)
    pub fn gradient(&self, _x: f64, _param: &[f64]) -> Vec<f64> {
        todo!("Parametric::gradient")
    }

    // Java: void validateParameters(double[] param)
    pub fn validateParameters(&self, _param: &[f64]) {
        todo!("Parametric::validateParameters")
    }

}
