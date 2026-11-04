use std::sync::{Arc, Mutex};
// Java: import org.apache.commons.math3.complex.Complex
use crate::org::apache::commons::math3::complex::Complex::Complex;
// Java: import org.apache.commons.math3.exception.MathArithmeticException
use crate::org::apache::commons::math3::exception::MathArithmeticException;
// Java: import org.apache.commons.math3.exception.MathUnsupportedOperationException
use crate::org::apache::commons::math3::exception::MathUnsupportedOperationException;
// Java: import org.apache.commons.math3.exception.MaxCountExceededException
use crate::org::apache::commons::math3::exception::MaxCountExceededException;
// Java: import org.apache.commons.math3.exception.DimensionMismatchException
use crate::org::apache::commons::math3::exception::DimensionMismatchException;
// Java: import org.apache.commons.math3.exception.util.LocalizedFormats
use crate::org::apache::commons::math3::exception::util::LocalizedFormats;
// Java: import org.apache.commons.math3.util.Precision
use crate::org::apache::commons::math3::util::Precision;
// Java: import org.apache.commons.math3.util.FastMath
use crate::org::apache::commons::math3::util::FastMath;

// Java field: double EPSILON
pub(crate) const EPSILON: f64 = 0.0;

// Java: public class is
pub struct r#is {
}

// Java: public class supports
pub struct supports {
}

// Java: public class EigenDecomposition
pub struct EigenDecomposition {
    // Java field: byte maxIter
    pub(crate) maxIter: i8,
    // Java field: double[] main
    pub(crate) main: Arc<Mutex<Vec<f64>>>,
    // Java field: double[] secondary
    pub(crate) secondary: Arc<Mutex<Vec<f64>>>,
    // Java field: TriDiagonalTransformer transformer
    pub(crate) transformer: Arc<Mutex<Box<dyn std::any::Any>>>,
    // Java field: double[] realEigenvalues
    pub(crate) realEigenvalues: Arc<Mutex<Vec<f64>>>,
    // Java field: double[] imagEigenvalues
    pub(crate) imagEigenvalues: Arc<Mutex<Vec<f64>>>,
    // Java field: ArrayRealVector[] eigenvectors
    pub(crate) eigenvectors: Arc<Mutex<Vec<Box<dyn std::any::Any>>>>,
    // Java field: RealMatrix cachedV
    pub(crate) cachedV: Arc<Mutex<Box<dyn std::any::Any>>>,
    // Java field: RealMatrix cachedD
    pub(crate) cachedD: Arc<Mutex<Box<dyn std::any::Any>>>,
    // Java field: RealMatrix cachedVt
    pub(crate) cachedVt: Arc<Mutex<Box<dyn std::any::Any>>>,
    // Java field: boolean isSymmetric
    pub(crate) isSymmetric: bool,
}

// Java: public class Solver
pub struct Solver {
    // Java field: double[] realEigenvalues
    pub(crate) realEigenvalues: Arc<Mutex<Vec<f64>>>,
    // Java field: double[] imagEigenvalues
    pub(crate) imagEigenvalues: Arc<Mutex<Vec<f64>>>,
    // Java field: ArrayRealVector[] eigenvectors
    pub(crate) eigenvectors: Arc<Vec<Box<dyn std::any::Any>>>,
}

// Java: public class is
impl is {
}

// Java: public class supports
impl supports {
}

// Java: public class EigenDecomposition
impl EigenDecomposition {
    // Java: EigenDecomposition(final RealMatrix matrix)
    pub fn EigenDecomposition(&self, matrix: &dyn std::any::Any) {
    }

    // Java: EigenDecomposition()
    pub fn EigenDecomposition_1(&self) {
    }

    // Java: EigenDecomposition(final double[] main, final double[] secondary)
    pub fn EigenDecomposition_2(&self, main: &[f64], secondary: &[f64]) {
        // Java: isSymmetric = true;
        // Java: this.main      = main.clone();
        // Java: this.secondary = secondary.clone();
        // Java: transformer    = null;
        // Java: final int size = main.length;
        // Java: final double[][] z = new double[size][size];
        // Java: for (int i = 0; i < size; i++)
        {
            // Java: z[i][i] = 1.0;
        }
        // Java: findEigenVectors(z);
    }

    // Java: EigenDecomposition()
    pub fn EigenDecomposition_3(&self) {
    }

    // Java: RealMatrix getV()
    pub fn getV(&self) -> Box<dyn std::any::Any> {
        // Java: if (cachedV == null)
        {
            // Java: final int m = eigenvectors.length;
            // Java: cachedV = MatrixUtils.createRealMatrix(m, m);
            // Java: for (int k = 0; k < m; ++k)
            {
                // Java: cachedV.setColumnVector(k, eigenvectors[k]);
            }
        }
        // Java: // return the cached matrix
        // Java: return cachedV;
        todo!();
    }

