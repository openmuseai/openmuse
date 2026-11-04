#![allow(unused_imports, dead_code, non_snake_case, unused_variables, unused_mut, unused_assignments)]

// Java: import java.util.Comparator
// Java: import org.apache.commons.math3.analysis.MultivariateFunction
use crate::org::apache::commons::math3::analysis::MultivariateFunction;
// Java: import org.apache.commons.math3.exception.NullArgumentException
use crate::org::apache::commons::math3::exception::NullArgumentException;
// Java: import org.apache.commons.math3.exception.MathUnsupportedOperationException
use crate::org::apache::commons::math3::exception::MathUnsupportedOperationException;
// Java: import org.apache.commons.math3.exception.util.LocalizedFormats
use crate::org::apache::commons::math3::exception::util::LocalizedFormats;
// Java: import org.apache.commons.math3.optim.nonlinear.scalar.GoalType
use crate::org::apache::commons::math3::optim::nonlinear::scalar::GoalType;
// Java: import org.apache.commons.math3.optim.ConvergenceChecker
use crate::org::apache::commons::math3::optim::ConvergenceChecker;
// Java: import org.apache.commons.math3.optim.PointValuePair
use crate::org::apache::commons::math3::optim::PointValuePair;
// Java: import org.apache.commons.math3.optim.SimpleValueChecker
use crate::org::apache::commons::math3::optim::SimpleValueChecker;
// Java: import org.apache.commons.math3.optim.OptimizationData
use crate::org::apache::commons::math3::optim::OptimizationData;
// Java: import org.apache.commons.math3.optim.nonlinear.scalar.MultivariateOptimizer
use crate::org::apache::commons::math3::optim::nonlinear::scalar::MultivariateOptimizer;

use std::any::Any;

// Java: public class implements implements simplex
pub struct implements {}

// Java: public class SimplexOptimizer extends MultivariateOptimizer
pub struct SimplexOptimizer {
    // Java field: AbstractSimplex simplex
    simplex: Option<Box<dyn Any>>,
}

// Java: public class implements implements simplex
impl implements {}

// Java: public class SimplexOptimizer extends MultivariateOptimizer
impl SimplexOptimizer {
    // Java: SimplexOptimizer(ConvergenceChecker<PointValuePair> checker)
    pub fn new(checker: Box<dyn Any>) -> Self {
        // Java: super(checker);
        SimplexOptimizer { simplex: None }
    }

    // Java: SimplexOptimizer(double rel, double abs)
    pub fn new_with_tolerances(rel: f64, abs: f64) -> Self {
        // Java: this(new SimpleValueChecker(rel, abs));
        SimplexOptimizer { simplex: None }
    }

    // Java: PointValuePair optimize(OptimizationData... optData)
    pub fn optimize(&self, optData: Vec<Box<dyn Any>>) -> Box<dyn Any> {
        // Java: // Set up base class and perform computation.
        // Java: return super.optimize(optData);
        todo!()
    }

    // Java: PointValuePair doOptimize()
    pub fn doOptimize(&self) -> Box<dyn Any> {
        // Java: checkParameters();
        // Java: // Indirect call to "computeObjectiveValue" in order to update the
        // Java: // evaluations counter.
        // Java: final MultivariateFunction evalFunc
        // Java: = new MultivariateFunction()
        // Java: /** {@inheritDoc} */
        // Java: public double value(double[] point)
        // Java: return computeObjectiveValue(point);
        // Java: final boolean isMinim = getGoalType() == GoalType.MINIMIZE;
        // Java: final Comparator<PointValuePair> comparator
        // Java: = new Comparator<PointValuePair>()
        // Java: /** {@inheritDoc} */
        // Java: public int compare(final PointValuePair o1,
        // Java: final PointValuePair o2)
        // Java: final double v1 = o1.getValue();
        // Java: final double v2 = o2.getValue();
        // Java: return isMinim ? Double.compare(v1, v2) : Double.compare(v2, v1);
        // Java: // Initialize search.
        // Java: simplex.build(getStartPoint());
        // Java: simplex.evaluate(evalFunc, comparator);
        // Java: PointValuePair[] previous = null;
        // Java: int iteration = 0;
        // Java: final ConvergenceChecker<PointValuePair> checker = getConvergenceChecker();
        // Java: while (true)
        // Java: if (getIterations() > 0)
        // Java: boolean converged = true;
        // Java: for (int i = 0; i < simplex.getSize(); i++)
        // Java: PointValuePair prev = previous[i];
        // Java: converged = converged &&
        // Java: checker.converged(iteration, prev, simplex.getPoint(i));
        // Java: if (converged)
        // Java: // We have found an optimum.
        // Java: return simplex.getPoint(0);
        // Java: // We still need to search.
        // Java: previous = simplex.getPoints();
        // Java: simplex.iterate(evalFunc, comparator);
        // Java: incrementIterationCount();
        todo!()
    }

    // Java: void parseOptimizationData(OptimizationData... optData)
    pub fn parseOptimizationData(&mut self, optData: Vec<Box<dyn Any>>) {
        // Java: // Allow base class to register its own data.
        // Java: super.parseOptimizationData(optData);
        // Java: // The existing values (as set by the previous call) are reused if
        // Java: // not provided in the argument list.
        // Java: for (OptimizationData data : optData)
        for data in &optData {
            // Java: if (data instanceof AbstractSimplex)
            // Java: simplex = (AbstractSimplex) data;
            // Java: // If more data must be parsed, this statement _must_ be
            // Java: // changed to "continue".
            // Java: break;
            break;
        }
    }

    // Java: void checkParameters()
    pub fn checkParameters(&self) {
        // Java: if (simplex == null)
        if self.simplex.is_none() {
            // Java: throw new NullArgumentException();
            panic!("NullArgumentException");
        }
        // Java: if (getLowerBound() != null ||
        // Java: getUpperBound() != null)
        // Java: throw new MathUnsupportedOperationException(LocalizedFormats.CONSTRAINT);
    }
}
