// Java: import java.io.BufferedReader
// Java: import java.io.File
// Java: import java.io.FileInputStream
// Java: import java.io.IOException
// Java: import java.io.InputStream
// Java: import java.io.InputStreamReader
// Java: import java.net.URL
// Java: import java.nio.charset.Charset
// Java: import java.util.ArrayList
// Java: import java.util.List
// Java: import org.apache.commons.math3.distribution.AbstractRealDistribution
use crate::org::apache::commons::math3::distribution::AbstractRealDistribution;
// Java: import org.apache.commons.math3.distribution.ConstantRealDistribution
use crate::org::apache::commons::math3::distribution::ConstantRealDistribution;
// Java: import org.apache.commons.math3.distribution.NormalDistribution
use crate::org::apache::commons::math3::distribution::NormalDistribution;
// Java: import org.apache.commons.math3.distribution.RealDistribution
use crate::org::apache::commons::math3::distribution::RealDistribution;
// Java: import org.apache.commons.math3.exception.MathIllegalStateException
use crate::org::apache::commons::math3::exception::MathIllegalStateException;
// Java: import org.apache.commons.math3.exception.MathInternalError
use crate::org::apache::commons::math3::exception::MathInternalError;
// Java: import org.apache.commons.math3.exception.NullArgumentException
use crate::org::apache::commons::math3::exception::NullArgumentException;
// Java: import org.apache.commons.math3.exception.OutOfRangeException
use crate::org::apache::commons::math3::exception::OutOfRangeException;
// Java: import org.apache.commons.math3.exception.ZeroException
use crate::org::apache::commons::math3::exception::ZeroException;
// Java: import org.apache.commons.math3.exception.NotStrictlyPositiveException
use crate::org::apache::commons::math3::exception::NotStrictlyPositiveException;
// Java: import org.apache.commons.math3.exception.util.LocalizedFormats
use crate::org::apache::commons::math3::exception::util::LocalizedFormats;
// Java: import org.apache.commons.math3.stat.descriptive.StatisticalSummary
use crate::org::apache::commons::math3::stat::descriptive::StatisticalSummary;
// Java: import org.apache.commons.math3.stat.descriptive.SummaryStatistics
use crate::org::apache::commons::math3::stat::descriptive::SummaryStatistics;
// Java: import org.apache.commons.math3.util.FastMath
use crate::org::apache::commons::math3::util::FastMath;
// Java: import org.apache.commons.math3.util.MathUtils
use crate::org::apache::commons::math3::util::MathUtils;

// Java field: int DEFAULT_BIN_COUNT
pub(crate) const DEFAULT_BIN_COUNT: i32 = 0;
// Java field: String FILE_CHARSET
pub(crate) const FILE_CHARSET: &str = "UTF-8";
// Java field: long serialVersionUID

// Java: public class EmpiricalDistribution extends AbstractRealDistribution
pub struct EmpiricalDistribution {
    // Java field: RandomDataGenerator randomData
    pub(crate) randomData: Box<dyn std::any::Any>,
    // Java field: List<SummaryStatistics> binStats
    pub(crate) binStats: Box<dyn std::any::Any>,
    // Java field: SummaryStatistics sampleStats
    pub(crate) sampleStats: Box<dyn std::any::Any>,
    // Java field: double max
    pub(crate) max: f64,
    // Java field: double min
    pub(crate) min: f64,
    // Java field: double delta
    delta: f64,
    // Java field: int binCount
    pub(crate) binCount: i32,
    // Java field: boolean loaded
    pub(crate) loaded: bool,
    // Java field: double[] upperBounds
    upperBounds: Option<Vec<f64>>,
}

// Java: public class DataAdapter
pub struct DataAdapter {
}

// Java: public class StreamDataAdapter
pub struct StreamDataAdapter {
    // Java field: BufferedReader inputStream
    pub(crate) inputStream: Box<dyn std::any::Any>,
}

// Java: public class ArrayDataAdapter
pub struct ArrayDataAdapter {
    // Java field: double[] inputArray
    pub(crate) inputArray: Box<dyn std::any::Any>,
}

// Java: public class EmpiricalDistribution extends AbstractRealDistribution
impl EmpiricalDistribution {
    // Java: EmpiricalDistribution()
    pub fn EmpiricalDistribution(&self) {
        // Java: this(DEFAULT_BIN_COUNT);
    }

    // Java: EmpiricalDistribution(int binCount)
    pub fn EmpiricalDistribution_2(&self) {
        // Java: this(binCount, new RandomDataGenerator());
    }

