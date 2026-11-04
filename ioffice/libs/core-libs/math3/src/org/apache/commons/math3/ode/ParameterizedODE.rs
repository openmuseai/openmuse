// Java: public interface ParameterizedODE extends Parameterizable
pub trait ParameterizedODE {
    // Java: abstract double getParameter(String name)
    fn getParameter(&self) -> f64;

    // Java: abstract void setParameter(String name, double value)
    fn setParameter(&self);

}
