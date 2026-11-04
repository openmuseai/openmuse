// Java: import org.apache.commons.math3.linear.RealVector
use crate::org::apache::commons::math3::linear::RealVector;

// Java: public interface ParameterValidator
pub trait ParameterValidator {
    // Java: abstract RealVector validate(RealVector params)
    fn validate(&self, params: &dyn std::any::Any) -> Box<dyn std::any::Any>;

}
