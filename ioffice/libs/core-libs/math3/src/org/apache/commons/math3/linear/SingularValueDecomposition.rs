// Java: import org.apache.commons.math3.exception.NumberIsTooLargeException
use crate::org::apache::commons::math3::exception::NumberIsTooLargeException;
// Java: import org.apache.commons.math3.exception.util.LocalizedFormats
use crate::org::apache::commons::math3::exception::util::LocalizedFormats;
// Java: import org.apache.commons.math3.util.FastMath
use crate::org::apache::commons::math3::util::FastMath;
// Java: import org.apache.commons.math3.util.Precision
use crate::org::apache::commons::math3::util::Precision;

// Java field: double EPS
pub(crate) const EPS: f64 = 0.0;
// Java field: double TINY
pub(crate) const TINY: f64 = 0.0;

// Java: public class is
pub struct is {
}

// Java: public class in
pub struct r#in {
}

// Java: public class SingularValueDecomposition
pub struct SingularValueDecomposition {
    // Java field: double[] singularValues
    pub(crate) singularValues: Box<dyn std::any::Any>,
    // Java field: int m
    pub(crate) m: i32,
    // Java field: int n
    pub(crate) n: i32,
    // Java field: boolean transposed
    pub(crate) transposed: bool,
    // Java field: RealMatrix cachedU
    pub(crate) cachedU: Box<dyn std::any::Any>,
    // Java field: RealMatrix cachedUt
    pub(crate) cachedUt: Box<dyn std::any::Any>,
    // Java field: RealMatrix cachedS
    pub(crate) cachedS: Box<dyn std::any::Any>,
    // Java field: RealMatrix cachedV
    pub(crate) cachedV: Box<dyn std::any::Any>,
    // Java field: RealMatrix cachedVt
    pub(crate) cachedVt: Box<dyn std::any::Any>,
    // Java field: double tol
    pub(crate) tol: f64,
}

// Java: public class Solver
pub struct Solver {
    // Java field: RealMatrix pseudoInverse
    pub(crate) pseudoInverse: Box<dyn std::any::Any>,
    // Java field: boolean nonSingular
    pub(crate) nonSingular: bool,
}

// Java: public class is
impl is {
}

// Java: public class in
impl r#in {
}

