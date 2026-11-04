// Java: import java.util.Arrays
// Java: import org.apache.commons.math3.fitting.leastsquares.LeastSquaresProblem.Evaluation
// Java: import org.apache.commons.math3.linear.ArrayRealVector
use crate::org::apache::commons::math3::linear::ArrayRealVector;
// Java: import org.apache.commons.math3.linear.RealMatrix
use crate::org::apache::commons::math3::linear::RealMatrix;
// Java: import org.apache.commons.math3.exception.ConvergenceException
use crate::org::apache::commons::math3::exception::ConvergenceException;
// Java: import org.apache.commons.math3.exception.util.LocalizedFormats
use crate::org::apache::commons::math3::exception::util::LocalizedFormats;
// Java: import org.apache.commons.math3.optim.ConvergenceChecker
use crate::org::apache::commons::math3::optim::ConvergenceChecker;
// Java: import org.apache.commons.math3.util.Incrementor
use crate::org::apache::commons::math3::util::Incrementor;
// Java: import org.apache.commons.math3.util.Precision
use crate::org::apache::commons::math3::util::Precision;
// Java: import org.apache.commons.math3.util.FastMath
use crate::org::apache::commons::math3::util::FastMath;

use std::sync::Arc;

// Java field: double TWO_EPS
pub(crate) const TWO_EPS: f64 = 2.0 * f64::EPSILON;

// Java: public class solves
pub struct solves {
}

// Java: public class LevenbergMarquardtOptimizer
pub struct LevenbergMarquardtOptimizer {
    // Java field: double initialStepBoundFactor
    pub(crate) initialStepBoundFactor: f64,
    // Java field: double costRelativeTolerance
    pub(crate) costRelativeTolerance: f64,
    // Java field: double parRelativeTolerance
    pub(crate) parRelativeTolerance: f64,
    // Java field: double orthoTolerance
    pub(crate) orthoTolerance: f64,
    // Java field: double qrRankingThreshold
    pub(crate) qrRankingThreshold: f64,
}

// Java: public class InternalData
pub struct InternalData {
    // Java field: double[][] weightedJacobian
    pub(crate) weightedJacobian: Vec<Vec<f64>>,
    // Java field: int[] permutation
    pub(crate) permutation: Vec<i32>,
    // Java field: int rank
    rank: i32,
    // Java field: double[] diagR
    pub(crate) diagR: Vec<f64>,
    // Java field: double[] jacNorm
    pub(crate) jacNorm: Vec<f64>,
    // Java field: double[] beta
    beta: Vec<f64>,
}

// Java: public class solves
impl solves {
}

// Java: public class LevenbergMarquardtOptimizer
impl LevenbergMarquardtOptimizer {
    // Java: LevenbergMarquardtOptimizer()
    pub fn new() -> Self {
        // Java: this(100, 1e-10, 1e-10, 1e-10, Precision.SAFE_MIN);
        Self::new_with_params(100.0, 1e-10, 1e-10, 1e-10, Precision::SAFE_MIN)
    }

    // Java: LevenbergMarquardtOptimizer()
    pub fn new_with_params(
        initial_step_bound_factor: f64,
        cost_relative_tolerance: f64,
        par_relative_tolerance: f64,
        ortho_tolerance: f64,
        qr_ranking_threshold: f64,
    ) -> Self {
        LevenbergMarquardtOptimizer {
            initialStepBoundFactor: initial_step_bound_factor,
            costRelativeTolerance: cost_relative_tolerance,
            parRelativeTolerance: par_relative_tolerance,
            orthoTolerance: ortho_tolerance,
            qrRankingThreshold: qr_ranking_threshold,
        }
    }

    // Java: LevenbergMarquardtOptimizer withInitialStepBoundFactor(double newInitialStepBoundFactor)
    pub fn withInitialStepBoundFactor(&self, newInitialStepBoundFactor: f64) -> LevenbergMarquardtOptimizer {
        // Java: return new LevenbergMarquardtOptimizer(
                // Java: newInitialStepBoundFactor,
                // Java: costRelativeTolerance,
                // Java: parRelativeTolerance,
                // Java: orthoTolerance,
                // Java: qrRankingThreshold);
        Self::new_with_params(
            newInitialStepBoundFactor,
            self.costRelativeTolerance,
            self.parRelativeTolerance,
            self.orthoTolerance,
            self.qrRankingThreshold,
        )
    }

