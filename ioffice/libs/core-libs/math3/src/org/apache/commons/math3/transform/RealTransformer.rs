// Java: import org.apache.commons.math3.analysis.UnivariateFunction
use crate::org::apache::commons::math3::analysis::UnivariateFunction;
// Java: import org.apache.commons.math3.exception.MathIllegalArgumentException
use crate::org::apache::commons::math3::exception::MathIllegalArgumentException;
// Java: import org.apache.commons.math3.exception.NonMonotonicSequenceException
use crate::org::apache::commons::math3::exception::NonMonotonicSequenceException;
// Java: import org.apache.commons.math3.exception.NotStrictlyPositiveException
use crate::org::apache::commons::math3::exception::NotStrictlyPositiveException;

// Java: public interface since
pub trait since {
}

// Java field: throws NonMonotonicSequenceException, NotStrictlyPositiveException, MathIllegalArgumentException
// Java: public interface RealTransformer
pub trait RealTransformer {
    // Java: abstract double[] transform(double[] f, TransformType type)
    fn transform(&self) -> Vec<f64>;

    // Java: abstract double[] transform()
    fn transform_2(&self) -> Vec<f64>;

}
