// Java: import org.apache.commons.math3.exception.MathIllegalArgumentException
use crate::org::apache::commons::math3::exception::MathIllegalArgumentException;
// Java: import org.apache.commons.math3.util.MathArrays
use crate::org::apache::commons::math3::util::MathArrays;

// Java: public interface implemented
pub trait implemented {
}

// Java: public interface UnivariateStatistic
pub trait UnivariateStatistic {
    // Java: abstract double evaluate(double[] values)
    fn evaluate(&self) -> f64;

    // Java: abstract double evaluate(double[] values, int begin, int length)
    fn evaluate_2(&self) -> f64;

    // Java: abstract UnivariateStatistic copy()
    fn copy(&self) -> Box<dyn std::any::Any>;

}
