// Java: import org.apache.commons.math3.linear.RealMatrix
use crate::org::apache::commons::math3::linear::RealMatrix;
// Java: import org.apache.commons.math3.linear.RealVector
use crate::org::apache::commons::math3::linear::RealVector;
// Java: import org.apache.commons.math3.optim.OptimizationProblem
use crate::org::apache::commons::math3::optim::OptimizationProblem;

// Java: public interface LeastSquaresProblem extends OptimizationProblem
pub trait LeastSquaresProblem {
    // Java: abstract RealVector getStart()
    fn getStart(&self) -> Box<dyn std::any::Any>;

    // Java: abstract int getObservationSize()
    fn getObservationSize(&self) -> i32;

    // Java: abstract int getParameterSize()
    fn getParameterSize(&self) -> i32;

    // Java: abstract Evaluation evaluate(RealVector point)
    fn evaluate(&self, point: &dyn std::any::Any) -> Box<dyn Evaluation>;

}

// Java: public interface Evaluation
pub trait Evaluation {
    // Java: abstract RealMatrix getCovariances(double threshold)
    fn getCovariances(&self, threshold: f64) -> Box<dyn std::any::Any>;

    // Java: abstract RealVector getSigma(double covarianceSingularityThreshold)
    fn getSigma(&self, covarianceSingularityThreshold: f64) -> Box<dyn std::any::Any>;

    // Java: abstract double getRMS()
    fn getRMS(&self) -> f64;

    // Java: abstract RealMatrix getJacobian()
    fn getJacobian(&self) -> Box<dyn std::any::Any>;

    // Java: abstract double getCost()
    fn getCost(&self) -> f64;

    // Java: abstract RealVector getResiduals()
    fn getResiduals(&self) -> Box<dyn std::any::Any>;

    // Java: abstract RealVector getPoint()
    fn getPoint(&self) -> Box<dyn std::any::Any>;

}
