// Java: import java.util.Arrays
// Java: import org.apache.commons.math3.complex.Complex
use crate::org::apache::commons::math3::complex::Complex;
// Java: import org.apache.commons.math3.exception.DimensionMismatchException
use crate::org::apache::commons::math3::exception::DimensionMismatchException;
// Java: import org.apache.commons.math3.exception.MathIllegalArgumentException
use crate::org::apache::commons::math3::exception::MathIllegalArgumentException;
// Java: import org.apache.commons.math3.exception.util.LocalizedFormats
use crate::org::apache::commons::math3::exception::util::LocalizedFormats;

// Java: public class TransformUtils
pub struct TransformUtils {
}

// Java: public class TransformUtils
impl TransformUtils {
    // Java: TransformUtils()
    pub fn TransformUtils(&self) {
        // Java: super();
    }

    // Java: static double[] scaleArray(double[] f, double d)
    pub fn scaleArray(&self) -> Vec<f64> {
        // Java: for (int i = 0; i < f.length; i++)
        {
            // Java: f[i] *= d;
        }
        // Java: return f;
        todo!()
    }

    // Java: static Complex[] scaleArray(Complex[] f, double d)
    pub fn scaleArray_2(&self) -> Box<dyn std::any::Any> {
        // Java: for (int i = 0; i < f.length; i++)
        {
            // Java: f[i] = new Complex(d * f[i].getReal(), d * f[i].getImaginary());
        }
        // Java: return f;
        todo!()
    }

    // Java: static double[][] createRealImaginaryArray(final Complex[] dataC)
    pub fn createRealImaginaryArray(&self) -> Vec<f64> {
        // Java: final double[][] dataRI = new double[2][dataC.length];
        // Java: final double[] dataR = dataRI[0];
        // Java: final double[] dataI = dataRI[1];
        // Java: for (int i = 0; i < dataC.length; i++)
        {
            // Java: final Complex c = dataC[i];
            // Java: dataR[i] = c.getReal();
            // Java: dataI[i] = c.getImaginary();
        }
        // Java: return dataRI;
        todo!()
    }

    // Java: static Complex[] createComplexArray(final double[][] dataRI)
    pub fn createComplexArray(&self) -> Box<dyn std::any::Any> {
        todo!()
    }

    // Java: static int exactLog2(final int n)
    pub fn exactLog2(&self) -> i32 {
        todo!()
    }

}
