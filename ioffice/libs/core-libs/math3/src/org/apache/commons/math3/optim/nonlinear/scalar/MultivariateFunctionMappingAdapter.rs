#![allow(unused_imports, dead_code, non_snake_case, unused_variables, unused_mut, unused_assignments)]

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

use std::any::Any;

// Java field: MultivariateFunction bounded
// Java field: Mapper[] mappers

// Java: private interface Mapper
trait Mapper {
    // Java: double unboundedToBounded(double y)
    fn unboundedToBounded(&self, y: f64) -> f64;
    // Java: double boundedToUnbounded(double x)
    fn boundedToUnbounded(&self, x: f64) -> f64;
}

// Java: private static class NoBoundsMapper
struct NoBoundsMapper {}

impl Mapper for NoBoundsMapper {
    // Java: double unboundedToBounded(final double y)
    fn unboundedToBounded(&self, y: f64) -> f64 {
        // Java: return y;
        y
    }
    // Java: double boundedToUnbounded(final double x)
    fn boundedToUnbounded(&self, x: f64) -> f64 {
        // Java: return x;
        x
    }
}

// Java: private static class LowerBoundMapper
struct LowerBoundMapper {
    // Java field: double lower
    lower: f64,
}

impl Mapper for LowerBoundMapper {
    // Java: double unboundedToBounded(final double y)
    fn unboundedToBounded(&self, y: f64) -> f64 {
        // Java: return lower + FastMath.exp(y);
        self.lower + y.exp()
    }
    // Java: double boundedToUnbounded(final double x)
    fn boundedToUnbounded(&self, x: f64) -> f64 {
        // Java: return FastMath.log(x - lower);
        (x - self.lower).ln()
    }
}

// Java: private static class UpperBoundMapper
struct UpperBoundMapper {
    // Java field: double upper
    upper: f64,
}

impl Mapper for UpperBoundMapper {
    // Java: double unboundedToBounded(final double y)
    fn unboundedToBounded(&self, y: f64) -> f64 {
        // Java: return upper - FastMath.exp(-y);
        self.upper - (-y).exp()
    }
    // Java: double boundedToUnbounded(final double x)
    fn boundedToUnbounded(&self, x: f64) -> f64 {
        // Java: return -FastMath.log(upper - x);
        -(self.upper - x).ln()
    }
}

// Java: private static class LowerUpperBoundMapper
struct LowerUpperBoundMapper {
    // Java field: UnivariateFunction boundingFunction
    // Java field: UnivariateFunction unboundingFunction
    lower: f64,
    upper: f64,
}

impl Mapper for LowerUpperBoundMapper {
    // Java: double unboundedToBounded(final double y)
    fn unboundedToBounded(&self, y: f64) -> f64 {
        // Java: return boundingFunction.value(y);
        todo!()
    }
    // Java: double boundedToUnbounded(final double x)
    fn boundedToUnbounded(&self, x: f64) -> f64 {
        // Java: return unboundingFunction.value(x);
        todo!()
    }
}

// Java: public class MultivariateFunctionMappingAdapter
pub struct MultivariateFunctionMappingAdapter {
    bounded: Box<dyn Any>,
    mappers: Vec<Box<dyn Mapper>>,
}

// Java: public class MultivariateFunctionMappingAdapter
impl MultivariateFunctionMappingAdapter {
    // Java: MultivariateFunctionMappingAdapter(final MultivariateFunction bounded, final double[] lower, final double[] upper)
    pub fn new(bounded: Box<dyn Any>, lower: &[f64], upper: &[f64]) -> Self {
        // Java: if (lower.length != upper.length)
        if lower.len() != upper.len() {
            panic!("DimensionMismatchException: {} vs {}", lower.len(), upper.len());
        }
        // Java: for (int i = 0; i < lower.length; ++i)
        for i in 0..lower.len() {
            // Java: if (!(upper[i] >= lower[i]))
            if !(upper[i] >= lower[i]) {
                panic!("NumberIsTooSmallException: {} < {}", upper[i], lower[i]);
            }
        }
        // Java: this.bounded = bounded;
        // Java: this.mappers = new Mapper[lower.length];
        let mut mappers: Vec<Box<dyn Mapper>> = Vec::with_capacity(lower.len());
        // Java: for (int i = 0; i < mappers.length; ++i)
        for i in 0..lower.len() {
            // Java: if (Double.isInfinite(lower[i]))
            if lower[i].is_infinite() {
                // Java: if (Double.isInfinite(upper[i]))
                if upper[i].is_infinite() {
                    // Java: mappers[i] = new NoBoundsMapper();
                    mappers.push(Box::new(NoBoundsMapper {}));
                } else {
                    // Java: mappers[i] = new UpperBoundMapper(upper[i]);
                    mappers.push(Box::new(UpperBoundMapper { upper: upper[i] }));
                }
            } else {
                // Java: if (Double.isInfinite(upper[i]))
                if upper[i].is_infinite() {
                    // Java: mappers[i] = new LowerBoundMapper(lower[i]);
                    mappers.push(Box::new(LowerBoundMapper { lower: lower[i] }));
                } else {
                    // Java: mappers[i] = new LowerUpperBoundMapper(lower[i], upper[i]);
                    mappers.push(Box::new(LowerUpperBoundMapper { lower: lower[i], upper: upper[i] }));
                }
            }
        }
        MultivariateFunctionMappingAdapter { bounded, mappers }
    }

    // Java: double[] unboundedToBounded(double[] point)
    pub fn unboundedToBounded(&self, point: &[f64]) -> Vec<f64> {
        // Java: final double[] mapped = new double[mappers.length];
        let mut mapped = vec![0.0; self.mappers.len()];
        // Java: for (int i = 0; i < mappers.length; ++i)
        for i in 0..self.mappers.len() {
            // Java: mapped[i] = mappers[i].unboundedToBounded(point[i]);
            mapped[i] = self.mappers[i].unboundedToBounded(point[i]);
        }
        // Java: return mapped;
        mapped
    }

    // Java: double[] boundedToUnbounded(double[] point)
    pub fn boundedToUnbounded(&self, point: &[f64]) -> Vec<f64> {
        // Java: final double[] mapped = new double[mappers.length];
        let mut mapped = vec![0.0; self.mappers.len()];
        // Java: for (int i = 0; i < mappers.length; ++i)
        for i in 0..self.mappers.len() {
            // Java: mapped[i] = mappers[i].boundedToUnbounded(point[i]);
            mapped[i] = self.mappers[i].boundedToUnbounded(point[i]);
        }
        // Java: return mapped;
        mapped
    }

    // Java: double value(double[] point)
    pub fn value(&self, point: &[f64]) -> f64 {
        // Java: return bounded.value(unboundedToBounded(point));
        todo!()
    }
}
