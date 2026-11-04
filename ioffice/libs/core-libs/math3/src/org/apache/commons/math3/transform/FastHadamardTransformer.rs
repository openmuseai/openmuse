// Java: import java.io.Serializable
// Java: import org.apache.commons.math3.analysis.FunctionUtils
use crate::org::apache::commons::math3::analysis::FunctionUtils;
// Java: import org.apache.commons.math3.analysis.UnivariateFunction
use crate::org::apache::commons::math3::analysis::UnivariateFunction;
// Java: import org.apache.commons.math3.exception.MathIllegalArgumentException
use crate::org::apache::commons::math3::exception::MathIllegalArgumentException;
// Java: import org.apache.commons.math3.exception.util.LocalizedFormats
use crate::org::apache::commons::math3::exception::util::LocalizedFormats;
// Java: import org.apache.commons.math3.util.ArithmeticUtils
use crate::org::apache::commons::math3::util::ArithmeticUtils;

// Java field: long serialVersionUID

// Java: public class FastHadamardTransformer implements RealTransformer, Serializable
pub struct FastHadamardTransformer {
}

// Java: public class FastHadamardTransformer implements RealTransformer, Serializable
impl FastHadamardTransformer {
    // Java: double[] transform(final double[] f, final TransformType type)
    pub fn transform(&self) -> Vec<f64> {
        // Java: if (type == TransformType.FORWARD)
        {
            // Java: return fht(f);
        }
        // Java: return TransformUtils.scaleArray(fht(f), 1.0 / f.length);
        todo!()
    }

    // Java: double[] transform()
    pub fn transform_2(&self) -> Vec<f64> {
        todo!()
    }

    // Java: int[] transform(final int[] f)
    pub fn transform_3(&self) -> Vec<i32> {
        // Java: return fht(f);
        todo!()
    }

    // Java: double[] fht(double[] x)
    pub fn fht(&self) -> Vec<f64> {
        // Java: final int n     = x.length;
        // Java: final int halfN = n / 2;
        // Java: if (!ArithmeticUtils.isPowerOfTwo(n))
        {
            // Java: throw new MathIllegalArgumentException(
                    // Java: LocalizedFormats.NOT_POWER_OF_TWO,
                    // Java: Integer.valueOf(n));
        }
        // Java: /*
         // Java: * Instead of creating a matrix with p+1 columns and n rows, we use two
         // Java: * one dimension arrays which we are used in an alternating way.
         // Java: */
        // Java: double[] yPrevious = new double[n];
        // Java: double[] yCurrent  = x.clone();
        // Java: // iterate from left to right (column)
        // Java: for (int j = 1; j < n; j <<= 1)
        {
            // Java: // switch columns
            // Java: final double[] yTmp = yCurrent;
            // Java: yCurrent  = yPrevious;
            // Java: yPrevious = yTmp;
            // Java: // iterate from top to bottom (row)
            // Java: for (int i = 0; i < halfN; ++i)
            {
                // Java: // Dtop: the top part works with addition
                // Java: final int twoI = 2 * i;
                // Java: yCurrent[i] = yPrevious[twoI] + yPrevious[twoI + 1];
            }
            // Java: for (int i = halfN; i < n; ++i)
            {
                // Java: // Dbottom: the bottom part works with subtraction
                // Java: final int twoI = 2 * i;
                // Java: yCurrent[i] = yPrevious[twoI - n] - yPrevious[twoI - n + 1];
            }
        }
        // Java: return yCurrent;
        todo!()
    }

    // Java: int[] fht(int[] x)
    pub fn fht_2(&self) -> Vec<i32> {
        // Java: final int n     = x.length;
        // Java: final int halfN = n / 2;
        // Java: if (!ArithmeticUtils.isPowerOfTwo(n))
        {
            // Java: throw new MathIllegalArgumentException(
                    // Java: LocalizedFormats.NOT_POWER_OF_TWO,
                    // Java: Integer.valueOf(n));
        }
        // Java: /*
         // Java: * Instead of creating a matrix with p+1 columns and n rows, we use two
         // Java: * one dimension arrays which we are used in an alternating way.
         // Java: */
        // Java: int[] yPrevious = new int[n];
        // Java: int[] yCurrent  = x.clone();
        // Java: // iterate from left to right (column)
        // Java: for (int j = 1; j < n; j <<= 1)
        {
            // Java: // switch columns
            // Java: final int[] yTmp = yCurrent;
            // Java: yCurrent  = yPrevious;
            // Java: yPrevious = yTmp;
            // Java: // iterate from top to bottom (row)
            // Java: for (int i = 0; i < halfN; ++i)
            {
                // Java: // Dtop: the top part works with addition
                // Java: final int twoI = 2 * i;
                // Java: yCurrent[i] = yPrevious[twoI] + yPrevious[twoI + 1];
            }
            // Java: for (int i = halfN; i < n; ++i)
            {
                // Java: // Dbottom: the bottom part works with subtraction
                // Java: final int twoI = 2 * i;
                // Java: yCurrent[i] = yPrevious[twoI - n] - yPrevious[twoI - n + 1];
            }
        }
        // Java: // return the last computed output vector y
        // Java: return yCurrent;
        todo!()
    }

}


