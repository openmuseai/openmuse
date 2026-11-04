// Java: import java.io.Serializable
// Java: import java.util.Arrays
// Java: import java.util.BitSet
// Java: import org.apache.commons.math3.exception.MathIllegalArgumentException
use crate::org::apache::commons::math3::exception::MathIllegalArgumentException;
// Java: import org.apache.commons.math3.exception.MathUnsupportedOperationException
use crate::org::apache::commons::math3::exception::MathUnsupportedOperationException;
// Java: import org.apache.commons.math3.exception.NullArgumentException
use crate::org::apache::commons::math3::exception::NullArgumentException;
// Java: import org.apache.commons.math3.exception.OutOfRangeException
use crate::org::apache::commons::math3::exception::OutOfRangeException;
// Java: import org.apache.commons.math3.exception.util.LocalizedFormats
use crate::org::apache::commons::math3::exception::util::LocalizedFormats;
// Java: import org.apache.commons.math3.stat.descriptive.AbstractUnivariateStatistic
use crate::org::apache::commons::math3::stat::descriptive::AbstractUnivariateStatistic;
// Java: import org.apache.commons.math3.stat.ranking.NaNStrategy
use crate::org::apache::commons::math3::stat::ranking::NaNStrategy;
// Java: import org.apache.commons.math3.util.FastMath
use crate::org::apache::commons::math3::util::FastMath;
// Java: import org.apache.commons.math3.util.KthSelector
use crate::org::apache::commons::math3::util::KthSelector;
// Java: import org.apache.commons.math3.util.MathArrays
use crate::org::apache::commons::math3::util::MathArrays;
// Java: import org.apache.commons.math3.util.MathUtils
use crate::org::apache::commons::math3::util::MathUtils;
// Java: import org.apache.commons.math3.util.MedianOf3PivotingStrategy
use crate::org::apache::commons::math3::util::MedianOf3PivotingStrategy;
// Java: import org.apache.commons.math3.util.PivotingStrategyInterface
use crate::org::apache::commons::math3::util::PivotingStrategyInterface;
// Java: import org.apache.commons.math3.util.Precision
use crate::org::apache::commons::math3::util::Precision;

// Java field: long serialVersionUID
// Java field: int MAX_CACHED_LEVELS
pub(crate) const MAX_CACHED_LEVELS: i32 = 0;
// Java field: int PIVOTS_HEAP_LENGTH
pub(crate) const PIVOTS_HEAP_LENGTH: i32 = 0;

// Java: public class concurrently
pub struct concurrently {
}

// Java: public class Percentile
pub struct Percentile {
    // Java field: KthSelector kthSelector
    pub(crate) kthSelector: Box<dyn std::any::Any>,
    // Java field: EstimationType estimationType
    pub(crate) estimationType: Box<dyn std::any::Any>,
    // Java field: NaNStrategy nanStrategy
    pub(crate) nanStrategy: Box<dyn std::any::Any>,
    // Java field: double quantile
    pub(crate) quantile: f64,
    // Java field: int[] cachedPivots
    pub(crate) cachedPivots: Box<dyn std::any::Any>,
}

// Java: public enum EstimationType
pub struct EstimationType {
    // Java field: String name
    pub(crate) name: Box<dyn std::any::Any>,
}

// Java: public class concurrently
impl concurrently {
}

// Java: public class Percentile
impl Percentile {
    // Java: Percentile()
    pub fn Percentile(&self) {
        // Java: // No try-catch or advertised exception here - arg is valid
        // Java: this(50.0);
    }

    // Java: Percentile(final double quantile)
    pub fn Percentile_2(&self) {
        // Java: this(quantile, EstimationType.LEGACY, NaNStrategy.REMOVED,
             // Java: new KthSelector(new MedianOf3PivotingStrategy()));
    }

    // Java: Percentile(final Percentile original)
    pub fn Percentile_3(&self) {
        // Java: MathUtils.checkNotNull(original);
        // Java: estimationType   = original.getEstimationType();
        // Java: nanStrategy      = original.getNaNStrategy();
        // Java: kthSelector      = original.getKthSelector();
        // Java: setData(original.getDataRef());
        // Java: if (original.cachedPivots != null)
        {
            // Java: System.arraycopy(original.cachedPivots, 0, cachedPivots, 0, original.cachedPivots.length);
        }
        // Java: setQuantile(original.quantile);
    }

    // Java: Percentile()
    pub fn Percentile_4(&self) {
    }

    // Java: void setData(final double[] values)
    pub fn setData(&self) {
        // Java: if (values == null)
        {
            // Java: cachedPivots = null;
        }
        // Java: else
        {
            // Java: cachedPivots = new int[PIVOTS_HEAP_LENGTH];
            // Java: Arrays.fill(cachedPivots, -1);
        }
        // Java: super.setData(values);
    }

    // Java: void setData(final double[] values, final int begin, final int length)
    pub fn setData_2(&self) {
    }

    // Java: double evaluate(final double p)
    pub fn evaluate(&self) -> f64 {
        // Java: return evaluate(getDataRef(), p);
        todo!()
    }

    // Java: double evaluate(final double[] values, final double p)
    pub fn evaluate_2(&self) -> f64 {
        todo!()
    }

    // Java: double evaluate(final double[] values, final int start, final int length)
    pub fn evaluate_3(&self) -> f64 {
        todo!()
    }

    // Java: double evaluate()
    pub fn evaluate_4(&self) -> f64 {
        todo!()
    }

