// Java: import org.apache.commons.math3.RealFieldElement
use crate::org::apache::commons::math3::RealFieldElement;

// Java: public interface represents
pub trait represents {
}

// Java: public interface FieldButcherArrayProvider
pub trait FieldButcherArrayProvider {
    // Java: abstract T[] getC()
    fn getC(&self) -> Box<dyn std::any::Any>;

    // Java: abstract T[][] getA()
    fn getA(&self) -> Box<dyn std::any::Any>;

    // Java: abstract T[] getB()
    fn getB(&self) -> Box<dyn std::any::Any>;

}
