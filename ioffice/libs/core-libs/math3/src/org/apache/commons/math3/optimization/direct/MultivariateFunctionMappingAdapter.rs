// Java: import org.apache.commons.math3.analysis.MultivariateFunction
use crate::org::apache::commons::math3::analysis::MultivariateFunction;
// Java: import org.apache.commons.math3.analysis.UnivariateFunction
use crate::org::apache::commons::math3::analysis::UnivariateFunction;
// Java: import org.apache.commons.math3.analysis.function.Logit
use crate::org::apache::commons::math3::analysis::function::Logit;
// Java: import org.apache.commons.math3.analysis.function.Sigmoid
use crate::org::apache::commons::math3::analysis::function::Sigmoid;
// Java: import org.apache.commons.math3.exception.DimensionMismatchException
use crate::org::apache::commons::math3::exception::DimensionMismatchException;
// Java: import org.apache.commons.math3.exception.NumberIsTooSmallException
use crate::org::apache::commons::math3::exception::NumberIsTooSmallException;
// Java: import org.apache.commons.math3.util.FastMath
use crate::org::apache::commons::math3::util::FastMath;
// Java: import org.apache.commons.math3.util.MathUtils
use crate::org::apache::commons::math3::util::MathUtils;

// Java: public interface Mapper
pub trait Mapper {
    // Java: abstract double unboundedToBounded(double y)
    fn unboundedToBounded(&self, y: f64) -> f64;
    // Java: abstract double boundedToUnbounded(double x)
    fn boundedToUnbounded(&self, x: f64) -> f64;
}

// Java: public class MultivariateFunctionMappingAdapter implements MultivariateFunction
pub struct MultivariateFunctionMappingAdapter {
    // Java field: MultivariateFunction bounded
    pub(crate) bounded: Box<dyn std::any::Any>,
    // Java field: Mapper[] mappers
    mappers: Vec<Box<dyn std::any::Any>>,
}

// Java: public class NoBoundsMapper
pub struct NoBoundsMapper {
}

// Java: public class LowerBoundMapper
pub struct LowerBoundMapper {
    // Java field: double lower
    pub(crate) lower: f64,
}

// Java: public class UpperBoundMapper
pub struct UpperBoundMapper {
    // Java field: double upper
    pub(crate) upper: f64,
}

// Java: public class LowerUpperBoundMapper
pub struct LowerUpperBoundMapper {
    // Java field: UnivariateFunction boundingFunction
    pub(crate) boundingFunction: Box<dyn std::any::Any>,
    // Java field: UnivariateFunction unboundingFunction
    pub(crate) unboundingFunction: Box<dyn std::any::Any>,
}

// Java: public class MultivariateFunctionMappingAdapter implements MultivariateFunction
impl MultivariateFunctionMappingAdapter {
    // Java: MultivariateFunctionMappingAdapter()
    pub fn MultivariateFunctionMappingAdapter(&self) {
    }

    // Java: double[] unboundedToBounded(double[] point)
    pub fn unboundedToBounded_2(&self) -> Vec<f64> {
        // Java: // map unbounded input point to bounded point
        // Java: final double[] mapped = new double[mappers.length];
        // Java: for (int i = 0; i < mappers.length; ++i)
        {
            // Java: mapped[i] = mappers[i].unboundedToBounded(point[i]);
        }
        // Java: return mapped;
        todo!()
    }

    // Java: double[] boundedToUnbounded(double[] point)
    pub fn boundedToUnbounded_array(&self) -> Vec<f64> {
        // Java: // map bounded input point to unbounded point
        // Java: final double[] mapped = new double[mappers.length];
        // Java: for (int i = 0; i < mappers.length; ++i)
        {
            // Java: mapped[i] = mappers[i].boundedToUnbounded(point[i]);
        }
        // Java: return mapped;
        todo!()
    }

    // Java: double value(double[] point)
    pub fn value(&self) -> f64 {
        // Java: return bounded.value(unboundedToBounded(point));
        todo!()
    }

}

// Java: public class NoBoundsMapper
impl NoBoundsMapper {
    // Java: double unboundedToBounded(final double y)
    pub fn unboundedToBounded_impl(&self) -> f64 {
            // Java: return y;
            todo!()
    }

    // Java: double boundedToUnbounded(final double x)
    pub fn boundedToUnbounded_impl(&self) -> f64 {
            // Java: return x;
            todo!()
    }

}

// Java: public class LowerBoundMapper
impl LowerBoundMapper {
    // Java: double unboundedToBounded(final double y)
    pub fn unboundedToBounded_impl_2(&self) -> f64 {
            // Java: return lower + FastMath.exp(y);
            todo!()
    }

    // Java: double boundedToUnbounded(final double x)
    pub fn boundedToUnbounded_impl_2(&self) -> f64 {
            // Java: return FastMath.log(x - lower);
            todo!()
    }

}

// Java: public class UpperBoundMapper
impl UpperBoundMapper {
    // Java: double unboundedToBounded(final double y)
    pub fn unboundedToBounded_impl_3(&self) -> f64 {
            // Java: return upper - FastMath.exp(-y);
            todo!()
    }

    // Java: double boundedToUnbounded(final double x)
    pub fn boundedToUnbounded_impl_3(&self) -> f64 {
            // Java: return -FastMath.log(upper - x);
            todo!()
    }

}

// Java: public class LowerUpperBoundMapper
impl LowerUpperBoundMapper {
    // Java: double unboundedToBounded(final double y)
    pub fn unboundedToBounded_impl_4(&self) -> f64 {
            // Java: return boundingFunction.value(y);
            todo!()
    }

    // Java: double boundedToUnbounded(final double x)
    pub fn boundedToUnbounded_impl_4(&self) -> f64 {
            // Java: return unboundingFunction.value(x);
            todo!()
    }

}

