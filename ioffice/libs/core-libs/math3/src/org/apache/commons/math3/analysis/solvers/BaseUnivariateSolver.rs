// Java: import org.apache.commons.math3.analysis.UnivariateFunction
use crate::org::apache::commons::math3::analysis::UnivariateFunction;
// Java: import org.apache.commons.math3.exception.MathIllegalArgumentException
use crate::org::apache::commons::math3::exception::MathIllegalArgumentException;
// Java: import org.apache.commons.math3.exception.TooManyEvaluationsException
use crate::org::apache::commons::math3::exception::TooManyEvaluationsException;

// Java: public class is
pub struct r#is {
}

// Java: public class is
impl r#is {
}

// Java field: throws MathIllegalArgumentException, TooManyEvaluationsException
// Java field: throws MathIllegalArgumentException, TooManyEvaluationsException
// Java: public interface BaseUnivariateSolver extends UnivariateFunction
pub trait BaseUnivariateSolver {
    // Java: abstract int getMaxEvaluations()
    fn getMaxEvaluations(&self) -> i32;

    // Java: abstract int getEvaluations()
    fn getEvaluations(&self) -> i32;

    // Java: abstract double getAbsoluteAccuracy()
    fn getAbsoluteAccuracy(&self) -> f64;

    // Java: abstract double getRelativeAccuracy()
    fn getRelativeAccuracy(&self) -> f64;

    // Java: abstract double getFunctionValueAccuracy()
    fn getFunctionValueAccuracy(&self) -> f64;

    // Java: abstract double solve(int maxEval, FUNC f, double min, double max)
    fn solve(&self) -> f64;

    // Java: abstract double solve(int maxEval, FUNC f, double min, double max, double startValue)
    fn solve_2(&self) -> f64;

    // Java: abstract double solve(int maxEval, FUNC f, double startValue)
    fn solve_3(&self) -> f64;

}