    // Java: EmpiricalDistribution(int binCount, RandomGenerator generator)
    pub fn EmpiricalDistribution_3(&self) {
        // Java: this(binCount, new RandomDataGenerator(generator));
    }

    // Java: EmpiricalDistribution(RandomGenerator generator)
    pub fn EmpiricalDistribution_4(&self) {
        // Java: this(DEFAULT_BIN_COUNT, generator);
    }

    // Java: EmpiricalDistribution(int binCount, RandomDataImpl randomData)
    pub fn EmpiricalDistribution_5(&self) {
        // Java: this(binCount, randomData.getDelegate());
    }

    // Java: EmpiricalDistribution(RandomDataImpl randomData)
    pub fn EmpiricalDistribution_6(&self) {
        // Java: this(DEFAULT_BIN_COUNT, randomData);
    }

    // Java: EmpiricalDistribution()
    pub fn EmpiricalDistribution_7(&self) {
    }

    // Java: void load(double[] in)
    pub fn load(&self) {
        // Java: DataAdapter da = new ArrayDataAdapter(in);
        // Java: try
        {
            // Java: da.computeStats();
            // Java: // new adapter for the second pass
            // Java: fillBinStats(new ArrayDataAdapter(in));
        }
        // Java: catch (IOException ex)
        {
            // Java: // Can't happen
            // Java: throw new MathInternalError();
        }
        // Java: loaded = true;
    }

    // Java: void load(URL url)
    pub fn load_2(&self) {
        // Java: MathUtils.checkNotNull(url);
        // Java: Charset charset = Charset.forName(FILE_CHARSET);
        // Java: BufferedReader in =
            // Java: new BufferedReader(new InputStreamReader(url.openStream(), charset));
        // Java: try
        {
            // Java: DataAdapter da = new StreamDataAdapter(in);
            // Java: da.computeStats();
            // Java: if (sampleStats.getN() == 0)
            {
                // Java: throw new ZeroException(LocalizedFormats.URL_CONTAINS_NO_DATA, url);
            }
            // Java: // new adapter for the second pass
            // Java: in = new BufferedReader(new InputStreamReader(url.openStream(), charset));
            // Java: fillBinStats(new StreamDataAdapter(in));
            // Java: loaded = true;
        }
        // Java: finally
        {
           // Java: try
           {
               // Java: in.close();
           }
               // Java: // ignore
        }
    }

    // Java: void load(File file)
    pub fn load_3(&self) {
        // Java: MathUtils.checkNotNull(file);
        // Java: Charset charset = Charset.forName(FILE_CHARSET);
        // Java: InputStream is = new FileInputStream(file);
        // Java: BufferedReader in = new BufferedReader(new InputStreamReader(is, charset));
        // Java: try
        {
            // Java: DataAdapter da = new StreamDataAdapter(in);
            // Java: da.computeStats();
            // Java: // new adapter for second pass
            // Java: is = new FileInputStream(file);
            // Java: in = new BufferedReader(new InputStreamReader(is, charset));
            // Java: fillBinStats(new StreamDataAdapter(in));
            // Java: loaded = true;
        }
        // Java: finally
        {
            // Java: try
            {
                // Java: in.close();
            }
                // Java: // ignore
        }
    }

    // Java: void fillBinStats(final DataAdapter da)
    pub fn fillBinStats(&self) {
    }

    // Java: int findBin(double value)
    pub fn findBin(&self) -> i32 {
        // Java: return FastMath.min(
                // Java: FastMath.max((int) FastMath.ceil((value - min) / delta) - 1, 0),
                // Java: binCount - 1);
        todo!()
    }

    // Java: double getNextValue()
    pub fn getNextValue(&self) -> f64 {
        // Java: if (!loaded)
        {
            // Java: throw new MathIllegalStateException(LocalizedFormats.DISTRIBUTION_NOT_LOADED);
        }
        // Java: return sample();
        todo!()
    }

    // Java: StatisticalSummary getSampleStats()
    pub fn getSampleStats(&self) -> Box<dyn std::any::Any> {
        // Java: return sampleStats;
        todo!()
    }

    // Java: int getBinCount()
    pub fn getBinCount(&self) -> i32 {
        // Java: return binCount;
        todo!()
    }

    // Java: List<SummaryStatistics> getBinStats()
    pub fn getBinStats(&self) -> Box<dyn std::any::Any> {
        // Java: return binStats;
        todo!()
    }

