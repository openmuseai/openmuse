// Java: import org.apache.commons.math3.util.Precision
use crate::org::apache::commons::math3::util::Precision;
// Java: import org.apache.commons.math3.util.FastMath
use crate::org::apache::commons::math3::util::FastMath;
// Java: import org.apache.commons.math3.exception.NumberIsTooSmallException
use crate::org::apache::commons::math3::exception::NumberIsTooSmallException;
// Java: import org.apache.commons.math3.exception.NotStrictlyPositiveException
use crate::org::apache::commons::math3::exception::NotStrictlyPositiveException;
// Java: import org.apache.commons.math3.optimization.ConvergenceChecker
use crate::org::apache::commons::math3::optimization::ConvergenceChecker;
// Java: import org.apache.commons.math3.optimization.GoalType
use crate::org::apache::commons::math3::optimization::GoalType;

// Java field: double GOLDEN_SECTION
pub(crate) const GOLDEN_SECTION: f64 = 0.0;
// Java field: double MIN_RELATIVE_TOLERANCE
pub(crate) const MIN_RELATIVE_TOLERANCE: f64 = 0.0;

// Java: public class BrentOptimizer extends BaseAbstractUnivariateOptimizer
pub struct BrentOptimizer {
    // Java field: double relativeThreshold
    relativeThreshold: f64,
    // Java field: double absoluteThreshold
    absoluteThreshold: f64,
}

// Java: public class BrentOptimizer extends BaseAbstractUnivariateOptimizer
impl BrentOptimizer {
    // Java: BrentOptimizer()
    pub fn BrentOptimizer(&self) {
    }

    // Java: BrentOptimizer()
    pub fn BrentOptimizer_2(&self) {
    }

    // Java: UnivariatePointValuePair doOptimize()
    pub fn doOptimize(&self) -> Box<dyn std::any::Any> {
        // Java: final boolean isMinim = getGoalType() == GoalType.MINIMIZE;
        // Java: final double lo = getMin();
        // Java: final double mid = getStartValue();
        // Java: final double hi = getMax();
        // Java: // Optional additional convergence criteria.
        // Java: final ConvergenceChecker<UnivariatePointValuePair> checker
            // Java: = getConvergenceChecker();
        // Java: double a;
        // Java: double b;
        // Java: if (lo < hi)
        {
            // Java: a = lo;
            // Java: b = hi;
        }
        // Java: else
        {
            // Java: a = hi;
            // Java: b = lo;
        }
        // Java: double x = mid;
        // Java: double v = x;
        // Java: double w = x;
        // Java: double d = 0;
        // Java: double e = 0;
        // Java: double fx = computeObjectiveValue(x);
        // Java: if (!isMinim)
        {
            // Java: fx = -fx;
        }
        // Java: double fv = fx;
        // Java: double fw = fx;
        // Java: UnivariatePointValuePair previous = null;
        // Java: UnivariatePointValuePair current
            // Java: = new UnivariatePointValuePair(x, isMinim ? fx : -fx);
        // Java: // Best point encountered so far (which is the initial guess).
        // Java: UnivariatePointValuePair best = current;
        // Java: int iter = 0;
        // Java: while (true)
        {
            // Java: final double m = 0.5 * (a + b);
            // Java: final double tol1 = relativeThreshold * FastMath.abs(x) + absoluteThreshold;
            // Java: final double tol2 = 2 * tol1;
            // Java: // Default stopping criterion.
            // Java: final boolean stop = FastMath.abs(x - m) <= tol2 - 0.5 * (b - a);
            // Java: if (!stop)
            {
                // Java: double p = 0;
                // Java: double q = 0;
                // Java: double r = 0;
                // Java: double u = 0;
                // Java: if (FastMath.abs(e) > tol1) { // Fit parabola.
                    // Java: r = (x - w) * (fx - fv);
                    // Java: q = (x - v) * (fx - fw);
                    // Java: p = (x - v) * q - (x - w) * r;
                    // Java: q = 2 * (q - r);
                    // Java: if (q > 0)
                    {
                        // Java: p = -p;
                    }
                    // Java: else
                    {
                        // Java: q = -q;
                    }
                    // Java: r = e;
                    // Java: e = d;
                    // Java: if (p > q * (a - x) &&
                        // Java: p < q * (b - x) &&
                        // Java: FastMath.abs(p) < FastMath.abs(0.5 * q * r))
                        {
                        // Java: // Parabolic interpolation step.
                        // Java: d = p / q;
                        // Java: u = x + d;
                        // Java: // f must not be evaluated too close to a or b.
                        // Java: if (u - a < tol2 || b - u < tol2)
                        {
                            // Java: if (x <= m)
                            {
                                // Java: d = tol1;
                            }
                            // Java: else
                            {
                                // Java: d = -tol1;
                            }
                        }
                        }
                    // Java: else
                    {
                        // Java: // Golden section step.
                        // Java: if (x < m)
                        {
                            // Java: e = b - x;
                        }
                        // Java: else
                        {
                            // Java: e = a - x;
                        }
                        // Java: d = GOLDEN_SECTION * e;
                    }
                // Java: else
                {
                    // Java: // Golden section step.
                    // Java: if (x < m)
                    {
                        // Java: e = b - x;
                    }
                    // Java: else
                    {
                        // Java: e = a - x;
                    }
                    // Java: d = GOLDEN_SECTION * e;
                }
                // Java: // Update by at least "tol1".
                // Java: if (FastMath.abs(d) < tol1)
                {
                    // Java: if (d >= 0)
                    {
                        // Java: u = x + tol1;
                    }
                    // Java: else
                    {
                        // Java: u = x - tol1;
                    }
                }
                // Java: else
                {
                    // Java: u = x + d;
                }
                // Java: double fu = computeObjectiveValue(u);
                // Java: if (!isMinim)
                {
                    // Java: fu = -fu;
                }
                // Java: // User-defined convergence checker.
                // Java: previous = current;
                // Java: current = new UnivariatePointValuePair(u, isMinim ? fu : -fu);
                // Java: best = best(best,
                            // Java: best(previous,
                                 // Java: current,
                                 // Java: isMinim),
                            // Java: isMinim);
                // Java: if (checker != null && checker.converged(iter, previous, current))
                {
                    // Java: return best;
                }
                // Java: // Update a, b, v, w and x.
                // Java: if (fu <= fx)
                {
                    // Java: if (u < x)
                    {
                        // Java: b = x;
                    }
                    // Java: else
                    {
                        // Java: a = x;
                    }
                    // Java: v = w;
                    // Java: fv = fw;
                    // Java: w = x;
                    // Java: fw = fx;
                    // Java: x = u;
                    // Java: fx = fu;
                }
                // Java: else
                {
                    // Java: if (u < x)
                    {
                        // Java: a = u;
                    }
                    // Java: else
                    {
                        // Java: b = u;
                    }
                    // Java: if (fu <= fw ||
                        // Java: Precision.equals(w, x))
                        {
                        // Java: v = w;
                        // Java: fv = fw;
                        // Java: w = u;
                        // Java: fw = fu;
                        }
                               // Java: Precision.equals(v, x) ||
                               // Java: Precision.equals(v, w))
                               {
                        // Java: v = u;
                        // Java: fv = fu;
                               }
                }
            }
                // Java: return best(best,
                            // Java: best(previous,
                                 // Java: current,
                                 // Java: isMinim),
                            // Java: isMinim);
            // Java: ++iter;
        }
        todo!()
    }

    // Java: UnivariatePointValuePair best()
    pub fn best(&self) -> Box<dyn std::any::Any> {
        todo!()
    }

}