    // Java: LevenbergMarquardtOptimizer withCostRelativeTolerance(double newCostRelativeTolerance)
    pub fn withCostRelativeTolerance(&self, newCostRelativeTolerance: f64) -> LevenbergMarquardtOptimizer {
        // Java: return new LevenbergMarquardtOptimizer(
                // Java: initialStepBoundFactor,
                // Java: newCostRelativeTolerance,
                // Java: parRelativeTolerance,
                // Java: orthoTolerance,
                // Java: qrRankingThreshold);
        Self::new_with_params(
            self.initialStepBoundFactor,
            newCostRelativeTolerance,
            self.parRelativeTolerance,
            self.orthoTolerance,
            self.qrRankingThreshold,
        )
    }

    // Java: LevenbergMarquardtOptimizer withParameterRelativeTolerance(double newParRelativeTolerance)
    pub fn withParameterRelativeTolerance(&self, newParRelativeTolerance: f64) -> LevenbergMarquardtOptimizer {
        // Java: return new LevenbergMarquardtOptimizer(
                // Java: initialStepBoundFactor,
                // Java: costRelativeTolerance,
                // Java: newParRelativeTolerance,
                // Java: orthoTolerance,
                // Java: qrRankingThreshold);
        Self::new_with_params(
            self.initialStepBoundFactor,
            self.costRelativeTolerance,
            newParRelativeTolerance,
            self.orthoTolerance,
            self.qrRankingThreshold,
        )
    }

    // Java: LevenbergMarquardtOptimizer withOrthoTolerance(double newOrthoTolerance)
    pub fn withOrthoTolerance(&self, newOrthoTolerance: f64) -> LevenbergMarquardtOptimizer {
        // Java: return new LevenbergMarquardtOptimizer(
                // Java: initialStepBoundFactor,
                // Java: costRelativeTolerance,
                // Java: parRelativeTolerance,
                // Java: newOrthoTolerance,
                // Java: qrRankingThreshold);
        Self::new_with_params(
            self.initialStepBoundFactor,
            self.costRelativeTolerance,
            self.parRelativeTolerance,
            newOrthoTolerance,
            self.qrRankingThreshold,
        )
    }

    // Java: LevenbergMarquardtOptimizer withRankingThreshold(double newQRRankingThreshold)
    pub fn withRankingThreshold(&self, newQRRankingThreshold: f64) -> LevenbergMarquardtOptimizer {
        // Java: return new LevenbergMarquardtOptimizer(
                // Java: initialStepBoundFactor,
                // Java: costRelativeTolerance,
                // Java: parRelativeTolerance,
                // Java: orthoTolerance,
                // Java: newQRRankingThreshold);
        Self::new_with_params(
            self.initialStepBoundFactor,
            self.costRelativeTolerance,
            self.parRelativeTolerance,
            self.orthoTolerance,
            newQRRankingThreshold,
        )
    }

    // Java: double getInitialStepBoundFactor()
    pub fn getInitialStepBoundFactor(&self) -> f64 {
        // Java: return initialStepBoundFactor;
        self.initialStepBoundFactor
    }

    // Java: double getCostRelativeTolerance()
    pub fn getCostRelativeTolerance(&self) -> f64 {
        // Java: return costRelativeTolerance;
        self.costRelativeTolerance
    }

    // Java: double getParameterRelativeTolerance()
    pub fn getParameterRelativeTolerance(&self) -> f64 {
        // Java: return parRelativeTolerance;
        self.parRelativeTolerance
    }

    // Java: double getOrthoTolerance()
    pub fn getOrthoTolerance(&self) -> f64 {
        // Java: return orthoTolerance;
        self.orthoTolerance
    }

    // Java: double getRankingThreshold()
    pub fn getRankingThreshold(&self) -> f64 {
        // Java: return qrRankingThreshold;
        self.qrRankingThreshold
    }

