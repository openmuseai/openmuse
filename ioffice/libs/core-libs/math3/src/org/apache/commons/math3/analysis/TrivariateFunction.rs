// Java: public interface representing
// Java: public interface TrivariateFunction
pub trait TrivariateFunction {
    // Java: abstract double value(double x, double y, double z)
    fn value(&self, x: f64, y: f64, z: f64) -> f64;
}
