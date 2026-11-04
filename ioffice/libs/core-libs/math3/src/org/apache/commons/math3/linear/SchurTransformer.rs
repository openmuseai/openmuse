// Java: import org.apache.commons.math3.exception.MaxCountExceededException
use crate::org::apache::commons::math3::exception::MaxCountExceededException;
// Java: import org.apache.commons.math3.exception.util.LocalizedFormats
use crate::org::apache::commons::math3::exception::util::LocalizedFormats;
// Java: import org.apache.commons.math3.util.FastMath
use crate::org::apache::commons::math3::util::FastMath;
// Java: import org.apache.commons.math3.util.Precision
use crate::org::apache::commons::math3::util::Precision;

// Java field: int MAX_ITERATIONS
pub(crate) const MAX_ITERATIONS: i32 = 0;

// Java: public class is
pub struct is {
}

// Java: public class is
pub struct is_2 {
}

// Java: public class SchurTransformer
pub struct SchurTransformer {
    // Java field: RealMatrix cachedP
    pub(crate) cachedP: Box<dyn std::any::Any>,
    // Java field: RealMatrix cachedT
    pub(crate) cachedT: Box<dyn std::any::Any>,
    // Java field: RealMatrix cachedPt
    pub(crate) cachedPt: Box<dyn std::any::Any>,
    // Java field: double epsilon
    pub(crate) epsilon: f64,
}

// Java: public class ShiftInfo
pub struct ShiftInfo {
    // Java field: double x
    pub(crate) x: f64,
    // Java field: double y
    pub(crate) y: f64,
    // Java field: double w
    pub(crate) w: f64,
    // Java field: double exShift
    pub(crate) exShift: f64,
}

// Java: public class is
impl is {
}

// Java: public class is
impl is_2 {
}

