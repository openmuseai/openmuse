// Java: public interface represents
pub trait represents {
}

// Java: public interface RandomVectorGenerator
pub trait RandomVectorGenerator {
    // Java: abstract double[] nextVector()
    fn nextVector(&self) -> Vec<f64>;

}
