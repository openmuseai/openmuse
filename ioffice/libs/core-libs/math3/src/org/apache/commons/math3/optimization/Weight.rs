// Java: import org.apache.commons.math3.linear.RealMatrix
use crate::org::apache::commons::math3::linear::RealMatrix;
// Java: import org.apache.commons.math3.linear.DiagonalMatrix
use crate::org::apache::commons::math3::linear::DiagonalMatrix;
// Java: import org.apache.commons.math3.linear.NonSquareMatrixException
use crate::org::apache::commons::math3::linear::NonSquareMatrixException;

// Java: public class Weight implements OptimizationData
pub struct Weight {
    // Java field: RealMatrix weightMatrix
    pub(crate) weightMatrix: Box<dyn std::any::Any>,
}

// Java: public class Weight implements OptimizationData
impl Weight {
    // Java: Weight(double[] weight)
    pub fn Weight(&self) {
        // Java: weightMatrix = new DiagonalMatrix(weight);
    }

    // Java: Weight(RealMatrix weight)
    pub fn Weight_2(&self) {
        // Java: if (weight.getColumnDimension() != weight.getRowDimension())
        {
            // Java: throw new NonSquareMatrixException(weight.getColumnDimension(),
                                               // Java: weight.getRowDimension());
        }
        // Java: weightMatrix = weight.copy();
    }

    // Java: RealMatrix getWeight()
    pub fn getWeight(&self) -> Box<dyn std::any::Any> {
        // Java: return weightMatrix.copy();
        todo!()
    }

}

