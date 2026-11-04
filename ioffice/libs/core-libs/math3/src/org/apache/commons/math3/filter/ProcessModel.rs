// Java: import org.apache.commons.math3.linear.RealMatrix
use crate::org::apache::commons::math3::linear::RealMatrix;
// Java: import org.apache.commons.math3.linear.RealVector
use crate::org::apache::commons::math3::linear::RealVector;

// Java: public interface ProcessModel
pub trait ProcessModel {
    // Java: abstract RealMatrix getStateTransitionMatrix()
    fn getStateTransitionMatrix(&self) -> Box<dyn std::any::Any>;

    // Java: abstract RealMatrix getControlMatrix()
    fn getControlMatrix(&self) -> Box<dyn std::any::Any>;

    // Java: abstract RealMatrix getProcessNoise()
    fn getProcessNoise(&self) -> Box<dyn std::any::Any>;

    // Java: abstract RealVector getInitialStateEstimate()
    fn getInitialStateEstimate(&self) -> Box<dyn std::any::Any>;

    // Java: abstract RealMatrix getInitialErrorCovariance()
    fn getInitialErrorCovariance(&self) -> Box<dyn std::any::Any>;

}
