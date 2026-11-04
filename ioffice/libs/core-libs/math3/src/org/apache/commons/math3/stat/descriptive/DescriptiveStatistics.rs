// Java: import java.io.Serializable
// Java: import java.lang.reflect.InvocationTargetException
// Java: import java.util.Arrays
// Java: import org.apache.commons.math3.exception.MathIllegalArgumentException
use crate::org::apache::commons::math3::exception::MathIllegalArgumentException;
// Java: import org.apache.commons.math3.exception.NullArgumentException
use crate::org::apache::commons::math3::exception::NullArgumentException;
// Java: import org.apache.commons.math3.exception.MathIllegalStateException
use crate::org::apache::commons::math3::exception::MathIllegalStateException;
// Java: import org.apache.commons.math3.exception.util.LocalizedFormats
use crate::org::apache::commons::math3::exception::util::LocalizedFormats;
// Java: import org.apache.commons.math3.stat.descriptive.moment.GeometricMean
use crate::org::apache::commons::math3::stat::descriptive::moment::GeometricMean;
// Java: import org.apache.commons.math3.stat.descriptive.moment.Kurtosis
use crate::org::apache::commons::math3::stat::descriptive::moment::Kurtosis;
// Java: import org.apache.commons.math3.stat.descriptive.moment.Mean
use crate::org::apache::commons::math3::stat::descriptive::moment::Mean;
// Java: import org.apache.commons.math3.stat.descriptive.moment.Skewness
use crate::org::apache::commons::math3::stat::descriptive::moment::Skewness;
// Java: import org.apache.commons.math3.stat.descriptive.moment.Variance
use crate::org::apache::commons::math3::stat::descriptive::moment::Variance;
// Java: import org.apache.commons.math3.stat.descriptive.rank.Max
use crate::org::apache::commons::math3::stat::descriptive::rank::Max;
// Java: import org.apache.commons.math3.stat.descriptive.rank.Min
use crate::org::apache::commons::math3::stat::descriptive::rank::Min;
// Java: import org.apache.commons.math3.stat.descriptive.rank.Percentile
use crate::org::apache::commons::math3::stat::descriptive::rank::Percentile;
// Java: import org.apache.commons.math3.stat.descriptive.summary.Sum
use crate::org::apache::commons::math3::stat::descriptive::summary::Sum;
// Java: import org.apache.commons.math3.stat.descriptive.summary.SumOfSquares
use crate::org::apache::commons::math3::stat::descriptive::summary::SumOfSquares;
// Java: import org.apache.commons.math3.util.MathUtils
use crate::org::apache::commons::math3::util::MathUtils;
// Java: import org.apache.commons.math3.util.ResizableDoubleArray
use crate::org::apache::commons::math3::util::ResizableDoubleArray;
// Java: import org.apache.commons.math3.util.FastMath
use crate::org::apache::commons::math3::util::FastMath;

// Java field: int INFINITE_WINDOW
pub(crate) const INFINITE_WINDOW: i32 = 0;
// Java field: long serialVersionUID
// Java field: String SET_QUANTILE_METHOD_NAME
pub(crate) const SET_QUANTILE_METHOD_NAME: &str = "";

// Java: public class is
pub struct is {
}

// Java: public class DescriptiveStatistics implements StatisticalSummary, Serializable
pub struct DescriptiveStatistics {
    // Java field: int windowSize
    pub(crate) windowSize: i32,
    // Java field: ResizableDoubleArray eDA
    pub(crate) eDA: Box<dyn std::any::Any>,
    // Java field: UnivariateStatistic meanImpl
    pub(crate) meanImpl: Box<dyn std::any::Any>,
    // Java field: UnivariateStatistic geometricMeanImpl
    pub(crate) geometricMeanImpl: Box<dyn std::any::Any>,
    // Java field: UnivariateStatistic kurtosisImpl
    pub(crate) kurtosisImpl: Box<dyn std::any::Any>,
    // Java field: UnivariateStatistic maxImpl
    pub(crate) maxImpl: Box<dyn std::any::Any>,
    // Java field: UnivariateStatistic minImpl
    pub(crate) minImpl: Box<dyn std::any::Any>,
    // Java field: UnivariateStatistic percentileImpl
    pub(crate) percentileImpl: Box<dyn std::any::Any>,
    // Java field: UnivariateStatistic skewnessImpl
    pub(crate) skewnessImpl: Box<dyn std::any::Any>,
    // Java field: UnivariateStatistic varianceImpl
    pub(crate) varianceImpl: Box<dyn std::any::Any>,
    // Java field: UnivariateStatistic sumsqImpl
    pub(crate) sumsqImpl: Box<dyn std::any::Any>,
    // Java field: UnivariateStatistic sumImpl
    pub(crate) sumImpl: Box<dyn std::any::Any>,
}

