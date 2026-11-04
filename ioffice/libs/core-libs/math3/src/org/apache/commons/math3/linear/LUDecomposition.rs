// Java: import org.apache.commons.math3.exception.DimensionMismatchException
use crate::org::apache::commons::math3::exception::DimensionMismatchException;
// Java: import org.apache.commons.math3.util.FastMath
use crate::org::apache::commons::math3::util::FastMath;

// Java field: double DEFAULT_TOO_SMALL
pub(crate) const DEFAULT_TOO_SMALL: f64 = 1e-11;

// Java: public class is
pub struct is {
}

// Java: public class in
pub struct r#in {
}

// Java: public class LUDecomposition
pub struct LUDecomposition {
    // Java field: double[][] lu
    pub(crate) lu: Box<dyn std::any::Any>,
    // Java field: int[] pivot
    pub(crate) pivot: Box<dyn std::any::Any>,
    // Java field: boolean even
    pub(crate) even: bool,
    // Java field: boolean singular
    pub(crate) singular: bool,
    // Java field: RealMatrix cachedL
    pub(crate) cachedL: Box<dyn std::any::Any>,
    // Java field: RealMatrix cachedU
    pub(crate) cachedU: Box<dyn std::any::Any>,
    // Java field: RealMatrix cachedP
    pub(crate) cachedP: Box<dyn std::any::Any>,
}

// Java: public class Solver
pub struct Solver {
    // Java field: double[][] lu
    pub(crate) lu: Box<dyn std::any::Any>,
    // Java field: int[] pivot
    pub(crate) pivot: Box<dyn std::any::Any>,
    // Java field: boolean singular
    pub(crate) singular: bool,
}

// Java: public class is
impl is {
}

// Java: public class in
impl r#in {
}

// Java: public class LUDecomposition
impl LUDecomposition {
    // Java: LUDecomposition(RealMatrix matrix)
    pub fn LUDecomposition(&self) {
        // Java: this(matrix, DEFAULT_TOO_SMALL);
    }

    // Java: LUDecomposition(RealMatrix matrix, double singularityThreshold)
    pub fn LUDecomposition_2(&self) {
        // Java: if (!matrix.isSquare())
        {
            // Java: throw new NonSquareMatrixException(matrix.getRowDimension(),
                                               // Java: matrix.getColumnDimension());
        }
        // Java: final int m = matrix.getColumnDimension();
        // Java: lu = matrix.getData();
        // Java: pivot = new int[m];
        // Java: cachedL = null;
        // Java: cachedU = null;
        // Java: cachedP = null;
        // Java: // Initialize permutation array and parity
        // Java: for (int row = 0; row < m; row++)
        {
            // Java: pivot[row] = row;
        }
        // Java: even     = true;
        // Java: singular = false;
        // Java: // Loop over columns
        // Java: for (int col = 0; col < m; col++)
        {
            // Java: // upper
            // Java: for (int row = 0; row < col; row++)
            {
                // Java: final double[] luRow = lu[row];
                // Java: double sum = luRow[col];
                // Java: for (int i = 0; i < row; i++)
                {
                    // Java: sum -= luRow[i] * lu[i][col];
                }
                // Java: luRow[col] = sum;
            }
            // Java: // lower
            // Java: int max = col; // permutation row
            // Java: double largest = Double.NEGATIVE_INFINITY;
            // Java: for (int row = col; row < m; row++)
            {
                // Java: final double[] luRow = lu[row];
                // Java: double sum = luRow[col];
                // Java: for (int i = 0; i < col; i++)
                {
                    // Java: sum -= luRow[i] * lu[i][col];
                }
                // Java: luRow[col] = sum;
                // Java: // maintain best permutation choice
                // Java: if (FastMath.abs(sum) > largest)
                {
                    // Java: largest = FastMath.abs(sum);
                    // Java: max = row;
                }
            }
            // Java: // Singularity check
            // Java: if (FastMath.abs(lu[max][col]) < singularityThreshold)
            {
                // Java: singular = true;
                // Java: return;
            }
            // Java: // Pivot if necessary
            // Java: if (max != col)
            {
                // Java: double tmp = 0;
                // Java: final double[] luMax = lu[max];
                // Java: final double[] luCol = lu[col];
                // Java: for (int i = 0; i < m; i++)
                {
                    // Java: tmp = luMax[i];
                    // Java: luMax[i] = luCol[i];
                    // Java: luCol[i] = tmp;
                }
                // Java: int temp = pivot[max];
                // Java: pivot[max] = pivot[col];
                // Java: pivot[col] = temp;
                // Java: even = !even;
            }
            // Java: // Divide the lower elements by the "winning" diagonal elt.
            // Java: final double luDiag = lu[col][col];
            // Java: for (int row = col + 1; row < m; row++)
            {
                // Java: lu[row][col] /= luDiag;
            }
        }
    }

    // Java: RealMatrix getL()
    pub fn getL(&self) -> Box<dyn std::any::Any> {
        // Java: if ((cachedL == null) && !singular)
        {
            // Java: final int m = pivot.length;
            // Java: cachedL = MatrixUtils.createRealMatrix(m, m);
            // Java: for (int i = 0; i < m; ++i)
            {
                // Java: final double[] luI = lu[i];
                // Java: for (int j = 0; j < i; ++j)
                {
                    // Java: cachedL.setEntry(i, j, luI[j]);
                }
                // Java: cachedL.setEntry(i, i, 1.0);
            }
        }
        // Java: return cachedL;
        todo!()
    }

