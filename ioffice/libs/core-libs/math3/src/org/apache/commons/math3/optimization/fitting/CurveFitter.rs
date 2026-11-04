// Java: import java.util.ArrayList
// Java: import java.util.List
// Java: import org.apache.commons.math3.analysis.DifferentiableMultivariateVectorFunction
use crate::org::apache::commons::math3::analysis::DifferentiableMultivariateVectorFunction;
// Java: import org.apache.commons.math3.analysis.MultivariateMatrixFunction
use crate::org::apache::commons::math3::analysis::MultivariateMatrixFunction;
// Java: import org.apache.commons.math3.analysis.ParametricUnivariateFunction
use crate::org::apache::commons::math3::analysis::ParametricUnivariateFunction;
// Java: import org.apache.commons.math3.analysis.differentiation.DerivativeStructure
use crate::org::apache::commons::math3::analysis::differentiation::DerivativeStructure;
// Java: import org.apache.commons.math3.analysis.differentiation.MultivariateDifferentiableVectorFunction
use crate::org::apache::commons::math3::analysis::differentiation::MultivariateDifferentiableVectorFunction;
// Java: import org.apache.commons.math3.optimization.DifferentiableMultivariateVectorOptimizer
use crate::org::apache::commons::math3::optimization::DifferentiableMultivariateVectorOptimizer;
// Java: import org.apache.commons.math3.optimization.MultivariateDifferentiableVectorOptimizer
use crate::org::apache::commons::math3::optimization::MultivariateDifferentiableVectorOptimizer;
// Java: import org.apache.commons.math3.optimization.PointVectorValuePair
use crate::org::apache::commons::math3::optimization::PointVectorValuePair;

// Java: public class can
pub struct can {
}

// Java: public class CurveFitter
pub struct CurveFitter {
    // Java field: DifferentiableMultivariateVectorOptimizer oldOptimizer
    pub(crate) oldOptimizer: Box<dyn std::any::Any>,
    // Java field: MultivariateDifferentiableVectorOptimizer optimizer
    pub(crate) optimizer: Box<dyn std::any::Any>,
    // Java field: List<WeightedObservedPoint> observations
    pub(crate) observations: Box<dyn std::any::Any>,
}

// Java: public class OldTheoreticalValuesFunction
pub struct OldTheoreticalValuesFunction {
}

// Java: public class TheoreticalValuesFunction
pub struct TheoreticalValuesFunction {
    // Java field: ParametricUnivariateFunction f
    pub(crate) f: Box<dyn std::any::Any>,
}

// Java: public class can
impl can {
}

// Java: public class CurveFitter
impl CurveFitter {
    // Java: CurveFitter(final DifferentiableMultivariateVectorOptimizer optimizer)
    pub fn CurveFitter(&self) {
        // Java: this.oldOptimizer = optimizer;
        // Java: this.optimizer    = null;
        // Java: observations      = new ArrayList<WeightedObservedPoint>();
    }

    // Java: CurveFitter(final MultivariateDifferentiableVectorOptimizer optimizer)
    pub fn CurveFitter_2(&self) {
        // Java: this.oldOptimizer = null;
        // Java: this.optimizer    = optimizer;
        // Java: observations      = new ArrayList<WeightedObservedPoint>();
    }

    // Java: void addObservedPoint(double x, double y)
    pub fn addObservedPoint(&self) {
        // Java: addObservedPoint(1.0, x, y);
    }

    // Java: void addObservedPoint(double weight, double x, double y)
    pub fn addObservedPoint_2(&self) {
        // Java: observations.add(new WeightedObservedPoint(weight, x, y));
    }

    // Java: void addObservedPoint(WeightedObservedPoint observed)
    pub fn addObservedPoint_3(&self) {
        // Java: observations.add(observed);
    }

    // Java: WeightedObservedPoint[] getObservations()
    pub fn getObservations(&self) -> Box<dyn std::any::Any> {
        // Java: return observations.toArray(new WeightedObservedPoint[observations.size()]);
        todo!()
    }

    // Java: void clearObservations()
    pub fn clearObservations(&self) {
        // Java: observations.clear();
    }

    // Java: double[] fit(T f, final double[] initialGuess)
    pub fn fit(&self) -> Vec<f64> {
        // Java: return fit(Integer.MAX_VALUE, f, initialGuess);
        todo!()
    }

    // Java: double[] fit()
    pub fn fit_2(&self) -> Vec<f64> {
        todo!()
    }

}

// Java: public class OldTheoreticalValuesFunction
impl OldTheoreticalValuesFunction {
}

// Java: public class TheoreticalValuesFunction
impl TheoreticalValuesFunction {
    // Java: double[] value(double[] point)
    pub fn value(&self) -> Vec<f64> {
            // Java: // compute the residuals
            // Java: final double[] values = new double[observations.size()];
            // Java: int i = 0;
            // Java: for (WeightedObservedPoint observed : observations)
            {
                // Java: values[i++] = f.value(observed.getX(), point);
            }
            // Java: return values;
        todo!()
    }

    // Java: DerivativeStructure[] value(DerivativeStructure[] point)
    pub fn value_2(&self) -> Box<dyn std::any::Any> {
            // Java: // extract parameters
            // Java: final double[] parameters = new double[point.length];
            // Java: for (int k = 0; k < point.length; ++k)
            {
                // Java: parameters[k] = point[k].getValue();
            }
            // Java: // compute the residuals
            // Java: final DerivativeStructure[] values = new DerivativeStructure[observations.size()];
            // Java: int i = 0;
            // Java: for (WeightedObservedPoint observed : observations)
            {
                // Java: // build the DerivativeStructure by adding first the value as a constant
                // Java: // and then adding derivatives
                // Java: DerivativeStructure vi = new DerivativeStructure(point.length, 1, f.value(observed.getX(), parameters));
                // Java: for (int k = 0; k < point.length; ++k)
                {
                    // Java: vi = vi.add(new DerivativeStructure(point.length, 1, k, 0.0));
                }
                // Java: values[i++] = vi;
            }
            // Java: return values;
        todo!()
    }

}
