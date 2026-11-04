// Java: import org.apache.commons.math3.analysis.solvers.UnivariateSolver
use crate::org::apache::commons::math3::analysis::solvers::UnivariateSolver;
// Java: import org.apache.commons.math3.exception.DimensionMismatchException
use crate::org::apache::commons::math3::exception::DimensionMismatchException;
// Java: import org.apache.commons.math3.exception.MaxCountExceededException
use crate::org::apache::commons::math3::exception::MaxCountExceededException;
// Java: import org.apache.commons.math3.exception.NoBracketingException
use crate::org::apache::commons::math3::exception::NoBracketingException;
// Java: import org.apache.commons.math3.exception.NumberIsTooSmallException
use crate::org::apache::commons::math3::exception::NumberIsTooSmallException;
// Java: import org.apache.commons.math3.ode.ExpandableStatefulODE
use crate::org::apache::commons::math3::ode::ExpandableStatefulODE;
// Java: import org.apache.commons.math3.ode.events.EventHandler
use crate::org::apache::commons::math3::ode::events::EventHandler;
// Java: import org.apache.commons.math3.ode.sampling.AbstractStepInterpolator
use crate::org::apache::commons::math3::ode::sampling::AbstractStepInterpolator;
// Java: import org.apache.commons.math3.ode.sampling.StepHandler
use crate::org::apache::commons::math3::ode::sampling::StepHandler;
// Java: import org.apache.commons.math3.util.FastMath
use crate::org::apache::commons::math3::util::FastMath;

// Java field: String METHOD_NAME
pub(crate) const METHOD_NAME: &str = "";

// Java: public class implements implements a Gragg
pub struct implements {
}

// Java: public class GraggBulirschStoerIntegrator extends AdaptiveStepsizeIntegrator
pub struct GraggBulirschStoerIntegrator {
    // Java field: int maxOrder
    maxOrder: i32,
    // Java field: int[] sequence
    sequence: Option<Vec<i32>>,
    // Java field: int[] costPerStep
    costPerStep: Option<Vec<i32>>,
    // Java field: double[] costPerTimeUnit
    costPerTimeUnit: Option<Vec<f64>>,
    // Java field: double[] optimalStep
    optimalStep: Option<Vec<f64>>,
    // Java field: double[][] coeff
    coeff: Option<Vec<Vec<f64>>>,
    // Java field: boolean performTest
    performTest: bool,
    // Java field: int maxChecks
    maxChecks: i32,
    // Java field: int maxIter
    maxIter: i32,
    // Java field: double stabilityReduction
    stabilityReduction: f64,
    // Java field: double stepControl1
    stepControl1: f64,
    // Java field: double stepControl2
    stepControl2: f64,
    // Java field: double stepControl3
    stepControl3: f64,
    // Java field: double stepControl4
    stepControl4: f64,
    // Java field: double orderControl1
    orderControl1: f64,
    // Java field: double orderControl2
    orderControl2: f64,
    // Java field: boolean useInterpolationError
    useInterpolationError: bool,
    // Java field: int mudif
    mudif: i32,
}

// Java: public class implements implements a Gragg
impl implements {
}

impl r#implements {
}

// Java: public class GraggBulirschStoerIntegrator extends AdaptiveStepsizeIntegrator
impl GraggBulirschStoerIntegrator {
    // Java: GraggBulirschStoerIntegrator()
    pub fn GraggBulirschStoerIntegrator(&self) {
    }

    // Java: GraggBulirschStoerIntegrator()
    pub fn GraggBulirschStoerIntegrator_2(&self) {
    }

    // Java: void setStabilityCheck()
    pub fn setStabilityCheck(&self) {
    }

    // Java: void setControlFactors()
    pub fn setControlFactors(&self) {
    }

    // Java: void setOrderControl()
    pub fn setOrderControl(&self) {
    }

    // Java: void addStepHandler(final StepHandler handler)
    pub fn addStepHandler(&self) {
    // Java: super.addStepHandler(handler);
    // Java: // reinitialize the arrays
    // Java: initializeArrays();
    }

    // Java: void addEventHandler()
    pub fn addEventHandler(&self) {
    }

    // Java: void initializeArrays()
    pub fn initializeArrays(&self) {
    // Java: final int size = maxOrder / 2;
    // Java: if ((sequence == null) || (sequence.length != size))
    {
      // Java: // all arrays should be reallocated with the right size
      // Java: sequence        = new int[size];
      // Java: costPerStep     = new int[size];
      // Java: coeff           = new double[size][];
      // Java: costPerTimeUnit = new double[size];
      // Java: optimalStep     = new double[size];
    }
    // Java: // step size sequence: 2, 6, 10, 14, ...
    // Java: for (int k = 0; k < size; ++k)
    {
        // Java: sequence[k] = 4 * k + 2;
    }
    // Java: // initialize the order selection cost array
    // Java: // (number of function calls for each column of the extrapolation table)
    // Java: costPerStep[0] = sequence[0] + 1;
    // Java: for (int k = 1; k < size; ++k)
    {
      // Java: costPerStep[k] = costPerStep[k-1] + sequence[k];
    }
    // Java: // initialize the extrapolation tables
    // Java: for (int k = 0; k < size; ++k)
    {
      // Java: coeff[k] = (k > 0) ? new double[k] : null;
      // Java: for (int l = 0; l < k; ++l)
      {
        // Java: final double ratio = ((double) sequence[k]) / sequence[k-l-1];
        // Java: coeff[k][l] = 1.0 / (ratio * ratio - 1.0);
      }
    }
    }

    // Java: void setInterpolationControl()
    pub fn setInterpolationControl(&self) {
    }

    // Java: void rescale(final double[] y1, final double[] y2, final double[] scale)
    pub fn rescale(&self) {
    // Java: if (vecAbsoluteTolerance == null)
    {
      // Java: for (int i = 0; i < scale.length; ++i)
      {
        // Java: final double yi = FastMath.max(FastMath.abs(y1[i]), FastMath.abs(y2[i]));
        // Java: scale[i] = scalAbsoluteTolerance + scalRelativeTolerance * yi;
      }
    }
    // Java: else
    {
      // Java: for (int i = 0; i < scale.length; ++i)
      {
        // Java: final double yi = FastMath.max(FastMath.abs(y1[i]), FastMath.abs(y2[i]));
        // Java: scale[i] = vecAbsoluteTolerance[i] + vecRelativeTolerance[i] * yi;
      }
    }
    }

    // Java: boolean tryStep()
    pub fn tryStep(&self) -> bool {
        todo!()
    }

    // Java: void extrapolate()
    pub fn extrapolate(&self) {
    }

    // Java: void integrate(final ExpandableStatefulODE equations, final double t)
    pub fn integrate(&self) {
    }

}
