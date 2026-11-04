// Java: import org.apache.commons.math3.linear.RealMatrix
use crate::org::apache::commons::math3::linear::RealMatrix;
// Java: import org.apache.commons.math3.linear.RealVector
use crate::org::apache::commons::math3::linear::RealVector;
// Java: import org.apache.commons.math3.util.Pair
use crate::org::apache::commons::math3::util::Pair;

// Java: public interface for
pub trait r#for {
}

// Java: public interface MultivariateJacobianFunction
pub trait MultivariateJacobianFunction {
    // Java: abstract Pair<RealVector, RealMatrix> value(RealVector point)
    fn value(&self, point: &dyn std::any::Any) -> Box<dyn std::any::Any>;

}
