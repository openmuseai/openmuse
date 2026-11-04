// Java: import org.apache.commons.math3.util.FastMath
use crate::org::apache::commons::math3::util::FastMath;
// Java: import org.apache.commons.math3.util.Precision
use crate::org::apache::commons::math3::util::Precision;

// Java: public class is
pub struct is {
}

// Java: public class is
pub struct is_2 {
}

// Java: public class HessenbergTransformer
pub struct HessenbergTransformer {
    // Java field: RealMatrix cachedP
    pub(crate) cachedP: Box<dyn std::any::Any>,
    // Java field: RealMatrix cachedPt
    pub(crate) cachedPt: Box<dyn std::any::Any>,
    // Java field: RealMatrix cachedH
    pub(crate) cachedH: Box<dyn std::any::Any>,
}

// Java: public class is
impl is {
}

// Java: public class is
impl is_2 {
}

// Java: public class HessenbergTransformer
impl HessenbergTransformer {
    // Java: RealMatrix getP()
    pub fn getP(&self) -> Box<dyn std::any::Any> {
        // Java: if (cachedP == null)
        {
            // Java: final int n = householderVectors.length;
            // Java: final int high = n - 1;
            // Java: final double[][] pa = new double[n][n];
            // Java: for (int i = 0; i < n; i++)
            {
                // Java: for (int j = 0; j < n; j++)
                {
                    // Java: pa[i][j] = (i == j) ? 1 : 0;
                }
            }
            // Java: for (int m = high - 1; m >= 1; m--)
            {
                // Java: if (householderVectors[m][m - 1] != 0.0)
                {
                    // Java: for (int i = m + 1; i <= high; i++)
                    {
                        // Java: ort[i] = householderVectors[i][m - 1];
                    }
                    // Java: for (int j = m; j <= high; j++)
                    {
                        // Java: double g = 0.0;
                        // Java: for (int i = m; i <= high; i++)
                        {
                            // Java: g += ort[i] * pa[i][j];
                        }
                        // Java: // Double division avoids possible underflow
                        // Java: g = (g / ort[m]) / householderVectors[m][m - 1];
                        // Java: for (int i = m; i <= high; i++)
                        {
                            // Java: pa[i][j] += g * ort[i];
                        }
                    }
                }
            }
            // Java: cachedP = MatrixUtils.createRealMatrix(pa);
        }
        // Java: return cachedP;
        todo!()
    }

    // Java: RealMatrix getPT()
    pub fn getPT(&self) -> Box<dyn std::any::Any> {
        // Java: if (cachedPt == null)
        {
            // Java: cachedPt = getP().transpose();
        }
        // Java: // return the cached matrix
        // Java: return cachedPt;
        todo!()
    }

    // Java: RealMatrix getH()
    pub fn getH(&self) -> Box<dyn std::any::Any> {
        // Java: if (cachedH == null)
        {
            // Java: final int m = householderVectors.length;
            // Java: final double[][] h = new double[m][m];
            // Java: for (int i = 0; i < m; ++i)
            {
                // Java: if (i > 0)
                {
                    // Java: // copy the entry of the lower sub-diagonal
                    // Java: h[i][i - 1] = householderVectors[i][i - 1];
                }
                // Java: // copy upper triangular part of the matrix
                // Java: for (int j = i; j < m; ++j)
                {
                    // Java: h[i][j] = householderVectors[i][j];
                }
            }
            // Java: cachedH = MatrixUtils.createRealMatrix(h);
        }
        // Java: // return the cached matrix
        // Java: return cachedH;
        todo!()
    }

    // Java: double[][] getHouseholderVectorsRef()
    pub fn getHouseholderVectorsRef(&self) -> Vec<f64> {
        // Java: return householderVectors;
        todo!()
    }

    // Java: void transform()
    pub fn transform(&self) {
        // Java: final int n = householderVectors.length;
        // Java: final int high = n - 1;
        // Java: for (int m = 1; m <= high - 1; m++)
        {
            // Java: // Scale column.
            // Java: double scale = 0;
            // Java: for (int i = m; i <= high; i++)
            {
                // Java: scale += FastMath.abs(householderVectors[i][m - 1]);
            }
            // Java: if (!Precision.equals(scale, 0))
            {
                // Java: // Compute Householder transformation.
                // Java: double h = 0;
                // Java: for (int i = high; i >= m; i--)
                {
                    // Java: ort[i] = householderVectors[i][m - 1] / scale;
                    // Java: h += ort[i] * ort[i];
                }
                // Java: final double g = (ort[m] > 0) ? -FastMath.sqrt(h) : FastMath.sqrt(h);
                // Java: h -= ort[m] * g;
                // Java: ort[m] -= g;
                // Java: // Apply Householder similarity transformation
                // Java: // H = (I - u*u' / h) * H * (I - u*u' / h)
                // Java: for (int j = m; j < n; j++)
                {
                    // Java: double f = 0;
                    // Java: for (int i = high; i >= m; i--)
                    {
                        // Java: f += ort[i] * householderVectors[i][j];
                    }
                    // Java: f /= h;
                    // Java: for (int i = m; i <= high; i++)
                    {
                        // Java: householderVectors[i][j] -= f * ort[i];
                    }
                }
                // Java: for (int i = 0; i <= high; i++)
                {
                    // Java: double f = 0;
                    // Java: for (int j = high; j >= m; j--)
                    {
                        // Java: f += ort[j] * householderVectors[i][j];
                    }
                    // Java: f /= h;
                    // Java: for (int j = m; j <= high; j++)
                    {
                        // Java: householderVectors[i][j] -= f * ort[j];
                    }
                }
                // Java: ort[m] = scale * ort[m];
                // Java: householderVectors[m][m - 1] = scale * g;
            }
        }
    }

}
