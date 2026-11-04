// Java: public interface represents
pub trait represents {
}

// Java: public interface Preconditioner
pub trait Preconditioner {
    // Java: abstract double[] precondition(double[] point, double[] r)
    fn precondition(&self) -> Vec<f64>;

}
