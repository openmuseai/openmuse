// Java: import java.util.Arrays
// Java: import org.apache.commons.math3.exception.ConvergenceException
use crate::org::apache::commons::math3::exception::ConvergenceException;
// Java: import org.apache.commons.math3.exception.util.LocalizedFormats
use crate::org::apache::commons::math3::exception::util::LocalizedFormats;
// Java: import org.apache.commons.math3.optimization.PointVectorValuePair
use crate::org::apache::commons::math3::optimization::PointVectorValuePair;
// Java: import org.apache.commons.math3.optimization.ConvergenceChecker
use crate::org::apache::commons::math3::optimization::ConvergenceChecker;
// Java: import org.apache.commons.math3.linear.RealMatrix
use crate::org::apache::commons::math3::linear::RealMatrix;
// Java: import org.apache.commons.math3.util.Precision
use crate::org::apache::commons::math3::util::Precision;
// Java: import org.apache.commons.math3.util.FastMath
use crate::org::apache::commons::math3::util::FastMath;

// Java: public class solves
pub struct solves {
}

// Java: public class LevenbergMarquardtOptimizer
pub struct LevenbergMarquardtOptimizer {
    // Java field: int solvedCols
    solvedCols: i32,
    // Java field: double[] diagR
    diagR: Option<Vec<f64>>,
    // Java field: double[] jacNorm
    jacNorm: Option<Vec<f64>>,
    // Java field: double[] beta
    beta: Option<Vec<f64>>,
    // Java field: int[] permutation
    permutation: Option<Vec<i32>>,
    // Java field: int rank
    rank: i32,
    // Java field: double lmPar
    lmPar: f64,
    // Java field: double[] lmDir
    lmDir: Option<Vec<f64>>,
    // Java field: double initialStepBoundFactor
    initialStepBoundFactor: f64,
    // Java field: double costRelativeTolerance
    costRelativeTolerance: f64,
    // Java field: double parRelativeTolerance
    parRelativeTolerance: f64,
    // Java field: double orthoTolerance
    orthoTolerance: f64,
    // Java field: double qrRankingThreshold
    qrRankingThreshold: f64,
    // Java field: double[] weightedResidual
    weightedResidual: Option<Vec<f64>>,
    // Java field: double[][] weightedJacobian
    weightedJacobian: Option<Vec<Vec<f64>>>,
}

// Java: public class assumes
pub struct assumes {
    // Java field: int nR
    nR: i32,
    // Java field: int nC
    nC: i32,
}

// Java: public class solves
impl solves {
}

// Java: public class LevenbergMarquardtOptimizer
impl LevenbergMarquardtOptimizer {
    // Java: LevenbergMarquardtOptimizer()
    pub fn LevenbergMarquardtOptimizer(&self) {
        // Java: this(100, 1e-10, 1e-10, 1e-10, Precision.SAFE_MIN);
    }

    // Java: LevenbergMarquardtOptimizer(ConvergenceChecker<PointVectorValuePair> checker)
    pub fn LevenbergMarquardtOptimizer_2(&self) {
        // Java: this(100, checker, 1e-10, 1e-10, 1e-10, Precision.SAFE_MIN);
    }

    // Java: LevenbergMarquardtOptimizer()
    pub fn LevenbergMarquardtOptimizer_3(&self) {
    }

}

// Java: public class assumes
impl assumes {
}