// Java: public class SingularValueDecomposition
impl SingularValueDecomposition {
    // Java: SingularValueDecomposition(final RealMatrix matrix)
    pub fn SingularValueDecomposition(&self) {
        // Java: final double[][] A;
         // Java: // "m" is always the largest dimension.
        // Java: if (matrix.getRowDimension() < matrix.getColumnDimension())
        {
            // Java: transposed = true;
            // Java: A = matrix.transpose().getData();
            // Java: m = matrix.getColumnDimension();
            // Java: n = matrix.getRowDimension();
        }
        // Java: else
        {
            // Java: transposed = false;
            // Java: A = matrix.getData();
            // Java: m = matrix.getRowDimension();
            // Java: n = matrix.getColumnDimension();
        }
        // Java: singularValues = new double[n];
        // Java: final double[][] U = new double[m][n];
        // Java: final double[][] V = new double[n][n];
        // Java: final double[] e = new double[n];
        // Java: final double[] work = new double[m];
        // Java: // Reduce A to bidiagonal form, storing the diagonal elements
        // Java: // in s and the super-diagonal elements in e.
        // Java: final int nct = FastMath.min(m - 1, n);
        // Java: final int nrt = FastMath.max(0, n - 2);
        // Java: for (int k = 0; k < FastMath.max(nct, nrt); k++)
        {
            // Java: if (k < nct)
            {
                // Java: // Compute the transformation for the k-th column and
                // Java: // place the k-th diagonal in s[k].
                // Java: // Compute 2-norm of k-th column without under/overflow.
                // Java: singularValues[k] = 0;
                // Java: for (int i = k; i < m; i++)
                {
                    // Java: singularValues[k] = FastMath.hypot(singularValues[k], A[i][k]);
                }
                // Java: if (singularValues[k] != 0)
                {
                    // Java: if (A[k][k] < 0)
                    {
                        // Java: singularValues[k] = -singularValues[k];
                    }
                    // Java: for (int i = k; i < m; i++)
                    {
                        // Java: A[i][k] /= singularValues[k];
                    }
                    // Java: A[k][k] += 1;
                }
                // Java: singularValues[k] = -singularValues[k];
            }
            // Java: for (int j = k + 1; j < n; j++)
            {
                // Java: if (k < nct &&
                    // Java: singularValues[k] != 0)
                    {
                    // Java: // Apply the transformation.
                    // Java: double t = 0;
                    // Java: for (int i = k; i < m; i++)
                    {
                        // Java: t += A[i][k] * A[i][j];
                    }
                    // Java: t = -t / A[k][k];
                    // Java: for (int i = k; i < m; i++)
                    {
                        // Java: A[i][j] += t * A[i][k];
                    }
                    }
                // Java: // Place the k-th row of A into e for the
                // Java: // subsequent calculation of the row transformation.
                // Java: e[j] = A[k][j];
            }
            // Java: if (k < nct)
            {
                // Java: // Place the transformation in U for subsequent back
                // Java: // multiplication.
                // Java: for (int i = k; i < m; i++)
                {
                    // Java: U[i][k] = A[i][k];
                }
            }
            // Java: if (k < nrt)
            {
                // Java: // Compute the k-th row transformation and place the
                // Java: // k-th super-diagonal in e[k].
                // Java: // Compute 2-norm without under/overflow.
                // Java: e[k] = 0;
                // Java: for (int i = k + 1; i < n; i++)
                {
                    // Java: e[k] = FastMath.hypot(e[k], e[i]);
                }
                // Java: if (e[k] != 0)
                {
                    // Java: if (e[k + 1] < 0)
                    {
                        // Java: e[k] = -e[k];
                    }
                    // Java: for (int i = k + 1; i < n; i++)
                    {
                        // Java: e[i] /= e[k];
                    }
                    // Java: e[k + 1] += 1;
                }
                // Java: e[k] = -e[k];
                // Java: if (k + 1 < m &&
                    // Java: e[k] != 0)
                    {
                    // Java: // Apply the transformation.
                    // Java: for (int i = k + 1; i < m; i++)
                    {
                        // Java: work[i] = 0;
                    }
                    // Java: for (int j = k + 1; j < n; j++)
                    {
                        // Java: for (int i = k + 1; i < m; i++)
                        {
                            // Java: work[i] += e[j] * A[i][j];
                        }
                    }
                    // Java: for (int j = k + 1; j < n; j++)
                    {
                        // Java: final double t = -e[j] / e[k + 1];
                        // Java: for (int i = k + 1; i < m; i++)
                        {
                            // Java: A[i][j] += t * work[i];
                        }
                    }
                    }
                // Java: // Place the transformation in V for subsequent
                // Java: // back multiplication.
                // Java: for (int i = k + 1; i < n; i++)
                {
                    // Java: V[i][k] = e[i];
                }
            }
        }
        // Java: // Set up the final bidiagonal matrix or order p.
        // Java: int p = n;
        // Java: if (nct < n)
        {
            // Java: singularValues[nct] = A[nct][nct];
        }
        // Java: if (m < p)
        {
            // Java: singularValues[p - 1] = 0;
        }
        // Java: if (nrt + 1 < p)
        {
            // Java: e[nrt] = A[nrt][p - 1];
        }
        // Java: e[p - 1] = 0;
        // Java: // Generate U.
        // Java: for (int j = nct; j < n; j++)
        {
            // Java: for (int i = 0; i < m; i++)
            {
                // Java: U[i][j] = 0;
            }
            // Java: U[j][j] = 1;
        }
        // Java: for (int k = nct - 1; k >= 0; k--)
        {
            // Java: if (singularValues[k] != 0)
            {
                // Java: for (int j = k + 1; j < n; j++)
                {
                    // Java: double t = 0;
                    // Java: for (int i = k; i < m; i++)
                    {
                        // Java: t += U[i][k] * U[i][j];
                    }
                    // Java: t = -t / U[k][k];
                    // Java: for (int i = k; i < m; i++)
                    {
                        // Java: U[i][j] += t * U[i][k];
                    }
                }
                // Java: for (int i = k; i < m; i++)
                {
                    // Java: U[i][k] = -U[i][k];
                }
                // Java: U[k][k] = 1 + U[k][k];
                // Java: for (int i = 0; i < k - 1; i++)
                {
                    // Java: U[i][k] = 0;
                }
            }
            // Java: else
            {
                // Java: for (int i = 0; i < m; i++)
                {
                    // Java: U[i][k] = 0;
                }
                // Java: U[k][k] = 1;
            }
        }
        // Java: // Generate V.
        // Java: for (int k = n - 1; k >= 0; k--)
        {
            // Java: if (k < nrt &&
                // Java: e[k] != 0)
                {
                // Java: for (int j = k + 1; j < n; j++)
                {
                    // Java: double t = 0;
                    // Java: for (int i = k + 1; i < n; i++)
                    {
                        // Java: t += V[i][k] * V[i][j];
                    }
                    // Java: t = -t / V[k + 1][k];
                    // Java: for (int i = k + 1; i < n; i++)
                    {
                        // Java: V[i][j] += t * V[i][k];
                    }
                }
                }
            // Java: for (int i = 0; i < n; i++)
            {
                // Java: V[i][k] = 0;
            }
            // Java: V[k][k] = 1;
        }
        // Java: // Main iteration loop for the singular values.
        // Java: final int pp = p - 1;
        // Java: while (p > 0)
        {
            // Java: int k;
            // Java: int kase;
            // Java: // Here is where a test for too many iterations would go.
            // Java: // This section of the program inspects for
            // Java: // negligible elements in the s and e arrays.  On
            // Java: // completion the variables kase and k are set as follows.
            // Java: // kase = 1     if s(p) and e[k-1] are negligible and k<p
            // Java: // kase = 2     if s(k) is negligible and k<p
            // Java: // kase = 3     if e[k-1] is negligible, k<p, and
            // Java: //              s(k), ..., s(p) are not negligible (qr step).
            // Java: // kase = 4     if e(p-1) is negligible (convergence).
            // Java: for (k = p - 2; k >= 0; k--)
            {
                // Java: final double threshold
                    // Java: = TINY + EPS * (FastMath.abs(singularValues[k]) +
                                    // Java: FastMath.abs(singularValues[k + 1]));
                // Java: // the following condition is written this way in order
                // Java: // to break out of the loop when NaN occurs, writing it
                // Java: // as "if (FastMath.abs(e[k]) <= threshold)" would loop
                // Java: // indefinitely in case of NaNs because comparison on NaNs
                // Java: // always return false, regardless of what is checked
                // Java: // see issue MATH-947
                // Java: if (!(FastMath.abs(e[k]) > threshold))
                {
                    // Java: e[k] = 0;
                    // Java: break;
                }
            }
            // Java: if (k == p - 2)
            {
                // Java: kase = 4;
            }
            // Java: else
            {
                // Java: int ks;
                // Java: for (ks = p - 1; ks >= k; ks--)
                {
                    // Java: if (ks == k)
                    {
                        // Java: break;
                    }
                    // Java: final double t = (ks != p ? FastMath.abs(e[ks]) : 0) +
                        // Java: (ks != k + 1 ? FastMath.abs(e[ks - 1]) : 0);
                    // Java: if (FastMath.abs(singularValues[ks]) <= TINY + EPS * t)
                    {
                        // Java: singularValues[ks] = 0;
                        // Java: break;
                    }
                }
                // Java: if (ks == k)
                {
                    // Java: kase = 3;
                }
                // Java: else if (ks == p - 1)
                {
                    // Java: kase = 1;
                }
                // Java: else
                {
                    // Java: kase = 2;
                    // Java: k = ks;
                }
            }
            // Java: k++;
            // Java: // Perform the task indicated by kase.
            // Java: switch (kase)
            {
                // Java: // Deflate negligible s(p).
                // Java: case 1:
                {
                    // Java: double f = e[p - 2];
                    // Java: e[p - 2] = 0;
                    // Java: for (int j = p - 2; j >= k; j--)
                    {
                        // Java: double t = FastMath.hypot(singularValues[j], f);
                        // Java: final double cs = singularValues[j] / t;
                        // Java: final double sn = f / t;
                        // Java: singularValues[j] = t;
                        // Java: if (j != k)
                        {
                            // Java: f = -sn * e[j - 1];
                            // Java: e[j - 1] = cs * e[j - 1];
                        }
                        // Java: for (int i = 0; i < n; i++)
                        {
                            // Java: t = cs * V[i][j] + sn * V[i][p - 1];
                            // Java: V[i][p - 1] = -sn * V[i][j] + cs * V[i][p - 1];
                            // Java: V[i][j] = t;
                        }
                    }
                }
                // Java: break;
                // Java: // Split at negligible s(k).
                // Java: case 2:
                {
                    // Java: double f = e[k - 1];
                    // Java: e[k - 1] = 0;
                    // Java: for (int j = k; j < p; j++)
                    {
                        // Java: double t = FastMath.hypot(singularValues[j], f);
                        // Java: final double cs = singularValues[j] / t;
                        // Java: final double sn = f / t;
                        // Java: singularValues[j] = t;
                        // Java: f = -sn * e[j];
                        // Java: e[j] = cs * e[j];
                        // Java: for (int i = 0; i < m; i++)
                        {
                            // Java: t = cs * U[i][j] + sn * U[i][k - 1];
                            // Java: U[i][k - 1] = -sn * U[i][j] + cs * U[i][k - 1];
                            // Java: U[i][j] = t;
                        }
                    }
                }
                // Java: break;
                // Java: // Perform one qr step.
                // Java: case 3:
                {
                    // Java: // Calculate the shift.
                    // Java: final double maxPm1Pm2 = FastMath.max(FastMath.abs(singularValues[p - 1]),
                                                          // Java: FastMath.abs(singularValues[p - 2]));
                    // Java: final double scale = FastMath.max(FastMath.max(FastMath.max(maxPm1Pm2,
                                                                                // Java: FastMath.abs(e[p - 2])),
                                                                   // Java: FastMath.abs(singularValues[k])),
                                                      // Java: FastMath.abs(e[k]));
                    // Java: final double sp = singularValues[p - 1] / scale;
                    // Java: final double spm1 = singularValues[p - 2] / scale;
                    // Java: final double epm1 = e[p - 2] / scale;
                    // Java: final double sk = singularValues[k] / scale;
                    // Java: final double ek = e[k] / scale;
                    // Java: final double b = ((spm1 + sp) * (spm1 - sp) + epm1 * epm1) / 2.0;
                    // Java: final double c = (sp * epm1) * (sp * epm1);
                    // Java: double shift = 0;
                    // Java: if (b != 0 ||
                        // Java: c != 0)
                        {
                        // Java: shift = FastMath.sqrt(b * b + c);
                        // Java: if (b < 0)
                        {
                            // Java: shift = -shift;
                        }
                        // Java: shift = c / (b + shift);
                        }
                    // Java: double f = (sk + sp) * (sk - sp) + shift;
                    // Java: double g = sk * ek;
                    // Java: // Chase zeros.
                    // Java: for (int j = k; j < p - 1; j++)
                    {
                        // Java: double t = FastMath.hypot(f, g);
                        // Java: double cs = f / t;
                        // Java: double sn = g / t;
                        // Java: if (j != k)
                        {
                            // Java: e[j - 1] = t;
                        }
                        // Java: f = cs * singularValues[j] + sn * e[j];
                        // Java: e[j] = cs * e[j] - sn * singularValues[j];
                        // Java: g = sn * singularValues[j + 1];
                        // Java: singularValues[j + 1] = cs * singularValues[j + 1];
                        // Java: for (int i = 0; i < n; i++)
                        {
                            // Java: t = cs * V[i][j] + sn * V[i][j + 1];
                            // Java: V[i][j + 1] = -sn * V[i][j] + cs * V[i][j + 1];
                            // Java: V[i][j] = t;
                        }
                        // Java: t = FastMath.hypot(f, g);
                        // Java: cs = f / t;
                        // Java: sn = g / t;
                        // Java: singularValues[j] = t;
                        // Java: f = cs * e[j] + sn * singularValues[j + 1];
                        // Java: singularValues[j + 1] = -sn * e[j] + cs * singularValues[j + 1];
                        // Java: g = sn * e[j + 1];
                        // Java: e[j + 1] = cs * e[j + 1];
                        // Java: if (j < m - 1)
                        {
                            // Java: for (int i = 0; i < m; i++)
                            {
                                // Java: t = cs * U[i][j] + sn * U[i][j + 1];
                                // Java: U[i][j + 1] = -sn * U[i][j] + cs * U[i][j + 1];
                                // Java: U[i][j] = t;
                            }
                        }
                    }
                    // Java: e[p - 2] = f;
                }
                // Java: break;
                // Java: // Convergence.
                // Java: default:
                {
                    // Java: // Make the singular values positive.
                    // Java: if (singularValues[k] <= 0)
                    {
                        // Java: singularValues[k] = singularValues[k] < 0 ? -singularValues[k] : 0;
                        // Java: for (int i = 0; i <= pp; i++)
                        {
                            // Java: V[i][k] = -V[i][k];
                        }
                    }
                    // Java: // Order the singular values.
                    // Java: while (k < pp)
                    {
                        // Java: if (singularValues[k] >= singularValues[k + 1])
                        {
                            // Java: break;
                        }
                        // Java: double t = singularValues[k];
                        // Java: singularValues[k] = singularValues[k + 1];
                        // Java: singularValues[k + 1] = t;
                        // Java: if (k < n - 1)
                        {
                            // Java: for (int i = 0; i < n; i++)
                            {
                                // Java: t = V[i][k + 1];
                                // Java: V[i][k + 1] = V[i][k];
                                // Java: V[i][k] = t;
                            }
                        }
                        // Java: if (k < m - 1)
                        {
                            // Java: for (int i = 0; i < m; i++)
                            {
                                // Java: t = U[i][k + 1];
                                // Java: U[i][k + 1] = U[i][k];
                                // Java: U[i][k] = t;
                            }
                        }
                        // Java: k++;
                    }
                    // Java: p--;
                }
                // Java: break;
            }
        }
        // Java: // Set the small value tolerance used to calculate rank and pseudo-inverse
        // Java: tol = FastMath.max(m * singularValues[0] * EPS,
                           // Java: FastMath.sqrt(Precision.SAFE_MIN));
        // Java: if (!transposed)
        {
            // Java: cachedU = MatrixUtils.createRealMatrix(U);
            // Java: cachedV = MatrixUtils.createRealMatrix(V);
        }
        // Java: else
        {
            // Java: cachedU = MatrixUtils.createRealMatrix(V);
            // Java: cachedV = MatrixUtils.createRealMatrix(U);
        }
    }

