// Java: import org.apache.commons.math3.util.FastMath
use crate::org::apache::commons::math3::util::FastMath;

// Java: public class compute
pub struct compute {
}

// Java: public class RRQRDecomposition extends QRDecomposition
pub struct RRQRDecomposition {
    // Java field: int[] p
    pub(crate) p: Box<dyn std::any::Any>,
    // Java field: RealMatrix cachedP
    pub(crate) cachedP: Box<dyn std::any::Any>,
}

// Java: public class Solver
pub struct Solver {
    // Java field: DecompositionSolver upper
    pub(crate) upper: Box<dyn std::any::Any>,
    // Java field: RealMatrix p
    pub(crate) p: Box<dyn std::any::Any>,
}

// Java: public class compute
impl compute {
}

// Java: public class RRQRDecomposition extends QRDecomposition
impl RRQRDecomposition {
    // Java: RRQRDecomposition(RealMatrix matrix)
    pub fn RRQRDecomposition(&self) {
        // Java: this(matrix, 0d);
    }

    // Java: RRQRDecomposition(RealMatrix matrix, double threshold)
    pub fn RRQRDecomposition_2(&self) {
        // Java: super(matrix, threshold);
    }

    // Java: void decompose(double[][] qrt)
    pub fn decompose(&self) {
        // Java: p = new int[qrt.length];
        // Java: for (int i = 0; i < p.length; i++)
        {
            // Java: p[i] = i;
        }
        // Java: super.decompose(qrt);
    }

    // Java: void performHouseholderReflection(int minor, double[][] qrt)
    pub fn performHouseholderReflection(&self) {
        // Java: double l2NormSquaredMax = 0;
        // Java: // Find the unreduced column with the greatest L2-Norm
        // Java: int l2NormSquaredMaxIndex = minor;
        // Java: for (int i = minor; i < qrt.length; i++)
        {
            // Java: double l2NormSquared = 0;
            // Java: for (int j = 0; j < qrt[i].length; j++)
            {
                // Java: l2NormSquared += qrt[i][j] * qrt[i][j];
            }
            // Java: if (l2NormSquared > l2NormSquaredMax)
            {
                // Java: l2NormSquaredMax = l2NormSquared;
                // Java: l2NormSquaredMaxIndex = i;
            }
        }
        // Java: // swap the current column with that with the greated L2-Norm and record in p
        // Java: if (l2NormSquaredMaxIndex != minor)
        {
            // Java: double[] tmp1 = qrt[minor];
            // Java: qrt[minor] = qrt[l2NormSquaredMaxIndex];
            // Java: qrt[l2NormSquaredMaxIndex] = tmp1;
            // Java: int tmp2 = p[minor];
            // Java: p[minor] = p[l2NormSquaredMaxIndex];
            // Java: p[l2NormSquaredMaxIndex] = tmp2;
        }
        // Java: super.performHouseholderReflection(minor, qrt);
    }

    // Java: RealMatrix getP()
    pub fn getP(&self) -> Box<dyn std::any::Any> {
        // Java: if (cachedP == null)
        {
            // Java: int n = p.length;
            // Java: cachedP = MatrixUtils.createRealMatrix(n,n);
            // Java: for (int i = 0; i < n; i++)
            {
                // Java: cachedP.setEntry(p[i], i, 1);
            }
        }
        // Java: return cachedP ;
        todo!()
    }

    // Java: int getRank(final double dropThreshold)
    pub fn getRank(&self) -> i32 {
        // Java: RealMatrix r    = getR();
        // Java: int rows        = r.getRowDimension();
        // Java: int columns     = r.getColumnDimension();
        // Java: int rank        = 1;
        // Java: double lastNorm = r.getFrobeniusNorm();
        // Java: double rNorm    = lastNorm;
        // Java: while (rank < FastMath.min(rows, columns))
        {
            // Java: double thisNorm = r.getSubMatrix(rank, rows - 1, rank, columns - 1).getFrobeniusNorm();
            // Java: if (thisNorm == 0 || (thisNorm / lastNorm) * rNorm < dropThreshold)
            {
                // Java: break;
            }
            // Java: lastNorm = thisNorm;
            // Java: rank++;
        }
        // Java: return rank;
        todo!()
    }

    // Java: DecompositionSolver getSolver()
    pub fn getSolver(&self) -> Box<dyn std::any::Any> {
        // Java: return new Solver(super.getSolver(), this.getP());
        todo!()
    }

}

// Java: public class Solver
impl Solver {
    // Java: Solver(final DecompositionSolver upper, final RealMatrix p)
    pub fn Solver(&self) {
            // Java: this.upper = upper;
            // Java: this.p     = p;
    }

    // Java: boolean isNonSingular()
    pub fn isNonSingular(&self) -> bool {
            // Java: return upper.isNonSingular();
        todo!()
    }

    // Java: RealVector solve(RealVector b)
    pub fn solve(&self) -> Box<dyn std::any::Any> {
            // Java: return p.operate(upper.solve(b));
        todo!()
    }

    // Java: RealMatrix solve(RealMatrix b)
    pub fn solve_2(&self) -> Box<dyn std::any::Any> {
            // Java: return p.multiply(upper.solve(b));
        todo!()
    }

    // Java: RealMatrix getInverse()
    pub fn getInverse(&self) -> Box<dyn std::any::Any> {
            // Java: return solve(MatrixUtils.createRealIdentityMatrix(p.getRowDimension()));
        todo!()
    }

}
