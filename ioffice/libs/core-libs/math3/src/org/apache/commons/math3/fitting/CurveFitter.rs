// Java: import java.util.ArrayList
// Java: import java.util.List
// Java: import org.apache.commons.math3.analysis.MultivariateVectorFunction
use crate::org::apache::commons::math3::analysis::MultivariateVectorFunction;
// Java: import org.apache.commons.math3.analysis.MultivariateMatrixFunction
use crate::org::apache::commons::math3::analysis::MultivariateMatrixFunction;
// Java: import org.apache.commons.math3.analysis.ParametricUnivariateFunction
use crate::org::apache::commons::math3::analysis::ParametricUnivariateFunction;
// Java: import org.apache.commons.math3.optim.MaxEval
use crate::org::apache::commons::math3::optim::MaxEval;
// Java: import org.apache.commons.math3.optim.InitialGuess
use crate::org::apache::commons::math3::optim::InitialGuess;
// Java: import org.apache.commons.math3.optim.PointVectorValuePair
use crate::org::apache::commons::math3::optim::PointVectorValuePair;
// Java: import org.apache.commons.math3.optim.nonlinear.vector.MultivariateVectorOptimizer
use crate::org::apache::commons::math3::optim::nonlinear::vector::MultivariateVectorOptimizer;
// Java: import org.apache.commons.math3.optim.nonlinear.vector.ModelFunction
use crate::org::apache::commons::math3::optim::nonlinear::vector::ModelFunction;
// Java: import org.apache.commons.math3.optim.nonlinear.vector.ModelFunctionJacobian
use crate::org::apache::commons::math3::optim::nonlinear::vector::ModelFunctionJacobian;
// Java: import org.apache.commons.math3.optim.nonlinear.vector.Target
use crate::org::apache::commons::math3::optim::nonlinear::vector::Target;
// Java: import org.apache.commons.math3.optim.nonlinear.vector.Weight
use crate::org::apache::commons::math3::optim::nonlinear::vector::Weight;

// Java: public class can
pub struct can {
}

#[derive(Clone)]
pub struct WeightedObservedPoint {
    pub weight: f64,
    pub x: f64,
    pub y: f64,
}

impl WeightedObservedPoint {
    pub fn new(weight: f64, x: f64, y: f64) -> Self {
        WeightedObservedPoint { weight, x, y }
    }
    pub fn getWeight(&self) -> f64 { self.weight }
    pub fn getX(&self) -> f64 { self.x }
    pub fn getY(&self) -> f64 { self.y }
}

// Java: public class CurveFitter extends ParametricUnivariateFunction
pub struct CurveFitter {
    // Java field: MultivariateVectorOptimizer optimizer
    optimizer: MultivariateVectorOptimizer::MultivariateVectorOptimizer,
    // Java field: List<WeightedObservedPoint> observations
    pub(crate) observations: Vec<WeightedObservedPoint>,
}

// Java: public class TheoreticalValuesFunction
pub struct CurveFitter_TheoreticalValuesFunction {
    // Java field: ParametricUnivariateFunction f
    pub(crate) f: Box<dyn std::any::Any>,
}

// Java: public class can
impl can {
}

// Java: public class CurveFitter extends ParametricUnivariateFunction
impl CurveFitter {
    // Java: CurveFitter(final MultivariateVectorOptimizer optimizer)
    pub fn new(optimizer: MultivariateVectorOptimizer::MultivariateVectorOptimizer) -> Self {
        // Java: this.optimizer = optimizer;
        // Java: observations = new ArrayList<WeightedObservedPoint>();
        CurveFitter {
            optimizer,
            observations: Vec::new(),
        }
    }

    // Java: void addObservedPoint(double x, double y)
    pub fn addObservedPoint(&mut self, x: f64, y: f64) {
        // Java: addObservedPoint(1.0, x, y);
        self.addObservedPoint_weighted(1.0, x, y);
    }

    // Java: void addObservedPoint(double weight, double x, double y)
    pub fn addObservedPoint_weighted(&mut self, weight: f64, x: f64, y: f64) {
        // Java: observations.add(new WeightedObservedPoint(weight, x, y));
        self.observations.push(WeightedObservedPoint::new(weight, x, y));
    }

    // Java: void addObservedPoint(WeightedObservedPoint observed)
    pub fn addObservedPoint_obj(&mut self, observed: WeightedObservedPoint) {
        // Java: observations.add(observed);
        self.observations.push(observed);
    }

    // Java: WeightedObservedPoint[] getObservations()
    pub fn getObservations(&self) -> Vec<WeightedObservedPoint> {
        // Java: return observations.toArray(new WeightedObservedPoint[observations.size()]);
        self.observations.clone()
    }

    // Java: void clearObservations()
    pub fn clearObservations(&mut self) {
        // Java: observations.clear();
        self.observations.clear();
    }

    // Java: double[] fit(T f, final double[] initialGuess)
    pub fn fit(&self, _f: &dyn std::any::Any, _initial_guess: &[f64]) -> Vec<f64> {
        // Java: return fit(Integer.MAX_VALUE, f, initialGuess);
        todo!("CurveFitter::fit")
    }

    // Java: double[] fit()
    pub fn fit_with_max(&self, _max_eval: i32, _f: &dyn std::any::Any, _initial_guess: &[f64]) -> Vec<f64> {
        todo!("CurveFitter::fit_with_max")
    }

}

// Java: public class TheoreticalValuesFunction
impl CurveFitter_TheoreticalValuesFunction {
    // Java: ModelFunction getModelFunction()
    pub fn getModelFunction(&self) -> Box<dyn std::any::Any> {
            // Java: return new ModelFunction(new MultivariateVectorFunction()
            {
                    // Java: /** {@inheritDoc} */
                    // Java: public double[] value(double[] point)
                    {
                        // Java: // compute the residuals
                        // Java: final double[] values = new double[observations.size()];
                        // Java: int i = 0;
                        // Java: for (WeightedObservedPoint observed : observations)
                        {
                            // Java: values[i++] = f.value(observed.getX(), point);
                        }
                        // Java: return values;
                    }
            }
        todo!("TheoreticalValuesFunction::getModelFunction")
    }

    // Java: ModelFunctionJacobian getModelFunctionJacobian()
    pub fn getModelFunctionJacobian(&self) -> Box<dyn std::any::Any> {
            // Java: return new ModelFunctionJacobian(new MultivariateMatrixFunction()
            {
                    // Java: /** {@inheritDoc} */
                    // Java: public double[][] value(double[] point)
                    {
                        // Java: final double[][] jacobian = new double[observations.size()][];
                        // Java: int i = 0;
                        // Java: for (WeightedObservedPoint observed : observations)
                        {
                            // Java: jacobian[i++] = f.gradient(observed.getX(), point);
                        }
                        // Java: return jacobian;
                    }
            }
        todo!("TheoreticalValuesFunction::getModelFunctionJacobian")
    }

}
