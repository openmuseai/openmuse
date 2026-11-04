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

// Java: public class FastCosineTransformer implements RealTransformer, Serializable
pub struct FastCosineTransformer {
    // Java field: DctNormalization normalization
    normalization: Box<dyn std::any::Any>,
}

// Java: public class FastCosineTransformer implements RealTransformer, Serializable
impl FastCosineTransformer {
    // Java: FastCosineTransformer(final DctNormalization normalization)
    pub fn FastCosineTransformer(&self) {
        // Java: this.normalization = normalization;
    }

    // Java: double[] transform(final double[] f, final TransformType type)
    pub fn transform(&self) -> Vec<f64> {
        todo!()
    }

    // Java: double[] transform()
    pub fn transform_2(&self) -> Vec<f64> {
        todo!()
    }

    // Java: double[] fct(double[] f)
    pub fn fct(&self) -> Vec<f64> {
        todo!()
    }

}


