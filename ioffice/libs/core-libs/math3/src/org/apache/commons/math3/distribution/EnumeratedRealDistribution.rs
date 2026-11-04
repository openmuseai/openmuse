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
// Java: import org.apache.commons.math3.exception.OutOfRangeException
use crate::org::apache::commons::math3::exception::OutOfRangeException;
// Java: import org.apache.commons.math3.random.RandomGenerator
use crate::org::apache::commons::math3::random::RandomGenerator::RandomGenerator;
// Java: import org.apache.commons.math3.random.Well19937c
use crate::org::apache::commons::math3::random::Well19937c;
// Java: import org.apache.commons.math3.util.Pair
use crate::org::apache::commons::math3::util::Pair;

// Java field: long serialVersionUID
pub(crate) const serialVersionUID: i64 = 0;

// Java: public class EnumeratedRealDistribution extends AbstractRealDistribution
pub struct EnumeratedRealDistribution {
    // Java field: EnumeratedDistribution<Double> innerDistribution
    pub(crate) innerDistribution: super::EnumeratedDistribution::EnumeratedDistribution<f64>,
}

// Java: public class EnumeratedRealDistribution extends AbstractRealDistribution
impl EnumeratedRealDistribution {
    // Java: EnumeratedRealDistribution(final double[] singletons, final double[] probabilities)
    pub fn EnumeratedRealDistribution(&self, singletons: &[f64], probabilities: &[f64]) {
    }

    // Java: EnumeratedRealDistribution()
    pub fn EnumeratedRealDistribution_1(&self) {
    }

    // Java: EnumeratedRealDistribution(final RandomGenerator rng, final double[] data)
    pub fn EnumeratedRealDistribution_2(&self, rng: &dyn RandomGenerator, data: &[f64]) {
        // Java: super(rng);
        // Java: final Map<Double, Integer> dataMap = new HashMap<Double, Integer>();
        // Java: for (double value : data)
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
        // Java: final double[] values = new double[massPoints];
        // Java: final double[] probabilities = new double[massPoints];
        // Java: int index = 0;
        // Java: for (Entry<Double, Integer> entry : dataMap.entrySet())
        {
            // Java: values[index] = entry.getKey();
            // Java: probabilities[index] = entry.getValue().intValue() / denom;
            // Java: index++;
        }
        // Java: innerDistribution = new EnumeratedDistribution<Double>(rng, createDistribution(values, probabilities));
    }

    // Java: EnumeratedRealDistribution(final double[] data)
    pub fn EnumeratedRealDistribution_3(&self, data: &[f64]) {
        // Java: this(new Well19937c(), data);
    }

    // Java: static private static List<Pair<Double, Double>> createDistribution(double[] singletons, double[] probabilities)
    pub fn createDistribution(&self, singletons: &[f64], probabilities: &[f64]) -> Vec<(f64, f64)> {
        // Java: if (singletons.length != probabilities.length)
        {
            // Java: throw new DimensionMismatchException(probabilities.length, singletons.length);
        }
        // Java: final List<Pair<Double, Double>> samples = new ArrayList<Pair<Double, Double>>(singletons.length);
        // Java: for (int i = 0; i < singletons.length; i++)
        {
            // Java: samples.add(new Pair<Double, Double>(singletons[i], probabilities[i]));
        }
        // Java: return samples;
        todo!();
    }

    // Java: double probability(final double x)
    pub fn probability(&self, x: f64) -> f64 {
        // Java: return innerDistribution.probability(x);
        todo!();
    }

    // Java: double density(final double x)
    pub fn density(&self, x: f64) -> f64 {
        // Java: return probability(x);
        todo!();
    }

    // Java: double cumulativeProbability(final double x)
    pub fn cumulativeProbability(&self, x: f64) -> f64 {
        // Java: double probability = 0;
        // Java: for (final Pair<Double, Double> sample : innerDistribution.getPmf())
        {
            // Java: if (sample.getKey() <= x)
            {
                // Java: probability += sample.getValue();
            }
        }
        // Java: return probability;
        todo!();
    }

    // Java: double inverseCumulativeProbability(final double p)
    pub fn inverseCumulativeProbability(&self, p: f64) -> f64 {
        // Java: if (p < 0.0 || p > 1.0)
        {
            // Java: throw new OutOfRangeException(p, 0, 1);
        }
        // Java: double probability = 0;
        // Java: double x = getSupportLowerBound();
        // Java: for (final Pair<Double, Double> sample : innerDistribution.getPmf())
        {
            // Java: if (sample.getValue() == 0.0)
            {
                // Java: continue;
            }
            // Java: probability += sample.getValue();
            // Java: x = sample.getKey();
            // Java: if (probability >= p)
            {
                // Java: break;
            }
        }
        // Java: return x;
        todo!();
    }

    // Java: double getNumericalMean()
    pub fn getNumericalMean(&self) -> f64 {
        // Java: double mean = 0;
        // Java: for (final Pair<Double, Double> sample : innerDistribution.getPmf())
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
        // Java: for (final Pair<Double, Double> sample : innerDistribution.getPmf())
        {
            // Java: mean += sample.getValue() * sample.getKey();
            // Java: meanOfSquares += sample.getValue() * sample.getKey() * sample.getKey();
        }
        // Java: return meanOfSquares - mean * mean;
        todo!();
    }

    // Java: double getSupportLowerBound()
    pub fn getSupportLowerBound(&self) -> f64 {
        // Java: double min = Double.POSITIVE_INFINITY;
        // Java: for (final Pair<Double, Double> sample : innerDistribution.getPmf())
        {
            // Java: if (sample.getKey() < min && sample.getValue() > 0)
            {
                // Java: min = sample.getKey();
            }
        }
        // Java: return min;
        todo!();
    }

    // Java: double getSupportUpperBound()
    pub fn getSupportUpperBound(&self) -> f64 {
        // Java: double max = Double.NEGATIVE_INFINITY;
        // Java: for (final Pair<Double, Double> sample : innerDistribution.getPmf())
        {
            // Java: if (sample.getKey() > max && sample.getValue() > 0)
            {
                // Java: max = sample.getKey();
            }
        }
        // Java: return max;
        todo!();
    }

    // Java: boolean isSupportLowerBoundInclusive()
    pub fn isSupportLowerBoundInclusive(&self) -> bool {
        // Java: return true;
        todo!();
    }

    // Java: boolean isSupportUpperBoundInclusive()
    pub fn isSupportUpperBoundInclusive(&self) -> bool {
        // Java: return true;
        todo!();
    }

    // Java: boolean isSupportConnected()
    pub fn isSupportConnected(&self) -> bool {
        // Java: return true;
        todo!();
    }

    // Java: double sample()
    pub fn sample(&self) -> f64 {
        // Java: return innerDistribution.sample();
        todo!();
    }

}
