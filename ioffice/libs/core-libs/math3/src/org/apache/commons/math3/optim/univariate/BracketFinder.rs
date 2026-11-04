// Java: import org.apache.commons.math3.util.FastMath
use crate::org::apache::commons::math3::util::FastMath;
// Java: import org.apache.commons.math3.util.IntegerSequence
use crate::org::apache::commons::math3::util::IntegerSequence;
// Java: import org.apache.commons.math3.exception.NotStrictlyPositiveException
use crate::org::apache::commons::math3::exception::NotStrictlyPositiveException;
// Java: import org.apache.commons.math3.exception.TooManyEvaluationsException
use crate::org::apache::commons::math3::exception::TooManyEvaluationsException;
// Java: import org.apache.commons.math3.exception.MaxCountExceededException
use crate::org::apache::commons::math3::exception::MaxCountExceededException;
// Java: import org.apache.commons.math3.analysis.UnivariateFunction
use crate::org::apache::commons::math3::analysis::UnivariateFunction;
// Java: import org.apache.commons.math3.optim.nonlinear.scalar.GoalType
use crate::org::apache::commons::math3::optim::nonlinear::scalar::GoalType;

// Java field: double EPS_MIN
pub(crate) const EPS_MIN: f64 = 0.0;
// Java field: double GOLD
pub(crate) const GOLD: f64 = 0.0;

// Java: public class BracketFinder
pub struct BracketFinder {
    // Java field: double growLimit
    growLimit: f64,
    // Java field: IntegerSequence.Incrementor evaluations
    pub(crate) evaluations: Box<dyn std::any::Any>,
    // Java field: double lo
    pub(crate) lo: f64,
    // Java field: double hi
    pub(crate) hi: f64,
    // Java field: double mid
    pub(crate) mid: f64,
    // Java field: double fLo
    pub(crate) fLo: f64,
    // Java field: double fHi
    pub(crate) fHi: f64,
    // Java field: double fMid
    pub(crate) fMid: f64,
}

// Java: public class BracketFinder
impl BracketFinder {
    // Java: BracketFinder()
    pub fn BracketFinder(&self) {
        // Java: this(100, 500);
    }

    // Java: BracketFinder()
    pub fn BracketFinder_2(&self) {
    }

    // Java: void search()
    pub fn search(&self) {
    }

    // Java: int getMaxEvaluations()
    pub fn getMaxEvaluations(&self) -> i32 {
        // Java: return evaluations.getMaximalCount();
        todo!()
    }

    // Java: int getEvaluations()
    pub fn getEvaluations(&self) -> i32 {
        // Java: return evaluations.getCount();
        todo!()
    }

    // Java: double getLo()
    pub fn getLo(&self) -> f64 {
        // Java: return lo;
        todo!()
    }

    // Java: double getFLo()
    pub fn getFLo(&self) -> f64 {
        // Java: return fLo;
        todo!()
    }

    // Java: double getHi()
    pub fn getHi(&self) -> f64 {
        // Java: return hi;
        todo!()
    }

    // Java: double getFHi()
    pub fn getFHi(&self) -> f64 {
        // Java: return fHi;
        todo!()
    }

    // Java: double getMid()
    pub fn getMid(&self) -> f64 {
        // Java: return mid;
        todo!()
    }

    // Java: double getFMid()
    pub fn getFMid(&self) -> f64 {
        // Java: return fMid;
        todo!()
    }

    // Java: double eval(UnivariateFunction f, double x)
    pub fn eval(&self) -> f64 {
        // Java: try
        {
            // Java: evaluations.increment();
        }
        // Java: catch (MaxCountExceededException e)
        {
            // Java: throw new TooManyEvaluationsException(e.getMax());
        }
        // Java: return f.value(x);
        todo!()
    }

}