    // Java: double[] getUpperBounds()
    pub fn getUpperBounds(&self) -> Vec<f64> {
        // Java: double[] binUpperBounds = new double[binCount];
        // Java: for (int i = 0; i < binCount - 1; i++)
        {
            // Java: binUpperBounds[i] = min + delta * (i + 1);
        }
        // Java: binUpperBounds[binCount - 1] = max;
        // Java: return binUpperBounds;
        todo!()
    }

    // Java: double[] getGeneratorUpperBounds()
    pub fn getGeneratorUpperBounds(&self) -> Vec<f64> {
        // Java: int len = upperBounds.length;
        // Java: double[] out = new double[len];
        // Java: System.arraycopy(upperBounds, 0, out, 0, len);
        // Java: return out;
        todo!()
    }

    // Java: boolean isLoaded()
    pub fn isLoaded(&self) -> bool {
        // Java: return loaded;
        todo!()
    }

    // Java: void reSeed(long seed)
    pub fn reSeed(&self) {
        // Java: randomData.reSeed(seed);
    }

    // Java: double probability(double x)
    pub fn probability(&self) -> f64 {
        // Java: return 0;
        todo!()
    }

    // Java: double density(double x)
    pub fn density(&self) -> f64 {
        // Java: if (x < min || x > max)
        {
            // Java: return 0d;
        }
        // Java: final int binIndex = findBin(x);
        // Java: final RealDistribution kernel = getKernel(binStats.get(binIndex));
        // Java: return kernel.density(x) * pB(binIndex) / kB(binIndex);
        todo!()
    }

    // Java: double cumulativeProbability(double x)
    pub fn cumulativeProbability(&self) -> f64 {
        // Java: if (x < min)
        {
            // Java: return 0d;
        }
        // Java: else if (x >= max)
        {
            // Java: return 1d;
        }
        // Java: final int binIndex = findBin(x);
        // Java: final double pBminus = pBminus(binIndex);
        // Java: final double pB = pB(binIndex);
        // Java: final RealDistribution kernel = k(x);
        // Java: if (kernel instanceof ConstantRealDistribution)
        {
            // Java: if (x < kernel.getNumericalMean())
            {
                // Java: return pBminus;
            }
            // Java: else
            {
                // Java: return pBminus + pB;
            }
        }
        // Java: final double[] binBounds = getUpperBounds();
        // Java: final double kB = kB(binIndex);
        // Java: final double lower = binIndex == 0 ? min : binBounds[binIndex - 1];
        // Java: final double withinBinCum =
            // Java: (kernel.cumulativeProbability(x) -  kernel.cumulativeProbability(lower)) / kB;
        // Java: return pBminus + pB * withinBinCum;
        todo!()
    }

    // Java: double inverseCumulativeProbability(final double p)
    pub fn inverseCumulativeProbability(&self) -> f64 {
        // Java: if (p < 0.0 || p > 1.0)
        {
            // Java: throw new OutOfRangeException(p, 0, 1);
        }
        // Java: if (p == 0.0)
        {
            // Java: return getSupportLowerBound();
        }
        // Java: if (p == 1.0)
        {
            // Java: return getSupportUpperBound();
        }
        // Java: int i = 0;
        // Java: while (cumBinP(i) < p)
        {
            // Java: i++;
        }
        // Java: final RealDistribution kernel = getKernel(binStats.get(i));
        // Java: final double kB = kB(i);
        // Java: final double[] binBounds = getUpperBounds();
        // Java: final double lower = i == 0 ? min : binBounds[i - 1];
        // Java: final double kBminus = kernel.cumulativeProbability(lower);
        // Java: final double pB = pB(i);
        // Java: final double pBminus = pBminus(i);
        // Java: final double pCrit = p - pBminus;
        // Java: if (pCrit <= 0)
        {
            // Java: return lower;
        }
        // Java: return kernel.inverseCumulativeProbability(kBminus + pCrit * kB / pB);
        todo!()
    }

    // Java: double getNumericalMean()
    pub fn getNumericalMean(&self) -> f64 {
       // Java: return sampleStats.getMean();
        todo!()
    }

    // Java: double getNumericalVariance()
    pub fn getNumericalVariance(&self) -> f64 {
        // Java: return sampleStats.getVariance();
        todo!()
    }

    // Java: double getSupportLowerBound()
    pub fn getSupportLowerBound(&self) -> f64 {
       // Java: return min;
        todo!()
    }

    // Java: double getSupportUpperBound()
    pub fn getSupportUpperBound(&self) -> f64 {
        // Java: return max;
        todo!()
    }