    // Java: RealMatrix getU()
    pub fn getU(&self) -> Box<dyn std::any::Any> {
        // Java: // return the cached matrix
        // Java: return cachedU;
        todo!()
    }

    // Java: RealMatrix getUT()
    pub fn getUT(&self) -> Box<dyn std::any::Any> {
        // Java: if (cachedUt == null)
        {
            // Java: cachedUt = getU().transpose();
        }
        // Java: // return the cached matrix
        // Java: return cachedUt;
        todo!()
    }

    // Java: RealMatrix getS()
    pub fn getS(&self) -> Box<dyn std::any::Any> {
        // Java: if (cachedS == null)
        {
            // Java: // cache the matrix for subsequent calls
            // Java: cachedS = MatrixUtils.createRealDiagonalMatrix(singularValues);
        }
        // Java: return cachedS;
        todo!()
    }

    // Java: double[] getSingularValues()
    pub fn getSingularValues(&self) -> Vec<f64> {
        // Java: return singularValues.clone();
        todo!()
    }

    // Java: RealMatrix getV()
    pub fn getV(&self) -> Box<dyn std::any::Any> {
        // Java: // return the cached matrix
        // Java: return cachedV;
        todo!()
    }

    // Java: RealMatrix getVT()
    pub fn getVT(&self) -> Box<dyn std::any::Any> {
        // Java: if (cachedVt == null)
        {
            // Java: cachedVt = getV().transpose();
        }
        // Java: // return the cached matrix
        // Java: return cachedVt;
        todo!()
    }