    // Java: RealMatrix getU()
    pub fn getU(&self) -> Box<dyn std::any::Any> {
        // Java: if ((cachedU == null) && !singular)
        {
            // Java: final int m = pivot.length;
            // Java: cachedU = MatrixUtils.createRealMatrix(m, m);
            // Java: for (int i = 0; i < m; ++i)
            {
                // Java: final double[] luI = lu[i];
                // Java: for (int j = i; j < m; ++j)
                {
                    // Java: cachedU.setEntry(i, j, luI[j]);
                }
            }
        }
        // Java: return cachedU;
        todo!()
    }

    // Java: RealMatrix getP()
    pub fn getP(&self) -> Box<dyn std::any::Any> {
        // Java: if ((cachedP == null) && !singular)
        {
            // Java: final int m = pivot.length;
            // Java: cachedP = MatrixUtils.createRealMatrix(m, m);
            // Java: for (int i = 0; i < m; ++i)
            {
                // Java: cachedP.setEntry(i, pivot[i], 1.0);
            }
        }
        // Java: return cachedP;
        todo!()
    }

    // Java: int[] getPivot()
    pub fn getPivot(&self) -> Vec<i32> {
        // Java: return pivot.clone();
        todo!()
    }

    // Java: double getDeterminant()
    pub fn getDeterminant(&self) -> f64 {
        // Java: if (singular)
        {
            // Java: return 0;
        }
        // Java: else
        {
            // Java: final int m = pivot.length;
            // Java: double determinant = even ? 1 : -1;
            // Java: for (int i = 0; i < m; i++)
            {
                // Java: determinant *= lu[i][i];
            }
            // Java: return determinant;
        }
        todo!()
    }

    // Java: DecompositionSolver getSolver()
    pub fn getSolver(&self) -> Box<dyn std::any::Any> {
        // Java: return new Solver(lu, pivot, singular);
        todo!()
    }

}

// Java: public class Solver
impl Solver {
    // Java: Solver(final double[][] lu, final int[] pivot, final boolean singular)
    pub fn Solver(&self) {
            // Java: this.lu       = lu;
            // Java: this.pivot    = pivot;
            // Java: this.singular = singular;
    }

    // Java: boolean isNonSingular()
    pub fn isNonSingular(&self) -> bool {
            // Java: return !singular;
        todo!()
    }

    // Java: RealVector solve(RealVector b)
    pub fn solve(&self) -> Box<dyn std::any::Any> {
            // Java: final int m = pivot.length;
            // Java: if (b.getDimension() != m)
            {
                // Java: throw new DimensionMismatchException(b.getDimension(), m);
            }
            // Java: if (singular)
            {
                // Java: throw new SingularMatrixException();
            }
            // Java: final double[] bp = new double[m];
            // Java: // Apply permutations to b
            // Java: for (int row = 0; row < m; row++)
            {
                // Java: bp[row] = b.getEntry(pivot[row]);
            }
            // Java: // Solve LY = b
            // Java: for (int col = 0; col < m; col++)
            {
                // Java: final double bpCol = bp[col];
                // Java: for (int i = col + 1; i < m; i++)
                {
                    // Java: bp[i] -= bpCol * lu[i][col];
                }
            }
            // Java: // Solve UX = Y
            // Java: for (int col = m - 1; col >= 0; col--)
            {
                // Java: bp[col] /= lu[col][col];
                // Java: final double bpCol = bp[col];
                // Java: for (int i = 0; i < col; i++)
                {
                    // Java: bp[i] -= bpCol * lu[i][col];
                }
            }
            // Java: return new ArrayRealVector(bp, false);
        todo!()
    }

    // Java: RealMatrix solve(RealMatrix b)
    pub fn solve_2(&self) -> Box<dyn std::any::Any> {
            // Java: final int m = pivot.length;
            // Java: if (b.getRowDimension() != m)
            {
                // Java: throw new DimensionMismatchException(b.getRowDimension(), m);
            }
            // Java: if (singular)
            {
                // Java: throw new SingularMatrixException();
            }
            // Java: final int nColB = b.getColumnDimension();
            // Java: // Apply permutations to b
            // Java: final double[][] bp = new double[m][nColB];
            // Java: for (int row = 0; row < m; row++)
            {
                // Java: final double[] bpRow = bp[row];
                // Java: final int pRow = pivot[row];
                // Java: for (int col = 0; col < nColB; col++)
                {
                    // Java: bpRow[col] = b.getEntry(pRow, col);
                }
            }
            // Java: // Solve LY = b
            // Java: for (int col = 0; col < m; col++)
            {
                // Java: final double[] bpCol = bp[col];
                // Java: for (int i = col + 1; i < m; i++)
                {
                    // Java: final double[] bpI = bp[i];
                    // Java: final double luICol = lu[i][col];
                    // Java: for (int j = 0; j < nColB; j++)
                    {
                        // Java: bpI[j] -= bpCol[j] * luICol;
                    }
                }
            }
            // Java: // Solve UX = Y
            // Java: for (int col = m - 1; col >= 0; col--)
            {
                // Java: final double[] bpCol = bp[col];
                // Java: final double luDiag = lu[col][col];
                // Java: for (int j = 0; j < nColB; j++)
                {
                    // Java: bpCol[j] /= luDiag;
                }
                // Java: for (int i = 0; i < col; i++)
                {
                    // Java: final double[] bpI = bp[i];
                    // Java: final double luICol = lu[i][col];
                    // Java: for (int j = 0; j < nColB; j++)
                    {
                        // Java: bpI[j] -= bpCol[j] * luICol;
                    }
                }
            }
            // Java: return new Array2DRowRealMatrix(bp, false);
        todo!()
    }

    // Java: RealMatrix getInverse()
    pub fn getInverse(&self) -> Box<dyn std::any::Any> {
            // Java: return solve(MatrixUtils.createRealIdentityMatrix(pivot.length));
        todo!()
    }

}
