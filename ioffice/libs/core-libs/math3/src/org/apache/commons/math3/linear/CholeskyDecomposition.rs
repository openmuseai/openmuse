use std::sync::{Arc, Mutex};
// Java: import org.apache.commons.math3.exception.DimensionMismatchException
use crate::org::apache::commons::math3::exception::DimensionMismatchException;
// Java: import org.apache.commons.math3.util.FastMath
use crate::org::apache::commons::math3::util::FastMath;

// Java field: double DEFAULT_RELATIVE_SYMMETRY_THRESHOLD
pub(crate) const DEFAULT_RELATIVE_SYMMETRY_THRESHOLD: f64 = 0.0;
// Java field: double DEFAULT_ABSOLUTE_POSITIVITY_THRESHOLD
pub(crate) const DEFAULT_ABSOLUTE_POSITIVITY_THRESHOLD: f64 = 0.0;

// Java: public class is
pub struct r#is {
}

// Java: public class throws
pub struct r#throws {
}

// Java: public class in
pub struct r#in {
}

// Java: public class CholeskyDecomposition
pub struct CholeskyDecomposition {
    // Java field: double[][] lTData
    pub(crate) lTData: Arc<Mutex<Vec<Vec<f64>>>>,
    // Java field: RealMatrix cachedL
    pub(crate) cachedL: Arc<Mutex<Box<dyn std::any::Any>>>,
    // Java field: RealMatrix cachedLT
    pub(crate) cachedLT: Arc<Mutex<Box<dyn std::any::Any>>>,
}

// Java: public class Solver
pub struct Solver {
    // Java field: double[][] lTData
    pub(crate) lTData: Vec<Vec<f64>>,
}

// Java: public class is
impl r#is {
}

// Java: public class throws
impl throws {
}

// Java: public class in
impl r#in {
}

// Java: public class CholeskyDecomposition
impl CholeskyDecomposition {
    // Java: CholeskyDecomposition(final RealMatrix matrix)
    pub fn CholeskyDecomposition(&self, matrix: &dyn std::any::Any) {
        // Java: this(matrix, DEFAULT_RELATIVE_SYMMETRY_THRESHOLD,
             // Java: DEFAULT_ABSOLUTE_POSITIVITY_THRESHOLD);
    }

    // Java: CholeskyDecomposition()
    pub fn CholeskyDecomposition_1(&self) {
    }

    // Java: RealMatrix getL()
    pub fn getL(&self) -> Box<dyn std::any::Any> {
        // Java: if (cachedL == null)
        {
            // Java: cachedL = getLT().transpose();
        }
        // Java: return cachedL;
        todo!();
    }

    // Java: RealMatrix getLT()
    pub fn getLT(&self) -> Box<dyn std::any::Any> {
        // Java: if (cachedLT == null)
        {
            // Java: cachedLT = MatrixUtils.createRealMatrix(lTData);
        }
        // Java: // return the cached matrix
        // Java: return cachedLT;
        todo!();
    }

    // Java: double getDeterminant()
    pub fn getDeterminant(&self) -> f64 {
        // Java: double determinant = 1.0;
        // Java: for (int i = 0; i < lTData.length; ++i)
        {
            // Java: double lTii = lTData[i][i];
            // Java: determinant *= lTii * lTii;
        }
        // Java: return determinant;
        todo!();
    }

    // Java: DecompositionSolver getSolver()
    pub fn getSolver(&self) -> Box<dyn std::any::Any> {
        // Java: return new Solver(lTData);
        todo!();
    }

}

// Java: public class Solver
impl Solver {
    // Java: Solver(final double[][] lTData)
    pub fn Solver(&self) {
            // Java: this.lTData = lTData;
    }

    // Java: boolean isNonSingular()
    pub fn isNonSingular(&self) -> bool {
            // Java: // if we get this far, the matrix was positive definite, hence non-singular
            // Java: return true;
        todo!();
    }

    // Java: RealVector solve(final RealVector b)
    pub fn solve(&self, b: &dyn std::any::Any) -> Box<dyn std::any::Any> {
            // Java: final int m = lTData.length;
            // Java: if (b.getDimension() != m)
            {
                // Java: throw new DimensionMismatchException(b.getDimension(), m);
            }
            // Java: final double[] x = b.toArray();
            // Java: // Solve LY = b
            // Java: for (int j = 0; j < m; j++)
            {
                // Java: final double[] lJ = lTData[j];
                // Java: x[j] /= lJ[j];
                // Java: final double xJ = x[j];
                // Java: for (int i = j + 1; i < m; i++)
                {
                    // Java: x[i] -= xJ * lJ[i];
                }
            }
            // Java: // Solve LTX = Y
            // Java: for (int j = m - 1; j >= 0; j--)
            {
                // Java: x[j] /= lTData[j][j];
                // Java: final double xJ = x[j];
                // Java: for (int i = 0; i < j; i++)
                {
                    // Java: x[i] -= xJ * lTData[i][j];
                }
            }
            // Java: return new ArrayRealVector(x, false);
        todo!();
    }

    // Java: RealMatrix solve(RealMatrix b)
    pub fn solve_1(&self, b: &dyn std::any::Any) -> Box<dyn std::any::Any> {
            // Java: final int m = lTData.length;
            // Java: if (b.getRowDimension() != m)
            {
                // Java: throw new DimensionMismatchException(b.getRowDimension(), m);
            }
            // Java: final int nColB = b.getColumnDimension();
            // Java: final double[][] x = b.getData();
            // Java: // Solve LY = b
            // Java: for (int j = 0; j < m; j++)
            {
                // Java: final double[] lJ = lTData[j];
                // Java: final double lJJ = lJ[j];
                // Java: final double[] xJ = x[j];
                // Java: for (int k = 0; k < nColB; ++k)
                {
                    // Java: xJ[k] /= lJJ;
                }
                // Java: for (int i = j + 1; i < m; i++)
                {
                    // Java: final double[] xI = x[i];
                    // Java: final double lJI = lJ[i];
                    // Java: for (int k = 0; k < nColB; ++k)
                    {
                        // Java: xI[k] -= xJ[k] * lJI;
                    }
                }
            }
            // Java: // Solve LTX = Y
            // Java: for (int j = m - 1; j >= 0; j--)
            {
                // Java: final double lJJ = lTData[j][j];
                // Java: final double[] xJ = x[j];
                // Java: for (int k = 0; k < nColB; ++k)
                {
                    // Java: xJ[k] /= lJJ;
                }
                // Java: for (int i = 0; i < j; i++)
                {
                    // Java: final double[] xI = x[i];
                    // Java: final double lIJ = lTData[i][j];
                    // Java: for (int k = 0; k < nColB; ++k)
                    {
                        // Java: xI[k] -= xJ[k] * lIJ;
                    }
                }
            }
            // Java: return new Array2DRowRealMatrix(x);
        todo!();
    }

    // Java: RealMatrix getInverse()
    pub fn getInverse(&self) -> Box<dyn std::any::Any> {
            // Java: return solve(MatrixUtils.createRealIdentityMatrix(lTData.length));
        todo!();
    }

}