// Java: public class is
impl is {
}

// Java: public class DescriptiveStatistics implements StatisticalSummary, Serializable
impl DescriptiveStatistics {
    // Java: DescriptiveStatistics()
    pub fn DescriptiveStatistics(&self) {
    }

    // Java: DescriptiveStatistics(int window)
    pub fn DescriptiveStatistics_2(&self) {
        // Java: setWindowSize(window);
    }

    // Java: DescriptiveStatistics(double[] initialDoubleArray)
    pub fn DescriptiveStatistics_3(&self) {
        // Java: if (initialDoubleArray != null)
        {
            // Java: eDA = new ResizableDoubleArray(initialDoubleArray);
        }
    }

    // Java: DescriptiveStatistics(DescriptiveStatistics original)
    pub fn DescriptiveStatistics_4(&self) {
        // Java: copy(original, this);
    }

    // Java: void addValue(double v)
    pub fn addValue(&self) {
        // Java: if (windowSize != INFINITE_WINDOW)
        {
            // Java: if (getN() == windowSize)
            {
                // Java: eDA.addElementRolling(v);
            }
            // Java: else if (getN() < windowSize)
            {
                // Java: eDA.addElement(v);
            }
        }
        // Java: else
        {
            // Java: eDA.addElement(v);
        }
    }

    // Java: void removeMostRecentValue()
    pub fn removeMostRecentValue(&self) {
        // Java: try
        {
            // Java: eDA.discardMostRecentElements(1);
        }
        // Java: catch (MathIllegalArgumentException ex)
        {
            // Java: throw new MathIllegalStateException(LocalizedFormats.NO_DATA);
        }
    }

    // Java: double replaceMostRecentValue(double v)
    pub fn replaceMostRecentValue(&self) -> f64 {
        // Java: return eDA.substituteMostRecentElement(v);
        todo!()
    }

    // Java: double getMean()
    pub fn getMean(&self) -> f64 {
        // Java: return apply(meanImpl);
        todo!()
    }

    // Java: double getGeometricMean()
    pub fn getGeometricMean(&self) -> f64 {
        // Java: return apply(geometricMeanImpl);
        todo!()
    }

    // Java: double getVariance()
    pub fn getVariance(&self) -> f64 {
        // Java: return apply(varianceImpl);
        todo!()
    }

    // Java: double getPopulationVariance()
    pub fn getPopulationVariance(&self) -> f64 {
        // Java: return apply(new Variance(false));
        todo!()
    }

    // Java: double getStandardDeviation()
    pub fn getStandardDeviation(&self) -> f64 {
        // Java: double stdDev = Double.NaN;
        // Java: if (getN() > 0)
        {
            // Java: if (getN() > 1)
            {
                // Java: stdDev = FastMath.sqrt(getVariance());
            }
            // Java: else
            {
                // Java: stdDev = 0.0;
            }
        }
        // Java: return stdDev;
        todo!()
    }

    // Java: double getQuadraticMean()
    pub fn getQuadraticMean(&self) -> f64 {
        // Java: final long n = getN();
        // Java: return n > 0 ? FastMath.sqrt(getSumsq() / n) : Double.NaN;
        todo!()
    }

    // Java: double getSkewness()
    pub fn getSkewness(&self) -> f64 {
        // Java: return apply(skewnessImpl);
        todo!()
    }

    // Java: double getKurtosis()
    pub fn getKurtosis(&self) -> f64 {
        // Java: return apply(kurtosisImpl);
        todo!()
    }

    // Java: double getMax()
    pub fn getMax(&self) -> f64 {
        // Java: return apply(maxImpl);
        todo!()
    }

    // Java: double getMin()
    pub fn getMin(&self) -> f64 {
        // Java: return apply(minImpl);
        todo!()
    }

    // Java: long getN()
    pub fn getN(&self) -> i64 {
        // Java: return eDA.getNumElements();
        todo!()
    }

