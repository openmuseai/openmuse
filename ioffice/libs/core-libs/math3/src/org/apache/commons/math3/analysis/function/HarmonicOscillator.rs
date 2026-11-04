// Java: import org.apache.commons.math3.analysis.DifferentiableUnivariateFunction
use crate::org::apache::commons::math3::analysis::DifferentiableUnivariateFunction;
// Java: import org.apache.commons.math3.analysis.FunctionUtils
use crate::org::apache::commons::math3::analysis::FunctionUtils;
// Java: import org.apache.commons.math3.analysis.ParametricUnivariateFunction
use crate::org::apache::commons::math3::analysis::ParametricUnivariateFunction;
// Java: import org.apache.commons.math3.analysis.UnivariateFunction
use crate::org::apache::commons::math3::analysis::UnivariateFunction;
// Java: import org.apache.commons.math3.analysis.differentiation.DerivativeStructure
use crate::org::apache::commons::math3::analysis::differentiation::DerivativeStructure;
// Java: import org.apache.commons.math3.analysis.differentiation.UnivariateDifferentiableFunction
use crate::org::apache::commons::math3::analysis::differentiation::UnivariateDifferentiableFunction;
// Java: import org.apache.commons.math3.exception.DimensionMismatchException
use crate::org::apache::commons::math3::exception::DimensionMismatchException;
// Java: import org.apache.commons.math3.exception.NullArgumentException
use crate::org::apache::commons::math3::exception::NullArgumentException;
// Java: import org.apache.commons.math3.util.FastMath
use crate::org::apache::commons::math3::util::FastMath;

// Java: public class HarmonicOscillator implements UnivariateDifferentiableFunction, DifferentiableUnivariateFunction
pub struct HarmonicOscillator {
    // Java field: double amplitude
    amplitude: f64,
    // Java field: double omega
    omega: f64,
    // Java field: double phase
    phase: f64,
}

// Java: public class Parametric
pub struct HarmonicOscillator_Parametric {
}

// Java: public class HarmonicOscillator implements UnivariateDifferentiableFunction, DifferentiableUnivariateFunction
impl HarmonicOscillator {
    // Java: HarmonicOscillator()
    pub fn new(amplitude: f64, omega: f64, phase: f64) -> Self {
        HarmonicOscillator { amplitude, omega, phase }
    }

    // Java: double value(double x)
    pub fn value(&self, x: f64) -> f64 {
        // Java: return value(omega * x + phase, amplitude);
        Self::value_static(self.omega * x + self.phase, self.amplitude)
    }

    // Java: UnivariateFunction derivative()
    pub fn derivative(&self) -> Box<dyn std::any::Any> {
        // Java: return FunctionUtils.toDifferentiableUnivariateFunction(this).derivative();
        todo!("HarmonicOscillator::derivative")
    }

    // Java: static double value()
    pub fn value_static(x_times_omega_plus_phase: f64, amplitude: f64) -> f64 {
        amplitude * x_times_omega_plus_phase.cos()
    }

    // Java: DerivativeStructure value(final DerivativeStructure t)
    pub fn value_ds(&self, _t: &dyn std::any::Any) -> Box<dyn std::any::Any> {
        todo!("HarmonicOscillator::value_ds")
    }

}

// Java: public class Parametric
impl HarmonicOscillator_Parametric {
    // Java: double value(double x, double ... param)
    pub fn value(&self, _x: f64, _param: &[f64]) -> f64 {
        todo!("HarmonicOscillator_Parametric::value")
    }

    // Java: double[] gradient(double x, double ... param)
    pub fn gradient(&self, _x: f64, _param: &[f64]) -> Vec<f64> {
        todo!("HarmonicOscillator_Parametric::gradient")
    }

    // Java: void validateParameters(double[] param)
    pub fn validateParameters(&self, _param: &[f64]) {
        todo!("HarmonicOscillator_Parametric::validateParameters")
    }

}
