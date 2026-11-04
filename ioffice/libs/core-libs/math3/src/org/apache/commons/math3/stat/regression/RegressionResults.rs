// Java: import java.io.Serializable
// Java: import java.util.Arrays
// Java: import org.apache.commons.math3.util.FastMath
use crate::org::apache::commons::math3::util::FastMath;
// Java: import org.apache.commons.math3.util.MathArrays
use crate::org::apache::commons::math3::util::MathArrays;
// Java: import org.apache.commons.math3.exception.OutOfRangeException
use crate::org::apache::commons::math3::exception::OutOfRangeException;

// Java field: int SSE_IDX
pub(crate) const SSE_IDX: usize = 0;
// Java field: int SST_IDX
pub(crate) const SST_IDX: usize = 1;
// Java field: int RSQ_IDX
pub(crate) const RSQ_IDX: usize = 2;
// Java field: int MSE_IDX
pub(crate) const MSE_IDX: usize = 3;
// Java field: int ADJRSQ_IDX
pub(crate) const ADJRSQ_IDX: usize = 4;
// Java field: long serialVersionUID
#[allow(non_upper_case_globals)]
pub(crate) const serialVersionUID: i64 = 1;

// Java: public class RegressionResults implements Serializable
pub struct RegressionResults {
    // Java field: double[] parameters
    pub(crate) parameters: Option<Vec<f64>>,
    // Java field: double[][] varCovData
    varCovData: Option<Vec<Vec<f64>>>,
    // Java field: boolean isSymmetricVCD
    isSymmetricVCD: bool,
    // Java field: int rank
    rank: i32,
    // Java field: long nobs
    pub(crate) nobs: i64,
    // Java field: boolean containsConstant
    containsConstant: bool,
    // Java field: double[] globalFitInfo
    pub(crate) globalFitInfo: Option<Vec<f64>>,
}

// Java: public class RegressionResults implements Serializable
#[allow(non_snake_case)]
impl RegressionResults {
    // Java: RegressionResults()
    pub fn new_default() -> Self {
        // Java: this.parameters = null;
        // Java: this.varCovData = null;
        // Java: this.rank = -1;
        // Java: this.nobs = -1;
        // Java: this.containsConstant = false;
        // Java: this.isSymmetricVCD = false;
        // Java: this.globalFitInfo = null;
        RegressionResults {
            parameters: None,
            varCovData: None,
            rank: -1,
            nobs: -1,
            containsConstant: false,
            isSymmetricVCD: false,
            globalFitInfo: None,
        }
    }

    // Java: RegressionResults()
    pub fn new(
        parameters: Option<Vec<f64>>,
        varcov: Option<Vec<Vec<f64>>>,
        isSymmetricCompressed: bool,
        nobs: i64,
        rank: i32,
        sumy: f64,
        sumysq: f64,
        sse: f64,
        containsConstant: bool,
        copyData: bool,
    ) -> Self {
        let params = if copyData {
            parameters.as_ref().map(|p| p.clone())
        } else {
            parameters
        };
        let var_cov = if copyData {
            varcov.as_ref().map(|v| v.clone())
        } else {
            varcov
        };

        let mut globalFitInfo = vec![f64::NAN; 5];

        if rank > 0 {
            globalFitInfo[SST_IDX] = if containsConstant {
                sumysq - sumy * sumy / nobs as f64
            } else {
                sumysq
            };
        }

        globalFitInfo[SSE_IDX] = sse;
        globalFitInfo[MSE_IDX] = globalFitInfo[SSE_IDX] / (nobs - rank as i64) as f64;
        globalFitInfo[RSQ_IDX] = 1.0 - globalFitInfo[SSE_IDX] / globalFitInfo[SST_IDX];

        if !containsConstant {
            globalFitInfo[ADJRSQ_IDX] = 1.0
                - (1.0 - globalFitInfo[RSQ_IDX])
                    * (nobs as f64 / (nobs - rank as i64) as f64);
        } else {
            globalFitInfo[ADJRSQ_IDX] =
                1.0 - (sse * (nobs as f64 - 1.0)) / (globalFitInfo[SST_IDX] * (nobs - rank as i64) as f64);
        }

        RegressionResults {
            parameters: params,
            varCovData: var_cov,
            isSymmetricVCD: isSymmetricCompressed,
            rank,
            nobs,
            containsConstant,
            globalFitInfo: Some(globalFitInfo),
        }
    }

    // Java: double getParameterEstimate(int index)
    pub fn getParameterEstimate(&self, index: i32) -> f64 {
        // Java: if (parameters == null)
        if self.parameters.is_none() {
            // Java: return Double.NaN;
            return f64::NAN;
        }
        let params = self.parameters.as_ref().unwrap();
        // Java: if (index < 0 || index >= this.parameters.length)
        if index < 0 || index >= params.len() as i32 {
            // Java: throw new OutOfRangeException(index, 0, this.parameters.length - 1);
            panic!("OutOfRangeException");
        }
        // Java: return this.parameters[index];
        params[index as usize]
    }

    // Java: double[] getParameterEstimates()
    pub fn getParameterEstimates(&self) -> Option<Vec<f64>> {
        // Java: if (this.parameters == null)
        if self.parameters.is_none() {
            // Java: return null;
            return None;
        }
        // Java: return MathArrays.copyOf(parameters);
        Some(self.parameters.as_ref().unwrap().clone())
    }

