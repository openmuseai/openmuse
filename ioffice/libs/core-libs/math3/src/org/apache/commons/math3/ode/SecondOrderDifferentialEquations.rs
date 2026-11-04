// Java: public interface represents
pub trait represents {
}

// Java: public interface SecondOrderDifferentialEquations
pub trait SecondOrderDifferentialEquations {
    // Java: abstract int getDimension()
    fn getDimension(&self) -> i32;

    // Java: abstract void computeSecondDerivatives(double t, double[] y, double[] yDot, double[] yDDot)
    fn computeSecondDerivatives(&self);

}