    // Java: Optimum optimize(final LeastSquaresProblem problem)
    pub fn optimize(&self, problem: &dyn std::any::Any) -> Box<dyn std::any::Any> {
        // Java: // Pull in relevant data from the problem as locals.
        // Java: final int nR = problem.getObservationSize(); // Number of observed data.
        // Java: final int nC = problem.getParameterSize(); // Number of parameters.
        // Java: // Counters.
        // Java: final Incrementor iterationCounter = problem.getIterationCounter();
        // Java: final Incrementor evaluationCounter = problem.getEvaluationCounter();
        // Java: // Convergence criterion.
        // Java: final ConvergenceChecker<Evaluation> checker = problem.getConvergenceChecker();
        // Java: // arrays shared with the other private methods
        // Java: final int solvedCols  = FastMath.min(nR, nC);
        // Java: /* Parameters evolution direction associated with lmPar. */
        // Java: double[] lmDir = new double[nC];
        // Java: /* Levenberg-Marquardt parameter. */
        // Java: double lmPar = 0;
        // Java: // local point
        // Java: double   delta   = 0;
        // Java: double   xNorm   = 0;
        // Java: double[] diag    = new double[nC];
        // Java: double[] oldX    = new double[nC];
        // Java: double[] oldRes  = new double[nR];
        // Java: double[] qtf     = new double[nR];
        // Java: double[] work1   = new double[nC];
        // Java: double[] work2   = new double[nC];
        // Java: double[] work3   = new double[nC];
        // Java: // Evaluate the function at the starting point and calculate its norm.
        // Java: evaluationCounter.incrementCount();
        // Java: //value will be reassigned in the loop
        // Java: Evaluation current = problem.evaluate(problem.getStart());
        // Java: double[] currentResiduals = current.getResiduals().toArray();
        // Java: double currentCost = current.getCost();
        // Java: double[] currentPoint = current.getPoint().toArray();
        // Java: // Outer loop.
        // Java: boolean firstIteration = true;
        // Java: while (true)
        {
            // Java: iterationCounter.incrementCount();
            // Java: final Evaluation previous = current;
            // Java: // QR decomposition of the jacobian matrix
            // Java: final InternalData internalData
                    // Java: = qrDecomposition(current.getJacobian(), solvedCols);
            // Java: final double[][] weightedJacobian = internalData.weightedJacobian;
            // Java: final int[] permutation = internalData.permutation;
            // Java: final double[] diagR = internalData.diagR;
            // Java: final double[] jacNorm = internalData.jacNorm;
            // Java: //residuals already have weights applied
            // Java: double[] weightedResidual = currentResiduals;
            // Java: for (int i = 0; i < nR; i++)
            {
                // Java: qtf[i] = weightedResidual[i];
            }
            // Java: // compute Qt.res
            // Java: qTy(qtf, internalData);
            // Java: // now we don't need Q anymore,
            // Java: // so let jacobian contain the R matrix with its diagonal elements
            // Java: for (int k = 0; k < solvedCols; ++k)
            {
                // Java: int pk = permutation[k];
                // Java: weightedJacobian[k][pk] = diagR[pk];
            }
            // Java: if (firstIteration)
            {
                // Java: // scale the point according to the norms of the columns
                // Java: // of the initial jacobian
                // Java: xNorm = 0;
                // Java: for (int k = 0; k < nC; ++k)
                {
                    // Java: double dk = jacNorm[k];
                    // Java: if (dk == 0)
                    {
                        // Java: dk = 1.0;
                    }
                    // Java: double xk = dk * currentPoint[k];
                    // Java: xNorm  += xk * xk;
                    // Java: diag[k] = dk;
                }
                // Java: xNorm = FastMath.sqrt(xNorm);
                // Java: // initialize the step bound delta
                // Java: delta = (xNorm == 0) ? initialStepBoundFactor : (initialStepBoundFactor * xNorm);
            }
            // Java: // check orthogonality between function vector and jacobian columns
            // Java: double maxCosine = 0;
            // Java: if (currentCost != 0)
            {
                // Java: for (int j = 0; j < solvedCols; ++j)
                {
                    // Java: int    pj = permutation[j];
                    // Java: double s  = jacNorm[pj];
                    // Java: if (s != 0)
                    {
                        // Java: double sum = 0;
                        // Java: for (int i = 0; i <= j; ++i)
                        {
                            // Java: sum += weightedJacobian[i][pj] * qtf[i];
                        }
                        // Java: maxCosine = FastMath.max(maxCosine, FastMath.abs(sum) / (s * currentCost));
                    }
                }
            }
            // Java: if (maxCosine <= orthoTolerance)
            {
                // Java: // Convergence has been reached.
                // Java: return new OptimumImpl(
                        // Java: current,
                        // Java: evaluationCounter.getCount(),
                        // Java: iterationCounter.getCount());
            }
            // Java: // rescale if necessary
            // Java: for (int j = 0; j < nC; ++j)
            {
                // Java: diag[j] = FastMath.max(diag[j], jacNorm[j]);
            }
            // Java: // Inner loop.
            // Java: for (double ratio = 0; ratio < 1.0e-4;)
            {
                // Java: // save the state
                // Java: for (int j = 0; j < solvedCols; ++j)
                {
                    // Java: int pj = permutation[j];
                    // Java: oldX[pj] = currentPoint[pj];
                }
                // Java: final double previousCost = currentCost;
                // Java: double[] tmpVec = weightedResidual;
                // Java: weightedResidual = oldRes;
                // Java: oldRes    = tmpVec;
                // Java: // determine the Levenberg-Marquardt parameter
                // Java: lmPar = determineLMParameter(qtf, delta, diag,
                                     // Java: internalData, solvedCols,
                                     // Java: work1, work2, work3, lmDir, lmPar);
                // Java: // compute the new point and the norm of the evolution direction
                // Java: double lmNorm = 0;
                // Java: for (int j = 0; j < solvedCols; ++j)
                {
                    // Java: int pj = permutation[j];
                    // Java: lmDir[pj] = -lmDir[pj];
                    // Java: currentPoint[pj] = oldX[pj] + lmDir[pj];
                    // Java: double s = diag[pj] * lmDir[pj];
                    // Java: lmNorm  += s * s;
                }
                // Java: lmNorm = FastMath.sqrt(lmNorm);
                // Java: // on the first iteration, adjust the initial step bound.
                // Java: if (firstIteration)
                {
                    // Java: delta = FastMath.min(delta, lmNorm);
                }
                // Java: // Evaluate the function at x + p and calculate its norm.
                // Java: evaluationCounter.incrementCount();
                // Java: current = problem.evaluate(new ArrayRealVector(currentPoint));
                // Java: currentResiduals = current.getResiduals().toArray();
                // Java: currentCost = current.getCost();
                // Java: currentPoint = current.getPoint().toArray();
                // Java: // compute the scaled actual reduction
                // Java: double actRed = -1.0;
                // Java: if (0.1 * currentCost < previousCost)
                {
                    // Java: double r = currentCost / previousCost;
                    // Java: actRed = 1.0 - r * r;
                }
                // Java: // compute the scaled predicted reduction
                // Java: // and the scaled directional derivative
                // Java: for (int j = 0; j < solvedCols; ++j)
                {
                    // Java: int pj = permutation[j];
                    // Java: double dirJ = lmDir[pj];
                    // Java: work1[j] = 0;
                    // Java: for (int i = 0; i <= j; ++i)
                    {
                        // Java: work1[i] += weightedJacobian[i][pj] * dirJ;
                    }
                }
                // Java: double coeff1 = 0;
                // Java: for (int j = 0; j < solvedCols; ++j)
                {
                    // Java: coeff1 += work1[j] * work1[j];
                }
                // Java: double pc2 = previousCost * previousCost;
                // Java: coeff1 /= pc2;
                // Java: double coeff2 = lmPar * lmNorm * lmNorm / pc2;
                // Java: double preRed = coeff1 + 2 * coeff2;
                // Java: double dirDer = -(coeff1 + coeff2);
                // Java: // ratio of the actual to the predicted reduction
                // Java: ratio = (preRed == 0) ? 0 : (actRed / preRed);
                // Java: // update the step bound
                // Java: if (ratio <= 0.25)
                {
                    // Java: double tmp =
                        // Java: (actRed < 0) ? (0.5 * dirDer / (dirDer + 0.5 * actRed)) : 0.5;
                        // Java: if ((0.1 * currentCost >= previousCost) || (tmp < 0.1))
                        {
                            // Java: tmp = 0.1;
                        }
                        // Java: delta = tmp * FastMath.min(delta, 10.0 * lmNorm);
                        // Java: lmPar /= tmp;
                }
                // Java: else if ((lmPar == 0) || (ratio >= 0.75))
                {
                    // Java: delta = 2 * lmNorm;
                    // Java: lmPar *= 0.5;
                }
                // Java: // test for successful iteration.
                // Java: if (ratio >= 1.0e-4)
                {
                    // Java: // successful iteration, update the norm
                    // Java: firstIteration = false;
                    // Java: xNorm = 0;
                    // Java: for (int k = 0; k < nC; ++k)
                    {
                        // Java: double xK = diag[k] * currentPoint[k];
                        // Java: xNorm += xK * xK;
                    }
                    // Java: xNorm = FastMath.sqrt(xNorm);
                    // Java: // tests for convergence.
                    // Java: if (checker != null && checker.converged(iterationCounter.getCount(), previous, current))
                    {
                        // Java: return new OptimumImpl(current, evaluationCounter.getCount(), iterationCounter.getCount());
                    }
                }
                // Java: else
                {
                    // Java: // failed iteration, reset the previous values
                    // Java: currentCost = previousCost;
                    // Java: for (int j = 0; j < solvedCols; ++j)
                    {
                        // Java: int pj = permutation[j];
                        // Java: currentPoint[pj] = oldX[pj];
                    }
                    // Java: tmpVec    = weightedResidual;
                    // Java: weightedResidual = oldRes;
                    // Java: oldRes    = tmpVec;
                    // Java: // Reset "current" to previous values.
                    // Java: current = previous;
                }
                // Java: // Default convergence criteria.
                // Java: if ((FastMath.abs(actRed) <= costRelativeTolerance &&
                     // Java: preRed <= costRelativeTolerance &&
                     // Java: ratio <= 2.0) ||
                    // Java: delta <= parRelativeTolerance * xNorm)
                    {
                    // Java: return new OptimumImpl(current, evaluationCounter.getCount(), iterationCounter.getCount());
                    }
                // Java: // tests for termination and stringent tolerances
                // Java: if (FastMath.abs(actRed) <= TWO_EPS &&
                    // Java: preRed <= TWO_EPS &&
                    // Java: ratio <= 2.0)
                    {
                    // Java: throw new ConvergenceException(LocalizedFormats.TOO_SMALL_COST_RELATIVE_TOLERANCE,
                                                   // Java: costRelativeTolerance);
                    }
                // Java: else if (delta <= TWO_EPS * xNorm)
                {
                    // Java: throw new ConvergenceException(LocalizedFormats.TOO_SMALL_PARAMETERS_RELATIVE_TOLERANCE,
                                                   // Java: parRelativeTolerance);
                }
                // Java: else if (maxCosine <= TWO_EPS)
                {
                    // Java: throw new ConvergenceException(LocalizedFormats.TOO_SMALL_ORTHOGONALITY_TOLERANCE,
                                                   // Java: orthoTolerance);
                }
            }
        }
        todo!("LevenbergMarquardtOptimizer::optimize")
    }

