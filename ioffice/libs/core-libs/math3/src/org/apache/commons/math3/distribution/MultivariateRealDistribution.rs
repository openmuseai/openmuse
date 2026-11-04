// Java: import org.apache.commons.math3.exception.NotStrictlyPositiveException
use crate::org::apache::commons::math3::exception::NotStrictlyPositiveException;

// Java: public interface for
pub trait r#for {
}

// Java: public interface MultivariateRealDistribution
pub trait MultivariateRealDistribution {
    // Java: abstract double density(double[] x)
    fn density(&self, x: &[f64]) -> f64;

    // Java: abstract void reseedRandomGenerator(long seed)
    fn reseedRandomGenerator(&self, seed: i64);

    // Java: abstract int getDimension()
    fn getDimension(&self) -> i32;

    // Java: abstract double[] sample()
    fn sample(&self) -> Vec<f64>;

    // Java: abstract double[][] sample(int sampleSize)
    fn sample_1(&self, sampleSize: i32) -> Vec<Vec<f64>>;

}
