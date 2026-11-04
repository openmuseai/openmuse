// Java: import org.apache.commons.math3.linear.RealMatrix
use crate::org::apache::commons::math3::linear::RealMatrix;
// Java: import org.apache.commons.math3.linear.RealVector
use crate::org::apache::commons::math3::linear::RealVector;

// Java: public interface for
pub trait r#for {
}

// Java: public interface ValueAndJacobianFunction
pub trait ValueAndJacobianFunction {
    // Java: abstract RealVector computeValue(final double[] params)
    fn computeValue(&self) -> Box<dyn std::any::Any>;

    // Java: abstract RealMatrix computeJacobian(final double[] params)
    fn computeJacobian(&self) -> Box<dyn std::any::Any>;

}
