// Java: import java.util.Arrays
// Java: import java.util.HashMap
// Java: import java.util.Map
// Java: import org.apache.commons.math3.Field
use crate::org::apache::commons::math3::Field;
// Java: import org.apache.commons.math3.RealFieldElement
use crate::org::apache::commons::math3::RealFieldElement;
// Java: import org.apache.commons.math3.linear.Array2DRowFieldMatrix
use crate::org::apache::commons::math3::linear::Array2DRowFieldMatrix;
// Java: import org.apache.commons.math3.linear.ArrayFieldVector
use crate::org::apache::commons::math3::linear::ArrayFieldVector;
// Java: import org.apache.commons.math3.linear.FieldDecompositionSolver
use crate::org::apache::commons::math3::linear::FieldDecompositionSolver;
// Java: import org.apache.commons.math3.linear.FieldLUDecomposition
use crate::org::apache::commons::math3::linear::FieldLUDecomposition;
// Java: import org.apache.commons.math3.linear.FieldMatrix
use crate::org::apache::commons::math3::linear::FieldMatrix;
// Java: import org.apache.commons.math3.util.MathArrays
use crate::org::apache::commons::math3::util::MathArrays;

// Java: public class is
pub struct is {
}

// Java: public class handles
pub struct handles {
}

// Java: public class AdamsNordsieckFieldTransformer
pub struct AdamsNordsieckFieldTransformer {
    // Java field: AdamsNordsieckFieldTransformer<? extends RealFieldElement<?>>>> CACHE
    CACHE: Option<Box<dyn std::any::Any>>,
    // Java field: Field<T> field
    pub(crate) field: Box<dyn std::any::Any>,
    // Java field: Array2DRowFieldMatrix<T> update
    pub(crate) update: Box<dyn std::any::Any>,
    // Java field: T[] c1
    pub(crate) c1: Box<dyn std::any::Any>,
}

// Java: public class is
impl is {
}

// Java: public class handles
impl handles {
}

// Java: public class AdamsNordsieckFieldTransformer
impl AdamsNordsieckFieldTransformer {
    // Java: AdamsNordsieckFieldTransformer(final Field<T> field, final int n)
    pub fn AdamsNordsieckFieldTransformer(&self) {
        // Java: this.field = field;
        // Java: final int rows = n - 1;
        // Java: // compute coefficients
        // Java: FieldMatrix<T> bigP = buildP(rows);
        // Java: FieldDecompositionSolver<T> pSolver =
            // Java: new FieldLUDecomposition<T>(bigP).getSolver();
        // Java: T[] u = MathArrays.buildArray(field, rows);
        // Java: Arrays.fill(u, field.getOne());
        // Java: c1 = pSolver.solve(new ArrayFieldVector<T>(u, false)).toArray();
        // Java: // update coefficients are computed by combining transform from
        // Java: // Nordsieck to multistep, then shifting rows to represent step advance
        // Java: // then applying inverse transform
        // Java: T[][] shiftedP = bigP.getData();
        // Java: for (int i = shiftedP.length - 1; i > 0; --i)
        {
            // Java: // shift rows
            // Java: shiftedP[i] = shiftedP[i - 1];
        }
        // Java: shiftedP[0] = MathArrays.buildArray(field, rows);
        // Java: Arrays.fill(shiftedP[0], field.getZero());
        // Java: update = new Array2DRowFieldMatrix<T>(pSolver.solve(new Array2DRowFieldMatrix<T>(shiftedP, false)).getData());
    }

    // Java: FieldMatrix<T> buildP(final int rows)
    pub fn buildP(&self) -> Box<dyn std::any::Any> {
        // Java: final T[][] pData = MathArrays.buildArray(field, rows, rows);
        // Java: for (int i = 1; i <= pData.length; ++i)
        {
            // Java: // build the P matrix elements from Taylor series formulas
            // Java: final T[] pI = pData[i - 1];
            // Java: final int factor = -i;
            // Java: T aj = field.getZero().add(factor);
            // Java: for (int j = 1; j <= pI.length; ++j)
            {
                // Java: pI[j - 1] = aj.multiply(j + 1);
                // Java: aj = aj.multiply(factor);
            }
        }
        // Java: return new Array2DRowFieldMatrix<T>(pData, false);
        todo!()
    }

    // Java: Array2DRowFieldMatrix<T> initializeHighOrderDerivatives()
    pub fn initializeHighOrderDerivatives(&self) -> Box<dyn std::any::Any> {
        todo!()
    }

    // Java: Array2DRowFieldMatrix<T> updateHighOrderDerivativesPhase1(final Array2DRowFieldMatrix<T> highOrder)
    pub fn updateHighOrderDerivativesPhase1(&self) -> Box<dyn std::any::Any> {
        // Java: return update.multiply(highOrder);
        todo!()
    }

    // Java: void updateHighOrderDerivativesPhase2()
    pub fn updateHighOrderDerivativesPhase2(&self) {
    }

}
