// Java: import java.io.Serializable
// Java: import java.lang.reflect.Array
// Java: import org.apache.commons.math3.analysis.FunctionUtils
use crate::org::apache::commons::math3::analysis::FunctionUtils;
// Java: import org.apache.commons.math3.analysis.UnivariateFunction
use crate::org::apache::commons::math3::analysis::UnivariateFunction;
// Java: import org.apache.commons.math3.complex.Complex
use crate::org::apache::commons::math3::complex::Complex;
// Java: import org.apache.commons.math3.exception.DimensionMismatchException
use crate::org::apache::commons::math3::exception::DimensionMismatchException;
// Java: import org.apache.commons.math3.exception.MathIllegalArgumentException
use crate::org::apache::commons::math3::exception::MathIllegalArgumentException;
// Java: import org.apache.commons.math3.exception.MathIllegalStateException
use crate::org::apache::commons::math3::exception::MathIllegalStateException;
// Java: import org.apache.commons.math3.exception.util.LocalizedFormats
use crate::org::apache::commons::math3::exception::util::LocalizedFormats;
// Java: import org.apache.commons.math3.util.ArithmeticUtils
use crate::org::apache::commons::math3::util::ArithmeticUtils;
// Java: import org.apache.commons.math3.util.FastMath
use crate::org::apache::commons::math3::util::FastMath;
// Java: import org.apache.commons.math3.util.MathArrays
use crate::org::apache::commons::math3::util::MathArrays;

// Java field: long serialVersionUID
// Java field: double[] W_SUB_N_R
static W_SUB_N_R: std::sync::OnceLock<Vec<f64>> = std::sync::OnceLock::new();
// Java field: double[] W_SUB_N_I
static W_SUB_N_I: std::sync::OnceLock<Vec<f64>> = std::sync::OnceLock::new();

// Java: public class FastFourierTransformer implements Serializable
pub struct FastFourierTransformer {
    // Java field: DftNormalization normalization
    normalization: Box<dyn std::any::Any>,
}

// Java: public class MultiDimensionalComplexMatrix
pub struct MultiDimensionalComplexMatrix {
}

// Java: public class FastFourierTransformer implements Serializable
impl FastFourierTransformer {
    // Java: FastFourierTransformer(final DftNormalization normalization)
    pub fn FastFourierTransformer(&self) {
        // Java: this.normalization = normalization;
    }

    // Java: static void bitReversalShuffle2(double[] a, double[] b)
    pub fn bitReversalShuffle2(&self) {
        // Java: final int n = a.length;
        // Java: assert b.length == n;
        // Java: final int halfOfN = n >> 1;
        // Java: int j = 0;
        // Java: for (int i = 0; i < n; i++)
        {
            // Java: if (i < j)
            {
                // Java: // swap indices i & j
                // Java: double temp = a[i];
                // Java: a[i] = a[j];
                // Java: a[j] = temp;
                // Java: temp = b[i];
                // Java: b[i] = b[j];
                // Java: b[j] = temp;
            }
            // Java: int k = halfOfN;
            // Java: while (k <= j && k > 0)
            {
                // Java: j -= k;
                // Java: k >>= 1;
            }
            // Java: j += k;
        }
    }

    // Java: static void normalizeTransformedData()
    pub fn normalizeTransformedData(&self) {
    }

    // Java: static void transformInPlace()
    pub fn transformInPlace(&self) {
    }

    // Java: Complex[] transform(final double[] f, final TransformType type)
    pub fn transform(&self) -> Box<dyn std::any::Any> {
        // Java: final double[][] dataRI = new double[][]
        {
            // Java: MathArrays.copyOf(f, f.length), new double[f.length]
        }
        // Java: transformInPlace(dataRI, normalization, type);
        // Java: return TransformUtils.createComplexArray(dataRI);
        todo!()
    }

    // Java: Complex[] transform()
    pub fn transform_2(&self) -> Vec<Box<dyn std::any::Any>> {
        todo!()
    }

    // Java: Complex[] transform(final Complex[] f, final TransformType type)
    pub fn transform_3(&self) -> Box<dyn std::any::Any> {
        // Java: final double[][] dataRI = TransformUtils.createRealImaginaryArray(f);
        // Java: transformInPlace(dataRI, normalization, type);
        // Java: return TransformUtils.createComplexArray(dataRI);
        todo!()
    }

    // Java: Object mdfft(Object mdca, TransformType type)
    pub fn mdfft(&self) -> Box<dyn std::any::Any> {
        // Java: MultiDimensionalComplexMatrix mdcm = (MultiDimensionalComplexMatrix)
                // Java: new MultiDimensionalComplexMatrix(mdca).clone();
        // Java: int[] dimensionSize = mdcm.getDimensionSizes();
        // Java: //cycle through each dimension
        // Java: for (int i = 0; i < dimensionSize.length; i++)
        {
            // Java: mdfft(mdcm, type, i, new int[0]);
        }
        // Java: return mdcm.getArray();
        todo!()
    }

    // Java: void mdfft()
    pub fn mdfft_2(&self) {
    }

}

// Java: public class MultiDimensionalComplexMatrix
impl MultiDimensionalComplexMatrix {
}