    // Java: double getSum()
    pub fn getSum(&self) -> f64 {
        // Java: return apply(sumImpl);
        todo!()
    }

    // Java: double getSumsq()
    pub fn getSumsq(&self) -> f64 {
        // Java: return apply(sumsqImpl);
        todo!()
    }

    // Java: void clear()
    pub fn clear(&self) {
        // Java: eDA.clear();
    }

    // Java: int getWindowSize()
    pub fn getWindowSize(&self) -> i32 {
        // Java: return windowSize;
        todo!()
    }

    // Java: void setWindowSize(int windowSize)
    pub fn setWindowSize(&self) {
        // Java: if (windowSize < 1 && windowSize != INFINITE_WINDOW)
        {
            // Java: throw new MathIllegalArgumentException(
                    // Java: LocalizedFormats.NOT_POSITIVE_WINDOW_SIZE, windowSize);
        }
        // Java: this.windowSize = windowSize;
        // Java: // We need to check to see if we need to discard elements
        // Java: // from the front of the array.  If the windowSize is less than
        // Java: // the current number of elements.
        // Java: if (windowSize != INFINITE_WINDOW && windowSize < eDA.getNumElements())
        {
            // Java: eDA.discardFrontElements(eDA.getNumElements() - windowSize);
        }
    }

    // Java: double[] getValues()
    pub fn getValues(&self) -> Vec<f64> {
        // Java: return eDA.getElements();
        todo!()
    }

    // Java: double[] getSortedValues()
    pub fn getSortedValues(&self) -> Vec<f64> {
        // Java: double[] sort = getValues();
        // Java: Arrays.sort(sort);
        // Java: return sort;
        todo!()
    }

    // Java: double getElement(int index)
    pub fn getElement(&self) -> f64 {
        // Java: return eDA.getElement(index);
        todo!()
    }

    // Java: double getPercentile(double p)
    pub fn getPercentile(&self) -> f64 {
        // Java: if (percentileImpl instanceof Percentile)
        {
            // Java: ((Percentile) percentileImpl).setQuantile(p);
        }
        // Java: else
        {
            // Java: try
            {
                // Java: percentileImpl.getClass().getMethod(SET_QUANTILE_METHOD_NAME,
                        // Java: new Class[] {Double.TYPE}).invoke(percentileImpl,
                                // Java: new Object[] {Double.valueOf(p)});
            }
                // Java: throw new MathIllegalStateException(
                      // Java: LocalizedFormats.PERCENTILE_IMPLEMENTATION_UNSUPPORTED_METHOD,
                      // Java: percentileImpl.getClass().getName(), SET_QUANTILE_METHOD_NAME);
            // Java: catch (IllegalAccessException e2)
            {
                // Java: throw new MathIllegalStateException(
                      // Java: LocalizedFormats.PERCENTILE_IMPLEMENTATION_CANNOT_ACCESS_METHOD,
                      // Java: SET_QUANTILE_METHOD_NAME, percentileImpl.getClass().getName());
            }
            // Java: catch (InvocationTargetException e3)
            {
                // Java: throw new IllegalStateException(e3.getCause());
            }
        }
        // Java: return apply(percentileImpl);
        todo!()
    }

    // Java: String toString()
    pub fn toString(&self) -> String {
        // Java: StringBuilder outBuffer = new StringBuilder();
        // Java: String endl = "\n";
        // Java: outBuffer.append("DescriptiveStatistics:").append(endl);
        // Java: outBuffer.append("n: ").append(getN()).append(endl);
        // Java: outBuffer.append("min: ").append(getMin()).append(endl);
        // Java: outBuffer.append("max: ").append(getMax()).append(endl);
        // Java: outBuffer.append("mean: ").append(getMean()).append(endl);
        // Java: outBuffer.append("std dev: ").append(getStandardDeviation())
            // Java: .append(endl);
        // Java: try
        {
            // Java: // No catch for MIAE because actual parameter is valid below
            // Java: outBuffer.append("median: ").append(getPercentile(50)).append(endl);
        }
        // Java: catch (MathIllegalStateException ex)
        {
            // Java: outBuffer.append("median: unavailable").append(endl);
        }
        // Java: outBuffer.append("skewness: ").append(getSkewness()).append(endl);
        // Java: outBuffer.append("kurtosis: ").append(getKurtosis()).append(endl);
        // Java: return outBuffer.toString();
        todo!()
    }