    // Java: RealMatrix getCovariance(final double minSingularValue)
    pub fn getCovariance(&self) -> Box<dyn std::any::Any> {
        // Java: // get the number of singular values to consider
        // Java: final int p = singularValues.length;
        // Java: int dimension = 0;
        // Java: while (dimension < p &&
               // Java: singularValues[dimension] >= minSingularValue)
               {
            // Java: ++dimension;
               }
        // Java: if (dimension == 0)
        {
            // Java: throw new NumberIsTooLargeException(LocalizedFormats.TOO_LARGE_CUTOFF_SINGULAR_VALUE,
                                                // Java: minSingularValue, singularValues[0], true);
        }
        // Java: final double[][] data = new double[dimension][p];
        // Java: getVT().walkInOptimizedOrder(new DefaultRealMatrixPreservingVisitor()
        {
            // Java: /** {@inheritDoc} */
            // Java: @Override
            // Java: public void visit(final int row, final int column,
                    // Java: final double value)
                    {
                // Java: data[row][column] = value / singularValues[row];
                    }
        }
        // Java: RealMatrix jv = new Array2DRowRealMatrix(data, false);
        // Java: return jv.transpose().multiply(jv);
        todo!()
    }

    // Java: double getNorm()
    pub fn getNorm(&self) -> f64 {
        // Java: return singularValues[0];
        todo!()
    }

