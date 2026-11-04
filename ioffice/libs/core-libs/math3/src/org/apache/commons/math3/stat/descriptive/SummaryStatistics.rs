// Java: import java.io.Serializable
// Java: import org.apache.commons.math3.exception.MathIllegalStateException
use crate::org::apache::commons::math3::exception::MathIllegalStateException;
// Java: import org.apache.commons.math3.exception.NullArgumentException
use crate::org::apache::commons::math3::exception::NullArgumentException;
// Java: import org.apache.commons.math3.exception.util.LocalizedFormats
use crate::org::apache::commons::math3::exception::util::LocalizedFormats;
// Java: import org.apache.commons.math3.stat.descriptive.moment.GeometricMean
use crate::org::apache::commons::math3::stat::descriptive::moment::GeometricMean;
// Java: import org.apache.commons.math3.stat.descriptive.moment.Mean
use crate::org::apache::commons::math3::stat::descriptive::moment::Mean;
// Java: import org.apache.commons.math3.stat.descriptive.moment.SecondMoment
use crate::org::apache::commons::math3::stat::descriptive::moment::SecondMoment;
// Java: import org.apache.commons.math3.stat.descriptive.moment.Variance
use crate::org::apache::commons::math3::stat::descriptive::moment::Variance;
// Java: import org.apache.commons.math3.stat.descriptive.rank.Max
use crate::org::apache::commons::math3::stat::descriptive::rank::Max;
// Java: import org.apache.commons.math3.stat.descriptive.rank.Min
use crate::org::apache::commons::math3::stat::descriptive::rank::Min;
// Java: import org.apache.commons.math3.stat.descriptive.summary.Sum
use crate::org::apache::commons::math3::stat::descriptive::summary::Sum;
// Java: import org.apache.commons.math3.stat.descriptive.summary.SumOfLogs
use crate::org::apache::commons::math3::stat::descriptive::summary::SumOfLogs;
// Java: import org.apache.commons.math3.stat.descriptive.summary.SumOfSquares
use crate::org::apache::commons::math3::stat::descriptive::summary::SumOfSquares;
// Java: import org.apache.commons.math3.util.MathUtils
use crate::org::apache::commons::math3::util::MathUtils;
// Java: import org.apache.commons.math3.util.Precision
use crate::org::apache::commons::math3::util::Precision;
// Java: import org.apache.commons.math3.util.FastMath
use crate::org::apache::commons::math3::util::FastMath;

// Java field: long serialVersionUID

// Java: public class can
pub struct can {
}

// Java: public class SummaryStatistics implements StatisticalSummary, Serializable
pub struct SummaryStatistics {
    // Java field: long n
    pub(crate) n: i64,
    // Java field: SecondMoment secondMoment
    pub(crate) secondMoment: Box<dyn std::any::Any>,
    // Java field: Sum sum
    sum: Option<Box<dyn std::any::Any>>,
    // Java field: SumOfSquares sumsq
    sumsq: Option<Box<dyn std::any::Any>>,
    // Java field: Min min
    min: Option<Box<dyn std::any::Any>>,
    // Java field: Max max
    max: Option<Box<dyn std::any::Any>>,
    // Java field: SumOfLogs sumLog
    sumLog: Option<Box<dyn std::any::Any>>,
    // Java field: GeometricMean geoMean
    geoMean: Option<Box<dyn std::any::Any>>,
    // Java field: Mean mean
    mean: Option<Box<dyn std::any::Any>>,
    // Java field: Variance variance
    variance: Option<Box<dyn std::any::Any>>,
    // Java field: StorelessUnivariateStatistic sumImpl
    pub(crate) sumImpl: Box<dyn std::any::Any>,
    // Java field: StorelessUnivariateStatistic sumsqImpl
    pub(crate) sumsqImpl: Box<dyn std::any::Any>,
    // Java field: StorelessUnivariateStatistic minImpl
    pub(crate) minImpl: Box<dyn std::any::Any>,
    // Java field: StorelessUnivariateStatistic maxImpl
    pub(crate) maxImpl: Box<dyn std::any::Any>,
    // Java field: StorelessUnivariateStatistic sumLogImpl
    pub(crate) sumLogImpl: Box<dyn std::any::Any>,
    // Java field: StorelessUnivariateStatistic geoMeanImpl
    pub(crate) geoMeanImpl: Box<dyn std::any::Any>,
    // Java field: StorelessUnivariateStatistic meanImpl
    pub(crate) meanImpl: Box<dyn std::any::Any>,
    // Java field: StorelessUnivariateStatistic varianceImpl
    pub(crate) varianceImpl: Box<dyn std::any::Any>,
}

// Java: public interface and
pub trait and {
}

// Java: public class can
impl can {
}

// Java: public class SummaryStatistics implements StatisticalSummary, Serializable
impl SummaryStatistics {
    // Java: SummaryStatistics()
    pub fn SummaryStatistics(&self) {
    }

    // Java: SummaryStatistics(SummaryStatistics original)
    pub fn SummaryStatistics_2(&self) {
        // Java: copy(original, this);
    }