    // Java: int medianOf3(final double[] work, final int begin, final int end)
    pub fn medianOf3(&self) -> i32 {
        // Java: return new MedianOf3PivotingStrategy().pivotIndex(work, begin, end);
        // Java: //throw new MathUnsupportedOperationException();
        todo!()
    }

    // Java: double getQuantile()
    pub fn getQuantile(&self) -> f64 {
        // Java: return quantile;
        todo!()
    }

    // Java: void setQuantile(final double p)
    pub fn setQuantile(&self) {
        // Java: if (p <= 0 || p > 100)
        {
            // Java: throw new OutOfRangeException(
                    // Java: LocalizedFormats.OUT_OF_BOUNDS_QUANTILE_VALUE, p, 0, 100);
        }
        // Java: quantile = p;
    }

    // Java: Percentile copy()
    pub fn copy(&self) -> Box<dyn std::any::Any> {
        // Java: return new Percentile(this);
        todo!()
    }

    // Java: static void copy(final Percentile source, final Percentile dest)
    pub fn copy_2(&self) {
    }

    // Java: double[] getWorkArray(final double[] values, final int begin, final int length)
    pub fn getWorkArray(&self) -> Vec<f64> {
            // Java: final double[] work;
            // Java: if (values == getDataRef())
            {
                // Java: work = getDataRef();
            }
            // Java: else
            {
                // Java: switch (nanStrategy)
                {
                // Java: case MAXIMAL:// Replace NaNs with +INFs
                    // Java: work = replaceAndSlice(values, begin, length, Double.NaN, Double.POSITIVE_INFINITY);
                    // Java: break;
                // Java: case MINIMAL:// Replace NaNs with -INFs
                    // Java: work = replaceAndSlice(values, begin, length, Double.NaN, Double.NEGATIVE_INFINITY);
                    // Java: break;
                // Java: case REMOVED:// Drop NaNs from data
                    // Java: work = removeAndSlice(values, begin, length, Double.NaN);
                    // Java: break;
                // Java: case FAILED:// just throw exception as NaN is un-acceptable
                    // Java: work = copyOf(values, begin, length);
                    // Java: MathArrays.checkNotNaN(work);
                    // Java: break;
                // Java: default: //FIXED
                    // Java: work = copyOf(values,begin,length);
                    // Java: break;
                }
            }
            // Java: return work;
        todo!()
    }

    // Java: static double[] copyOf(final double[] values, final int begin, final int length)
    pub fn copyOf(&self) -> Vec<f64> {
        // Java: MathArrays.verifyValues(values, begin, length);
        // Java: return MathArrays.copyOfRange(values, begin, begin + length);
        todo!()
    }

    // Java: static double[] replaceAndSlice()
    pub fn replaceAndSlice(&self) -> Vec<f64> {
        todo!()
    }

    // Java: static double[] removeAndSlice()
    pub fn removeAndSlice(&self) -> Vec<f64> {
        todo!()
    }

    // Java: int[] getPivots(final double[] values)
    pub fn getPivots(&self) -> Vec<i32> {
        // Java: final int[] pivotsHeap;
        // Java: if (values == getDataRef())
        {
            // Java: pivotsHeap = cachedPivots;
        }
        // Java: else
        {
            // Java: pivotsHeap = new int[PIVOTS_HEAP_LENGTH];
            // Java: Arrays.fill(pivotsHeap, -1);
        }
        // Java: return pivotsHeap;
        todo!()
    }

    // Java: EstimationType getEstimationType()
    pub fn getEstimationType(&self) -> EstimationType {
        // Java: return estimationType;
        todo!()
    }

    // Java: Percentile withEstimationType(final EstimationType newEstimationType)
    pub fn withEstimationType(&self) -> Box<dyn std::any::Any> {
        // Java: return new Percentile(quantile, newEstimationType, nanStrategy, kthSelector);
        todo!()
    }

    // Java: NaNStrategy getNaNStrategy()
    pub fn getNaNStrategy(&self) -> Box<dyn std::any::Any> {
        // Java: return nanStrategy;
        todo!()
    }

    // Java: Percentile withNaNStrategy(final NaNStrategy newNaNStrategy)
    pub fn withNaNStrategy(&self) -> Box<dyn std::any::Any> {
        // Java: return new Percentile(quantile, estimationType, newNaNStrategy, kthSelector);
        todo!()
    }

    // Java: KthSelector getKthSelector()
    pub fn getKthSelector(&self) -> Box<dyn std::any::Any> {
        // Java: return kthSelector;
        todo!()
    }

    // Java: PivotingStrategyInterface getPivotingStrategy()
    pub fn getPivotingStrategy(&self) -> Box<dyn std::any::Any> {
        // Java: return kthSelector.getPivotingStrategy();
        todo!()
    }

    // Java: Percentile withKthSelector(final KthSelector newKthSelector)
    pub fn withKthSelector(&self) -> Box<dyn std::any::Any> {
        // Java: return new Percentile(quantile, estimationType, nanStrategy,
                                // Java: newKthSelector);
        todo!()
    }

}

// Java: public enum EstimationType
impl EstimationType {
    // Java: abstract double index(final double p, final int length)
    pub fn index(&self) -> f64 {
        todo!()
    }

    // Java: double estimate()
    pub fn estimate(&self) -> f64 {
        todo!()
    }

    // Java: double evaluate()
    pub fn evaluate_5(&self) -> f64 {
        todo!()
    }

    // Java: double evaluate(final double[] work, final double p, final KthSelector selector)
    pub fn evaluate_6(&self) -> f64 {
            // Java: return this.evaluate(work, null, p, selector);
        todo!()
    }

    // Java: String getName()
    pub fn getName(&self) -> String {
            // Java: return name;
        todo!()
    }

}
