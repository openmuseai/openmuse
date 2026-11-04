// Java: import org.apache.commons.math3.exception.ConvergenceException
use crate::org::apache::commons::math3::exception::ConvergenceException;
// Java: import org.apache.commons.math3.exception.NullArgumentException
use crate::org::apache::commons::math3::exception::NullArgumentException;
// Java: import org.apache.commons.math3.exception.MathInternalError
use crate::org::apache::commons::math3::exception::MathInternalError;
// Java: import org.apache.commons.math3.exception.util.LocalizedFormats
use crate::org::apache::commons::math3::exception::util::LocalizedFormats;
// Java: import org.apache.commons.math3.linear.ArrayRealVector
use crate::org::apache::commons::math3::linear::ArrayRealVector;
// Java: import org.apache.commons.math3.linear.BlockRealMatrix
use crate::org::apache::commons::math3::linear::BlockRealMatrix;
// Java: import org.apache.commons.math3.linear.DecompositionSolver
use crate::org::apache::commons::math3::linear::DecompositionSolver;
// Java: import org.apache.commons.math3.linear.LUDecomposition
use crate::org::apache::commons::math3::linear::LUDecomposition;
// Java: import org.apache.commons.math3.linear.QRDecomposition
use crate::org::apache::commons::math3::linear::QRDecomposition;
// Java: import org.apache.commons.math3.linear.RealMatrix
use crate::org::apache::commons::math3::linear::RealMatrix;
// Java: import org.apache.commons.math3.linear.SingularMatrixException
use crate::org::apache::commons::math3::linear::SingularMatrixException;
// Java: import org.apache.commons.math3.optimization.ConvergenceChecker
use crate::org::apache::commons::math3::optimization::ConvergenceChecker;
// Java: import org.apache.commons.math3.optimization.SimpleVectorValueChecker
use crate::org::apache::commons::math3::optimization::SimpleVectorValueChecker;
// Java: import org.apache.commons.math3.optimization.PointVectorValuePair
use crate::org::apache::commons::math3::optimization::PointVectorValuePair;

// Java: public class solve
pub struct solve {
}

// Java: public class GaussNewtonOptimizer
pub struct GaussNewtonOptimizer {
    // Java field: boolean useLU
    pub(crate) useLU: bool,
}

// Java: public class solve
impl solve {
}

// Java: public class GaussNewtonOptimizer
impl GaussNewtonOptimizer {
    // Java: GaussNewtonOptimizer()
    pub fn GaussNewtonOptimizer(&self) {
        // Java: this(true);
    }

    // Java: GaussNewtonOptimizer(ConvergenceChecker<PointVectorValuePair> checker)
    pub fn GaussNewtonOptimizer_2(&self) {
        // Java: this(true, checker);
    }

    // Java: GaussNewtonOptimizer(final boolean useLU)
    pub fn GaussNewtonOptimizer_3(&self) {
        // Java: this(useLU, new SimpleVectorValueChecker());
    }

    // Java: GaussNewtonOptimizer()
    pub fn GaussNewtonOptimizer_4(&self) {
    }

    // Java: PointVectorValuePair doOptimize()
    pub fn doOptimize(&self) -> Box<dyn std::any::Any> {
        // Java: final ConvergenceChecker<PointVectorValuePair> checker
            // Java: = getConvergenceChecker();
        // Java: // Computation will be useless without a checker (see "for-loop").
        // Java: if (checker == null)
        {
            // Java: throw new NullArgumentException();
        }
        // Java: final double[] targetValues = getTarget();
        // Java: final int nR = targetValues.length; // Number of observed data.
        // Java: final RealMatrix weightMatrix = getWeight();
        // Java: // Diagonal of the weight matrix.
        // Java: final double[] residualsWeights = new double[nR];
        // Java: for (int i = 0; i < nR; i++)
        {
            // Java: residualsWeights[i] = weightMatrix.getEntry(i, i);
        }
        // Java: final double[] currentPoint = getStartPoint();
        // Java: final int nC = currentPoint.length;
        // Java: // iterate until convergence is reached
        // Java: PointVectorValuePair current = null;
        // Java: int iter = 0;
        // Java: for (boolean converged = false; !converged;)
        {
            // Java: ++iter;
            // Java: // evaluate the objective function and its jacobian
            // Java: PointVectorValuePair previous = current;
            // Java: // Value of the objective function at "currentPoint".
            // Java: final double[] currentObjective = computeObjectiveValue(currentPoint);
            // Java: final double[] currentResiduals = computeResiduals(currentObjective);
            // Java: final RealMatrix weightedJacobian = computeWeightedJacobian(currentPoint);
            // Java: current = new PointVectorValuePair(currentPoint, currentObjective);
            // Java: // build the linear problem
            // Java: final double[]   b = new double[nC];
            // Java: final double[][] a = new double[nC][nC];
            // Java: for (int i = 0; i < nR; ++i)
            {
                // Java: final double[] grad   = weightedJacobian.getRow(i);
                // Java: final double weight   = residualsWeights[i];
                // Java: final double residual = currentResiduals[i];
                // Java: // compute the normal equation
                // Java: final double wr = weight * residual;
                // Java: for (int j = 0; j < nC; ++j)
                {
                    // Java: b[j] += wr * grad[j];
                }
                // Java: // build the contribution matrix for measurement i
                // Java: for (int k = 0; k < nC; ++k)
                {
                    // Java: double[] ak = a[k];
                    // Java: double wgk = weight * grad[k];
                    // Java: for (int l = 0; l < nC; ++l)
                    {
                        // Java: ak[l] += wgk * grad[l];
                    }
                }
            }
            // Java: try
            {
                // Java: // solve the linearized least squares problem
                // Java: RealMatrix mA = new BlockRealMatrix(a);
                // Java: DecompositionSolver solver = useLU ?
                        // Java: new LUDecomposition(mA).getSolver() :
                        // Java: new QRDecomposition(mA).getSolver();
                // Java: final double[] dX = solver.solve(new ArrayRealVector(b, false)).toArray();
                // Java: // update the estimated parameters
                // Java: for (int i = 0; i < nC; ++i)
                {
                    // Java: currentPoint[i] += dX[i];
                }
            }
            // Java: catch (SingularMatrixException e)
            {
                // Java: throw new ConvergenceException(LocalizedFormats.UNABLE_TO_SOLVE_SINGULAR_PROBLEM);
            }
            // Java: // Check convergence.
            // Java: if (previous != null)
            {
                // Java: converged = checker.converged(iter, previous, current);
                // Java: if (converged)
                {
                    // Java: cost = computeCost(currentResiduals);
                    // Java: // Update (deprecated) "point" field.
                    // Java: point = current.getPoint();
                    // Java: return current;
                }
            }
        }
        // Java: // Must never happen.
        // Java: throw new MathInternalError();
        todo!()
    }

}
