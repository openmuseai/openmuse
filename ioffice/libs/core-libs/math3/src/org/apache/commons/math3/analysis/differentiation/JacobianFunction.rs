// Java: import org.apache.commons.math3.analysis.MultivariateMatrixFunction
use crate::org::apache::commons::math3::analysis::MultivariateMatrixFunction;

// Java: public class JacobianFunction implements MultivariateMatrixFunction
pub struct JacobianFunction {
    // Java field: MultivariateDifferentiableVectorFunction f
    f: Box<dyn std::any::Any>,
}

// Java: public class JacobianFunction implements MultivariateMatrixFunction
impl JacobianFunction {
    // Java: JacobianFunction(final MultivariateDifferentiableVectorFunction f)
    pub fn JacobianFunction(&self) {
        // Java: this.f = f;
    }

    // Java: double[][] value(double[] point)
    pub fn value(&self) -> Vec<f64> {
        // Java: // set up parameters
        // Java: final DerivativeStructure[] dsX = new DerivativeStructure[point.length];
        // Java: for (int i = 0; i < point.length; ++i)
        {
            // Java: dsX[i] = new DerivativeStructure(point.length, 1, i, point[i]);
        }
        // Java: // compute the derivatives
        // Java: final DerivativeStructure[] dsY = f.value(dsX);
        // Java: // extract the Jacobian
        // Java: final double[][] y = new double[dsY.length][point.length];
        // Java: final int[] orders = new int[point.length];
        // Java: for (int i = 0; i < dsY.length; ++i)
        {
            // Java: for (int j = 0; j < point.length; ++j)
            {
                // Java: orders[j] = 1;
                // Java: y[i][j] = dsY[i].getPartialDerivative(orders);
                // Java: orders[j] = 0;
            }
        }
        // Java: return y;
        todo!()
    }

}

