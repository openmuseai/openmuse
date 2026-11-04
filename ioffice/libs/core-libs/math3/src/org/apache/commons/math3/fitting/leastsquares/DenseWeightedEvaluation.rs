// Java: import org.apache.commons.math3.fitting.leastsquares.LeastSquaresProblem.Evaluation
use crate::org::apache::commons::math3::fitting::leastsquares::LeastSquaresProblem::Evaluation;
// Java: import org.apache.commons.math3.linear.RealMatrix
use crate::org::apache::commons::math3::linear::RealMatrix;
// Java: import org.apache.commons.math3.linear.RealVector
use crate::org::apache::commons::math3::linear::RealVector;

// Java: public class DenseWeightedEvaluation extends AbstractEvaluation
pub struct DenseWeightedEvaluation {
    // Java field: Evaluation unweighted
    pub(crate) unweighted: Box<dyn std::any::Any>,
    // Java field: RealMatrix weightSqrt
    pub(crate) weightSqrt: Box<dyn std::any::Any>,
}

// Java: public class DenseWeightedEvaluation extends AbstractEvaluation
impl DenseWeightedEvaluation {
    // Java: RealMatrix getJacobian()
    pub fn getJacobian(&self) -> Box<dyn std::any::Any> {
        // Java: return weightSqrt.multiply(this.unweighted.getJacobian());
        todo!()
    }

    // Java: RealVector getResiduals()
    pub fn getResiduals(&self) -> Box<dyn std::any::Any> {
        // Java: return this.weightSqrt.operate(this.unweighted.getResiduals());
        todo!()
    }

    // Java: RealVector getPoint()
    pub fn getPoint(&self) -> Box<dyn std::any::Any> {
        // Java: return unweighted.getPoint();
        todo!()
    }

}
