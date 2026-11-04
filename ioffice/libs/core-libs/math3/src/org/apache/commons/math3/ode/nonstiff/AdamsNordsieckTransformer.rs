// Java: import java.util.Arrays
// Java: import java.util.HashMap
// Java: import java.util.Map
// Java: import org.apache.commons.math3.fraction.BigFraction
use crate::org::apache::commons::math3::fraction::BigFraction;
// Java: import org.apache.commons.math3.linear.Array2DRowFieldMatrix
use crate::org::apache::commons::math3::linear::Array2DRowFieldMatrix;
// Java: import org.apache.commons.math3.linear.Array2DRowRealMatrix
use crate::org::apache::commons::math3::linear::Array2DRowRealMatrix;
// Java: import org.apache.commons.math3.linear.ArrayFieldVector
use crate::org::apache::commons::math3::linear::ArrayFieldVector;
// Java: import org.apache.commons.math3.linear.FieldDecompositionSolver
use crate::org::apache::commons::math3::linear::FieldDecompositionSolver;
// Java: import org.apache.commons.math3.linear.FieldLUDecomposition
use crate::org::apache::commons::math3::linear::FieldLUDecomposition;
// Java: import org.apache.commons.math3.linear.FieldMatrix
use crate::org::apache::commons::math3::linear::FieldMatrix;
// Java: import org.apache.commons.math3.linear.MatrixUtils
use crate::org::apache::commons::math3::linear::MatrixUtils;
// Java: import org.apache.commons.math3.linear.QRDecomposition
use crate::org::apache::commons::math3::linear::QRDecomposition;
// Java: import org.apache.commons.math3.linear.RealMatrix
use crate::org::apache::commons::math3::linear::RealMatrix;

// Java field: Map<Integer, AdamsNordsieckTransformer> CACHE
static CACHE: std::sync::OnceLock<std::collections::HashMap<i32, Box<dyn std::any::Any + Send + Sync>>> = std::sync::OnceLock::new();

// Java: public class is
pub struct is {
}

// Java: public class handles
pub struct handles {
}

// Java: public class AdamsNordsieckTransformer
pub struct AdamsNordsieckTransformer {
    // Java field: Array2DRowRealMatrix update
    pub(crate) update: Box<dyn std::any::Any>,
    // Java field: double[] c1
    pub(crate) c1: Box<dyn std::any::Any>,
}

// Java: public class is
impl is {
}

// Java: public class handles
impl handles {
}

// Java: public class AdamsNordsieckTransformer
impl AdamsNordsieckTransformer {
    // Java: AdamsNordsieckTransformer(final int n)
    pub fn AdamsNordsieckTransformer(&self) {
        // Java: final int rows = n - 1;
        // Java: // compute exact coefficients
        // Java: FieldMatrix<BigFraction> bigP = buildP(rows);
        // Java: FieldDecompositionSolver<BigFraction> pSolver =
            // Java: new FieldLUDecomposition<BigFraction>(bigP).getSolver();
        // Java: BigFraction[] u = new BigFraction[rows];
        // Java: Arrays.fill(u, BigFraction.ONE);
        // Java: BigFraction[] bigC1 = pSolver.solve(new ArrayFieldVector<BigFraction>(u, false)).toArray();
        // Java: // update coefficients are computed by combining transform from
        // Java: // Nordsieck to multistep, then shifting rows to represent step advance
        // Java: // then applying inverse transform
        // Java: BigFraction[][] shiftedP = bigP.getData();
        // Java: for (int i = shiftedP.length - 1; i > 0; --i)
        {
            // Java: // shift rows
            // Java: shiftedP[i] = shiftedP[i - 1];
        }
        // Java: shiftedP[0] = new BigFraction[rows];
        // Java: Arrays.fill(shiftedP[0], BigFraction.ZERO);
        // Java: FieldMatrix<BigFraction> bigMSupdate =
            // Java: pSolver.solve(new Array2DRowFieldMatrix<BigFraction>(shiftedP, false));
        // Java: // convert coefficients to double
        // Java: update         = MatrixUtils.bigFractionMatrixToRealMatrix(bigMSupdate);
        // Java: c1             = new double[rows];
        // Java: for (int i = 0; i < rows; ++i)
        {
            // Java: c1[i] = bigC1[i].doubleValue();
        }
    }

    // Java: static AdamsNordsieckTransformer getInstance(final int nSteps)
    pub fn getInstance(&self) -> Box<dyn std::any::Any> {
        // Java: synchronized(CACHE)
        {
            // Java: AdamsNordsieckTransformer t = CACHE.get(nSteps);
            // Java: if (t == null)
            {
                // Java: t = new AdamsNordsieckTransformer(nSteps);
                // Java: CACHE.put(nSteps, t);
            }
            // Java: return t;
        }
        todo!()
    }

    // Java: int getNSteps()
    pub fn getNSteps(&self) -> i32 {
        // Java: return c1.length;
        todo!()
    }

    // Java: FieldMatrix<BigFraction> buildP(final int rows)
    pub fn buildP(&self) -> Box<dyn std::any::Any> {
        // Java: final BigFraction[][] pData = new BigFraction[rows][rows];
        // Java: for (int i = 1; i <= pData.length; ++i)
        {
            // Java: // build the P matrix elements from Taylor series formulas
            // Java: final BigFraction[] pI = pData[i - 1];
            // Java: final int factor = -i;
            // Java: int aj = factor;
            // Java: for (int j = 1; j <= pI.length; ++j)
            {
                // Java: pI[j - 1] = new BigFraction(aj * (j + 1));
                // Java: aj *= factor;
            }
        }
        // Java: return new Array2DRowFieldMatrix<BigFraction>(pData, false);
        todo!()
    }

    // Java: Array2DRowRealMatrix initializeHighOrderDerivatives()
    pub fn initializeHighOrderDerivatives(&self) -> Box<dyn std::any::Any> {
        todo!()
    }

    // Java: Array2DRowRealMatrix updateHighOrderDerivativesPhase1(final Array2DRowRealMatrix highOrder)
    pub fn updateHighOrderDerivativesPhase1(&self) -> Box<dyn std::any::Any> {
        // Java: return update.multiply(highOrder);
        todo!()
    }

    // Java: void updateHighOrderDerivativesPhase2()
    pub fn updateHighOrderDerivativesPhase2(&self) {
    }

}
