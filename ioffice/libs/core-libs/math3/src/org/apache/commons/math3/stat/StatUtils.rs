// Java: import java.util.List
// Java: import org.apache.commons.math3.exception.MathIllegalArgumentException
use crate::org::apache::commons::math3::exception::MathIllegalArgumentException;
// Java: import org.apache.commons.math3.exception.NotPositiveException
use crate::org::apache::commons::math3::exception::NotPositiveException;
// Java: import org.apache.commons.math3.exception.NullArgumentException
use crate::org::apache::commons::math3::exception::NullArgumentException;
// Java: import org.apache.commons.math3.exception.NumberIsTooSmallException
use crate::org::apache::commons::math3::exception::NumberIsTooSmallException;
// Java: import org.apache.commons.math3.exception.DimensionMismatchException
use crate::org::apache::commons::math3::exception::DimensionMismatchException;
// Java: import org.apache.commons.math3.exception.NoDataException
use crate::org::apache::commons::math3::exception::NoDataException;
// Java: import org.apache.commons.math3.exception.util.LocalizedFormats
use crate::org::apache::commons::math3::exception::util::LocalizedFormats;
// Java: import org.apache.commons.math3.stat.descriptive.DescriptiveStatistics
use crate::org::apache::commons::math3::stat::descriptive::DescriptiveStatistics;
// Java: import org.apache.commons.math3.stat.descriptive.UnivariateStatistic
use crate::org::apache::commons::math3::stat::descriptive::UnivariateStatistic;
// Java: import org.apache.commons.math3.stat.descriptive.moment.GeometricMean
use crate::org::apache::commons::math3::stat::descriptive::moment::GeometricMean;
// Java: import org.apache.commons.math3.stat.descriptive.moment.Mean
use crate::org::apache::commons::math3::stat::descriptive::moment::Mean;
// Java: import org.apache.commons.math3.stat.descriptive.moment.Variance
use crate::org::apache::commons::math3::stat::descriptive::moment::Variance;
// Java: import org.apache.commons.math3.stat.descriptive.rank.Max
use crate::org::apache::commons::math3::stat::descriptive::rank::Max;
// Java: import org.apache.commons.math3.stat.descriptive.rank.Min
use crate::org::apache::commons::math3::stat::descriptive::rank::Min;
// Java: import org.apache.commons.math3.stat.descriptive.rank.Percentile
use crate::org::apache::commons::math3::stat::descriptive::rank::Percentile;
// Java: import org.apache.commons.math3.stat.descriptive.summary.Product
use crate::org::apache::commons::math3::stat::descriptive::summary::Product;
// Java: import org.apache.commons.math3.stat.descriptive.summary.Sum
use crate::org::apache::commons::math3::stat::descriptive::summary::Sum;
// Java: import org.apache.commons.math3.stat.descriptive.summary.SumOfLogs
use crate::org::apache::commons::math3::stat::descriptive::summary::SumOfLogs;
// Java: import org.apache.commons.math3.stat.descriptive.summary.SumOfSquares
use crate::org::apache::commons::math3::stat::descriptive::summary::SumOfSquares;

// Java field: UnivariateStatistic SUM
static SUM: std::sync::OnceLock<Box<dyn std::any::Any + Send + Sync>> = std::sync::OnceLock::new();
// Java field: UnivariateStatistic SUM_OF_SQUARES
static SUM_OF_SQUARES: std::sync::OnceLock<Box<dyn std::any::Any + Send + Sync>> = std::sync::OnceLock::new();
// Java field: UnivariateStatistic PRODUCT
static PRODUCT: std::sync::OnceLock<Box<dyn std::any::Any + Send + Sync>> = std::sync::OnceLock::new();
// Java field: UnivariateStatistic SUM_OF_LOGS
static SUM_OF_LOGS: std::sync::OnceLock<Box<dyn std::any::Any + Send + Sync>> = std::sync::OnceLock::new();
// Java field: UnivariateStatistic MIN
static MIN: std::sync::OnceLock<Box<dyn std::any::Any + Send + Sync>> = std::sync::OnceLock::new();
// Java field: UnivariateStatistic MAX
static MAX: std::sync::OnceLock<Box<dyn std::any::Any + Send + Sync>> = std::sync::OnceLock::new();
// Java field: UnivariateStatistic MEAN
static MEAN: std::sync::OnceLock<Box<dyn std::any::Any + Send + Sync>> = std::sync::OnceLock::new();
// Java field: Variance VARIANCE
static VARIANCE: std::sync::OnceLock<Box<dyn std::any::Any + Send + Sync>> = std::sync::OnceLock::new();
// Java field: Percentile PERCENTILE
static PERCENTILE: std::sync::OnceLock<Box<dyn std::any::Any + Send + Sync>> = std::sync::OnceLock::new();
// Java field: GeometricMean GEOMETRIC_MEAN
static GEOMETRIC_MEAN: std::sync::OnceLock<Box<dyn std::any::Any + Send + Sync>> = std::sync::OnceLock::new();

// Java: public class StatUtils
pub struct StatUtils {
}

// Java: public class StatUtils
impl StatUtils {
    // Java: StatUtils()
    pub fn StatUtils(&self) {
    }

    // Java: static double sum(final double[] values)
    pub fn sum(&self) -> f64 {
        todo!()
    }

    // Java: static double sum()
    pub fn sum_2(&self) -> f64 {
        todo!()
    }

    // Java: static double sumSq(final double[] values)
    pub fn sumSq(&self) -> f64 {
        // Java: return SUM_OF_SQUARES.evaluate(values);
        todo!()
    }

    // Java: static double sumSq()
    pub fn sumSq_2(&self) -> f64 {
        todo!()
    }