    // Java: double determineLMParameter()
    pub fn determineLMParameter(&self, _qy: &mut [f64], _delta: f64, _diag: &[f64],
                                _internal_data: &mut InternalData, _solved_cols: i32,
                                _work1: &mut [f64], _work2: &mut [f64], _work3: &mut [f64],
                                _lm_dir: &mut [f64], _lm_par: f64) -> f64 {
        todo!("LevenbergMarquardtOptimizer::determineLMParameter")
    }

    // Java: void determineLMDirection()
    pub fn determineLMDirection(&self, _qy: &[f64], _diag: &[f64], _lm_diag: &mut [f64],
                                _internal_data: &mut InternalData, _solved_cols: i32,
                                _work: &mut [f64], _lm_dir: &mut [f64]) {
        todo!("LevenbergMarquardtOptimizer::determineLMDirection")
    }

    // Java: InternalData qrDecomposition()
    pub fn qrDecomposition(&self, _jacobian: &dyn std::any::Any, _solved_cols: i32) -> InternalData {
        todo!("LevenbergMarquardtOptimizer::qrDecomposition")
    }

    // Java: void qTy()
    pub fn qTy(&self, _y: &mut [f64], _internal_data: &InternalData) {
        todo!("LevenbergMarquardtOptimizer::qTy")
    }

}

// Java: public class InternalData
impl InternalData {
}
