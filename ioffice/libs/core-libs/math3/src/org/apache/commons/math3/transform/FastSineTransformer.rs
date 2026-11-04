// Java: import java.io.Serializable
// Java: import org.apache.commons.math3.analysis.FunctionUtils
use crate::org::apache::commons::math3::analysis::FunctionUtils;
// Java: import org.apache.commons.math3.analysis.UnivariateFunction
use crate::org::apache::commons::math3::analysis::UnivariateFunction;
// Java: import org.apache.commons.math3.complex.Complex
use crate::org::apache::commons::math3::complex::Complex;
// Java: import org.apache.commons.math3.exception.MathIllegalArgumentException
use crate::org::apache::commons::math3::exception::MathIllegalArgumentException;
// Java: import org.apache.commons.math3.exception.util.LocalizedFormats
use crate::org::apache::commons::math3::exception::util::LocalizedFormats;
// Java: import org.apache.commons.math3.util.ArithmeticUtils
use crate::org::apache::commons::math3::util::ArithmeticUtils;
// Java: import org.apache.commons.math3.util.FastMath
use crate::org::apache::commons::math3::util::FastMath;

// Java field: long serialVersionUID

// Java: public class FastSineTransformer implements RealTransformer, Serializable
pub struct FastSineTransformer {
    // Java field: DstNormalization normalization
    normalization: Box<dyn std::any::Any>,
}

// Java: public class FastSineTransformer implements RealTransformer, Serializable
impl FastSineTransformer {
    // Java: FastSineTransformer(final DstNormalization normalization)
    pub fn FastSineTransformer(&self) {
        // Java: this.normalization = normalization;
    }

    // Java: double[] transform(final double[] f, final TransformType type)
    pub fn transform(&self) -> Vec<f64> {
        // Java: if (normalization == DstNormalization.ORTHOGONAL_DST_I)
        {
            // Java: final double s = FastMath.sqrt(2.0 / f.length);
            // Java: return TransformUtils.scaleArray(fst(f), s);
        }
        // Java: if (type == TransformType.FORWARD)
        {
            // Java: return fst(f);
        }
        // Java: final double s = 2.0 / f.length;
        // Java: return TransformUtils.scaleArray(fst(f), s);
        todo!()
    }

    // Java: double[] transform()
    pub fn transform_2(&self) -> Vec<f64> {
        todo!()
    }

    // Java: double[] fst(double[] f)
    pub fn fst(&self) -> Vec<f64> {
        // Java: final double[] transformed = new double[f.length];
        // Java: if (!ArithmeticUtils.isPowerOfTwo(f.length))
        {
            // Java: throw new MathIllegalArgumentException(
                    // Java: LocalizedFormats.NOT_POWER_OF_TWO_CONSIDER_PADDING,
                    // Java: Integer.valueOf(f.length));
        }
        // Java: if (f[0] != 0.0)
        {
            // Java: throw new MathIllegalArgumentException(
                    // Java: LocalizedFormats.FIRST_ELEMENT_NOT_ZERO,
                    // Java: Double.valueOf(f[0]));
        }
        // Java: final int n = f.length;
        // Java: if (n == 1) {       // trivial case
            // Java: transformed[0] = 0.0;
            // Java: return transformed;
        // Java: // construct a new array and perform FFT on it
        // Java: final double[] x = new double[n];
        // Java: x[0] = 0.0;
        // Java: x[n >> 1] = 2.0 * f[n >> 1];
        // Java: for (int i = 1; i < (n >> 1); i++)
        {
            // Java: final double a = FastMath.sin(i * FastMath.PI / n) * (f[i] + f[n - i]);
            // Java: final double b = 0.5 * (f[i] - f[n - i]);
            // Java: x[i]     = a + b;
            // Java: x[n - i] = a - b;
        }
        // Java: FastFourierTransformer transformer;
        // Java: transformer = new FastFourierTransformer(DftNormalization.STANDARD);
        // Java: Complex[] y = transformer.transform(x, TransformType.FORWARD);
        // Java: // reconstruct the FST result for the original array
        // Java: transformed[0] = 0.0;
        // Java: transformed[1] = 0.5 * y[0].getReal();
        // Java: for (int i = 1; i < (n >> 1); i++)
        {
            // Java: transformed[2 * i]     = -y[i].getImaginary();
            // Java: transformed[2 * i + 1] = y[i].getReal() + transformed[2 * i - 1];
        }
        // Java: return transformed;
        todo!()
    }

}


