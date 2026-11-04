// Java: import org.apache.commons.math3.linear.LUDecomposition
use crate::org::apache::commons::math3::linear::LUDecomposition;
// Java: import org.apache.commons.math3.linear.RealMatrix
use crate::org::apache::commons::math3::linear::RealMatrix;
// Java: import org.apache.commons.math3.linear.Array2DRowRealMatrix
use crate::org::apache::commons::math3::linear::Array2DRowRealMatrix;
// Java: import org.apache.commons.math3.linear.RealVector
use crate::org::apache::commons::math3::linear::RealVector;

// Java: public class GLSMultipleLinearRegression extends AbstractMultipleLinearRegression
pub struct GLSMultipleLinearRegression {
    // Java field: RealMatrix Omega
    pub(crate) Omega: Option<Box<dyn std::any::Any>>,
    // Java field: RealMatrix OmegaInverse
    pub(crate) OmegaInverse: Option<Box<dyn std::any::Any>>,
}

// Java: public class GLSMultipleLinearRegression extends AbstractMultipleLinearRegression
impl GLSMultipleLinearRegression {
    // Java: void newSampleData(double[] y, double[][] x, double[][] covariance)
    pub fn newSampleData(&mut self, y: &[f64], x: &[Vec<f64>], covariance: &[Vec<f64>]) {
        // Java: validateSampleData(x, y);
        // Java: newYSampleData(y);
        // Java: newXSampleData(x);
        // Java: validateCovarianceData(x, covariance);
        // Java: newCovarianceData(covariance);
        todo!()
    }

    // Java: void newCovarianceData(double[][] omega)
    pub fn newCovarianceData(&mut self, omega: &[Vec<f64>]) {
        // Java: this.Omega = new Array2DRowRealMatrix(omega);
        // Java: this.OmegaInverse = null;
        todo!()
    }

    // Java: RealMatrix getOmegaInverse()
    pub fn getOmegaInverse(&self) -> Box<dyn std::any::Any> {
        // Java: if (OmegaInverse == null)
        // Java: OmegaInverse = new LUDecomposition(Omega).getSolver().getInverse();
        // Java: return OmegaInverse;
        todo!()
    }

    // Java: RealVector calculateBeta()
    pub fn calculateBeta(&self) -> RealVector::RealVector {
        // Java: RealMatrix OI = getOmegaInverse();
        // Java: RealMatrix XT = getX().transpose();
        // Java: RealMatrix XTOIX = XT.multiply(OI).multiply(getX());
        // Java: RealMatrix inverse = new LUDecomposition(XTOIX).getSolver().getInverse();
        // Java: return inverse.multiply(XT).multiply(OI).operate(getY());
        todo!()
    }

    // Java: RealMatrix calculateBetaVariance()
    pub fn calculateBetaVariance(&self) -> Box<dyn std::any::Any> {
        // Java: RealMatrix OI = getOmegaInverse();
        // Java: RealMatrix XTOIX = getX().transpose().multiply(OI).multiply(getX());
        // Java: return new LUDecomposition(XTOIX).getSolver().getInverse();
        todo!()
    }

    // Java: double calculateErrorVariance()
    pub fn calculateErrorVariance(&self) -> f64 {
        // Java: RealVector residuals = calculateResiduals();
        // Java: double t = residuals.dotProduct(getOmegaInverse().operate(residuals));
        // Java: return t / (getX().getRowDimension() - getX().getColumnDimension());
        todo!()
    }
}
