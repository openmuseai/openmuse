// Java: import org.apache.commons.math3.exception.MathIllegalArgumentException
use crate::org::apache::commons::math3::exception::MathIllegalArgumentException;
// Java: import org.apache.commons.math3.exception.NotStrictlyPositiveException
use crate::org::apache::commons::math3::exception::NotStrictlyPositiveException;
// Java: import org.apache.commons.math3.exception.util.LocalizedFormats
use crate::org::apache::commons::math3::exception::util::LocalizedFormats;
// Java: import org.apache.commons.math3.linear.RealMatrix
use crate::org::apache::commons::math3::linear::RealMatrix;
// Java: import org.apache.commons.math3.linear.BlockRealMatrix
use crate::org::apache::commons::math3::linear::BlockRealMatrix;
// Java: import org.apache.commons.math3.stat.descriptive.moment.Mean
use crate::org::apache::commons::math3::stat::descriptive::moment::Mean;
// Java: import org.apache.commons.math3.stat.descriptive.moment.Variance
use crate::org::apache::commons::math3::stat::descriptive::moment::Variance;

// Java: public class Covariance
pub struct Covariance {
    // Java field: RealMatrix covarianceMatrix
    pub(crate) covarianceMatrix: Box<dyn std::any::Any>,
    // Java field: int n
    pub(crate) n: i32,
}

// Java: public class Covariance
impl Covariance {
    // Java: Covariance()
    pub fn Covariance(&self) {
        // Java: super();
        // Java: covarianceMatrix = null;
        // Java: n = 0;
    }

    // Java: Covariance(double[][] data, boolean biasCorrected)
    pub fn Covariance_2(&self) {
    }

    // Java: Covariance(double[][] data)
    pub fn Covariance_3(&self) {
    }

    // Java: Covariance(RealMatrix matrix, boolean biasCorrected)
    pub fn Covariance_4(&self) {
    }

    // Java: Covariance(RealMatrix matrix)
    pub fn Covariance_5(&self) {
        // Java: this(matrix, true);
    }

    // Java: RealMatrix getCovarianceMatrix()
    pub fn getCovarianceMatrix(&self) -> Box<dyn std::any::Any> {
        // Java: return covarianceMatrix;
        todo!()
    }

    // Java: int getN()
    pub fn getN(&self) -> i32 {
        // Java: return n;
        todo!()
    }

    // Java: RealMatrix computeCovarianceMatrix(RealMatrix matrix, boolean biasCorrected)
    pub fn computeCovarianceMatrix(&self) -> Box<dyn std::any::Any> {
        todo!()
    }

    // Java: RealMatrix computeCovarianceMatrix(RealMatrix matrix)
    pub fn computeCovarianceMatrix_2(&self) -> Box<dyn std::any::Any> {
        todo!()
    }

    // Java: RealMatrix computeCovarianceMatrix(double[][] data, boolean biasCorrected)
    pub fn computeCovarianceMatrix_3(&self) -> Box<dyn std::any::Any> {
        todo!()
    }

    // Java: RealMatrix computeCovarianceMatrix(double[][] data)
    pub fn computeCovarianceMatrix_4(&self) -> Box<dyn std::any::Any> {
        todo!()
    }

    // Java: double covariance(final double[] xArray, final double[] yArray, boolean biasCorrected)
    pub fn covariance(&self) -> f64 {
        todo!()
    }

    // Java: double covariance(final double[] xArray, final double[] yArray)
    pub fn covariance_2(&self) -> f64 {
        todo!()
    }

    // Java: void checkSufficientData(final RealMatrix matrix)
    pub fn checkSufficientData(&self) {
        // Java: int nRows = matrix.getRowDimension();
        // Java: int nCols = matrix.getColumnDimension();
        // Java: if (nRows < 2 || nCols < 1)
        {
            // Java: throw new MathIllegalArgumentException(
                    // Java: LocalizedFormats.INSUFFICIENT_ROWS_AND_COLUMNS,
                    // Java: nRows, nCols);
        }
    }

}
