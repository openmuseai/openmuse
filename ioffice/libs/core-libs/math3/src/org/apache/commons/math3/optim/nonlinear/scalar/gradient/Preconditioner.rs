#![allow(unused_imports, dead_code, non_snake_case, unused_variables, unused_mut, unused_assignments)]

// Java: public interface represents
pub trait represents {}

// Java: public interface Preconditioner
pub trait Preconditioner {
    // Java: abstract double[] precondition(double[] point, double[] r)
    fn precondition(&self, point: &[f64], r: &[f64]) -> Vec<f64>;
}
