// Java: public interface for
pub trait r#for {
}

// Java: public interface StatisticalSummary
pub trait StatisticalSummary {
    // Java: abstract double getMean()
    fn getMean(&self) -> f64;

    // Java: abstract double getVariance()
    fn getVariance(&self) -> f64;

    // Java: abstract double getStandardDeviation()
    fn getStandardDeviation(&self) -> f64;

    // Java: abstract double getMax()
    fn getMax(&self) -> f64;

    // Java: abstract double getMin()
    fn getMin(&self) -> f64;

    // Java: abstract long getN()
    fn getN(&self) -> i64;

    // Java: abstract double getSum()
    fn getSum(&self) -> f64;

}