    // Java: RealMatrix getD()
    pub fn getD(&self) -> Box<dyn std::any::Any> {
        // Java: if (cachedD == null)
        {
            // Java: // cache the matrix for subsequent calls
            // Java: cachedD = MatrixUtils.createRealDiagonalMatrix(realEigenvalues);
            // Java: for (int i = 0; i < imagEigenvalues.length; i++)
            {
                // Java: if (Precision.compareTo(imagEigenvalues[i], 0.0, EPSILON) > 0)
                {
                    // Java: cachedD.setEntry(i, i+1, imagEigenvalues[i]);
                }
                // Java: else if (Precision.compareTo(imagEigenvalues[i], 0.0, EPSILON) < 0)
                {
                    // Java: cachedD.setEntry(i, i-1, imagEigenvalues[i]);
                }
            }
        }
        // Java: return cachedD;
        todo!();
    }

    // Java: RealMatrix getVT()
    pub fn getVT(&self) -> Box<dyn std::any::Any> {
        // Java: if (cachedVt == null)
        {
            // Java: final int m = eigenvectors.length;
            // Java: cachedVt = MatrixUtils.createRealMatrix(m, m);
            // Java: for (int k = 0; k < m; ++k)
            {
                // Java: cachedVt.setRowVector(k, eigenvectors[k]);
            }
        }
        // Java: // return the cached matrix
        // Java: return cachedVt;
        todo!();
    }

    // Java: boolean hasComplexEigenvalues()
    pub fn hasComplexEigenvalues(&self) -> bool {
        // Java: for (int i = 0; i < imagEigenvalues.length; i++)
        {
            // Java: if (!Precision.equals(imagEigenvalues[i], 0.0, EPSILON))
            {
                // Java: return true;
            }
        }
        // Java: return false;
        todo!();
    }

    // Java: double[] getRealEigenvalues()
    pub fn getRealEigenvalues(&self) -> Vec<f64> {
        // Java: return realEigenvalues.clone();
        todo!();
    }

    // Java: double getRealEigenvalue(final int i)
    pub fn getRealEigenvalue(&self, i: i32) -> f64 {
        // Java: return realEigenvalues[i];
        todo!();
    }

    // Java: double[] getImagEigenvalues()
    pub fn getImagEigenvalues(&self) -> Vec<f64> {
        // Java: return imagEigenvalues.clone();
        todo!();
    }

    // Java: double getImagEigenvalue(final int i)
    pub fn getImagEigenvalue(&self, i: i32) -> f64 {
        // Java: return imagEigenvalues[i];
        todo!();
    }

    // Java: RealVector getEigenvector(final int i)
    pub fn getEigenvector(&self, i: i32) -> Box<dyn std::any::Any> {
        // Java: return eigenvectors[i].copy();
        todo!();
    }

    // Java: double getDeterminant()
    pub fn getDeterminant(&self) -> f64 {
        // Java: double determinant = 1;
        // Java: for (double lambda : realEigenvalues)
        {
            // Java: determinant *= lambda;
        }
        // Java: return determinant;
        todo!();
    }

    // Java: RealMatrix getSquareRoot()
    pub fn getSquareRoot(&self) -> Box<dyn std::any::Any> {
        // Java: if (!isSymmetric)
        {
            // Java: throw new MathUnsupportedOperationException();
        }
        // Java: final double[] sqrtEigenValues = new double[realEigenvalues.length];
        // Java: for (int i = 0; i < realEigenvalues.length; i++)
        {
            // Java: final double eigen = realEigenvalues[i];
            // Java: if (eigen <= 0)
            {
                // Java: throw new MathUnsupportedOperationException();
            }
            // Java: sqrtEigenValues[i] = FastMath.sqrt(eigen);
        }
        // Java: final RealMatrix sqrtEigen = MatrixUtils.createRealDiagonalMatrix(sqrtEigenValues);
        // Java: final RealMatrix v = getV();
        // Java: final RealMatrix vT = getVT();
        // Java: return v.multiply(sqrtEigen).multiply(vT);
        todo!();
    }

    // Java: DecompositionSolver getSolver()
    pub fn getSolver(&self) -> Box<dyn std::any::Any> {
        // Java: if (hasComplexEigenvalues())
        {
            // Java: throw new MathUnsupportedOperationException();
        }
        // Java: return new Solver(realEigenvalues, imagEigenvalues, eigenvectors);
        todo!();
    }