    // Java: StatisticalSummary getSummary()
    pub fn getSummary(&self) -> Box<dyn std::any::Any> {
        // Java: return new StatisticalSummaryValues(getMean(), getVariance(), getN(),
                // Java: getMax(), getMin(), getSum());
        todo!()
    }

    // Java: void addValue(double value)
    pub fn addValue(&self) {
        // Java: sumImpl.increment(value);
        // Java: sumsqImpl.increment(value);
        // Java: minImpl.increment(value);
        // Java: maxImpl.increment(value);
        // Java: sumLogImpl.increment(value);
        // Java: secondMoment.increment(value);
        // Java: // If mean, variance or geomean have been overridden,
        // Java: // need to increment these
        // Java: if (meanImpl != mean)
        {
            // Java: meanImpl.increment(value);
        }
        // Java: if (varianceImpl != variance)
        {
            // Java: varianceImpl.increment(value);
        }
        // Java: if (geoMeanImpl != geoMean)
        {
            // Java: geoMeanImpl.increment(value);
        }
        // Java: n++;
    }

    // Java: long getN()
    pub fn getN(&self) -> i64 {
        // Java: return n;
        todo!()
    }

    // Java: double getSum()
    pub fn getSum(&self) -> f64 {
        // Java: return sumImpl.getResult();
        todo!()
    }

    // Java: double getSumsq()
    pub fn getSumsq(&self) -> f64 {
        // Java: return sumsqImpl.getResult();
        todo!()
    }

    // Java: double getMean()
    pub fn getMean(&self) -> f64 {
        // Java: return meanImpl.getResult();
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
        // Java: final long size = getN();
        // Java: return size > 0 ? FastMath.sqrt(getSumsq() / size) : Double.NaN;
        todo!()
    }

    // Java: double getVariance()
    pub fn getVariance(&self) -> f64 {
        // Java: return varianceImpl.getResult();
        todo!()
    }

    // Java: double getPopulationVariance()
    pub fn getPopulationVariance(&self) -> f64 {
        // Java: Variance populationVariance = new Variance(secondMoment);
        // Java: populationVariance.setBiasCorrected(false);
        // Java: return populationVariance.getResult();
        todo!()
    }

    // Java: double getMax()
    pub fn getMax(&self) -> f64 {
        // Java: return maxImpl.getResult();
        todo!()
    }

    // Java: double getMin()
    pub fn getMin(&self) -> f64 {
        // Java: return minImpl.getResult();
        todo!()
    }

    // Java: double getGeometricMean()
    pub fn getGeometricMean(&self) -> f64 {
        // Java: return geoMeanImpl.getResult();
        todo!()
    }

    // Java: double getSumOfLogs()
    pub fn getSumOfLogs(&self) -> f64 {
        // Java: return sumLogImpl.getResult();
        todo!()
    }

    // Java: double getSecondMoment()
    pub fn getSecondMoment(&self) -> f64 {
        // Java: return secondMoment.getResult();
        todo!()
    }

    // Java: String toString()
    pub fn toString(&self) -> String {
        // Java: StringBuilder outBuffer = new StringBuilder();
        // Java: String endl = "\n";
        // Java: outBuffer.append("SummaryStatistics:").append(endl);
        // Java: outBuffer.append("n: ").append(getN()).append(endl);
        // Java: outBuffer.append("min: ").append(getMin()).append(endl);
        // Java: outBuffer.append("max: ").append(getMax()).append(endl);
        // Java: outBuffer.append("sum: ").append(getSum()).append(endl);
        // Java: outBuffer.append("mean: ").append(getMean()).append(endl);
        // Java: outBuffer.append("geometric mean: ").append(getGeometricMean())
            // Java: .append(endl);
        // Java: outBuffer.append("variance: ").append(getVariance()).append(endl);
        // Java: outBuffer.append("population variance: ").append(getPopulationVariance()).append(endl);
        // Java: outBuffer.append("second moment: ").append(getSecondMoment()).append(endl);
        // Java: outBuffer.append("sum of squares: ").append(getSumsq()).append(endl);
        // Java: outBuffer.append("standard deviation: ").append(getStandardDeviation())
            // Java: .append(endl);
        // Java: outBuffer.append("sum of logs: ").append(getSumOfLogs()).append(endl);
        // Java: return outBuffer.toString();
        todo!()
    }

    // Java: void clear()
    pub fn clear(&self) {
        // Java: this.n = 0;
        // Java: minImpl.clear();
        // Java: maxImpl.clear();
        // Java: sumImpl.clear();
        // Java: sumLogImpl.clear();
        // Java: sumsqImpl.clear();
        // Java: geoMeanImpl.clear();
        // Java: secondMoment.clear();
        // Java: if (meanImpl != mean)
        {
            // Java: meanImpl.clear();
        }
        // Java: if (varianceImpl != variance)
        {
            // Java: varianceImpl.clear();
        }
    }

