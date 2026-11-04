// Java: import org.apache.commons.math3.exception.DimensionMismatchException
use crate::org::apache::commons::math3::exception::DimensionMismatchException;
// Java: import org.apache.commons.math3.exception.MathUnsupportedOperationException
use crate::org::apache::commons::math3::exception::MathUnsupportedOperationException;
// Java: import org.apache.commons.math3.exception.NumberIsTooSmallException
use crate::org::apache::commons::math3::exception::NumberIsTooSmallException;
// Java: import org.apache.commons.math3.linear.MatrixUtils
use crate::org::apache::commons::math3::linear::MatrixUtils;
// Java: import org.apache.commons.math3.linear.RealMatrix
use crate::org::apache::commons::math3::linear::RealMatrix;

// Java: public class is
pub struct is {
}

// Java: public class StorelessCovariance
pub struct StorelessCovariance {
    // Java field: StorelessBivariateCovariance[] covMatrix
    pub(crate) covMatrix: Box<dyn std::any::Any>,
    // Java field: int dimension
    pub(crate) dimension: i32,
}

// Java: public class is
impl is {
}

// Java: public class StorelessCovariance
impl StorelessCovariance {
    // Java: StorelessCovariance(final int dim)
    pub fn StorelessCovariance(&self) {
        // Java: this(dim, true);
    }

    // Java: StorelessCovariance(final int dim, final boolean biasCorrected)
    pub fn StorelessCovariance_2(&self) {
        // Java: dimension = dim;
        // Java: covMatrix = new StorelessBivariateCovariance[dimension * (dimension + 1) / 2];
        // Java: initializeMatrix(biasCorrected);
    }

    // Java: void initializeMatrix(final boolean biasCorrected)
    pub fn initializeMatrix(&self) {
        // Java: for(int i = 0; i < dimension; i++)
        {
            // Java: for(int j = 0; j < dimension; j++)
            {
                // Java: setElement(i, j, new StorelessBivariateCovariance(biasCorrected));
            }
        }
    }

    // Java: int indexOf(final int i, final int j)
    pub fn indexOf(&self) -> i32 {
        // Java: return j < i ? i * (i + 1) / 2 + j : j * (j + 1) / 2 + i;
        todo!()
    }

    // Java: StorelessBivariateCovariance getElement(final int i, final int j)
    pub fn getElement(&self) -> Box<dyn std::any::Any> {
        // Java: return covMatrix[indexOf(i, j)];
        todo!()
    }

    // Java: void setElement()
    pub fn setElement(&self) {
    }

    // Java: double getCovariance()
    pub fn getCovariance(&self) -> f64 {
        todo!()
    }

    // Java: void increment(final double[] data)
    pub fn increment(&self) {
    }

    // Java: void append(StorelessCovariance sc)
    pub fn append(&self) {
        // Java: if (sc.dimension != dimension)
        {
            // Java: throw new DimensionMismatchException(sc.dimension, dimension);
        }
        // Java: // only update the upper triangular part of the covariance matrix
        // Java: // as only these parts are actually stored
        // Java: for (int i = 0; i < dimension; i++)
        {
            // Java: for (int j = i; j < dimension; j++)
            {
                // Java: getElement(i, j).append(sc.getElement(i, j));
            }
        }
    }

    // Java: RealMatrix getCovarianceMatrix()
    pub fn getCovarianceMatrix(&self) -> Box<dyn std::any::Any> {
        // Java: return MatrixUtils.createRealMatrix(getData());
        todo!()
    }

    // Java: double[][] getData()
    pub fn getData(&self) -> Vec<f64> {
        // Java: final double[][] data = new double[dimension][dimension];
        // Java: for (int i = 0; i < dimension; i++)
        {
            // Java: for (int j = 0; j < dimension; j++)
            {
                // Java: data[i][j] = getElement(i, j).getResult();
            }
        }
        // Java: return data;
        todo!()
    }

    // Java: int getN()
    pub fn getN(&self) -> i32 {
        todo!()
    }

}