    // Java: static double product(final double[] values)
    pub fn product(&self) -> f64 {
        todo!()
    }

    // Java: static double product()
    pub fn product_2(&self) -> f64 {
        todo!()
    }

    // Java: static double sumLog(final double[] values)
    pub fn sumLog(&self) -> f64 {
        todo!()
    }

    // Java: static double sumLog()
    pub fn sumLog_2(&self) -> f64 {
        todo!()
    }

    // Java: static double mean(final double[] values)
    pub fn mean(&self) -> f64 {
        todo!()
    }

    // Java: static double mean()
    pub fn mean_2(&self) -> f64 {
        todo!()
    }

    // Java: static double geometricMean(final double[] values)
    pub fn geometricMean(&self) -> f64 {
        todo!()
    }

    // Java: static double geometricMean()
    pub fn geometricMean_2(&self) -> f64 {
        todo!()
    }

    // Java: static double variance(final double[] values)
    pub fn variance(&self) -> f64 {
        // Java: return VARIANCE.evaluate(values);
        todo!()
    }

    // Java: static double variance()
    pub fn variance_2(&self) -> f64 {
        todo!()
    }

    // Java: static double variance()
    pub fn variance_3(&self) -> f64 {
        todo!()
    }

    // Java: static double variance(final double[] values, final double mean)
    pub fn variance_4(&self) -> f64 {
        todo!()
    }

    // Java: static double populationVariance(final double[] values)
    pub fn populationVariance(&self) -> f64 {
        todo!()
    }

    // Java: static double populationVariance()
    pub fn populationVariance_2(&self) -> f64 {
        todo!()
    }

    // Java: static double populationVariance()
    pub fn populationVariance_3(&self) -> f64 {
        todo!()
    }

    // Java: static double populationVariance(final double[] values, final double mean)
    pub fn populationVariance_4(&self) -> f64 {
        todo!()
    }

    // Java: static double max(final double[] values)
    pub fn max(&self) -> f64 {
        // Java: return MAX.evaluate(values);
        todo!()
    }

    // Java: static double max()
    pub fn max_2(&self) -> f64 {
        todo!()
    }

    // Java: static double min(final double[] values)
    pub fn min(&self) -> f64 {
        // Java: return MIN.evaluate(values);
        todo!()
    }

    // Java: static double min()
    pub fn min_2(&self) -> f64 {
        todo!()
    }

    // Java: static double percentile(final double[] values, final double p)
    pub fn percentile(&self) -> f64 {
        todo!()
    }

    // Java: static double percentile()
    pub fn percentile_2(&self) -> f64 {
        todo!()
    }

    // Java: static double sumDifference(final double[] sample1, final double[] sample2)
    pub fn sumDifference(&self) -> f64 {
        todo!()
    }

    // Java: static double meanDifference(final double[] sample1, final double[] sample2)
    pub fn meanDifference(&self) -> f64 {
        todo!()
    }

    // Java: static double varianceDifference()
    pub fn varianceDifference(&self) -> f64 {
        todo!()
    }

    // Java: static double[] normalize(final double[] sample)
    pub fn normalize(&self) -> Vec<f64> {
        // Java: DescriptiveStatistics stats = new DescriptiveStatistics();
        // Java: // Add the data from the series to stats
        // Java: for (int i = 0; i < sample.length; i++)
        {
            // Java: stats.addValue(sample[i]);
        }
        // Java: // Compute mean and standard deviation
        // Java: double mean = stats.getMean();
        // Java: double standardDeviation = stats.getStandardDeviation();
        // Java: // initialize the standardizedSample, which has the same length as the sample
        // Java: double[] standardizedSample = new double[sample.length];
        // Java: for (int i = 0; i < sample.length; i++)
        {
            // Java: // z = (x- mean)/standardDeviation
            // Java: standardizedSample[i] = (sample[i] - mean) / standardDeviation;
        }
        // Java: return standardizedSample;
        todo!()
    }

    // Java: static double[] mode(double[] sample)
    pub fn mode(&self) -> Vec<f64> {
        // Java: if (sample == null)
        {
            // Java: throw new NullArgumentException(LocalizedFormats.INPUT_ARRAY);
        }
        // Java: return getMode(sample, 0, sample.length);
        todo!()
    }

    // Java: static double[] mode(double[] sample, final int begin, final int length)
    pub fn mode_2(&self) -> Vec<f64> {
        // Java: if (sample == null)
        {
            // Java: throw new NullArgumentException(LocalizedFormats.INPUT_ARRAY);
        }
        // Java: if (begin < 0)
        {
            // Java: throw new NotPositiveException(LocalizedFormats.START_POSITION, Integer.valueOf(begin));
        }
        // Java: if (length < 0)
        {
            // Java: throw new NotPositiveException(LocalizedFormats.LENGTH, Integer.valueOf(length));
        }
        // Java: return getMode(sample, begin, length);
        todo!()
    }

    // Java: static double[] getMode(double[] values, final int begin, final int length)
    pub fn getMode(&self) -> Vec<f64> {
        // Java: // Add the values to the frequency table
        // Java: Frequency freq = new Frequency();
        // Java: for (int i = begin; i < begin + length; i++)
        {
            // Java: final double value = values[i];
            // Java: if (!Double.isNaN(value))
            {
                // Java: freq.addValue(Double.valueOf(value));
            }
        }
        // Java: List<Comparable<?>> list = freq.getMode();
        // Java: // Convert the list to an array of primitive double
        // Java: double[] modes = new double[list.size()];
        // Java: int i = 0;
        // Java: for(Comparable<?> c : list)
        {
            // Java: modes[i++] = ((Double) c).doubleValue();
        }
        // Java: return modes;
        todo!()
    }

}