    // Java: boolean isSupportLowerBoundInclusive()
    pub fn isSupportLowerBoundInclusive(&self) -> bool {
        // Java: return true;
        todo!()
    }

    // Java: boolean isSupportUpperBoundInclusive()
    pub fn isSupportUpperBoundInclusive(&self) -> bool {
        // Java: return true;
        todo!()
    }

    // Java: boolean isSupportConnected()
    pub fn isSupportConnected(&self) -> bool {
        // Java: return true;
        todo!()
    }

    // Java: void reseedRandomGenerator(long seed)
    pub fn reseedRandomGenerator(&self) {
        // Java: randomData.reSeed(seed);
    }

    // Java: double pB(int i)
    pub fn pB(&self) -> f64 {
        // Java: return i == 0 ? upperBounds[0] :
            // Java: upperBounds[i] - upperBounds[i - 1];
        todo!()
    }

    // Java: double pBminus(int i)
    pub fn pBminus(&self) -> f64 {
        // Java: return i == 0 ? 0 : upperBounds[i - 1];
        todo!()
    }

    // Java: double kB(int i)
    pub fn kB(&self) -> f64 {
        // Java: final double[] binBounds = getUpperBounds();
        // Java: final RealDistribution kernel = getKernel(binStats.get(i));
        // Java: return i == 0 ? kernel.cumulativeProbability(min, binBounds[0]) :
            // Java: kernel.cumulativeProbability(binBounds[i - 1], binBounds[i]);
        todo!()
    }

    // Java: RealDistribution k(double x)
    pub fn k(&self) -> Box<dyn std::any::Any> {
        // Java: final int binIndex = findBin(x);
        // Java: return getKernel(binStats.get(binIndex));
        todo!()
    }

    // Java: double cumBinP(int binIndex)
    pub fn cumBinP(&self) -> f64 {
        // Java: return upperBounds[binIndex];
        todo!()
    }

    // Java: RealDistribution getKernel(SummaryStatistics bStats)
    pub fn getKernel(&self) -> Box<dyn std::any::Any> {
        // Java: if (bStats.getN() == 1 || bStats.getVariance() == 0)
        {
            // Java: return new ConstantRealDistribution(bStats.getMean());
        }
        // Java: else
        {
            // Java: return new NormalDistribution(randomData.getRandomGenerator(),
                // Java: bStats.getMean(), bStats.getStandardDeviation(),
                // Java: NormalDistribution.DEFAULT_INVERSE_ABSOLUTE_ACCURACY);
        }
        todo!()
    }

}

// Java: public class DataAdapter
impl DataAdapter {
    // Java: abstract void computeBinStats()
    pub fn computeBinStats(&self) {
    }

    // Java: abstract void computeStats()
    pub fn computeStats(&self) {
    }

}

// Java: public class StreamDataAdapter
impl StreamDataAdapter {
    // Java: void computeBinStats()
    pub fn computeBinStats_2(&self) {
            // Java: String str = null;
            // Java: double val = 0.0d;
            // Java: while ((str = inputStream.readLine()) != null)
            {
                // Java: val = Double.parseDouble(str);
                // Java: SummaryStatistics stats = binStats.get(findBin(val));
                // Java: stats.addValue(val);
            }
            // Java: inputStream.close();
            // Java: inputStream = null;
    }

    // Java: void computeStats()
    pub fn computeStats_2(&self) {
            // Java: String str = null;
            // Java: double val = 0.0;
            // Java: sampleStats = new SummaryStatistics();
            // Java: while ((str = inputStream.readLine()) != null)
            {
                // Java: val = Double.parseDouble(str);
                // Java: sampleStats.addValue(val);
            }
            // Java: inputStream.close();
            // Java: inputStream = null;
    }

}

// Java: public class ArrayDataAdapter
impl ArrayDataAdapter {
    // Java: void computeStats()
    pub fn computeStats_3(&self) {
            // Java: sampleStats = new SummaryStatistics();
            // Java: for (int i = 0; i < inputArray.length; i++)
            {
                // Java: sampleStats.addValue(inputArray[i]);
            }
    }

    // Java: void computeBinStats()
    pub fn computeBinStats_3(&self) {
            // Java: for (int i = 0; i < inputArray.length; i++)
            {
                // Java: SummaryStatistics stats =
                    // Java: binStats.get(findBin(inputArray[i]));
                // Java: stats.addValue(inputArray[i]);
            }
    }

}