    // Java: void transformToTridiagonal(final RealMatrix matrix)
    pub fn transformToTridiagonal(&self, matrix: &dyn std::any::Any) {
        // Java: // transform the matrix to tridiagonal
        // Java: transformer = new TriDiagonalTransformer(matrix);
        // Java: main = transformer.getMainDiagonalRef();
        // Java: secondary = transformer.getSecondaryDiagonalRef();
    }

    // Java: void findEigenVectors(final double[][] householderMatrix)
    pub fn findEigenVectors(&self) {
        // Java: final double[][]z = householderMatrix.clone();
        // Java: final int n = main.length;
        // Java: realEigenvalues = new double[n];
        // Java: imagEigenvalues = new double[n];
        // Java: final double[] e = new double[n];
        // Java: for (int i = 0; i < n - 1; i++)
        {
            // Java: realEigenvalues[i] = main[i];
            // Java: e[i] = secondary[i];
        }
        // Java: realEigenvalues[n - 1] = main[n - 1];
        // Java: e[n - 1] = 0;
        // Java: // Determine the largest main and secondary value in absolute term.
        // Java: double maxAbsoluteValue = 0;
        // Java: for (int i = 0; i < n; i++)
        {
            // Java: if (FastMath.abs(realEigenvalues[i]) > maxAbsoluteValue)
            {
                // Java: maxAbsoluteValue = FastMath.abs(realEigenvalues[i]);
            }
            // Java: if (FastMath.abs(e[i]) > maxAbsoluteValue)
            {
                // Java: maxAbsoluteValue = FastMath.abs(e[i]);
            }
        }
        // Java: // Make null any main and secondary value too small to be significant
        // Java: if (maxAbsoluteValue != 0)
        {
            // Java: for (int i=0; i < n; i++)
            {
                // Java: if (FastMath.abs(realEigenvalues[i]) <= Precision.EPSILON * maxAbsoluteValue)
                {
                    // Java: realEigenvalues[i] = 0;
                }
                // Java: if (FastMath.abs(e[i]) <= Precision.EPSILON * maxAbsoluteValue)
                {
                    // Java: e[i]=0;
                }
            }
        }
        // Java: for (int j = 0; j < n; j++)
        {
            // Java: int its = 0;
            // Java: int m;
            // Java: do
            {
                // Java: for (m = j; m < n - 1; m++)
                {
                    // Java: double delta = FastMath.abs(realEigenvalues[m]) +
                        // Java: FastMath.abs(realEigenvalues[m + 1]);
                    // Java: if (FastMath.abs(e[m]) + delta == delta)
                    {
                        // Java: break;
                    }
                }
                // Java: if (m != j)
                {
                    // Java: if (its == maxIter)
                    {
                        // Java: throw new MaxCountExceededException(LocalizedFormats.CONVERGENCE_FAILED,
                                                            // Java: maxIter);
                    }
                    // Java: its++;
                    // Java: double q = (realEigenvalues[j + 1] - realEigenvalues[j]) / (2 * e[j]);
                    // Java: double t = FastMath.sqrt(1 + q * q);
                    // Java: if (q < 0.0)
                    {
                        // Java: q = realEigenvalues[m] - realEigenvalues[j] + e[j] / (q - t);
                    }
                    // Java: else
                    {
                        // Java: q = realEigenvalues[m] - realEigenvalues[j] + e[j] / (q + t);
                    }
                    // Java: double u = 0.0;
                    // Java: double s = 1.0;
                    // Java: double c = 1.0;
                    // Java: int i;
                    // Java: for (i = m - 1; i >= j; i--)
                    {
                        // Java: double p = s * e[i];
                        // Java: double h = c * e[i];
                        // Java: if (FastMath.abs(p) >= FastMath.abs(q))
                        {
                            // Java: c = q / p;
                            // Java: t = FastMath.sqrt(c * c + 1.0);
                            // Java: e[i + 1] = p * t;
                            // Java: s = 1.0 / t;
                            // Java: c *= s;
                        }
                        // Java: else
                        {
                            // Java: s = p / q;
                            // Java: t = FastMath.sqrt(s * s + 1.0);
                            // Java: e[i + 1] = q * t;
                            // Java: c = 1.0 / t;
                            // Java: s *= c;
                        }
                        // Java: if (e[i + 1] == 0.0)
                        {
                            // Java: realEigenvalues[i + 1] -= u;
                            // Java: e[m] = 0.0;
                            // Java: break;
                        }
                        // Java: q = realEigenvalues[i + 1] - u;
                        // Java: t = (realEigenvalues[i] - q) * s + 2.0 * c * h;
                        // Java: u = s * t;
                        // Java: realEigenvalues[i + 1] = q + u;
                        // Java: q = c * t - h;
                        // Java: for (int ia = 0; ia < n; ia++)
                        {
                            // Java: p = z[ia][i + 1];
                            // Java: z[ia][i + 1] = s * z[ia][i] + c * p;
                            // Java: z[ia][i] = c * z[ia][i] - s * p;
                        }
                    }
                    // Java: if (t == 0.0 && i >= j)
                    {
                        // Java: continue;
                    }
                    // Java: realEigenvalues[j] -= u;
                    // Java: e[j] = q;
                    // Java: e[m] = 0.0;
                }
            }
        }
        // Java: //Sort the eigen values (and vectors) in increase order
        // Java: for (int i = 0; i < n; i++)
        {
            // Java: int k = i;
            // Java: double p = realEigenvalues[i];
            // Java: for (int j = i + 1; j < n; j++)
            {
                // Java: if (realEigenvalues[j] > p)
                {
                    // Java: k = j;
                    // Java: p = realEigenvalues[j];
                }
            }
            // Java: if (k != i)
            {
                // Java: realEigenvalues[k] = realEigenvalues[i];
                // Java: realEigenvalues[i] = p;
                // Java: for (int j = 0; j < n; j++)
                {
                    // Java: p = z[j][i];
                    // Java: z[j][i] = z[j][k];
                    // Java: z[j][k] = p;
                }
            }
        }
        // Java: // Determine the largest eigen value in absolute term.
        // Java: maxAbsoluteValue = 0;
        // Java: for (int i = 0; i < n; i++)
        {
            // Java: if (FastMath.abs(realEigenvalues[i]) > maxAbsoluteValue)
            {
                // Java: maxAbsoluteValue=FastMath.abs(realEigenvalues[i]);
            }
        }
        // Java: // Make null any eigen value too small to be significant
        // Java: if (maxAbsoluteValue != 0.0)
        {
            // Java: for (int i=0; i < n; i++)
            {
                // Java: if (FastMath.abs(realEigenvalues[i]) < Precision.EPSILON * maxAbsoluteValue)
                {
                    // Java: realEigenvalues[i] = 0;
                }
            }
        }
        // Java: eigenvectors = new ArrayRealVector[n];
        // Java: final double[] tmp = new double[n];
        // Java: for (int i = 0; i < n; i++)
        {
            // Java: for (int j = 0; j < n; j++)
            {
                // Java: tmp[j] = z[j][i];
            }
            // Java: eigenvectors[i] = new ArrayRealVector(tmp);
        }
    }