    // Java: boolean equals(Object object)
    pub fn equals(&self) -> bool {
        // Java: if (object == this)
        {
            // Java: return true;
        }
        // Java: if (object instanceof SummaryStatistics == false)
        {
            // Java: return false;
        }
        // Java: SummaryStatistics stat = (SummaryStatistics)object;
        // Java: return Precision.equalsIncludingNaN(stat.getGeometricMean(), getGeometricMean()) &&
               // Java: Precision.equalsIncludingNaN(stat.getMax(),           getMax())           &&
               // Java: Precision.equalsIncludingNaN(stat.getMean(),          getMean())          &&
               // Java: Precision.equalsIncludingNaN(stat.getMin(),           getMin())           &&
               // Java: Precision.equalsIncludingNaN(stat.getN(),             getN())             &&
               // Java: Precision.equalsIncludingNaN(stat.getSum(),           getSum())           &&
               // Java: Precision.equalsIncludingNaN(stat.getSumsq(),         getSumsq())         &&
               // Java: Precision.equalsIncludingNaN(stat.getVariance(),      getVariance());
        todo!()
    }

    // Java: int hashCode()
    pub fn hashCode(&self) -> i32 {
        // Java: int result = 31 + MathUtils.hash(getGeometricMean());
        // Java: result = result * 31 + MathUtils.hash(getGeometricMean());
        // Java: result = result * 31 + MathUtils.hash(getMax());
        // Java: result = result * 31 + MathUtils.hash(getMean());
        // Java: result = result * 31 + MathUtils.hash(getMin());
        // Java: result = result * 31 + MathUtils.hash(getN());
        // Java: result = result * 31 + MathUtils.hash(getSum());
        // Java: result = result * 31 + MathUtils.hash(getSumsq());
        // Java: result = result * 31 + MathUtils.hash(getVariance());
        // Java: return result;
        todo!()
    }

    // Java: StorelessUnivariateStatistic getSumImpl()
    pub fn getSumImpl(&self) -> Box<dyn std::any::Any> {
        // Java: return sumImpl;
        todo!()
    }

    // Java: void setSumImpl(StorelessUnivariateStatistic sumImpl)
    pub fn setSumImpl(&self) {
    }

    // Java: StorelessUnivariateStatistic getSumsqImpl()
    pub fn getSumsqImpl(&self) -> Box<dyn std::any::Any> {
        // Java: return sumsqImpl;
        todo!()
    }

    // Java: void setSumsqImpl(StorelessUnivariateStatistic sumsqImpl)
    pub fn setSumsqImpl(&self) {
    }

    // Java: StorelessUnivariateStatistic getMinImpl()
    pub fn getMinImpl(&self) -> Box<dyn std::any::Any> {
        // Java: return minImpl;
        todo!()
    }

    // Java: void setMinImpl(StorelessUnivariateStatistic minImpl)
    pub fn setMinImpl(&self) {
    }

    // Java: StorelessUnivariateStatistic getMaxImpl()
    pub fn getMaxImpl(&self) -> Box<dyn std::any::Any> {
        // Java: return maxImpl;
        todo!()
    }

    // Java: void setMaxImpl(StorelessUnivariateStatistic maxImpl)
    pub fn setMaxImpl(&self) {
    }

    // Java: StorelessUnivariateStatistic getSumLogImpl()
    pub fn getSumLogImpl(&self) -> Box<dyn std::any::Any> {
        // Java: return sumLogImpl;
        todo!()
    }

    // Java: void setSumLogImpl(StorelessUnivariateStatistic sumLogImpl)
    pub fn setSumLogImpl(&self) {
    }

    // Java: StorelessUnivariateStatistic getGeoMeanImpl()
    pub fn getGeoMeanImpl(&self) -> Box<dyn std::any::Any> {
        // Java: return geoMeanImpl;
        todo!()
    }

    // Java: void setGeoMeanImpl(StorelessUnivariateStatistic geoMeanImpl)
    pub fn setGeoMeanImpl(&self) {
    }

    // Java: StorelessUnivariateStatistic getMeanImpl()
    pub fn getMeanImpl(&self) -> Box<dyn std::any::Any> {
        // Java: return meanImpl;
        todo!()
    }

    // Java: void setMeanImpl(StorelessUnivariateStatistic meanImpl)
    pub fn setMeanImpl(&self) {
    }

    // Java: StorelessUnivariateStatistic getVarianceImpl()
    pub fn getVarianceImpl(&self) -> Box<dyn std::any::Any> {
        // Java: return varianceImpl;
        todo!()
    }

    // Java: void setVarianceImpl(StorelessUnivariateStatistic varianceImpl)
    pub fn setVarianceImpl(&self) {
    }

    // Java: void checkEmpty()
    pub fn checkEmpty(&self) {
        // Java: if (n > 0)
        {
            // Java: throw new MathIllegalStateException(
                // Java: LocalizedFormats.VALUES_ADDED_BEFORE_CONFIGURING_STATISTIC, n);
        }
    }

    // Java: SummaryStatistics copy()
    pub fn copy(&self) -> SummaryStatistics {
        // Java: SummaryStatistics result = new SummaryStatistics();
        // Java: // No try-catch or advertised exception because arguments are guaranteed non-null
        // Java: copy(this, result);
        // Java: return result;
        todo!()
    }

    // Java: static void copy(SummaryStatistics source, SummaryStatistics dest)
    pub fn copy_2(&self) {
    }

}


