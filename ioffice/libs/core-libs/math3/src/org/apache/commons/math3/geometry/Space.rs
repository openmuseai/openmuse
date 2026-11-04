// Java: import java.io.Serializable
// Java: import org.apache.commons.math3.exception.MathUnsupportedOperationException
use crate::org::apache::commons::math3::exception::MathUnsupportedOperationException;

// Java: public interface represents
pub trait represents {
}

// Java: public interface Space
pub trait Space {
    // Java: abstract int getDimension()
    fn getDimension(&self) -> i32;

    // Java: abstract Space getSubSpace()
    fn getSubSpace(&self) -> Box<dyn std::any::Any>;

}
