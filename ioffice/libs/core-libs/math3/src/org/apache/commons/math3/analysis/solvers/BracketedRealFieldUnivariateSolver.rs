// Java: import org.apache.commons.math3.RealFieldElement
use crate::org::apache::commons::math3::RealFieldElement;
// Java: import org.apache.commons.math3.analysis.RealFieldUnivariateFunction
use crate::org::apache::commons::math3::analysis::RealFieldUnivariateFunction;

// Java: public interface BracketedRealFieldUnivariateSolver extends RealFieldElement
pub trait BracketedRealFieldUnivariateSolver {
    // Java: abstract int getMaxEvaluations()
    fn getMaxEvaluations(&self) -> i32;

    // Java: abstract int getEvaluations()
    fn getEvaluations(&self) -> i32;

    // Java: abstract T getAbsoluteAccuracy()
    fn getAbsoluteAccuracy(&self) -> Box<dyn std::any::Any>;

    // Java: abstract T getRelativeAccuracy()
    fn getRelativeAccuracy(&self) -> Box<dyn std::any::Any>;

    // Java: abstract T getFunctionValueAccuracy()
    fn getFunctionValueAccuracy(&self) -> Box<dyn std::any::Any>;

    // Java: abstract T solve()
    fn solve(&self) -> Box<dyn std::any::Any>;

    // Java: abstract T solve()
    fn solve_2(&self) -> Box<dyn std::any::Any>;

}
