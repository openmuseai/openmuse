// Java: import org.apache.commons.math3.exception.DimensionMismatchException
use crate::org::apache::commons::math3::exception::DimensionMismatchException;
// Java: import org.apache.commons.math3.exception.InsufficientDataException
use crate::org::apache::commons::math3::exception::InsufficientDataException;
// Java: import org.apache.commons.math3.exception.MathIllegalArgumentException
use crate::org::apache::commons::math3::exception::MathIllegalArgumentException;
// Java: import org.apache.commons.math3.exception.NoDataException
use crate::org::apache::commons::math3::exception::NoDataException;
// Java: import org.apache.commons.math3.exception.NullArgumentException
use crate::org::apache::commons::math3::exception::NullArgumentException;
// Java: import org.apache.commons.math3.exception.util.LocalizedFormats
use crate::org::apache::commons::math3::exception::util::LocalizedFormats;
// Java: import org.apache.commons.math3.linear.NonSquareMatrixException
use crate::org::apache::commons::math3::linear::NonSquareMatrixException;
// Java: import org.apache.commons.math3.linear.RealMatrix
use crate::org::apache::commons::math3::linear::RealMatrix;
// Java: import org.apache.commons.math3.linear.Array2DRowRealMatrix
use crate::org::apache::commons::math3::linear::Array2DRowRealMatrix;
// Java: import org.apache.commons.math3.linear.RealVector
use crate::org::apache::commons::math3::linear::RealVector;
// Java: import org.apache.commons.math3.linear.ArrayRealVector
use crate::org::apache::commons::math3::linear::ArrayRealVector;
// Java: import org.apache.commons.math3.stat.descriptive.moment.Variance
use crate::org::apache::commons::math3::stat::descriptive::moment::Variance;
// Java: import org.apache.commons.math3.util.FastMath
use crate::org::apache::commons::math3::util::FastMath;

// Java: public class for
pub struct r#for {
}

// Java: public class AbstractMultipleLinearRegression
pub struct AbstractMultipleLinearRegression {
    // Java field: RealMatrix xMatrix
    pub(crate) xMatrix: Box<dyn std::any::Any>,
    // Java field: RealVector yVector
    pub(crate) yVector: RealVector::RealVector,
    // Java field: boolean noIntercept
    pub(crate) noIntercept: bool,
}

// Java: public class for
impl r#for {
}

// Java: public class AbstractMultipleLinearRegression
impl AbstractMultipleLinearRegression {
    // Java: protected RealMatrix getX()
    pub fn getX(&self) -> &dyn std::any::Any {
        &*self.xMatrix
    }

    // Java: protected RealVector getY()
    pub fn getY(&self) -> &RealVector::RealVector {
        &self.yVector
    }

    // Java: public boolean isNoIntercept()
    pub fn isNoIntercept(&self) -> bool {
        self.noIntercept
    }

    // Java: public void setNoIntercept(boolean noIntercept)
    pub fn setNoIntercept(&mut self, noIntercept: bool) {
        self.noIntercept = noIntercept;
    }

    // Java: public void newSampleData(double[] data, int nobs, int nvars)
    pub fn newSampleData(&mut self, data: &[f64], nobs: i32, nvars: i32) {
        if data.is_empty() {
            panic!("NullArgumentException");
        }
        if data.len() as i32 != nobs * (nvars + 1) {
            panic!("DimensionMismatchException");
        }
        if nobs <= nvars {
            panic!("InsufficientDataException");
        }
        let mut y = vec![0.0f64; nobs as usize];
        let cols = if self.noIntercept { nvars } else { nvars + 1 };
        let mut x = vec![vec![0.0f64; cols as usize]; nobs as usize];
        let mut pointer: usize = 0;
        for i in 0..nobs as usize {
            y[i] = data[pointer];
            pointer += 1;
            if !self.noIntercept {
                x[i][0] = 1.0;
            }
            let j_start = if self.noIntercept { 0 } else { 1 };
            for j in j_start..cols as usize {
                x[i][j] = data[pointer];
                pointer += 1;
            }
        }
        // Java: this.xMatrix = new Array2DRowRealMatrix(x);
        // Java: this.yVector = new ArrayRealVector(y);
        todo!()
    }

    // Java: protected void newYSampleData(double[] y)
    pub fn newYSampleData(&mut self, y: &[f64]) {
        if y.is_empty() {
            panic!("NoDataException");
        }
        // Java: this.yVector = new ArrayRealVector(y);
        todo!()
    }