    // Java: SchurTransformer transformToSchur(final RealMatrix matrix)
    pub fn transformToSchur(&self, matrix: &dyn std::any::Any) -> Box<dyn std::any::Any> {
        // Java: final SchurTransformer schurTransform = new SchurTransformer(matrix);
        // Java: final double[][] matT = schurTransform.getT().getData();
        // Java: realEigenvalues = new double[matT.length];
        // Java: imagEigenvalues = new double[matT.length];
        // Java: for (int i = 0; i < realEigenvalues.length; i++)
        {
            // Java: if (i == (realEigenvalues.length - 1) ||
                // Java: Precision.equals(matT[i + 1][i], 0.0, EPSILON))
                {
                // Java: realEigenvalues[i] = matT[i][i];
                }
            // Java: else
            {
                // Java: final double x = matT[i + 1][i + 1];
                // Java: final double p = 0.5 * (matT[i][i] - x);
                // Java: final double z = FastMath.sqrt(FastMath.abs(p * p + matT[i + 1][i] * matT[i][i + 1]));
                // Java: realEigenvalues[i] = x + p;
                // Java: imagEigenvalues[i] = z;
                // Java: realEigenvalues[i + 1] = x + p;
                // Java: imagEigenvalues[i + 1] = -z;
                // Java: i++;
            }
        }
        // Java: return schurTransform;
        todo!();
    }

    // Java: Complex cdiv()
    pub fn cdiv(&self) -> Complex {
        todo!();
    }

    // Java: void findEigenVectorsFromSchur(final SchurTransformer schur)
    pub fn findEigenVectorsFromSchur(&self, schur: &dyn std::any::Any) {
    }

}

// Java: public class Solver
impl Solver {
    // Java: Solver()
    pub fn Solver(&self) {
    }

