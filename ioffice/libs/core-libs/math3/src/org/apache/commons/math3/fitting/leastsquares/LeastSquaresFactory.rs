// Java: import org.apache.commons.math3.exception.MathIllegalStateException
use crate::org::apache::commons::math3::exception::MathIllegalStateException;
// Java: import org.apache.commons.math3.exception.util.LocalizedFormats
use crate::org::apache::commons::math3::exception::util::LocalizedFormats;
// Java: import org.apache.commons.math3.analysis.MultivariateMatrixFunction
use crate::org::apache::commons::math3::analysis::MultivariateMatrixFunction;
// Java: import org.apache.commons.math3.analysis.MultivariateVectorFunction
use crate::org::apache::commons::math3::analysis::MultivariateVectorFunction;
// Java: import org.apache.commons.math3.fitting.leastsquares.LeastSquaresProblem.Evaluation
use crate::org::apache::commons::math3::fitting::leastsquares::LeastSquaresProblem::Evaluation;
// Java: import org.apache.commons.math3.linear.Array2DRowRealMatrix
use crate::org::apache::commons::math3::linear::Array2DRowRealMatrix;
// Java: import org.apache.commons.math3.linear.ArrayRealVector
use crate::org::apache::commons::math3::linear::ArrayRealVector;
// Java: import org.apache.commons.math3.linear.DiagonalMatrix
use crate::org::apache::commons::math3::linear::DiagonalMatrix;
// Java: import org.apache.commons.math3.linear.EigenDecomposition
use crate::org::apache::commons::math3::linear::EigenDecomposition;
// Java: import org.apache.commons.math3.linear.RealMatrix
use crate::org::apache::commons::math3::linear::RealMatrix;
// Java: import org.apache.commons.math3.linear.RealVector
use crate::org::apache::commons::math3::linear::RealVector;
// Java: import org.apache.commons.math3.optim.AbstractOptimizationProblem
use crate::org::apache::commons::math3::optim::AbstractOptimizationProblem;
// Java: import org.apache.commons.math3.optim.ConvergenceChecker
use crate::org::apache::commons::math3::optim::ConvergenceChecker;
// Java: import org.apache.commons.math3.optim.PointVectorValuePair
use crate::org::apache::commons::math3::optim::PointVectorValuePair;
// Java: import org.apache.commons.math3.util.FastMath
use crate::org::apache::commons::math3::util::FastMath;
// Java: import org.apache.commons.math3.util.Incrementor
use crate::org::apache::commons::math3::util::Incrementor;
// Java: import org.apache.commons.math3.util.Pair
use crate::org::apache::commons::math3::util::Pair;

// Java: public class LeastSquaresFactory
pub struct LeastSquaresFactory {
}

// Java: public class LocalValueAndJacobianFunction
pub struct LocalValueAndJacobianFunction {
}

// Java: public class LocalLeastSquaresProblem
pub struct LocalLeastSquaresProblem {
}

// Java: public class LeastSquaresFactory
impl LeastSquaresFactory {
    // Java: LeastSquaresFactory()
    pub fn LeastSquaresFactory(&self) {
    }

    // Java: static LeastSquaresProblem create()
    pub fn create(&self) -> Box<dyn std::any::Any> {
        todo!()
    }

    // Java: static LeastSquaresProblem create()
    pub fn create_2(&self) -> Box<dyn std::any::Any> {
        todo!()
    }

    // Java: static LeastSquaresProblem create()
    pub fn create_3(&self) -> Box<dyn std::any::Any> {
        todo!()
    }

    // Java: static LeastSquaresProblem create()
    pub fn create_4(&self) -> Box<dyn std::any::Any> {
        todo!()
    }

    // Java: static LeastSquaresProblem weightMatrix()
    pub fn weightMatrix(&self) -> Box<dyn std::any::Any> {
        todo!()
    }

    // Java: static LeastSquaresProblem weightDiagonal()
    pub fn weightDiagonal(&self) -> Box<dyn std::any::Any> {
        todo!()
    }

    // Java: static LeastSquaresProblem countEvaluations()
    pub fn countEvaluations(&self) -> Box<dyn std::any::Any> {
        todo!()
    }

    // Java: static ConvergenceChecker<Evaluation> evaluationChecker(final ConvergenceChecker<PointVectorValuePair> checker)
    pub fn evaluationChecker(&self) -> Box<dyn std::any::Any> {
        // Java: return new ConvergenceChecker<Evaluation>()
        {
            // Java: /** {@inheritDoc} */
            // Java: public boolean converged(final int iteration,
                                     // Java: final Evaluation previous,
                                     // Java: final Evaluation current)
                                     {
                // Java: return checker.converged(
                        // Java: iteration,
                        // Java: new PointVectorValuePair(
                                // Java: previous.getPoint().toArray(),
                                // Java: previous.getResiduals().toArray(),
                                // Java: false),
                        // Java: new PointVectorValuePair(
                                // Java: current.getPoint().toArray(),
                                // Java: current.getResiduals().toArray(),
                                // Java: false)
                // Java: );
                                     }
        }
        todo!()
    }

    // Java: static RealMatrix squareRoot(final RealMatrix m)
    pub fn squareRoot(&self) -> Box<dyn std::any::Any> {
        // Java: if (m instanceof DiagonalMatrix)
        {
            // Java: final int dim = m.getRowDimension();
            // Java: final RealMatrix sqrtM = new DiagonalMatrix(dim);
            // Java: for (int i = 0; i < dim; i++)
            {
                // Java: sqrtM.setEntry(i, i, FastMath.sqrt(m.getEntry(i, i)));
            }
            // Java: return sqrtM;
        }
        // Java: else
        {
            // Java: final EigenDecomposition dec = new EigenDecomposition(m);
            // Java: return dec.getSquareRoot();
        }
        todo!()
    }

    // Java: static MultivariateJacobianFunction model()
    pub fn model(&self) -> Box<dyn std::any::Any> {
        todo!()
    }

}

// Java: public class LocalValueAndJacobianFunction
impl LocalValueAndJacobianFunction {
}

// Java: public class LocalLeastSquaresProblem
impl LocalLeastSquaresProblem {
}
