// Java: import org.apache.commons.math3.exception.MathIllegalArgumentException
use crate::org::apache::commons::math3::exception::MathIllegalArgumentException;

// Java: public interface is
pub trait is {
}

// Java: public interface StorelessUnivariateStatistic
pub trait StorelessUnivariateStatistic {
    // Java: abstract void increment(double d)
    fn increment(&self);

    // Java: abstract void incrementAll(double[] values)
    fn incrementAll(&self);

    // Java: abstract void incrementAll(double[] values, int start, int length)
    fn incrementAll_2(&self);

    // Java: abstract double getResult()
    fn getResult(&self) -> f64;

    // Java: abstract long getN()
    fn getN(&self) -> i64;

    // Java: abstract void clear()
    fn clear(&self);

    // Java: abstract StorelessUnivariateStatistic copy()
    fn copy(&self) -> Box<dyn std::any::Any>;

}