    // Java: protected void newXSampleData(double[][] x)
    pub fn newXSampleData(&mut self, x: &[Vec<f64>]) {
        if x.is_empty() {
            panic!("NoDataException");
        }
        if self.noIntercept {
            // Java: this.xMatrix = new Array2DRowRealMatrix(x, true);
            todo!()
        } else {
            let n_vars = x[0].len();
            let mut x_aug = vec![vec![0.0f64; n_vars + 1]; x.len()];
            for i in 0..x.len() {
                if x[i].len() != n_vars {
                    panic!("DimensionMismatchException");
                }
                x_aug[i][0] = 1.0;
                x_aug[i][1..=n_vars].copy_from_slice(&x[i]);
            }
            // Java: this.xMatrix = new Array2DRowRealMatrix(xAug, false);
            todo!()
        }
    }

    // Java: protected void validateSampleData(double[][] x, double[] y)
    pub fn validateSampleData(&self, x: &[Vec<f64>], y: &[f64]) {
        if x.is_empty() || y.is_empty() {
            panic!("NullArgumentException");
        }
        if x.len() != y.len() {
            panic!("DimensionMismatchException");
        }
        if x.is_empty() {
            panic!("NoDataException");
        }
        if x[0].len() + 1 > x.len() {
            panic!("MathIllegalArgumentException");
        }
    }

    // Java: protected void validateCovarianceData(double[][] x, double[][] covariance)
    pub fn validateCovarianceData(&self, x: &[Vec<f64>], covariance: &[Vec<f64>]) {
        if x.len() != covariance.len() {
            panic!("DimensionMismatchException");
        }
        if !covariance.is_empty() && covariance.len() != covariance[0].len() {
            panic!("NonSquareMatrixException");
        }
    }

    // Java: public double[] estimateRegressionParameters()
    pub fn estimateRegressionParameters(&self) -> Vec<f64> {
        // Java: RealVector b = calculateBeta();
        // Java: return b.toArray();
        let b = self.calculateBeta();
        b.toArray()
    }

    // Java: public double[] estimateResiduals()
    pub fn estimateResiduals(&self) -> Vec<f64> {
        // Java: RealVector b = calculateBeta();
        // Java: RealVector e = yVector.subtract(xMatrix.operate(b));
        // Java: return e.toArray();
        todo!()
    }

    // Java: public double[][] estimateRegressionParametersVariance()
    pub fn estimateRegressionParametersVariance(&self) -> Vec<Vec<f64>> {
        // Java: return calculateBetaVariance().getData();
        todo!()
    }

    // Java: public double[] estimateRegressionParametersStandardErrors()
    pub fn estimateRegressionParametersStandardErrors(&self) -> Vec<f64> {
        // Java: double[][] betaVariance = estimateRegressionParametersVariance();
        // Java: double sigma = calculateErrorVariance();
        let beta_variance = self.estimateRegressionParametersVariance();
        let sigma = self.calculateErrorVariance();
        let length = beta_variance[0].len();
        let mut result = vec![0.0f64; length];
        for i in 0..length {
            result[i] = (sigma * beta_variance[i][i]).sqrt();
        }
        result
    }

    // Java: public double estimateRegressandVariance()
    pub fn estimateRegressandVariance(&self) -> f64 {
        // Java: return calculateYVariance();
        self.calculateYVariance()
    }

    // Java: public double estimateErrorVariance()
    pub fn estimateErrorVariance(&self) -> f64 {
        // Java: return calculateErrorVariance();
        self.calculateErrorVariance()
    }

    // Java: public double estimateRegressionStandardError()
    pub fn estimateRegressionStandardError(&self) -> f64 {
        // Java: return FastMath.sqrt(estimateErrorVariance());
        self.estimateErrorVariance().sqrt()
    }

    // Java: protected abstract RealVector calculateBeta()
    pub fn calculateBeta(&self) -> RealVector::RealVector {
        todo!()
    }

    // Java: protected abstract RealMatrix calculateBetaVariance()
    pub fn calculateBetaVariance(&self) -> RealVector::RealVector {
        todo!()
    }

    // Java: protected double calculateYVariance()
    pub fn calculateYVariance(&self) -> f64 {
        // Java: return new Variance().evaluate(yVector.toArray());
        todo!()
    }

    // Java: protected double calculateErrorVariance()
    pub fn calculateErrorVariance(&self) -> f64 {
        // Java: RealVector residuals = calculateResiduals();
        // Java: return residuals.dotProduct(residuals) / (xMatrix.getRowDimension() - xMatrix.getColumnDimension());
        todo!()
    }

    // Java: protected RealVector calculateResiduals()
    pub fn calculateResiduals(&self) -> RealVector::RealVector {
        // Java: RealVector b = calculateBeta();
        // Java: return yVector.subtract(xMatrix.operate(b));
        todo!()
    }
}
