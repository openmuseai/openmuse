// Java: import java.util.Arrays
// Java: import org.apache.commons.math3.exception.DimensionMismatchException
use crate::org::apache::commons::math3::exception::DimensionMismatchException;
// Java: import org.apache.commons.math3.util.FastMath
use crate::org::apache::commons::math3::util::FastMath;

// Java: public class compute
pub struct compute {
}

// Java: public class in
pub struct r#in {
}

// Java: public class QRDecomposition
pub struct QRDecomposition {
    // Java field: double[][] qrt
    pub(crate) qrt: Box<dyn std::any::Any>,
    // Java field: double[] rDiag
    pub(crate) rDiag: Box<dyn std::any::Any>,
    // Java field: RealMatrix cachedQ
    pub(crate) cachedQ: Box<dyn std::any::Any>,
    // Java field: RealMatrix cachedQT
    pub(crate) cachedQT: Box<dyn std::any::Any>,
    // Java field: RealMatrix cachedR
    pub(crate) cachedR: Box<dyn std::any::Any>,
    // Java field: RealMatrix cachedH
    pub(crate) cachedH: Box<dyn std::any::Any>,
    // Java field: double threshold
    pub(crate) threshold: f64,
}

// Java: public class Solver
pub struct Solver {
    // Java field: double[][] qrt
    pub(crate) qrt: Box<dyn std::any::Any>,
    // Java field: double[] rDiag
    pub(crate) rDiag: Box<dyn std::any::Any>,
    // Java field: double threshold
    pub(crate) threshold: f64,
}

// Java: public class compute
impl compute {
}

// Java: public class in
impl r#in {
}

// Java: public class QRDecomposition
impl QRDecomposition {
    // Java: QRDecomposition(RealMatrix matrix)
    pub fn QRDecomposition(&self) {
        // Java: this(matrix, 0d);
    }

    // Java: QRDecomposition()
    pub fn QRDecomposition_2(&self) {
    }

    // Java: void decompose(double[][] matrix)
    pub fn decompose(&self) {
        // Java: for (int minor = 0; minor < FastMath.min(matrix.length, matrix[0].length); minor++)
        {
            // Java: performHouseholderReflection(minor, matrix);
        }
    }

    // Java: void performHouseholderReflection(int minor, double[][] matrix)
    pub fn performHouseholderReflection(&self) {
        // Java: final double[] qrtMinor = matrix[minor];
        // Java: /*
         // Java: * Let x be the first column of the minor, and a^2 = |x|^2.
         // Java: * x will be in the positions qr[minor][minor] through qr[m][minor].
         // Java: * The first column of the transformed minor will be (a,0,0,..)'
         // Java: * The sign of a is chosen to be opposite to the sign of the first
         // Java: * component of x. Let's find a:
         // Java: */
        // Java: double xNormSqr = 0;
        // Java: for (int row = minor; row < qrtMinor.length; row++)
        {
            // Java: final double c = qrtMinor[row];
            // Java: xNormSqr += c * c;
        }
        // Java: final double a = (qrtMinor[minor] > 0) ? -FastMath.sqrt(xNormSqr) : FastMath.sqrt(xNormSqr);
        // Java: rDiag[minor] = a;
        // Java: if (a != 0.0)
        {
            // Java: /*
             // Java: * Calculate the normalized reflection vector v and transform
             // Java: * the first column. We know the norm of v beforehand: v = x-ae
             // Java: * so |v|^2 = <x-ae,x-ae> = <x,x>-2a<x,e>+a^2<e,e> =
             // Java: * a^2+a^2-2a<x,e> = 2a*(a - <x,e>).
             // Java: * Here <x, e> is now qr[minor][minor].
             // Java: * v = x-ae is stored in the column at qr:
             // Java: */
            // Java: qrtMinor[minor] -= a; // now |v|^2 = -2a*(qr[minor][minor])
            // Java: /*
             // Java: * Transform the rest of the columns of the minor:
             // Java: * They will be transformed by the matrix H = I-2vv'/|v|^2.
             // Java: * If x is a column vector of the minor, then
             // Java: * Hx = (I-2vv'/|v|^2)x = x-2vv'x/|v|^2 = x - 2<x,v>/|v|^2 v.
             // Java: * Therefore the transformation is easily calculated by
             // Java: * subtracting the column vector (2<x,v>/|v|^2)v from x.
             // Java: *
             // Java: * Let 2<x,v>/|v|^2 = alpha. From above we have
             // Java: * |v|^2 = -2a*(qr[minor][minor]), so
             // Java: * alpha = -<x,v>/(a*qr[minor][minor])
             // Java: */
            // Java: for (int col = minor+1; col < matrix.length; col++)
            {
                // Java: final double[] qrtCol = matrix[col];
                // Java: double alpha = 0;
                // Java: for (int row = minor; row < qrtCol.length; row++)
                {
                    // Java: alpha -= qrtCol[row] * qrtMinor[row];
                }
                // Java: alpha /= a * qrtMinor[minor];
                // Java: // Subtract the column vector alpha*v from x.
                // Java: for (int row = minor; row < qrtCol.length; row++)
                {
                    // Java: qrtCol[row] -= alpha * qrtMinor[row];
                }
            }
        }
    }

