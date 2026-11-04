// Java: import java.io.Serializable
// Java: import java.util.Arrays
// Java: import org.apache.commons.math3.exception.DimensionMismatchException
use crate::org::apache::commons::math3::exception::DimensionMismatchException;
// Java: import org.apache.commons.math3.linear.MatrixUtils
use crate::org::apache::commons::math3::linear::MatrixUtils;
// Java: import org.apache.commons.math3.linear.RealMatrix
use crate::org::apache::commons::math3::linear::RealMatrix;

// Java field: long serialVersionUID

// Java: public class VectorialCovariance implements Serializable
pub struct VectorialCovariance {
    // Java field: double[] sums
    sums: Vec<f64>,
    // Java field: double[] productsSums
    productsSums: Vec<f64>,
    // Java field: boolean isBiasCorrected
    isBiasCorrected: bool,
    // Java field: long n
    pub(crate) n: i64,
}

// Java: public class VectorialCovariance implements Serializable
impl VectorialCovariance {
    // Java: VectorialCovariance(int dimension, boolean isBiasCorrected)
    pub fn VectorialCovariance(&self) {
        // Java: sums         = new double[dimension];
        // Java: productsSums = new double[dimension * (dimension + 1) / 2];
        // Java: n            = 0;
        // Java: this.isBiasCorrected = isBiasCorrected;
    }

    // Java: void increment(double[] v)
    pub fn increment(&self) {
        // Java: if (v.length != sums.length)
        {
            // Java: throw new DimensionMismatchException(v.length, sums.length);
        }
        // Java: int k = 0;
        // Java: for (int i = 0; i < v.length; ++i)
        {
            // Java: sums[i] += v[i];
            // Java: for (int j = 0; j <= i; ++j)
            {
                // Java: productsSums[k++] += v[i] * v[j];
            }
        }
        // Java: n++;
    }

    // Java: RealMatrix getResult()
    pub fn getResult(&self) -> Box<dyn std::any::Any> {
        // Java: int dimension = sums.length;
        // Java: RealMatrix result = MatrixUtils.createRealMatrix(dimension, dimension);
        // Java: if (n > 1)
        {
            // Java: double c = 1.0 / (n * (isBiasCorrected ? (n - 1) : n));
            // Java: int k = 0;
            // Java: for (int i = 0; i < dimension; ++i)
            {
                // Java: for (int j = 0; j <= i; ++j)
                {
                    // Java: double e = c * (n * productsSums[k++] - sums[i] * sums[j]);
                    // Java: result.setEntry(i, j, e);
                    // Java: result.setEntry(j, i, e);
                }
            }
        }
        // Java: return result;
        todo!()
    }

    // Java: long getN()
    pub fn getN(&self) -> i64 {
        // Java: return n;
        todo!()
    }

    // Java: void clear()
    pub fn clear(&self) {
        // Java: n = 0;
        // Java: Arrays.fill(sums, 0.0);
        // Java: Arrays.fill(productsSums, 0.0);
    }

    // Java: int hashCode()
    pub fn hashCode(&self) -> i32 {
        // Java: final int prime = 31;
        // Java: int result = 1;
        // Java: result = prime * result + (isBiasCorrected ? 1231 : 1237);
        // Java: result = prime * result + (int) (n ^ (n >>> 32));
        // Java: result = prime * result + Arrays.hashCode(productsSums);
        // Java: result = prime * result + Arrays.hashCode(sums);
        // Java: return result;
        todo!()
    }

    // Java: boolean equals(Object obj)
    pub fn equals(&self) -> bool {
        // Java: if (this == obj)
        {
            // Java: return true;
        }
        // Java: if (!(obj instanceof VectorialCovariance))
        {
            // Java: return false;
        }
        // Java: VectorialCovariance other = (VectorialCovariance) obj;
        // Java: if (isBiasCorrected != other.isBiasCorrected)
        {
            // Java: return false;
        }
        // Java: if (n != other.n)
        {
            // Java: return false;
        }
        // Java: if (!Arrays.equals(productsSums, other.productsSums))
        {
            // Java: return false;
        }
        // Java: if (!Arrays.equals(sums, other.sums))
        {
            // Java: return false;
        }
        // Java: return true;
        todo!()
    }

}

