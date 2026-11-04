// Java: import org.apache.commons.math3.exception.MathIllegalArgumentException
use crate::org::apache::commons::math3::exception::MathIllegalArgumentException;

// Java field: throws MathIllegalArgumentException
// Java: public interface WeightedEvaluation
pub trait WeightedEvaluation {
    // Java: abstract double evaluate(double[] values, double[] weights)
    fn evaluate(&self) -> f64;

    // Java: abstract double evaluate(double[] values, double[] weights, int begin, int length)
    fn evaluate_2(&self) -> f64;

}
