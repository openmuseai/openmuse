// Java: import org.apache.commons.math3.exception.NumberIsTooLargeException
use crate::org::apache::commons::math3::exception::NumberIsTooLargeException;
// Java: import org.apache.commons.math3.exception.OutOfRangeException
use crate::org::apache::commons::math3::exception::OutOfRangeException;

// Java: public interface for
pub trait r#for {
}

// Java: public interface RealDistribution
pub trait RealDistribution {
    // Java: abstract double probability(double x)
    fn probability(&self, x: f64) -> f64;

    // Java: abstract double density(double x)
    fn density(&self, x: f64) -> f64;

    // Java: abstract double cumulativeProbability(double x)
    fn cumulativeProbability(&self, x: f64) -> f64;

    // Java: abstract double cumulativeProbability(double x0, double x1)
    fn cumulativeProbability_1(&self, x0: f64, x1: f64) -> f64;

    // Java: abstract double inverseCumulativeProbability(double p)
    fn inverseCumulativeProbability(&self, p: f64) -> f64;

    // Java: abstract double getNumericalMean()
    fn getNumericalMean(&self) -> f64;

    // Java: abstract double getNumericalVariance()
    fn getNumericalVariance(&self) -> f64;

    // Java: abstract double getSupportLowerBound()
    fn getSupportLowerBound(&self) -> f64;

    // Java: abstract double getSupportUpperBound()
    fn getSupportUpperBound(&self) -> f64;

    // Java: abstract boolean isSupportLowerBoundInclusive()
    fn isSupportLowerBoundInclusive(&self) -> bool;

    // Java: abstract boolean isSupportUpperBoundInclusive()
    fn isSupportUpperBoundInclusive(&self) -> bool;

    // Java: abstract boolean isSupportConnected()
    fn isSupportConnected(&self) -> bool;

    // Java: abstract void reseedRandomGenerator(long seed)
    fn reseedRandomGenerator(&self, seed: i64);

    // Java: abstract double sample()
    fn sample(&self) -> f64;

    // Java: abstract double[] sample(int sampleSize)
    fn sample_1(&self, sampleSize: i32) -> Vec<f64>;

}
