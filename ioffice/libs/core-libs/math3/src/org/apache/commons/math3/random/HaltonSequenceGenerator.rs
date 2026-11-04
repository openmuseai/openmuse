// Java: import org.apache.commons.math3.exception.DimensionMismatchException
use crate::org::apache::commons::math3::exception::DimensionMismatchException;
// Java: import org.apache.commons.math3.exception.NotPositiveException
use crate::org::apache::commons::math3::exception::NotPositiveException;
// Java: import org.apache.commons.math3.exception.NullArgumentException
use crate::org::apache::commons::math3::exception::NullArgumentException;
// Java: import org.apache.commons.math3.exception.OutOfRangeException
use crate::org::apache::commons::math3::exception::OutOfRangeException;
// Java: import org.apache.commons.math3.util.MathUtils
use crate::org::apache::commons::math3::util::MathUtils;

// Java: public class HaltonSequenceGenerator implements RandomVectorGenerator
pub struct HaltonSequenceGenerator {
    // Java field: int dimension
    dimension: i32,
    // Java field: int count
    pub(crate) count: i32,
    // Java field: int[] base
    base: Vec<i32>,
    // Java field: int[] weight
    weight: Vec<i32>,
}

// Java: public class HaltonSequenceGenerator implements RandomVectorGenerator
impl HaltonSequenceGenerator {
    // Java: HaltonSequenceGenerator(final int dimension)
    pub fn HaltonSequenceGenerator(&self) {
        // Java: this(dimension, PRIMES, WEIGHTS);
    }

    // Java: HaltonSequenceGenerator(final int dimension, final int[] bases, final int[] weights)
    pub fn HaltonSequenceGenerator_2(&self) {
    }

    // Java: double[] nextVector()
    pub fn nextVector(&self) -> Vec<f64> {
        // Java: final double[] v = new double[dimension];
        // Java: for (int i = 0; i < dimension; i++)
        {
            // Java: int index = count;
            // Java: double f = 1.0 / base[i];
            // Java: int j = 0;
            // Java: while (index > 0)
            {
                // Java: final int digit = scramble(i, j, base[i], index % base[i]);
                // Java: v[i] += f * digit;
                // Java: index /= base[i]; // floor( index / base )
                // Java: f /= base[i];
            }
        }
        // Java: count++;
        // Java: return v;
        todo!()
    }

    // Java: int scramble(final int i, final int j, final int b, final int digit)
    pub fn scramble(&self) -> i32 {
        // Java: return weight != null ? (weight[i] * digit) % b : digit;
        todo!()
    }

    // Java: double[] skipTo(final int index)
    pub fn skipTo(&self) -> Vec<f64> {
        // Java: count = index;
        // Java: return nextVector();
        todo!()
    }

    // Java: int getNextIndex()
    pub fn getNextIndex(&self) -> i32 {
        // Java: return count;
        todo!()
    }

}

