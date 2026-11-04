// Java: import org.apache.commons.math3.fitting.leastsquares.LeastSquaresProblem.Evaluation
use crate::org::apache::commons::math3::fitting::leastsquares::LeastSquaresProblem::Evaluation;
// Java: import org.apache.commons.math3.linear.ArrayRealVector
use crate::org::apache::commons::math3::linear::ArrayRealVector;
// Java: import org.apache.commons.math3.linear.DecompositionSolver
use crate::org::apache::commons::math3::linear::DecompositionSolver;
// Java: import org.apache.commons.math3.linear.QRDecomposition
use crate::org::apache::commons::math3::linear::QRDecomposition;
// Java: import org.apache.commons.math3.linear.RealMatrix
use crate::org::apache::commons::math3::linear::RealMatrix;
// Java: import org.apache.commons.math3.linear.RealVector
use crate::org::apache::commons::math3::linear::RealVector;
// Java: import org.apache.commons.math3.util.FastMath
use crate::org::apache::commons::math3::util::FastMath;

// Java: public class AbstractEvaluation implements Evaluation
pub struct AbstractEvaluation {
    // Java field: int observationSize
    observationSize: i32,
}

// Java: public class AbstractEvaluation implements Evaluation
impl AbstractEvaluation {
    // Java: RealMatrix getCovariances(double threshold)
    pub fn getCovariances(&self) -> Box<dyn std::any::Any> {
        // Java: // Set up the Jacobian.
        // Java: final RealMatrix j = this.getJacobian();
        // Java: // Compute transpose(J)J.
        // Java: final RealMatrix jTj = j.transpose().multiply(j);
        // Java: // Compute the covariances matrix.
        // Java: final DecompositionSolver solver
                // Java: = new QRDecomposition(jTj, threshold).getSolver();
        // Java: return solver.getInverse();
        todo!()
    }

    // Java: RealVector getSigma(double covarianceSingularityThreshold)
    pub fn getSigma(&self) -> Box<dyn std::any::Any> {
        // Java: final RealMatrix cov = this.getCovariances(covarianceSingularityThreshold);
        // Java: final int nC = cov.getColumnDimension();
        // Java: final RealVector sig = new ArrayRealVector(nC);
        // Java: for (int i = 0; i < nC; ++i)
        {
            // Java: sig.setEntry(i, FastMath.sqrt(cov.getEntry(i,i)));
        }
        // Java: return sig;
        todo!()
    }

    // Java: double getRMS()
    pub fn getRMS(&self) -> f64 {
        // Java: final double cost = this.getCost();
        // Java: return FastMath.sqrt(cost * cost / this.observationSize);
        todo!()
    }

    // Java: double getCost()
    pub fn getCost(&self) -> f64 {
        // Java: final ArrayRealVector r = new ArrayRealVector(this.getResiduals());
        // Java: return FastMath.sqrt(r.dotProduct(r));
        todo!()
    }

}