    // Java: RealMatrix getR()
    pub fn getR(&self) -> Box<dyn std::any::Any> {
        // Java: if (cachedR == null)
        {
            // Java: // R is supposed to be m x n
            // Java: final int n = qrt.length;
            // Java: final int m = qrt[0].length;
            // Java: double[][] ra = new double[m][n];
            // Java: // copy the diagonal from rDiag and the upper triangle of qr
            // Java: for (int row = FastMath.min(m, n) - 1; row >= 0; row--)
            {
                // Java: ra[row][row] = rDiag[row];
                // Java: for (int col = row + 1; col < n; col++)
                {
                    // Java: ra[row][col] = qrt[col][row];
                }
            }
            // Java: cachedR = MatrixUtils.createRealMatrix(ra);
        }
        // Java: // return the cached matrix
        // Java: return cachedR;
        todo!()
    }

    // Java: RealMatrix getQ()
    pub fn getQ(&self) -> Box<dyn std::any::Any> {
        // Java: if (cachedQ == null)
        {
            // Java: cachedQ = getQT().transpose();
        }
        // Java: return cachedQ;
        todo!()
    }

    // Java: RealMatrix getQT()
    pub fn getQT(&self) -> Box<dyn std::any::Any> {
        // Java: if (cachedQT == null)
        {
            // Java: // QT is supposed to be m x m
            // Java: final int n = qrt.length;
            // Java: final int m = qrt[0].length;
            // Java: double[][] qta = new double[m][m];
            // Java: /*
             // Java: * Q = Q1 Q2 ... Q_m, so Q is formed by first constructing Q_m and then
             // Java: * applying the Householder transformations Q_(m-1),Q_(m-2),...,Q1 in
             // Java: * succession to the result
             // Java: */
            // Java: for (int minor = m - 1; minor >= FastMath.min(m, n); minor--)
            {
                // Java: qta[minor][minor] = 1.0d;
            }
            // Java: for (int minor = FastMath.min(m, n)-1; minor >= 0; minor--)
            {
                // Java: final double[] qrtMinor = qrt[minor];
                // Java: qta[minor][minor] = 1.0d;
                // Java: if (qrtMinor[minor] != 0.0)
                {
                    // Java: for (int col = minor; col < m; col++)
                    {
                        // Java: double alpha = 0;
                        // Java: for (int row = minor; row < m; row++)
                        {
                            // Java: alpha -= qta[col][row] * qrtMinor[row];
                        }
                        // Java: alpha /= rDiag[minor] * qrtMinor[minor];
                        // Java: for (int row = minor; row < m; row++)
                        {
                            // Java: qta[col][row] += -alpha * qrtMinor[row];
                        }
                    }
                }
            }
            // Java: cachedQT = MatrixUtils.createRealMatrix(qta);
        }
        // Java: // return the cached matrix
        // Java: return cachedQT;
        todo!()
    }

    // Java: RealMatrix getH()
    pub fn getH(&self) -> Box<dyn std::any::Any> {
        // Java: if (cachedH == null)
        {
            // Java: final int n = qrt.length;
            // Java: final int m = qrt[0].length;
            // Java: double[][] ha = new double[m][n];
            // Java: for (int i = 0; i < m; ++i)
            {
                // Java: for (int j = 0; j < FastMath.min(i + 1, n); ++j)
                {
                    // Java: ha[i][j] = qrt[j][i] / -rDiag[j];
                }
            }
            // Java: cachedH = MatrixUtils.createRealMatrix(ha);
        }
        // Java: // return the cached matrix
        // Java: return cachedH;
        todo!()
    }

    // Java: DecompositionSolver getSolver()
    pub fn getSolver(&self) -> Box<dyn std::any::Any> {
        // Java: return new Solver(qrt, rDiag, threshold);
        todo!()
    }

}

// Java: public class Solver
impl Solver {
    // Java: Solver()
    pub fn Solver(&self) {
    }

    // Java: boolean isNonSingular()
    pub fn isNonSingular(&self) -> bool {
            // Java: for (double diag : rDiag)
            {
                // Java: if (FastMath.abs(diag) <= threshold)
                {
                    // Java: return false;
                }
            }
            // Java: return true;
        todo!()
    }

