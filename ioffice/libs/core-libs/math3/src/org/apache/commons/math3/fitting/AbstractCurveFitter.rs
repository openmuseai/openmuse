// Java: import java.util.Collection
// Java: import org.apache.commons.math3.analysis.MultivariateVectorFunction
use crate::org::apache::commons::math3::analysis::MultivariateVectorFunction;
// Java: import org.apache.commons.math3.analysis.MultivariateMatrixFunction
use crate::org::apache::commons::math3::analysis::MultivariateMatrixFunction;
// Java: import org.apache.commons.math3.analysis.ParametricUnivariateFunction
use crate::org::apache::commons::math3::analysis::ParametricUnivariateFunction;
// Java: import org.apache.commons.math3.fitting.leastsquares.LeastSquaresOptimizer
use crate::org::apache::commons::math3::fitting::leastsquares::LeastSquaresOptimizer;
// Java: import org.apache.commons.math3.fitting.leastsquares.LeastSquaresProblem
use crate::org::apache::commons::math3::fitting::leastsquares::LeastSquaresProblem;
// Java: import org.apache.commons.math3.fitting.leastsquares.LevenbergMarquardtOptimizer
use crate::org::apache::commons::math3::fitting::leastsquares::LevenbergMarquardtOptimizer;

use std::sync::Arc;

// Java: public class that
pub struct that {
}

// Java: public class contains
pub struct contains {
}

// Java: public class AbstractCurveFitter
pub struct AbstractCurveFitter {
}

// Java: public class TheoreticalValuesFunction
pub struct TheoreticalValuesFunction {
    // Java field: ParametricUnivariateFunction f
    pub(crate) f: Box<dyn std::any::Any>,
    // Java field: double[] points
    pub(crate) points: Vec<f64>,
}

// Java: public class that
impl that {
}

// Java: public class contains
impl contains {
}

// Java: public class AbstractCurveFitter
impl AbstractCurveFitter {
    // Java: double[] fit(Collection<WeightedObservedPoint> points)
    pub fn fit(&self, points: &dyn std::any::Any) -> Vec<f64> {
        // Java: // Perform the fit.
        // Java: return getOptimizer().optimize(getProblem(points)).getPoint().toArray();
        todo!("AbstractCurveFitter::fit")
    }

    // Java: LeastSquaresOptimizer getOptimizer()
    pub fn getOptimizer(&self) -> LevenbergMarquardtOptimizer::LevenbergMarquardtOptimizer {
        // Java: return new LevenbergMarquardtOptimizer();
        LevenbergMarquardtOptimizer::LevenbergMarquardtOptimizer::new()
    }

    // Java: abstract LeastSquaresProblem getProblem(Collection<WeightedObservedPoint> points)
    pub fn getProblem(&self, _points: &dyn std::any::Any) -> Box<dyn std::any::Any> {
        todo!("AbstractCurveFitter::getProblem")
    }

}

// Java: public class TheoreticalValuesFunction
impl TheoreticalValuesFunction {
    // Java: TheoreticalValuesFunction()
    pub fn new(f: Box<dyn std::any::Any>, observations: &[f64]) -> Self {
        TheoreticalValuesFunction {
            f,
            points: observations.to_vec(),
        }
    }

    // Java: MultivariateVectorFunction getModelFunction()
    pub fn getModelFunction(&self) -> Box<dyn std::any::Any> {
            // Java: return new MultivariateVectorFunction()
            {
                // Java: /** {@inheritDoc} */
                // Java: public double[] value(double[] p)
                {
                    // Java: final int len = points.length;
                    // Java: final double[] values = new double[len];
                    // Java: for (int i = 0; i < len; i++)
                    {
                        // Java: values[i] = f.value(points[i], p);
                    }
                    // Java: return values;
                }
            }
        todo!("TheoreticalValuesFunction::getModelFunction")
    }

    // Java: MultivariateMatrixFunction getModelFunctionJacobian()
    pub fn getModelFunctionJacobian(&self) -> Box<dyn std::any::Any> {
            // Java: return new MultivariateMatrixFunction()
            {
                // Java: /** {@inheritDoc} */
                // Java: public double[][] value(double[] p)
                {
                    // Java: final int len = points.length;
                    // Java: final double[][] jacobian = new double[len][];
                    // Java: for (int i = 0; i < len; i++)
                    {
                        // Java: jacobian[i] = f.gradient(points[i], p);
                    }
                    // Java: return jacobian;
                }
            }
        todo!("TheoreticalValuesFunction::getModelFunctionJacobian")
    }

}
