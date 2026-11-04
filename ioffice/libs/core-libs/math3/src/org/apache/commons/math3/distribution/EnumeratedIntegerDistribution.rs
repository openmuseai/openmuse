// Java: import java.util.ArrayList
// Java: import java.util.HashMap
// Java: import java.util.List
// Java: import java.util.Map
// Java: import java.util.Map.Entry
// Java: import org.apache.commons.math3.exception.DimensionMismatchException
use crate::org::apache::commons::math3::exception::DimensionMismatchException;
// Java: import org.apache.commons.math3.exception.MathArithmeticException
use crate::org::apache::commons::math3::exception::MathArithmeticException;
// Java: import org.apache.commons.math3.exception.NotANumberException
use crate::org::apache::commons::math3::exception::NotANumberException;
// Java: import org.apache.commons.math3.exception.NotFiniteNumberException
use crate::org::apache::commons::math3::exception::NotFiniteNumberException;
// Java: import org.apache.commons.math3.exception.NotPositiveException
use crate::org::apache::commons::math3::exception::NotPositiveException;
// Java: import org.apache.commons.math3.random.RandomGenerator
use crate::org::apache::commons::math3::random::RandomGenerator::RandomGenerator;
// Java: import org.apache.commons.math3.random.Well19937c
use crate::org::apache::commons::math3::random::Well19937c;
// Java: import org.apache.commons.math3.util.Pair
use crate::org::apache::commons::math3::util::Pair;

// Java field: long serialVersionUID
pub(crate) const serialVersionUID: i64 = 0;

// Java: public class EnumeratedIntegerDistribution extends AbstractIntegerDistribution
pub struct EnumeratedIntegerDistribution {
    // Java field: EnumeratedDistribution<Integer> innerDistribution
    pub(crate) innerDistribution: super::EnumeratedDistribution::EnumeratedDistribution<i32>,
}

// Java: public class EnumeratedIntegerDistribution extends AbstractIntegerDistribution
impl EnumeratedIntegerDistribution {
    // Java: EnumeratedIntegerDistribution(final int[] singletons, final double[] probabilities)
    pub fn EnumeratedIntegerDistribution(&self, singletons: &[i32], probabilities: &[f64]) {
    }

    // Java: EnumeratedIntegerDistribution()
    pub fn EnumeratedIntegerDistribution_1(&self) {
    }

    // Java: EnumeratedIntegerDistribution(final RandomGenerator rng, final int[] data)
    pub fn EnumeratedIntegerDistribution_2(&self, rng: &dyn RandomGenerator, data: &[i32]) {
        // Java: super(rng);
        // Java: final Map<Integer, Integer> dataMap = new HashMap<Integer, Integer>();
        // Java: for (int value : data)
        {
            // Java: Integer count = dataMap.get(value);
            // Java: if (count == null)
            {
                // Java: count = 0;
            }
            // Java: dataMap.put(value, ++count);
        }
        // Java: final int massPoints = dataMap.size();
        // Java: final double denom = data.length;
        // Java: final int[] values = new int[massPoints];
        // Java: final double[] probabilities = new double[massPoints];
        // Java: int index = 0;
        // Java: for (Entry<Integer, Integer> entry : dataMap.entrySet())
        {
            // Java: values[index] = entry.getKey();
            // Java: probabilities[index] = entry.getValue().intValue() / denom;
            // Java: index++;
        }
        // Java: innerDistribution = new EnumeratedDistribution<Integer>(rng, createDistribution(values, probabilities));
    }

    // Java: EnumeratedIntegerDistribution(final int[] data)
    pub fn EnumeratedIntegerDistribution_3(&self, data: &[i32]) {
        // Java: this(new Well19937c(), data);
    }

    // Java: static private static List<Pair<Integer, Double>> createDistribution(int[] singletons, double[] probabilities)
    pub fn createDistribution(&self, singletons: &[i32], probabilities: &[f64]) -> Vec<(i32, f64)> {
        // Java: if (singletons.length != probabilities.length)
        {
            // Java: throw new DimensionMismatchException(probabilities.length, singletons.length);
        }
        // Java: final List<Pair<Integer, Double>> samples = new ArrayList<Pair<Integer, Double>>(singletons.length);
        // Java: for (int i = 0; i < singletons.length; i++)
        {
            // Java: samples.add(new Pair<Integer, Double>(singletons[i], probabilities[i]));
        }
        // Java: return samples;
        todo!();
    }

    // Java: double probability(final int x)
    pub fn probability(&self, x: i32) -> f64 {
        // Java: return innerDistribution.probability(x);
        todo!();
    }

    // Java: double cumulativeProbability(final int x)
    pub fn cumulativeProbability(&self, x: i32) -> f64 {
        // Java: double probability = 0;
        // Java: for (final Pair<Integer, Double> sample : innerDistribution.getPmf())
        {
            // Java: if (sample.getKey() <= x)
            {
                // Java: probability += sample.getValue();
            }
        }
        // Java: return probability;
        todo!();
    }

    // Java: double getNumericalMean()
    pub fn getNumericalMean(&self) -> f64 {
        // Java: double mean = 0;
        // Java: for (final Pair<Integer, Double> sample : innerDistribution.getPmf())
        {
            // Java: mean += sample.getValue() * sample.getKey();
        }
        // Java: return mean;
        todo!();
    }

    // Java: double getNumericalVariance()
    pub fn getNumericalVariance(&self) -> f64 {
        // Java: double mean = 0;
        // Java: double meanOfSquares = 0;
        // Java: for (final Pair<Integer, Double> sample : innerDistribution.getPmf())
        {
            // Java: mean += sample.getValue() * sample.getKey();
            // Java: meanOfSquares += sample.getValue() * sample.getKey() * sample.getKey();
        }
        // Java: return meanOfSquares - mean * mean;
        todo!();
    }

    // Java: int getSupportLowerBound()
    pub fn getSupportLowerBound(&self) -> i32 {
        // Java: int min = Integer.MAX_VALUE;
        // Java: for (final Pair<Integer, Double> sample : innerDistribution.getPmf())
        {
            // Java: if (sample.getKey() < min && sample.getValue() > 0)
            {
                // Java: min = sample.getKey();
            }
        }
        // Java: return min;
        todo!();
    }

    // Java: int getSupportUpperBound()
    pub fn getSupportUpperBound(&self) -> i32 {
        // Java: int max = Integer.MIN_VALUE;
        // Java: for (final Pair<Integer, Double> sample : innerDistribution.getPmf())
        {
            // Java: if (sample.getKey() > max && sample.getValue() > 0)
            {
                // Java: max = sample.getKey();
            }
        }
        // Java: return max;
        todo!();
    }

    // Java: boolean isSupportConnected()
    pub fn isSupportConnected(&self) -> bool {
        // Java: return true;
        todo!();
    }

    // Java: int sample()
    pub fn sample(&self) -> i32 {
        // Java: return innerDistribution.sample();
        todo!();
    }

}
