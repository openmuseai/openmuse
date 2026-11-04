// Java: import org.apache.commons.math3.linear.RealMatrix
use crate::org::apache::commons::math3::linear::RealMatrix;

// Java: public interface for
pub trait r#for {
}

// Java: public interface StatisticalMultivariateSummary
pub trait StatisticalMultivariateSummary {
    // Java: abstract int getDimension()
    fn getDimension(&self) -> i32;

    // Java: abstract double[] getMean()
    fn getMean(&self) -> Vec<f64>;

    // Java: abstract RealMatrix getCovariance()
    fn getCovariance(&self) -> Box<dyn std::any::Any>;

    // Java: abstract double[] getStandardDeviation()
    fn getStandardDeviation(&self) -> Vec<f64>;

    // Java: abstract double[] getMax()
    fn getMax(&self) -> Vec<f64>;

    // Java: abstract double[] getMin()
    fn getMin(&self) -> Vec<f64>;

    // Java: abstract long getN()
    fn getN(&self) -> i64;

    // Java: abstract double[] getGeometricMean()
    fn getGeometricMean(&self) -> Vec<f64>;

    // Java: abstract double[] getSum()
    fn getSum(&self) -> Vec<f64>;

    // Java: abstract double[] getSumSq()
    fn getSumSq(&self) -> Vec<f64>;

    // Java: abstract double[] getSumLog()
    fn getSumLog(&self) -> Vec<f64>;

}