    // Java: double apply(UnivariateStatistic stat)
    pub fn apply(&self) -> f64 {
        // Java: // No try-catch or advertised exception here because arguments are guaranteed valid
        // Java: return eDA.compute(stat);
        todo!()
    }

    // Java: UnivariateStatistic getMeanImpl()
    pub fn getMeanImpl(&self) -> Box<dyn std::any::Any> {
        // Java: return meanImpl;
        todo!()
    }

    // Java: void setMeanImpl(UnivariateStatistic meanImpl)
    pub fn setMeanImpl(&self) {
        // Java: this.meanImpl = meanImpl;
    }

    // Java: UnivariateStatistic getGeometricMeanImpl()
    pub fn getGeometricMeanImpl(&self) -> Box<dyn std::any::Any> {
        // Java: return geometricMeanImpl;
        todo!()
    }

    // Java: void setGeometricMeanImpl()
    pub fn setGeometricMeanImpl(&self) {
    }

    // Java: UnivariateStatistic getKurtosisImpl()
    pub fn getKurtosisImpl(&self) -> Box<dyn std::any::Any> {
        // Java: return kurtosisImpl;
        todo!()
    }

    // Java: void setKurtosisImpl(UnivariateStatistic kurtosisImpl)
    pub fn setKurtosisImpl(&self) {
        // Java: this.kurtosisImpl = kurtosisImpl;
    }

    // Java: UnivariateStatistic getMaxImpl()
    pub fn getMaxImpl(&self) -> Box<dyn std::any::Any> {
        // Java: return maxImpl;
        todo!()
    }

    // Java: void setMaxImpl(UnivariateStatistic maxImpl)
    pub fn setMaxImpl(&self) {
        // Java: this.maxImpl = maxImpl;
    }

    // Java: UnivariateStatistic getMinImpl()
    pub fn getMinImpl(&self) -> Box<dyn std::any::Any> {
        // Java: return minImpl;
        todo!()
    }

    // Java: void setMinImpl(UnivariateStatistic minImpl)
    pub fn setMinImpl(&self) {
        // Java: this.minImpl = minImpl;
    }

    // Java: UnivariateStatistic getPercentileImpl()
    pub fn getPercentileImpl(&self) -> Box<dyn std::any::Any> {
        // Java: return percentileImpl;
        todo!()
    }

    // Java: void setPercentileImpl(UnivariateStatistic percentileImpl)
    pub fn setPercentileImpl(&self) {
    }

    // Java: UnivariateStatistic getSkewnessImpl()
    pub fn getSkewnessImpl(&self) -> Box<dyn std::any::Any> {
        // Java: return skewnessImpl;
        todo!()
    }

    // Java: void setSkewnessImpl()
    pub fn setSkewnessImpl(&self) {
    }

    // Java: UnivariateStatistic getVarianceImpl()
    pub fn getVarianceImpl(&self) -> Box<dyn std::any::Any> {
        // Java: return varianceImpl;
        todo!()
    }

    // Java: void setVarianceImpl()
    pub fn setVarianceImpl(&self) {
    }

    // Java: UnivariateStatistic getSumsqImpl()
    pub fn getSumsqImpl(&self) -> Box<dyn std::any::Any> {
        // Java: return sumsqImpl;
        todo!()
    }

    // Java: void setSumsqImpl(UnivariateStatistic sumsqImpl)
    pub fn setSumsqImpl(&self) {
        // Java: this.sumsqImpl = sumsqImpl;
    }

    // Java: UnivariateStatistic getSumImpl()
    pub fn getSumImpl(&self) -> Box<dyn std::any::Any> {
        // Java: return sumImpl;
        todo!()
    }

    // Java: void setSumImpl(UnivariateStatistic sumImpl)
    pub fn setSumImpl(&self) {
        // Java: this.sumImpl = sumImpl;
    }

    // Java: DescriptiveStatistics copy()
    pub fn copy(&self) -> Box<dyn std::any::Any> {
        // Java: DescriptiveStatistics result = new DescriptiveStatistics();
        // Java: // No try-catch or advertised exception because parms are guaranteed valid
        // Java: copy(this, result);
        // Java: return result;
        todo!()
    }

    // Java: static void copy(DescriptiveStatistics source, DescriptiveStatistics dest)
    pub fn copy_2(&self) {
    }

}