    // Java: RealVector solve(final RealVector b)
    pub fn solve(&self, b: &dyn std::any::Any) -> Box<dyn std::any::Any> {
            // Java: if (!isNonSingular())
            {
                // Java: throw new SingularMatrixException();
            }
            // Java: final int m = realEigenvalues.length;
            // Java: if (b.getDimension() != m)
            {
                // Java: throw new DimensionMismatchException(b.getDimension(), m);
            }
            // Java: final double[] bp = new double[m];
            // Java: for (int i = 0; i < m; ++i)
            {
                // Java: final ArrayRealVector v = eigenvectors[i];
                // Java: final double[] vData = v.getDataRef();
                // Java: final double s = v.dotProduct(b) / realEigenvalues[i];
                // Java: for (int j = 0; j < m; ++j)
                {
                    // Java: bp[j] += s * vData[j];
                }
            }
            // Java: return new ArrayRealVector(bp, false);
        todo!();
    }

    // Java: RealMatrix solve(RealMatrix b)
    pub fn solve_1(&self, b: &dyn std::any::Any) -> Box<dyn std::any::Any> {
            // Java: if (!isNonSingular())
            {
                // Java: throw new SingularMatrixException();
            }
            // Java: final int m = realEigenvalues.length;
            // Java: if (b.getRowDimension() != m)
            {
                // Java: throw new DimensionMismatchException(b.getRowDimension(), m);
            }
            // Java: final int nColB = b.getColumnDimension();
            // Java: final double[][] bp = new double[m][nColB];
            // Java: final double[] tmpCol = new double[m];
            // Java: for (int k = 0; k < nColB; ++k)
            {
                // Java: for (int i = 0; i < m; ++i)
                {
                    // Java: tmpCol[i] = b.getEntry(i, k);
                    // Java: bp[i][k]  = 0;
                }
                // Java: for (int i = 0; i < m; ++i)
                {
                    // Java: final ArrayRealVector v = eigenvectors[i];
                    // Java: final double[] vData = v.getDataRef();
                    // Java: double s = 0;
                    // Java: for (int j = 0; j < m; ++j)
                    {
                        // Java: s += v.getEntry(j) * tmpCol[j];
                    }
                    // Java: s /= realEigenvalues[i];
                    // Java: for (int j = 0; j < m; ++j)
                    {
                        // Java: bp[j][k] += s * vData[j];
                    }
                }
            }
            // Java: return new Array2DRowRealMatrix(bp, false);
        todo!();
    }

    // Java: boolean isNonSingular()
    pub fn isNonSingular(&self) -> bool {
            // Java: double largestEigenvalueNorm = 0.0;
            // Java: // Looping over all values (in case they are not sorted in decreasing
            // Java: // order of their norm).
            // Java: for (int i = 0; i < realEigenvalues.length; ++i)
            {
                // Java: largestEigenvalueNorm = FastMath.max(largestEigenvalueNorm, eigenvalueNorm(i));
            }
            // Java: // Corner case: zero matrix, all exactly 0 eigenvalues
            // Java: if (largestEigenvalueNorm == 0.0)
            {
                // Java: return false;
            }
            // Java: for (int i = 0; i < realEigenvalues.length; ++i)
            {
                // Java: // Looking for eigenvalues that are 0, where we consider anything much much smaller
                // Java: // than the largest eigenvalue to be effectively 0.
                // Java: if (Precision.equals(eigenvalueNorm(i) / largestEigenvalueNorm, 0, EPSILON))
                {
                    // Java: return false;
                }
            }
            // Java: return true;
        todo!();
    }

    // Java: double eigenvalueNorm(int i)
    pub fn eigenvalueNorm(&self, i: i32) -> f64 {
            // Java: final double re = realEigenvalues[i];
            // Java: final double im = imagEigenvalues[i];
            // Java: return FastMath.sqrt(re * re + im * im);
        todo!();
    }

    // Java: RealMatrix getInverse()
    pub fn getInverse(&self) -> Box<dyn std::any::Any> {
            // Java: if (!isNonSingular())
            {
                // Java: throw new SingularMatrixException();
            }
            // Java: final int m = realEigenvalues.length;
            // Java: final double[][] invData = new double[m][m];
            // Java: for (int i = 0; i < m; ++i)
            {
                // Java: final double[] invI = invData[i];
                // Java: for (int j = 0; j < m; ++j)
                {
                    // Java: double invIJ = 0;
                    // Java: for (int k = 0; k < m; ++k)
                    {
                        // Java: final double[] vK = eigenvectors[k].getDataRef();
                        // Java: invIJ += vK[i] * vK[j] / realEigenvalues[k];
                    }
                    // Java: invI[j] = invIJ;
                }
            }
            // Java: return MatrixUtils.createRealMatrix(invData);
        todo!();
    }

}
