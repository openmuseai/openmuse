// Java: import org.apache.commons.math3.linear.RealMatrix
use crate::org::apache::commons::math3::linear::RealMatrix;

// Java: public interface MeasurementModel
pub trait MeasurementModel {
    // Java: abstract RealMatrix getMeasurementMatrix()
    fn getMeasurementMatrix(&self) -> Box<dyn std::any::Any>;

    // Java: abstract RealMatrix getMeasurementNoise()
    fn getMeasurementNoise(&self) -> Box<dyn std::any::Any>;

}
