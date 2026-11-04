// Java: import org.apache.commons.math3.exception.ConvergenceException
use crate::org::apache::commons::math3::exception::ConvergenceException;
// Java: import org.apache.commons.math3.exception.NullArgumentException
use crate::org::apache::commons::math3::exception::NullArgumentException;
// Java: import org.apache.commons.math3.exception.util.LocalizedFormats
use crate::org::apache::commons::math3::exception::util::LocalizedFormats;
// Java: import org.apache.commons.math3.fitting.leastsquares.LeastSquaresProblem.Evaluation
use crate::org::apache::commons::math3::fitting::leastsquares::LeastSquaresProblem::Evaluation;
// Java: import org.apache.commons.math3.linear.ArrayRealVector
use crate::org::apache::commons::math3::linear::ArrayRealVector;
// Java: import org.apache.commons.math3.linear.CholeskyDecomposition
use crate::org::apache::commons::math3::linear::CholeskyDecomposition;
// Java: import org.apache.commons.math3.linear.LUDecomposition
use crate::org::apache::commons::math3::linear::LUDecomposition;
// Java: import org.apache.commons.math3.linear.MatrixUtils
use crate::org::apache::commons::math3::linear::MatrixUtils;
// Java: import org.apache.commons.math3.linear.NonPositiveDefiniteMatrixException
use crate::org::apache::commons::math3::linear::NonPositiveDefiniteMatrixException;
// Java: import org.apache.commons.math3.linear.QRDecomposition
use crate::org::apache::commons::math3::linear::QRDecomposition;
// Java: import org.apache.commons.math3.linear.RealMatrix
use crate::org::apache::commons::math3::linear::RealMatrix;
// Java: import org.apache.commons.math3.linear.RealVector
use crate::org::apache::commons::math3::linear::RealVector;
// Java: import org.apache.commons.math3.linear.SingularMatrixException
use crate::org::apache::commons::math3::linear::SingularMatrixException;
// Java: import org.apache.commons.math3.linear.SingularValueDecomposition
use crate::org::apache::commons::math3::linear::SingularValueDecomposition;
// Java: import org.apache.commons.math3.optim.ConvergenceChecker
use crate::org::apache::commons::math3::optim::ConvergenceChecker;
// Java: import org.apache.commons.math3.util.Incrementor
use crate::org::apache::commons::math3::util::Incrementor;
// Java: import org.apache.commons.math3.util.Pair
use crate::org::apache::commons::math3::util::Pair;

// Java field: double SINGULARITY_THRESHOLD
pub(crate) const SINGULARITY_THRESHOLD: f64 = 0.0;

// Java: public class solve
pub struct solve {
}

// Java: public class GaussNewtonOptimizer
pub struct GaussNewtonOptimizer {
    // Java field: Decomposition decomposition
    pub(crate) decomposition: Box<dyn std::any::Any>,
}

// Java: public enum Decomposition
pub struct Decomposition {
}

// Java: public class solve
impl solve {
}

// Java: public class GaussNewtonOptimizer
impl GaussNewtonOptimizer {
    // Java: GaussNewtonOptimizer()
    pub fn GaussNewtonOptimizer(&self) {
        // Java: this(Decomposition.QR);
    }

    // Java: GaussNewtonOptimizer(final Decomposition decomposition)
    pub fn GaussNewtonOptimizer_2(&self) {
        // Java: this.decomposition = decomposition;
    }

    // Java: Decomposition getDecomposition()
    pub fn getDecomposition(&self) -> Box<dyn std::any::Any> {
        // Java: return this.decomposition;
        todo!()
    }

    // Java: GaussNewtonOptimizer withDecomposition(final Decomposition newDecomposition)
    pub fn withDecomposition(&self) -> Box<dyn std::any::Any> {
        // Java: return new GaussNewtonOptimizer(newDecomposition);
        todo!()
    }

    // Java: Optimum optimize(final LeastSquaresProblem lsp)
    pub fn optimize(&self) -> Box<dyn std::any::Any> {
        // Java: //create local evaluation and iteration counts
        // Java: final Incrementor evaluationCounter = lsp.getEvaluationCounter();
        // Java: final Incrementor iterationCounter = lsp.getIterationCounter();
        // Java: final ConvergenceChecker<Evaluation> checker
                // Java: = lsp.getConvergenceChecker();
        // Java: // Computation will be useless without a checker (see "for-loop").
        // Java: if (checker == null)
        {
            // Java: throw new NullArgumentException();
        }
        // Java: RealVector currentPoint = lsp.getStart();
        // Java: // iterate until convergence is reached
        // Java: Evaluation current = null;
        // Java: while (true)
        {
            // Java: iterationCounter.incrementCount();
            // Java: // evaluate the objective function and its jacobian
            // Java: Evaluation previous = current;
            // Java: // Value of the objective function at "currentPoint".
            // Java: evaluationCounter.incrementCount();
            // Java: current = lsp.evaluate(currentPoint);
            // Java: final RealVector currentResiduals = current.getResiduals();
            // Java: final RealMatrix weightedJacobian = current.getJacobian();
            // Java: currentPoint = current.getPoint();
            // Java: // Check convergence.
            // Java: if (previous != null &&
                // Java: checker.converged(iterationCounter.getCount(), previous, current))
                {
                // Java: return new OptimumImpl(current,
                                       // Java: evaluationCounter.getCount(),
                                       // Java: iterationCounter.getCount());
                }
            // Java: // solve the linearized least squares problem
            // Java: final RealVector dX = this.decomposition.solve(weightedJacobian, currentResiduals);
            // Java: // update the estimated parameters
            // Java: currentPoint = currentPoint.add(dX);
        }
        todo!()
    }

    // Java: String toString()
    pub fn toString(&self) -> String {
        // Java: return "GaussNewtonOptimizer{" +
                // Java: "decomposition=" + decomposition +
                // Java: '}';
        todo!()
    }

    // Java: static private static Pair<RealMatrix, RealVector> computeNormalMatrix()
    pub fn computeNormalMatrix(&self) -> Box<dyn std::any::Any> {
        todo!()
    }

}

// Java: public enum Decomposition
impl Decomposition {
    // Java: abstract RealVector solve()
    pub fn solve(&self) -> Box<dyn std::any::Any> {
        todo!()
    }

}
