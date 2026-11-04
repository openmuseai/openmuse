// Java: import org.apache.commons.math3.util.Incrementor
use crate::org::apache::commons::math3::util::Incrementor;
// Java: import org.apache.commons.math3.exception.TooManyEvaluationsException
use crate::org::apache::commons::math3::exception::TooManyEvaluationsException;
// Java: import org.apache.commons.math3.exception.TooManyIterationsException
use crate::org::apache::commons::math3::exception::TooManyIterationsException;

// Java: public class for
pub struct r#for {
}

// Java: public class BaseOptimizer
pub struct BaseOptimizer {
    // Java field: Incrementor evaluations
    pub(crate) evaluations: Box<dyn std::any::Any>,
    // Java field: Incrementor iterations
    pub(crate) iterations: Box<dyn std::any::Any>,
    // Java field: ConvergenceChecker<PAIR> checker
    pub(crate) checker: Box<dyn std::any::Any>,
}

// Java: public class MaxEvalCallback
pub struct MaxEvalCallback {
}

// Java: public class MaxIterCallback
pub struct MaxIterCallback {
}

// Java: public class for
impl r#for {
}

// Java: public class BaseOptimizer
impl BaseOptimizer {
    // Java: BaseOptimizer(ConvergenceChecker<PAIR> checker)
    pub fn BaseOptimizer(&self) {
        // Java: this(checker, 0, Integer.MAX_VALUE);
    }

    // Java: BaseOptimizer()
    pub fn BaseOptimizer_2(&self) {
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

    // Java: int getMaxIterations()
    pub fn getMaxIterations(&self) -> i32 {
        // Java: return iterations.getMaximalCount();
        todo!()
    }

    // Java: int getIterations()
    pub fn getIterations(&self) -> i32 {
        // Java: return iterations.getCount();
        todo!()
    }

    // Java: ConvergenceChecker<PAIR> getConvergenceChecker()
    pub fn getConvergenceChecker(&self) -> Box<dyn std::any::Any> {
        // Java: return checker;
        todo!()
    }

    // Java: PAIR optimize(OptimizationData... optData)
    pub fn optimize(&self) -> Box<dyn std::any::Any> {
        todo!()
    }

    // Java: PAIR optimize()
    pub fn optimize_2(&self) -> Box<dyn std::any::Any> {
        todo!()
    }

    // Java: abstract PAIR doOptimize()
    pub fn doOptimize(&self) -> Box<dyn std::any::Any> {
        todo!()
    }

    // Java: void incrementEvaluationCount()
    pub fn incrementEvaluationCount(&self) {
    }

    // Java: void incrementIterationCount()
    pub fn incrementIterationCount(&self) {
    }

    // Java: void parseOptimizationData(OptimizationData... optData)
    pub fn parseOptimizationData(&self) {
        // Java: // The existing values (as set by the previous call) are reused if
        // Java: // not provided in the argument list.
        // Java: for (OptimizationData data : optData)
        {
            // Java: if (data instanceof MaxEval)
            {
                // Java: evaluations.setMaximalCount(((MaxEval) data).getMaxEval());
                // Java: continue;
            }
            // Java: if (data instanceof MaxIter)
            {
                // Java: iterations.setMaximalCount(((MaxIter) data).getMaxIter());
                // Java: continue;
            }
        }
    }

}

// Java: public class MaxEvalCallback
impl MaxEvalCallback {
}

// Java: public class MaxIterCallback
impl MaxIterCallback {
}
