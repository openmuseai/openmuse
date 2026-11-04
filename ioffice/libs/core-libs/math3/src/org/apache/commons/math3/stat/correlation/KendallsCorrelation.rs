// Java: import org.apache.commons.math3.exception.DimensionMismatchException
use crate::org::apache::commons::math3::exception::DimensionMismatchException;
// Java: import org.apache.commons.math3.linear.BlockRealMatrix
use crate::org::apache::commons::math3::linear::BlockRealMatrix;
// Java: import org.apache.commons.math3.linear.MatrixUtils
use crate::org::apache::commons::math3::linear::MatrixUtils;
// Java: import org.apache.commons.math3.linear.RealMatrix
use crate::org::apache::commons::math3::linear::RealMatrix;
// Java: import org.apache.commons.math3.util.FastMath
use crate::org::apache::commons::math3::util::FastMath;
// Java: import org.apache.commons.math3.util.Pair
use crate::org::apache::commons::math3::util::Pair;
// Java: import java.util.Arrays
// Java: import java.util.Comparator

// Java: public class KendallsCorrelation
pub struct KendallsCorrelation {
    // Java field: RealMatrix correlationMatrix
    pub(crate) correlationMatrix: Box<dyn std::any::Any>,
}

// Java: public class KendallsCorrelation
impl KendallsCorrelation {
    // Java: KendallsCorrelation()
    pub fn KendallsCorrelation(&self) {
        // Java: correlationMatrix = null;
    }

    // Java: KendallsCorrelation(double[][] data)
    pub fn KendallsCorrelation_2(&self) {
        // Java: this(MatrixUtils.createRealMatrix(data));
    }

    // Java: KendallsCorrelation(RealMatrix matrix)
    pub fn KendallsCorrelation_3(&self) {
        // Java: correlationMatrix = computeCorrelationMatrix(matrix);
    }

    // Java: RealMatrix getCorrelationMatrix()
    pub fn getCorrelationMatrix(&self) -> Box<dyn std::any::Any> {
        // Java: return correlationMatrix;
        todo!()
    }

    // Java: RealMatrix computeCorrelationMatrix(final RealMatrix matrix)
    pub fn computeCorrelationMatrix(&self) -> Box<dyn std::any::Any> {
        // Java: int nVars = matrix.getColumnDimension();
        // Java: RealMatrix outMatrix = new BlockRealMatrix(nVars, nVars);
        // Java: for (int i = 0; i < nVars; i++)
        {
            // Java: for (int j = 0; j < i; j++)
            {
                // Java: double corr = correlation(matrix.getColumn(i), matrix.getColumn(j));
                // Java: outMatrix.setEntry(i, j, corr);
                // Java: outMatrix.setEntry(j, i, corr);
            }
            // Java: outMatrix.setEntry(i, i, 1d);
        }
        // Java: return outMatrix;
        todo!()
    }

    // Java: RealMatrix computeCorrelationMatrix(final double[][] matrix)
    pub fn computeCorrelationMatrix_2(&self) -> Box<dyn std::any::Any> {
       // Java: return computeCorrelationMatrix(new BlockRealMatrix(matrix));
        todo!()
    }

    // Java: double correlation(final double[] xArray, final double[] yArray)
    pub fn correlation(&self) -> f64 {
        todo!()
    }

    // Java: static long sum(long n)
    pub fn sum(&self) -> i64 {
        // Java: return n * (n + 1) / 2l;
        todo!()
    }

}
