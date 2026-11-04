// Java: import org.apache.commons.math3.RealFieldElement
use crate::org::apache::commons::math3::RealFieldElement;

// Java: public interface represents
pub trait represents {
}

// Java: public interface FirstOrderFieldDifferentialEquations extends RealFieldElement
pub trait FirstOrderFieldDifferentialEquations {
    // Java: abstract int getDimension()
    fn getDimension(&self) -> i32;

    // Java: abstract void init(T t0, T[] y0, T finalTime)
    fn init(&self);

    // Java: abstract T[] computeDerivatives(T t, T[] y)
    fn computeDerivatives(&self) -> Box<dyn std::any::Any>;

}