    // Java: RealVector solve(RealVector b)
    pub fn solve(&self) -> Box<dyn std::any::Any> {
            // Java: final int n = qrt.length;
            // Java: final int m = qrt[0].length;
            // Java: if (b.getDimension() != m)
            {
                // Java: throw new DimensionMismatchException(b.getDimension(), m);
            }
            // Java: if (!isNonSingular())
            {
                // Java: throw new SingularMatrixException();
            }
            // Java: final double[] x = new double[n];
            // Java: final double[] y = b.toArray();
            // Java: // apply Householder transforms to solve Q.y = b
            // Java: for (int minor = 0; minor < FastMath.min(m, n); minor++)
            {
                // Java: final double[] qrtMinor = qrt[minor];
                // Java: double dotProduct = 0;
                // Java: for (int row = minor; row < m; row++)
                {
                    // Java: dotProduct += y[row] * qrtMinor[row];
                }
                // Java: dotProduct /= rDiag[minor] * qrtMinor[minor];
                // Java: for (int row = minor; row < m; row++)
                {
                    // Java: y[row] += dotProduct * qrtMinor[row];
                }
            }
            // Java: // solve triangular system R.x = y
            // Java: for (int row = rDiag.length - 1; row >= 0; --row)
            {
                // Java: y[row] /= rDiag[row];
                // Java: final double yRow = y[row];
                // Java: final double[] qrtRow = qrt[row];
                // Java: x[row] = yRow;
                // Java: for (int i = 0; i < row; i++)
                {
                    // Java: y[i] -= yRow * qrtRow[i];
                }
            }
            // Java: return new ArrayRealVector(x, false);
        todo!()
    }

    // Java: RealMatrix solve(RealMatrix b)
    pub fn solve_2(&self) -> Box<dyn std::any::Any> {
            // Java: final int n = qrt.length;
            // Java: final int m = qrt[0].length;
            // Java: if (b.getRowDimension() != m)
            {
                // Java: throw new DimensionMismatchException(b.getRowDimension(), m);
            }
            // Java: if (!isNonSingular())
            {
                // Java: throw new SingularMatrixException();
            }
            // Java: final int columns        = b.getColumnDimension();
            // Java: final int blockSize      = BlockRealMatrix.BLOCK_SIZE;
            // Java: final int cBlocks        = (columns + blockSize - 1) / blockSize;
            // Java: final double[][] xBlocks = BlockRealMatrix.createBlocksLayout(n, columns);
            // Java: final double[][] y       = new double[b.getRowDimension()][blockSize];
            // Java: final double[]   alpha   = new double[blockSize];
            // Java: for (int kBlock = 0; kBlock < cBlocks; ++kBlock)
            {
                // Java: final int kStart = kBlock * blockSize;
                // Java: final int kEnd   = FastMath.min(kStart + blockSize, columns);
                // Java: final int kWidth = kEnd - kStart;
                // Java: // get the right hand side vector
                // Java: b.copySubMatrix(0, m - 1, kStart, kEnd - 1, y);
                // Java: // apply Householder transforms to solve Q.y = b
                // Java: for (int minor = 0; minor < FastMath.min(m, n); minor++)
                {
                    // Java: final double[] qrtMinor = qrt[minor];
                    // Java: final double factor     = 1.0 / (rDiag[minor] * qrtMinor[minor]);
                    // Java: Arrays.fill(alpha, 0, kWidth, 0.0);
                    // Java: for (int row = minor; row < m; ++row)
                    {
                        // Java: final double   d    = qrtMinor[row];
                        // Java: final double[] yRow = y[row];
                        // Java: for (int k = 0; k < kWidth; ++k)
                        {
                            // Java: alpha[k] += d * yRow[k];
                        }
                    }
                    // Java: for (int k = 0; k < kWidth; ++k)
                    {
                        // Java: alpha[k] *= factor;
                    }
                    // Java: for (int row = minor; row < m; ++row)
                    {
                        // Java: final double   d    = qrtMinor[row];
                        // Java: final double[] yRow = y[row];
                        // Java: for (int k = 0; k < kWidth; ++k)
                        {
                            // Java: yRow[k] += alpha[k] * d;
                        }
                    }
                }
                // Java: // solve triangular system R.x = y
                // Java: for (int j = rDiag.length - 1; j >= 0; --j)
                {
                    // Java: final int      jBlock = j / blockSize;
                    // Java: final int      jStart = jBlock * blockSize;
                    // Java: final double   factor = 1.0 / rDiag[j];
                    // Java: final double[] yJ     = y[j];
                    // Java: final double[] xBlock = xBlocks[jBlock * cBlocks + kBlock];
                    // Java: int index = (j - jStart) * kWidth;
                    // Java: for (int k = 0; k < kWidth; ++k)
                    {
                        // Java: yJ[k]          *= factor;
                        // Java: xBlock[index++] = yJ[k];
                    }
                    // Java: final double[] qrtJ = qrt[j];
                    // Java: for (int i = 0; i < j; ++i)
                    {
                        // Java: final double rIJ  = qrtJ[i];
                        // Java: final double[] yI = y[i];
                        // Java: for (int k = 0; k < kWidth; ++k)
                        {
                            // Java: yI[k] -= yJ[k] * rIJ;
                        }
                    }
                }
            }
            // Java: return new BlockRealMatrix(n, columns, xBlocks, false);
        todo!()
    }

    // Java: RealMatrix getInverse()
    pub fn getInverse(&self) -> Box<dyn std::any::Any> {
            // Java: return solve(MatrixUtils.createRealIdentityMatrix(qrt[0].length));
        todo!()
    }

}
