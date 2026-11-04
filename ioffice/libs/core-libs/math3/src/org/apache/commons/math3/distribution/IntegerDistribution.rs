// Java: import org.apache.commons.math3.exception.NumberIsTooLargeException
use crate::org::apache::commons::math3::exception::NumberIsTooLargeException;
// Java: import org.apache.commons.math3.exception.OutOfRangeException
use crate::org::apache::commons::math3::exception::OutOfRangeException;

// Java: public interface IntegerDistribution
pub trait IntegerDistribution {
    // Java: abstract double probability(int x)
    fn probability(&self, x: i32) -> f64;

    // Java: abstract double cumulativeProbability(int x)
    fn cumulativeProbability(&self, x: i32) -> f64;

    // Java: abstract double cumulativeProbability(int x0, int x1)
    fn cumulativeProbability_1(&self, x0: i32, x1: i32) -> f64;

    // Java: abstract int inverseCumulativeProbability(double p)
    fn inverseCumulativeProbability(&self, p: f64) -> i32;

    // Java: abstract double getNumericalMean()
    fn getNumericalMean(&self) -> f64;

    // Java: abstract double getNumericalVariance()
    fn getNumericalVariance(&self) -> f64;

    // Java: abstract int getSupportLowerBound()
    fn getSupportLowerBound(&self) -> i32;

    // Java: abstract int getSupportUpperBound()
    fn getSupportUpperBound(&self) -> i32;

    // Java: abstract boolean isSupportConnected()
    fn isSupportConnected(&self) -> bool;

    // Java: abstract void reseedRandomGenerator(long seed)
    fn reseedRandomGenerator(&self, seed: i64);

    // Java: abstract int sample()
    fn sample(&self) -> i32;

    // Java: abstract int[] sample(int sampleSize)
    fn sample_1(&self, sampleSize: i32) -> Vec<i32>;

}