// Java: public class SchurTransformer
impl SchurTransformer {
    // Java: RealMatrix getP()
    pub fn getP(&self) -> Box<dyn std::any::Any> {
        // Java: if (cachedP == null)
        {
            // Java: cachedP = MatrixUtils.createRealMatrix(matrixP);
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

    // Java: RealMatrix getT()
    pub fn getT(&self) -> Box<dyn std::any::Any> {
        // Java: if (cachedT == null)
        {
            // Java: cachedT = MatrixUtils.createRealMatrix(matrixT);
        }
        // Java: // return the cached matrix
        // Java: return cachedT;
        todo!()
    }

    // Java: void transform()
    pub fn transform(&self) {
        // Java: final int n = matrixT.length;
        // Java: // compute matrix norm
        // Java: final double norm = getNorm();
        // Java: // shift information
        // Java: final ShiftInfo shift = new ShiftInfo();
        // Java: // Outer loop over eigenvalue index
        // Java: int iteration = 0;
        // Java: int iu = n - 1;
        // Java: while (iu >= 0)
        {
            // Java: // Look for single small sub-diagonal element
            // Java: final int il = findSmallSubDiagonalElement(iu, norm);
            // Java: // Check for convergence
            // Java: if (il == iu)
            {
                // Java: // One root found
                // Java: matrixT[iu][iu] += shift.exShift;
                // Java: iu--;
                // Java: iteration = 0;
            }
            // Java: else if (il == iu - 1)
            {
                // Java: // Two roots found
                // Java: double p = (matrixT[iu - 1][iu - 1] - matrixT[iu][iu]) / 2.0;
                // Java: double q = p * p + matrixT[iu][iu - 1] * matrixT[iu - 1][iu];
                // Java: matrixT[iu][iu] += shift.exShift;
                // Java: matrixT[iu - 1][iu - 1] += shift.exShift;
                // Java: if (q >= 0)
                {
                    // Java: double z = FastMath.sqrt(FastMath.abs(q));
                    // Java: if (p >= 0)
                    {
                        // Java: z = p + z;
                    }
                    // Java: else
                    {
                        // Java: z = p - z;
                    }
                    // Java: final double x = matrixT[iu][iu - 1];
                    // Java: final double s = FastMath.abs(x) + FastMath.abs(z);
                    // Java: p = x / s;
                    // Java: q = z / s;
                    // Java: final double r = FastMath.sqrt(p * p + q * q);
                    // Java: p /= r;
                    // Java: q /= r;
                    // Java: // Row modification
                    // Java: for (int j = iu - 1; j < n; j++)
                    {
                        // Java: z = matrixT[iu - 1][j];
                        // Java: matrixT[iu - 1][j] = q * z + p * matrixT[iu][j];
                        // Java: matrixT[iu][j] = q * matrixT[iu][j] - p * z;
                    }
                    // Java: // Column modification
                    // Java: for (int i = 0; i <= iu; i++)
                    {
                        // Java: z = matrixT[i][iu - 1];
                        // Java: matrixT[i][iu - 1] = q * z + p * matrixT[i][iu];
                        // Java: matrixT[i][iu] = q * matrixT[i][iu] - p * z;
                    }
                    // Java: // Accumulate transformations
                    // Java: for (int i = 0; i <= n - 1; i++)
                    {
                        // Java: z = matrixP[i][iu - 1];
                        // Java: matrixP[i][iu - 1] = q * z + p * matrixP[i][iu];
                        // Java: matrixP[i][iu] = q * matrixP[i][iu] - p * z;
                    }
                }
                // Java: iu -= 2;
                // Java: iteration = 0;
            }
            // Java: else
            {
                // Java: // No convergence yet
                // Java: computeShift(il, iu, iteration, shift);
                // Java: // stop transformation after too many iterations
                // Java: if (++iteration > MAX_ITERATIONS)
                {
                    // Java: throw new MaxCountExceededException(LocalizedFormats.CONVERGENCE_FAILED,
                                                        // Java: MAX_ITERATIONS);
                }
                // Java: // the initial houseHolder vector for the QR step
                // Java: final double[] hVec = new double[3];
                // Java: final int im = initQRStep(il, iu, shift, hVec);
                // Java: performDoubleQRStep(il, im, iu, shift, hVec);
            }
        }
    }

    // Java: double getNorm()
    pub fn getNorm(&self) -> f64 {
        // Java: double norm = 0.0;
        // Java: for (int i = 0; i < matrixT.length; i++)
        {
            // Java: // as matrix T is (quasi-)triangular, also take the sub-diagonal element into account
            // Java: for (int j = FastMath.max(i - 1, 0); j < matrixT.length; j++)
            {
                // Java: norm += FastMath.abs(matrixT[i][j]);
            }
        }
        // Java: return norm;
        todo!()
    }

    // Java: int findSmallSubDiagonalElement(final int startIdx, final double norm)
    pub fn findSmallSubDiagonalElement(&self) -> i32 {
        // Java: int l = startIdx;
        // Java: while (l > 0)
        {
            // Java: double s = FastMath.abs(matrixT[l - 1][l - 1]) + FastMath.abs(matrixT[l][l]);
            // Java: if (s == 0.0)
            {
                // Java: s = norm;
            }
            // Java: if (FastMath.abs(matrixT[l][l - 1]) < epsilon * s)
            {
                // Java: break;
            }
            // Java: l--;
        }
        // Java: return l;
        todo!()
    }

    // Java: void computeShift(final int l, final int idx, final int iteration, final ShiftInfo shift)
    pub fn computeShift(&self) {
        // Java: // Form shift
        // Java: shift.x = matrixT[idx][idx];
        // Java: shift.y = shift.w = 0.0;
        // Java: if (l < idx)
        {
            // Java: shift.y = matrixT[idx - 1][idx - 1];
            // Java: shift.w = matrixT[idx][idx - 1] * matrixT[idx - 1][idx];
        }
        // Java: // Wilkinson's original ad hoc shift
        // Java: if (iteration == 10)
        {
            // Java: shift.exShift += shift.x;
            // Java: for (int i = 0; i <= idx; i++)
            {
                // Java: matrixT[i][i] -= shift.x;
            }
            // Java: final double s = FastMath.abs(matrixT[idx][idx - 1]) + FastMath.abs(matrixT[idx - 1][idx - 2]);
            // Java: shift.x = 0.75 * s;
            // Java: shift.y = 0.75 * s;
            // Java: shift.w = -0.4375 * s * s;
        }
        // Java: // MATLAB's new ad hoc shift
        // Java: if (iteration == 30)
        {
            // Java: double s = (shift.y - shift.x) / 2.0;
            // Java: s = s * s + shift.w;
            // Java: if (s > 0.0)
            {
                // Java: s = FastMath.sqrt(s);
                // Java: if (shift.y < shift.x)
                {
                    // Java: s = -s;
                }
                // Java: s = shift.x - shift.w / ((shift.y - shift.x) / 2.0 + s);
                // Java: for (int i = 0; i <= idx; i++)
                {
                    // Java: matrixT[i][i] -= s;
                }
                // Java: shift.exShift += s;
                // Java: shift.x = shift.y = shift.w = 0.964;
            }
        }
    }

    // Java: int initQRStep(int il, final int iu, final ShiftInfo shift, double[] hVec)
    pub fn initQRStep(&self) -> i32 {
        // Java: // Look for two consecutive small sub-diagonal elements
        // Java: int im = iu - 2;
        // Java: while (im >= il)
        {
            // Java: final double z = matrixT[im][im];
            // Java: final double r = shift.x - z;
            // Java: double s = shift.y - z;
            // Java: hVec[0] = (r * s - shift.w) / matrixT[im + 1][im] + matrixT[im][im + 1];
            // Java: hVec[1] = matrixT[im + 1][im + 1] - z - r - s;
            // Java: hVec[2] = matrixT[im + 2][im + 1];
            // Java: if (im == il)
            {
                // Java: break;
            }
            // Java: final double lhs = FastMath.abs(matrixT[im][im - 1]) * (FastMath.abs(hVec[1]) + FastMath.abs(hVec[2]));
            // Java: final double rhs = FastMath.abs(hVec[0]) * (FastMath.abs(matrixT[im - 1][im - 1]) +
                                                        // Java: FastMath.abs(z) +
                                                        // Java: FastMath.abs(matrixT[im + 1][im + 1]));
            // Java: if (lhs < epsilon * rhs)
            {
                // Java: break;
            }
            // Java: im--;
        }
        // Java: return im;
        todo!()
    }

    // Java: void performDoubleQRStep()
    pub fn performDoubleQRStep(&self) {
    }

}

// Java: public class ShiftInfo
impl ShiftInfo {
}
