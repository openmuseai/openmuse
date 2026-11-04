// Java: import org.apache.commons.math3.exception.DimensionMismatchException
use crate::org::apache::commons::math3::exception::DimensionMismatchException;
// Java: import org.apache.commons.math3.exception.MathIllegalStateException
use crate::org::apache::commons::math3::exception::MathIllegalStateException;
// Java: import org.apache.commons.math3.exception.MaxCountExceededException
use crate::org::apache::commons::math3::exception::MaxCountExceededException;
// Java: import org.apache.commons.math3.exception.NoBracketingException
use crate::org::apache::commons::math3::exception::NoBracketingException;
// Java: import org.apache.commons.math3.exception.NumberIsTooSmallException
use crate::org::apache::commons::math3::exception::NumberIsTooSmallException;
// Java: import org.apache.commons.math3.exception.util.LocalizedFormats
use crate::org::apache::commons::math3::exception::util::LocalizedFormats;
// Java: import org.apache.commons.math3.linear.Array2DRowRealMatrix
use crate::org::apache::commons::math3::linear::Array2DRowRealMatrix;
// Java: import org.apache.commons.math3.ode.nonstiff.AdaptiveStepsizeIntegrator
use crate::org::apache::commons::math3::ode::nonstiff::AdaptiveStepsizeIntegrator;
// Java: import org.apache.commons.math3.ode.nonstiff.DormandPrince853Integrator
use crate::org::apache::commons::math3::ode::nonstiff::DormandPrince853Integrator;
// Java: import org.apache.commons.math3.ode.sampling.StepHandler
use crate::org::apache::commons::math3::ode::sampling::StepHandler;
// Java: import org.apache.commons.math3.ode.sampling.StepInterpolator
use crate::org::apache::commons::math3::ode::sampling::StepInterpolator;
// Java: import org.apache.commons.math3.util.FastMath
use crate::org::apache::commons::math3::util::FastMath;

// Java: public interface NordsieckTransformer
pub trait NordsieckTransformer {
}

// Java: public class is
pub struct is {
}

// Java: public class MultistepIntegrator
pub struct MultistepIntegrator {
    // Java field: double[] scaled
    pub(crate) scaled: Box<dyn std::any::Any>,
    // Java field: Array2DRowRealMatrix nordsieck
    pub(crate) nordsieck: Box<dyn std::any::Any>,
    // Java field: FirstOrderIntegrator starter
    pub(crate) starter: Box<dyn std::any::Any>,
    // Java field: int nSteps
    pub(crate) nSteps: i32,
    // Java field: double exp
    pub(crate) exp: f64,
    // Java field: double safety
    pub(crate) safety: f64,
    // Java field: double minReduction
    pub(crate) minReduction: f64,
    // Java field: double maxGrowth
    pub(crate) maxGrowth: f64,
}

// Java: public class NordsieckInitializer
pub struct NordsieckInitializer {
    // Java field: int count
    count: i32,
    // Java field: double[] t
    t: Vec<f64>,
    // Java field: double[][] y
    y: Vec<Vec<f64>>,
    // Java field: double[][] yDot
    yDot: Vec<Vec<f64>>,
}

// Java: public class InitializationCompletedMarkerException
pub struct InitializationCompletedMarkerException {
}

// Java: public class is
impl is {
}

// Java: public class MultistepIntegrator
impl MultistepIntegrator {
    // Java: MultistepIntegrator()
    pub fn MultistepIntegrator(&self) {
    }

    // Java: MultistepIntegrator()
    pub fn MultistepIntegrator_2(&self) {
    }

    // Java: ODEIntegrator getStarterIntegrator()
    pub fn getStarterIntegrator(&self) -> Box<dyn std::any::Any> {
        // Java: return starter;
        todo!()
    }

    // Java: void setStarterIntegrator(FirstOrderIntegrator starterIntegrator)
    pub fn setStarterIntegrator(&self) {
        // Java: this.starter = starterIntegrator;
    }

    // Java: void start(final double t0, final double[] y0, final double t)
    pub fn start(&self) {
    }

    // Java: abstract Array2DRowRealMatrix initializeHighOrderDerivatives()
    pub fn initializeHighOrderDerivatives(&self) -> Box<dyn std::any::Any> {
        todo!()
    }

    // Java: double getMinReduction()
    pub fn getMinReduction(&self) -> f64 {
        // Java: return minReduction;
        todo!()
    }

    // Java: void setMinReduction(final double minReduction)
    pub fn setMinReduction(&self) {
        // Java: this.minReduction = minReduction;
    }

    // Java: double getMaxGrowth()
    pub fn getMaxGrowth(&self) -> f64 {
        // Java: return maxGrowth;
        todo!()
    }

    // Java: void setMaxGrowth(final double maxGrowth)
    pub fn setMaxGrowth(&self) {
        // Java: this.maxGrowth = maxGrowth;
    }

    // Java: double getSafety()
    pub fn getSafety(&self) -> f64 {
      // Java: return safety;
        todo!()
    }

    // Java: void setSafety(final double safety)
    pub fn setSafety(&self) {
      // Java: this.safety = safety;
    }

    // Java: int getNSteps()
    pub fn getNSteps(&self) -> i32 {
      // Java: return nSteps;
        todo!()
    }

    // Java: double computeStepGrowShrinkFactor(final double error)
    pub fn computeStepGrowShrinkFactor(&self) -> f64 {
        // Java: return FastMath.min(maxGrowth, FastMath.max(minReduction, safety * FastMath.pow(error, exp)));
        todo!()
    }

}

// Java: public class NordsieckInitializer
impl NordsieckInitializer {
    // Java: void handleStep(StepInterpolator interpolator, boolean isLast)
    pub fn handleStep(&self) {
    }

    // Java: void init(double t0, double[] y0, double time)
    pub fn init(&self) {
            // Java: // nothing to do
    }

}

// Java: public class InitializationCompletedMarkerException
impl InitializationCompletedMarkerException {
}
