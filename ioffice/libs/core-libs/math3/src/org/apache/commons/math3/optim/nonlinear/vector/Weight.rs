// Java: import org.apache.commons.math3.optim.OptimizationData
use crate::org::apache::commons::math3::optim::OptimizationData;
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
    pub fn new_from_array(weight: Vec<f64>) -> Self {
        // Java: weightMatrix = new DiagonalMatrix(weight);
        Weight { weightMatrix: Box::new(weight) }
    }

    // Java: Weight(RealMatrix weight)
    pub fn new_from_matrix(weight: Box<dyn std::any::Any>) -> Self {
        // Java: if (weight.getColumnDimension() != weight.getRowDimension())
        {
            // Java: throw new NonSquareMatrixException(weight.getColumnDimension(),
                                               // Java: weight.getRowDimension());
        }
        // Java: weightMatrix = weight.copy();
        Weight { weightMatrix: weight }
    }

    // Java: RealMatrix getWeight()
    pub fn getWeight(&self) -> &dyn std::any::Any {
        // Java: return weightMatrix.copy();
        &*self.weightMatrix
    }

}
