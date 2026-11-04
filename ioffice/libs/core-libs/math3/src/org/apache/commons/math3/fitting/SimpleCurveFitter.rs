// Java: import java.util.Collection
// Java: import org.apache.commons.math3.analysis.ParametricUnivariateFunction
use crate::org::apache::commons::math3::analysis::ParametricUnivariateFunction;
// Java: import org.apache.commons.math3.fitting.leastsquares.LeastSquaresBuilder
use crate::org::apache::commons::math3::fitting::leastsquares::LeastSquaresBuilder;
// Java: import org.apache.commons.math3.fitting.leastsquares.LeastSquaresProblem
use crate::org::apache::commons::math3::fitting::leastsquares::LeastSquaresProblem;
// Java: import org.apache.commons.math3.linear.DiagonalMatrix
use crate::org::apache::commons::math3::linear::DiagonalMatrix;

// Java: public class SimpleCurveFitter extends AbstractCurveFitter
pub struct SimpleCurveFitter {
    // Java field: ParametricUnivariateFunction function
    function: Box<dyn std::any::Any>,
    // Java field: double[] initialGuess
    initialGuess: Vec<f64>,
    // Java field: int maxIter
    maxIter: i32,
}

// Java: public class SimpleCurveFitter extends AbstractCurveFitter
impl SimpleCurveFitter {
    // Java: SimpleCurveFitter()
    pub fn new(function: Box<dyn std::any::Any>, initial_guess: Vec<f64>, max_iter: i32) -> Self {
        SimpleCurveFitter { function, initialGuess: initial_guess, maxIter: max_iter }
    }

    // Java: static SimpleCurveFitter create()
    pub fn create(function: Box<dyn std::any::Any>, initial_guess: Vec<f64>) -> SimpleCurveFitter {
        SimpleCurveFitter::new(function, initial_guess, i32::MAX)
    }

    // Java: SimpleCurveFitter withStartPoint(double[] newStart)
    pub fn withStartPoint(&self, _new_start: Vec<f64>) -> Box<dyn std::any::Any> {
        // Java: return new SimpleCurveFitter(function,
                                     // Java: newStart.clone(),
                                     // Java: maxIter);
        todo!("SimpleCurveFitter::withStartPoint")
    }

    // Java: SimpleCurveFitter withMaxIterations(int newMaxIter)
    pub fn withMaxIterations(&self, _new_max_iter: i32) -> Box<dyn std::any::Any> {
        // Java: return new SimpleCurveFitter(function,
                                     // Java: initialGuess,
                                     // Java: newMaxIter);
        todo!("SimpleCurveFitter::withMaxIterations")
    }

    // Java: LeastSquaresProblem getProblem(Collection<WeightedObservedPoint> observations)
    pub fn getProblem(&self, _observations: &dyn std::any::Any) -> Box<dyn std::any::Any> {
        // Java: // Prepare least-squares problem.
        // Java: final int len = observations.size();
        // Java: final double[] target  = new double[len];
        // Java: final double[] weights = new double[len];
        // Java: int count = 0;
        // Java: for (WeightedObservedPoint obs : observations)
        {
            // Java: target[count]  = obs.getY();
            // Java: weights[count] = obs.getWeight();
            // Java: ++count;
        }
        // Java: final AbstractCurveFitter.TheoreticalValuesFunction model
            // Java: = new AbstractCurveFitter.TheoreticalValuesFunction(function,
                                                                // Java: observations);
        // Java: // Create an optimizer for fitting the curve to the observed points.
        // Java: return new LeastSquaresBuilder().
                // Java: maxEvaluations(Integer.MAX_VALUE).
                // Java: maxIterations(maxIter).
                // Java: start(initialGuess).
                // Java: target(target).
                // Java: weight(new DiagonalMatrix(weights)).
                // Java: model(model.getModelFunction(), model.getModelFunctionJacobian()).
                // Java: build();
        todo!("SimpleCurveFitter::getProblem")
    }

}
