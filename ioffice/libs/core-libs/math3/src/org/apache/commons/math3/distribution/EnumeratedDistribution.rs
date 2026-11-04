// Java: import java.io.Serializable
// Java: import java.lang.reflect.Array
// Java: import java.util.ArrayList
// Java: import java.util.Arrays
// Java: import java.util.List
// Java: import org.apache.commons.math3.exception.MathArithmeticException
use crate::org::apache::commons::math3::exception::MathArithmeticException;
// Java: import org.apache.commons.math3.exception.NotANumberException
use crate::org::apache::commons::math3::exception::NotANumberException;
// Java: import org.apache.commons.math3.exception.NotFiniteNumberException
use crate::org::apache::commons::math3::exception::NotFiniteNumberException;
// Java: import org.apache.commons.math3.exception.NotPositiveException
use crate::org::apache::commons::math3::exception::NotPositiveException;
// Java: import org.apache.commons.math3.exception.NotStrictlyPositiveException
use crate::org::apache::commons::math3::exception::NotStrictlyPositiveException;
// Java: import org.apache.commons.math3.exception.NullArgumentException
use crate::org::apache::commons::math3::exception::NullArgumentException;
// Java: import org.apache.commons.math3.exception.util.LocalizedFormats
use crate::org::apache::commons::math3::exception::util::LocalizedFormats;
// Java: import org.apache.commons.math3.random.RandomGenerator
use crate::org::apache::commons::math3::random::RandomGenerator::RandomGenerator;
// Java: import org.apache.commons.math3.random.Well19937c
use crate::org::apache::commons::math3::random::Well19937c;
// Java: import org.apache.commons.math3.util.MathArrays
use crate::org::apache::commons::math3::util::MathArrays;
// Java: import org.apache.commons.math3.util.Pair
use crate::org::apache::commons::math3::util::Pair;

// Java field: long serialVersionUID
pub(crate) const serialVersionUID: i64 = 0;

// Java: public class EnumeratedDistribution implements Serializable
pub struct EnumeratedDistribution<T> {
    // Java field: RandomGenerator random
    pub(crate) random: Box<dyn RandomGenerator>,
    // Java field: List<T> singletons
    pub(crate) singletons: Vec<T>,
    // Java field: double[] probabilities
    probabilities: Vec<f64>,
    // Java field: double[] cumulativeProbabilities
    cumulativeProbabilities: Vec<f64>,
}

// Java: public class EnumeratedDistribution implements Serializable
impl<T: Clone> EnumeratedDistribution<T> {
    // Java: EnumeratedDistribution(final List<Pair<T, Double>> pmf)
    pub fn EnumeratedDistribution(pmf: &[(T, f64)]) -> Self {
        todo!();
    }

    // Java: EnumeratedDistribution(final RandomGenerator rng, final List<Pair<T, Double>> pmf)
    pub fn EnumeratedDistribution_1(rng: Box<dyn RandomGenerator>, pmf: &[(T, f64)]) -> Self {
        todo!();
    }

    // Java: void reseedRandomGenerator(long seed)
    pub fn reseedRandomGenerator(&self, seed: i64) {
        // Java: random.setSeed(seed);
    }

    // Java: double probability(final T x)
    pub fn probability(&self, x: &T) -> f64 {
        // Java: double probability = 0;
        // Java: for (int i = 0; i < probabilities.length; i++)
        {
            // Java: if ((x == null && singletons.get(i) == null) ||
                // Java: (x != null && x.equals(singletons.get(i))))
                {
                // Java: probability += probabilities[i];
                }
        }
        // Java: return probability;
        todo!();
    }

    // Java: public List<Pair<T, Double>> getPmf()
    pub fn getPmf(&self) -> Vec<(T, f64)> {
        // Java: final List<Pair<T, Double>> samples = new ArrayList<Pair<T, Double>>(probabilities.length);
        // Java: for (int i = 0; i < probabilities.length; i++)
        {
            // Java: samples.add(new Pair<T, Double>(singletons.get(i), probabilities[i]));
        }
        // Java: return samples;
        todo!();
    }

    // Java: T sample()
    pub fn sample(&self) -> T {
        // Java: final double randomValue = random.nextDouble();
        // Java: int index = Arrays.binarySearch(cumulativeProbabilities, randomValue);
        // Java: if (index < 0)
        {
            // Java: index = -index-1;
        }
        // Java: if (index >= 0 &&
            // Java: index < probabilities.length &&
            // Java: randomValue < cumulativeProbabilities[index])
            {
            // Java: return singletons.get(index);
            }
        // Java: /* This should never happen, but it ensures we will return a correct
         // Java: * object in case there is some floating point inequality problem
         // Java: * wrt the cumulative probabilities. */
        // Java: return singletons.get(singletons.size() - 1);
        todo!();
    }

    // Java: Object[] sample(int sampleSize)
    pub fn sample_1(&self, sampleSize: i32) -> Vec<T> {
        // Java: if (sampleSize <= 0)
        {
            // Java: throw new NotStrictlyPositiveException(LocalizedFormats.NUMBER_OF_SAMPLES,
                    // Java: sampleSize);
        }
        // Java: final Object[] out = new Object[sampleSize];
        // Java: for (int i = 0; i < sampleSize; i++)
        {
            // Java: out[i] = sample();
        }
        // Java: return out;
        todo!();
    }

    // Java: T[] sample(int sampleSize, final T[] array)
    pub fn sample_2(&self, sampleSize: i32, array: &[T]) -> Vec<T> {
        // Java: if (sampleSize <= 0)
        {
            // Java: throw new NotStrictlyPositiveException(LocalizedFormats.NUMBER_OF_SAMPLES, sampleSize);
        }
        // Java: if (array == null)
        {
            // Java: throw new NullArgumentException(LocalizedFormats.INPUT_ARRAY);
        }
        // Java: T[] out;
        // Java: if (array.length < sampleSize)
        {
            // Java: @SuppressWarnings("unchecked") // safe as both are of type T
            // Java: final T[] unchecked = (T[]) Array.newInstance(array.getClass().getComponentType(), sampleSize);
            // Java: out = unchecked;
        }
        // Java: else
        {
            // Java: out = array;
        }
        // Java: for (int i = 0; i < sampleSize; i++)
        {
            // Java: out[i] = sample();
        }
        // Java: return out;
        todo!();
    }

}
