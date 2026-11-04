// Java: import org.apache.commons.math3.util.FastMath
use crate::org::apache::commons::math3::util::FastMath;
// Java: import org.apache.commons.math3.util.Incrementor
use crate::org::apache::commons::math3::util::Incrementor;
// Java: import org.apache.commons.math3.exception.NotStrictlyPositiveException
use crate::org::apache::commons::math3::exception::NotStrictlyPositiveException;
// Java: import org.apache.commons.math3.exception.TooManyEvaluationsException
use crate::org::apache::commons::math3::exception::TooManyEvaluationsException;
// Java: import org.apache.commons.math3.exception.MaxCountExceededException
use crate::org::apache::commons::math3::exception::MaxCountExceededException;
// Java: import org.apache.commons.math3.analysis.UnivariateFunction
use crate::org::apache::commons::math3::analysis::UnivariateFunction;
// Java: import org.apache.commons.math3.optimization.GoalType
use crate::org::apache::commons::math3::optimization::GoalType;

// Java field: double EPS_MIN
pub(crate) const EPS_MIN: f64 = 0.0;
// Java field: double GOLD
pub(crate) const GOLD: f64 = 0.0;

// Java: public class BracketFinder
pub struct BracketFinder {
    // Java field: double growLimit
    growLimit: f64,
    // Java field: Incrementor evaluations
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
        // Java: this(100, 50);
    }

    // Java: BracketFinder()
    pub fn BracketFinder_2(&self) {
    }

    // Java: void search(UnivariateFunction func, GoalType goal, double xA, double xB)
    pub fn search(&self) {
        // Java: evaluations.resetCount();
        // Java: final boolean isMinim = goal == GoalType.MINIMIZE;
        // Java: double fA = eval(func, xA);
        // Java: double fB = eval(func, xB);
        // Java: if (isMinim ?
            // Java: fA < fB :
            // Java: fA > fB)
            {
            // Java: double tmp = xA;
            // Java: xA = xB;
            // Java: xB = tmp;
            // Java: tmp = fA;
            // Java: fA = fB;
            // Java: fB = tmp;
            }
        // Java: double xC = xB + GOLD * (xB - xA);
        // Java: double fC = eval(func, xC);
        // Java: while (isMinim ? fC < fB : fC > fB)
        {
            // Java: double tmp1 = (xB - xA) * (fB - fC);
            // Java: double tmp2 = (xB - xC) * (fB - fA);
            // Java: double val = tmp2 - tmp1;
            // Java: double denom = FastMath.abs(val) < EPS_MIN ? 2 * EPS_MIN : 2 * val;
            // Java: double w = xB - ((xB - xC) * tmp2 - (xB - xA) * tmp1) / denom;
            // Java: double wLim = xB + growLimit * (xC - xB);
            // Java: double fW;
            // Java: if ((w - xC) * (xB - w) > 0)
            {
                // Java: fW = eval(func, w);
                // Java: if (isMinim ?
                    // Java: fW < fC :
                    // Java: fW > fC)
                    {
                    // Java: xA = xB;
                    // Java: xB = w;
                    // Java: fA = fB;
                    // Java: fB = fW;
                    // Java: break;
                    }
                           // Java: fW > fB :
                           // Java: fW < fB)
                           {
                    // Java: xC = w;
                    // Java: fC = fW;
                    // Java: break;
                           }
                // Java: w = xC + GOLD * (xC - xB);
                // Java: fW = eval(func, w);
            }
            // Java: else if ((w - wLim) * (wLim - xC) >= 0)
            {
                // Java: w = wLim;
                // Java: fW = eval(func, w);
            }
            // Java: else if ((w - wLim) * (xC - w) > 0)
            {
                // Java: fW = eval(func, w);
                // Java: if (isMinim ?
                    // Java: fW < fC :
                    // Java: fW > fC)
                    {
                    // Java: xB = xC;
                    // Java: xC = w;
                    // Java: w = xC + GOLD * (xC - xB);
                    // Java: fB = fC;
                    // Java: fC =fW;
                    // Java: fW = eval(func, w);
                    }
            }
            // Java: else
            {
                // Java: w = xC + GOLD * (xC - xB);
                // Java: fW = eval(func, w);
            }
            // Java: xA = xB;
            // Java: fA = fB;
            // Java: xB = xC;
            // Java: fB = fC;
            // Java: xC = w;
            // Java: fC = fW;
        }
        // Java: lo = xA;
        // Java: fLo = fA;
        // Java: mid = xB;
        // Java: fMid = fB;
        // Java: hi = xC;
        // Java: fHi = fC;
        // Java: if (lo > hi)
        {
            // Java: double tmp = lo;
            // Java: lo = hi;
            // Java: hi = tmp;
            // Java: tmp = fLo;
            // Java: fLo = fHi;
            // Java: fHi = tmp;
        }
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
            // Java: evaluations.incrementCount();
        }
        // Java: catch (MaxCountExceededException e)
        {
            // Java: throw new TooManyEvaluationsException(e.getMax());
        }
        // Java: return f.value(x);
        todo!()
    }

}