    // Java: double getStdErrorOfEstimate(int index)
    pub fn getStdErrorOfEstimate(&self, index: i32) -> f64 {
        // Java: if (parameters == null)
        if self.parameters.is_none() {
            // Java: return Double.NaN;
            return f64::NAN;
        }
        let params = self.parameters.as_ref().unwrap();
        // Java: if (index < 0 || index >= this.parameters.length)
        if index < 0 || index >= params.len() as i32 {
            // Java: throw new OutOfRangeException(index, 0, this.parameters.length - 1);
            panic!("OutOfRangeException");
        }
        // Java: double var = this.getVcvElement(index, index);
        let var = self.getVcvElement(index, index);
        // Java: if (!Double.isNaN(var) && var > Double.MIN_VALUE)
        if !var.is_nan() && var > f64::MIN_POSITIVE {
            // Java: return FastMath.sqrt(var);
            return var.sqrt();
        }
        // Java: return Double.NaN;
        f64::NAN
    }

    // Java: double[] getStdErrorOfEstimates()
    pub fn getStdErrorOfEstimates(&self) -> Option<Vec<f64>> {
        // Java: if (parameters == null)
        if self.parameters.is_none() {
            // Java: return null;
            return None;
        }
        let params = self.parameters.as_ref().unwrap();
        // Java: double[] se = new double[this.parameters.length];
        let mut se = vec![0.0f64; params.len()];
        // Java: for (int i = 0; i < this.parameters.length; i++)
        for i in 0..params.len() {
            // Java: double var = this.getVcvElement(i, i);
            let var = self.getVcvElement(i as i32, i as i32);
            // Java: if (!Double.isNaN(var) && var > Double.MIN_VALUE)
            if !var.is_nan() && var > f64::MIN_POSITIVE {
                // Java: se[i] = FastMath.sqrt(var);
                se[i] = var.sqrt();
                // Java: continue;
                continue;
            }
            // Java: se[i] = Double.NaN;
            se[i] = f64::NAN;
        }
        // Java: return se;
        Some(se)
    }

    // Java: double getCovarianceOfParameters(int i, int j)
    pub fn getCovarianceOfParameters(&self, i: i32, j: i32) -> f64 {
        // Java: if (parameters == null)
        if self.parameters.is_none() {
            // Java: return Double.NaN;
            return f64::NAN;
        }
        let params = self.parameters.as_ref().unwrap();
        // Java: if (i < 0 || i >= this.parameters.length)
        if i < 0 || i >= params.len() as i32 {
            // Java: throw new OutOfRangeException(i, 0, this.parameters.length - 1);
            panic!("OutOfRangeException");
        }
        // Java: if (j < 0 || j >= this.parameters.length)
        if j < 0 || j >= params.len() as i32 {
            // Java: throw new OutOfRangeException(j, 0, this.parameters.length - 1);
            panic!("OutOfRangeException");
        }
        // Java: return this.getVcvElement(i, j);
        self.getVcvElement(i, j)
    }

    // Java: int getNumberOfParameters()
    pub fn getNumberOfParameters(&self) -> i32 {
        // Java: if (this.parameters == null)
        if self.parameters.is_none() {
            // Java: return -1;
            return -1;
        }
        // Java: return this.parameters.length;
        self.parameters.as_ref().unwrap().len() as i32
    }

    // Java: long getN()
    pub fn getN(&self) -> i64 {
        // Java: return this.nobs;
        self.nobs
    }

    // Java: double getTotalSumSquares()
    pub fn getTotalSumSquares(&self) -> f64 {
        // Java: return this.globalFitInfo[SST_IDX];
        self.globalFitInfo.as_ref().map_or(f64::NAN, |g| g[SST_IDX])
    }

    // Java: double getRegressionSumSquares()
    pub fn getRegressionSumSquares(&self) -> f64 {
        // Java: return this.globalFitInfo[SST_IDX] - this.globalFitInfo[SSE_IDX];
        self.globalFitInfo.as_ref().map_or(f64::NAN, |g| g[SST_IDX] - g[SSE_IDX])
    }

    // Java: double getErrorSumSquares()
    pub fn getErrorSumSquares(&self) -> f64 {
        // Java: return this.globalFitInfo[ SSE_IDX];
        self.globalFitInfo.as_ref().map_or(f64::NAN, |g| g[SSE_IDX])
    }

    // Java: double getMeanSquareError()
    pub fn getMeanSquareError(&self) -> f64 {
        // Java: return this.globalFitInfo[ MSE_IDX];
        self.globalFitInfo.as_ref().map_or(f64::NAN, |g| g[MSE_IDX])
    }

    // Java: double getRSquared()
    pub fn getRSquared(&self) -> f64 {
        // Java: return this.globalFitInfo[ RSQ_IDX];
        self.globalFitInfo.as_ref().map_or(f64::NAN, |g| g[RSQ_IDX])
    }

    // Java: double getAdjustedRSquared()
    pub fn getAdjustedRSquared(&self) -> f64 {
        self.globalFitInfo.as_ref().map_or(f64::NAN, |g| g[ADJRSQ_IDX])
    }

    // Java: boolean hasIntercept()
    pub fn hasIntercept(&self) -> bool {
        self.containsConstant
    }

    // Java: double getVcvElement(int i, int j)
    fn getVcvElement(&self, i: i32, j: i32) -> f64 {
        let var_cov = match &self.varCovData {
            Some(v) => v,
            None => return f64::NAN,
        };
        let i = i as usize;
        let j = j as usize;
        if self.isSymmetricVCD {
            if var_cov.len() > 1 {
                if i == j {
                    var_cov[i][i]
                } else if i >= var_cov[j].len() {
                    var_cov[i][j]
                } else {
                    var_cov[j][i]
                }
            } else {
                if i > j {
                    var_cov[0][(i + 1) * i / 2 + j]
                } else {
                    var_cov[0][(j + 1) * j / 2 + i]
                }
            }
        } else {
            var_cov[i][j]
        }
    }
}
