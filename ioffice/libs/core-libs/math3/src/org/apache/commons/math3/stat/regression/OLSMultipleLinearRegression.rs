#![allow(unused_imports, dead_code, non_snake_case, unused_variables, unused_mut, unused_assignments)]

// Java: import org.apache.commons.math3.exception.MathIllegalArgumentException
use crate::org::apache::commons::math3::exception::MathIllegalArgumentException;
// Java: import org.apache.commons.math3.linear.Array2DRowRealMatrix
use crate::org::apache::commons::math3::linear::Array2DRowRealMatrix;
// Java: import org.apache.commons.math3.linear.LUDecomposition
use crate::org::apache::commons::math3::linear::LUDecomposition;
// Java: import org.apache.commons.math3.linear.QRDecomposition
use crate::org::apache::commons::math3::linear::QRDecomposition;
// Java: import org.apache.commons.math3.linear.RealMatrix
use crate::org::apache::commons::math3::linear::RealMatrix;
// Java: import org.apache.commons.math3.linear.RealVector
use crate::org::apache::commons::math3::linear::RealVector;
// Java: import org.apache.commons.math3.stat.StatUtils
use crate::org::apache::commons::math3::stat::StatUtils;
// Java: import org.apache.commons.math3.stat.descriptive.moment.SecondMoment
use crate::org::apache::commons::math3::stat::descriptive::moment::SecondMoment;

// Java: public class OLSMultipleLinearRegression extends AbstractMultipleLinearRegression
pub struct OLSMultipleLinearRegression {
    // Java field: QRDecomposition qr
    qr: Option<Box<dyn std::any::Any>>,
    // Java field: double threshold
    threshold: f64,
}

// Java: public class OLSMultipleLinearRegression extends AbstractMultipleLinearRegression
impl OLSMultipleLinearRegression {
    // Java: OLSMultipleLinearRegression()
    pub fn OLSMultipleLinearRegression() -> Self {
        // Java: this(0d);
        Self::OLSMultipleLinearRegression_threshold(0.0)
    }

    // Java: OLSMultipleLinearRegression(final double threshold)
    pub fn OLSMultipleLinearRegression_threshold(threshold: f64) -> Self {
        // Java: this.threshold = threshold;
        OLSMultipleLinearRegression {
            qr: None,
            threshold,
        }
    }

    // Java: void newSampleData(double[] y, double[][] x)
    pub fn newSampleData(&mut self, y: &[f64], x: &[Vec<f64>]) {
        // Java: validateSampleData(x, y);
        // Java: newYSampleData(y);
        // Java: newXSampleData(x);
        todo!()
    }

    // Java: void newSampleData(double[] data, int nobs, int nvars)
    pub fn newSampleData_flat(&mut self, data: &[f64], nobs: i32, nvars: i32) {
        // Java: super.newSampleData(data, nobs, nvars);
        // Java: qr = new QRDecomposition(getX(), threshold);
        todo!()
    }

    // Java: RealMatrix calculateHat()
    pub fn calculateHat(&self) -> Box<dyn std::any::Any> {
        // Java: // Create augmented identity matrix
        // Java: RealMatrix Q = qr.getQ();
        // Java: final int p = qr.getR().getColumnDimension();
        // Java: final int n = Q.getColumnDimension();
        // Java: // No try-catch or advertised NotStrictlyPositiveException - NPE above if n < 3
        // Java: Array2DRowRealMatrix augI = new Array2DRowRealMatrix(n, n);
        // Java: double[][] augIData = augI.getDataRef();
        // Java: for (int i = 0; i < n; i++)
        {
            // Java: for (int j =0; j < n; j++)
            {
                // Java: if (i == j && i < p)
                {
                    // Java: augIData[i][j] = 1d;
                }
                // Java: else
                {
                    // Java: augIData[i][j] = 0d;
                }
            }
        }
        // Java: // Compute and return Hat matrix
        // Java: // No DME advertised - args valid if we get here
        // Java: return Q.multiply(augI).multiply(Q.transpose());
        todo!()
    }

    // Java: double calculateTotalSumOfSquares()
    pub fn calculateTotalSumOfSquares(&self) -> f64 {
        // Java: if (isNoIntercept())
        {
            // Java: return StatUtils.sumSq(getY().toArray());
        }
        // Java: else
        {
            // Java: return new SecondMoment().evaluate(getY().toArray());
        }
        todo!()
    }

    // Java: double calculateResidualSumOfSquares()
    pub fn calculateResidualSumOfSquares(&self) -> f64 {
        // Java: final RealVector residuals = calculateResiduals();
        // Java: // No advertised DME, args are valid
        // Java: return residuals.dotProduct(residuals);
        todo!()
    }

    // Java: double calculateRSquared()
    pub fn calculateRSquared(&self) -> f64 {
        // Java: return 1 - calculateResidualSumOfSquares() / calculateTotalSumOfSquares();
        todo!()
    }

    // Java: double calculateAdjustedRSquared()
    pub fn calculateAdjustedRSquared(&self) -> f64 {
        // Java: final double n = getX().getRowDimension();
        // Java: if (isNoIntercept())
        {
            // Java: return 1 - (1 - calculateRSquared()) * (n / (n - getX().getColumnDimension()));
        }
        // Java: else
        {
            // Java: return 1 - (calculateResidualSumOfSquares() * (n - 1)) /
                // Java: (calculateTotalSumOfSquares() * (n - getX().getColumnDimension()));
        }
        todo!()
    }

    // Java: void newXSampleData(double[][] x)
    pub fn newXSampleData(&mut self, x: &[Vec<f64>]) {
        // Java: super.newXSampleData(x);
        // Java: qr = new QRDecomposition(getX(), threshold);
        todo!()
    }

    // Java: RealVector calculateBeta()
    pub fn calculateBeta(&self) -> Box<dyn std::any::Any> {
        // Java: return qr.getSolver().solve(getY());
        todo!()
    }

    // Java: RealMatrix calculateBetaVariance()
    pub fn calculateBetaVariance(&self) -> Box<dyn std::any::Any> {
        // Java: int p = getX().getColumnDimension();
        // Java: RealMatrix Raug = qr.getR().getSubMatrix(0, p - 1 , 0, p - 1);
        // Java: RealMatrix Rinv = new LUDecomposition(Raug).getSolver().getInverse();
        // Java: return Rinv.multiply(Rinv.transpose());
        todo!()
    }

}
