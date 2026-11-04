// Java: import java.util.Collection
// Java: import org.apache.commons.math3.analysis.polynomials.PolynomialFunction
use crate::org::apache::commons::math3::analysis::polynomials::PolynomialFunction;
// Java: import org.apache.commons.math3.exception.MathInternalError
use crate::org::apache::commons::math3::exception::MathInternalError;
// Java: import org.apache.commons.math3.fitting.leastsquares.LeastSquaresBuilder
use crate::org::apache::commons::math3::fitting::leastsquares::LeastSquaresBuilder;
// Java: import org.apache.commons.math3.fitting.leastsquares.LeastSquaresProblem
use crate::org::apache::commons::math3::fitting::leastsquares::LeastSquaresProblem;
// Java: import org.apache.commons.math3.linear.DiagonalMatrix
use crate::org::apache::commons::math3::linear::DiagonalMatrix;

// Java field: PolynomialFunction.Parametric FUNCTION

// Java: public class PolynomialCurveFitter extends AbstractCurveFitter
pub struct PolynomialCurveFitter {
    // Java field: double[] initialGuess
    initialGuess: Vec<f64>,
    // Java field: int maxIter
    maxIter: i32,
}

// Java: public class PolynomialCurveFitter extends AbstractCurveFitter
impl PolynomialCurveFitter {
    // Java: PolynomialCurveFitter()
    pub fn new(initial_guess: Vec<f64>, max_iter: i32) -> Self {
        PolynomialCurveFitter { initialGuess: initial_guess, maxIter: max_iter }
    }

    // Java: static PolynomialCurveFitter create(int degree)
    pub fn create(degree: i32) -> PolynomialCurveFitter {
        // Java: return new PolynomialCurveFitter(new double[degree + 1], Integer.MAX_VALUE);
        PolynomialCurveFitter::new(vec![0.0; (degree + 1) as usize], i32::MAX)
    }

    // Java: PolynomialCurveFitter withStartPoint(double[] newStart)
    pub fn withStartPoint(&self, new_start: Vec<f64>) -> PolynomialCurveFitter {
        // Java: return new PolynomialCurveFitter(newStart.clone(),
                                         // Java: maxIter);
        PolynomialCurveFitter::new(new_start, self.maxIter)
    }

    // Java: PolynomialCurveFitter withMaxIterations(int newMaxIter)
    pub fn withMaxIterations(&self, new_max_iter: i32) -> PolynomialCurveFitter {
        // Java: return new PolynomialCurveFitter(initialGuess,
                                         // Java: newMaxIter);
        PolynomialCurveFitter::new(self.initialGuess.clone(), new_max_iter)
    }

    // Java: LeastSquaresProblem getProblem(Collection<WeightedObservedPoint> observations)
    pub fn getProblem(&self, _observations: &dyn std::any::Any) -> Box<dyn std::any::Any> {
        // Java: // Prepare least-squares problem.
        // Java: final int len = observations.size();
        // Java: final double[] target  = new double[len];
        // Java: final double[] weights = new double[len];
        // Java: int i = 0;
        // Java: for (WeightedObservedPoint obs : observations)
        {
            // Java: target[i]  = obs.getY();
            // Java: weights[i] = obs.getWeight();
            // Java: ++i;
        }
        // Java: final AbstractCurveFitter.TheoreticalValuesFunction model =
                // Java: new AbstractCurveFitter.TheoreticalValuesFunction(FUNCTION, observations);
        // Java: if (initialGuess == null)
        {
            // Java: throw new MathInternalError();
        }
        // Java: // Return a new least squares problem set up to fit a polynomial curve to the
        // Java: // observed points.
        // Java: return new LeastSquaresBuilder().
                // Java: maxEvaluations(Integer.MAX_VALUE).
                // Java: maxIterations(maxIter).
                // Java: start(initialGuess).
                // Java: target(target).
                // Java: weight(new DiagonalMatrix(weights)).
                // Java: model(model.getModelFunction(), model.getModelFunctionJacobian()).
                // Java: build();
        todo!("PolynomialCurveFitter::getProblem")
    }

}