    // Java: double getConditionNumber()
    pub fn getConditionNumber(&self) -> f64 {
        // Java: return singularValues[0] / singularValues[n - 1];
        todo!()
    }

    // Java: double getInverseConditionNumber()
    pub fn getInverseConditionNumber(&self) -> f64 {
        // Java: return singularValues[n - 1] / singularValues[0];
        todo!()
    }

    // Java: int getRank()
    pub fn getRank(&self) -> i32 {
        // Java: int r = 0;
        // Java: for (int i = 0; i < singularValues.length; i++)
        {
            // Java: if (singularValues[i] > tol)
            {
                // Java: r++;
            }
        }
        // Java: return r;
        todo!()
    }

    // Java: DecompositionSolver getSolver()
    pub fn getSolver(&self) -> Box<dyn std::any::Any> {
        // Java: return new Solver(singularValues, getUT(), getV(), getRank() == m, tol);
        todo!()
    }

}

// Java: public class Solver
impl Solver {
    // Java: Solver()
    pub fn Solver(&self) {
    }

    // Java: RealVector solve(final RealVector b)
    pub fn solve(&self) -> Box<dyn std::any::Any> {
            // Java: return pseudoInverse.operate(b);
        todo!()
    }

    // Java: RealMatrix solve(final RealMatrix b)
    pub fn solve_2(&self) -> Box<dyn std::any::Any> {
            // Java: return pseudoInverse.multiply(b);
        todo!()
    }

    // Java: boolean isNonSingular()
    pub fn isNonSingular(&self) -> bool {
            // Java: return nonSingular;
        todo!()
    }

    // Java: RealMatrix getInverse()
    pub fn getInverse(&self) -> Box<dyn std::any::Any> {
            // Java: return pseudoInverse;
        todo!()
    }

}
