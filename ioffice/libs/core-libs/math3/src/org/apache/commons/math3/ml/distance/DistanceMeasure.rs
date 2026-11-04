// Java: import java.io.Serializable
// Java: import org.apache.commons.math3.exception.DimensionMismatchException
use crate::org::apache::commons::math3::exception::DimensionMismatchException;

// Java: public interface DistanceMeasure extends Serializable
pub trait DistanceMeasure {
    // Java: abstract double compute(double[] a, double[] b)
    fn compute(&self) -> f64;

}
