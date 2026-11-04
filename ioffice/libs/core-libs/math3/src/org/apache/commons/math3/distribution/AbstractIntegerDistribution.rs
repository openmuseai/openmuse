#![allow(unused_imports, dead_code, non_snake_case)]
// Java: import java.io.Serializable
// Java: import org.apache.commons.math3.exception.MathInternalError
use crate::org::apache::commons::math3::exception::MathInternalError;
// Java: import org.apache.commons.math3.exception.NotStrictlyPositiveException
use crate::org::apache::commons::math3::exception::NotStrictlyPositiveException;
// Java: import org.apache.commons.math3.exception.NumberIsTooLargeException
use crate::org::apache::commons::math3::exception::NumberIsTooLargeException;
// Java: import org.apache.commons.math3.exception.OutOfRangeException
use crate::org::apache::commons::math3::exception::OutOfRangeException;
// Java: import org.apache.commons.math3.exception.util.LocalizedFormats
use crate::org::apache::commons::math3::exception::util::LocalizedFormats;
// Java: import org.apache.commons.math3.random.RandomGenerator
use crate::org::apache::commons::math3::random::RandomGenerator;
// Java: import org.apache.commons.math3.util.FastMath
use crate::org::apache::commons::math3::util::FastMath;

// Java field: long serialVersionUID
pub(crate) const SERIAL_VERSION_UID: i64 = -1146319659338487221;

// Java: public class for
pub struct _AbstractIntegerDistributionDoc;

// Java: public class AbstractIntegerDistribution
pub struct AbstractIntegerDistribution {
    // Java field: org.apache.commons.math3.random.RandomDataImpl randomData
    // Java field: RandomGenerator random
}

// Java: public class AbstractIntegerDistribution
impl AbstractIntegerDistribution {
    // Java: AbstractIntegerDistribution()
    pub fn new() -> Self {
        // Java: // Legacy users are only allowed to access the deprecated "randomData".
        // Java: // New users are forbidden to use this constructor.
        // Java: random = null;
        AbstractIntegerDistribution {}
    }

    // Java: AbstractIntegerDistribution(RandomGenerator rng)
    pub fn new_with_rng(_rng: &dyn RandomGenerator::RandomGenerator) -> Self {
        // Java: random = rng;
        AbstractIntegerDistribution {}
    }

    // Java: double cumulativeProbability(int x0, int x1)
    pub fn cumulativeProbability_range(&self, x0: i32, x1: i32) -> f64 {
        // Java: if (x1 < x0)
        if x1 < x0 {
            // Java: throw new NumberIsTooLargeException(LocalizedFormats.LOWER_ENDPOINT_ABOVE_UPPER_ENDPOINT,
            // Java: x0, x1, true);
            panic!("lower endpoint above upper endpoint: {} > {}", x0, x1);
        }
        // Java: return cumulativeProbability(x1) - cumulativeProbability(x0);
        self.cumulativeProbability(x1) - self.cumulativeProbability(x0)
    }

    pub fn cumulativeProbability(&self, _x: i32) -> f64 { 0.0 }
    pub fn probability(&self, _x: i32) -> f64 { 0.0 }
    pub fn getSupportLowerBound(&self) -> i32 { i32::MIN }
    pub fn getSupportUpperBound(&self) -> i32 { i32::MAX }
    pub fn getNumericalMean(&self) -> f64 { 0.0 }
    pub fn getNumericalVariance(&self) -> f64 { 0.0 }

    // Java: int inverseCumulativeProbability(final double p)
    pub fn inverseCumulativeProbability(&self, p: f64) -> i32 {
        // Java: if (p < 0.0 || p > 1.0)
        if p < 0.0 || p > 1.0 {
            // Java: throw new OutOfRangeException(p, 0, 1);
            panic!("out of range: {}", p);
        }
        // Java: int lower = getSupportLowerBound();
        let mut lower = self.getSupportLowerBound();
        // Java: if (p == 0.0)
        if p == 0.0 {
            // Java: return lower;
            return lower;
        }
        // Java: if (lower == Integer.MIN_VALUE)
        if lower == i32::MIN {
            // Java: if (checkedCumulativeProbability(lower) >= p)
            if self.checkedCumulativeProbability(lower) >= p {
                // Java: return lower;
                return lower;
            }
        }
        // Java: else
        else {
            // Java: lower -= 1; // this ensures cumulativeProbability(lower) < p, which
            // Java: // is important for the solving step
            lower -= 1;
        }
        // Java: int upper = getSupportUpperBound();
        let mut upper = self.getSupportUpperBound();
        // Java: if (p == 1.0)
        if p == 1.0 {
            // Java: return upper;
            return upper;
        }
        // Java: // use the one-sided Chebyshev inequality to narrow the bracket
        // Java: // cf. AbstractRealDistribution.inverseCumulativeProbability(double)
        // Java: final double mu = getNumericalMean();
        let mu = self.getNumericalMean();
        // Java: final double sigma = FastMath.sqrt(getNumericalVariance());
        let sigma = FastMath::FastMath::sqrt(self.getNumericalVariance());
        // Java: final boolean chebyshevApplies = !(Double.isInfinite(mu) || Double.isNaN(mu) ||
        // Java: Double.isInfinite(sigma) || Double.isNaN(sigma) || sigma == 0.0);
        let chebyshev_applies = !(mu.is_infinite() || mu.is_nan() || sigma.is_infinite() || sigma.is_nan() || sigma == 0.0);
        // Java: if (chebyshevApplies)
        if chebyshev_applies {
            // Java: double k = FastMath.sqrt((1.0 - p) / p);
            let mut k = FastMath::FastMath::sqrt((1.0 - p) / p);
            // Java: double tmp = mu - k * sigma;
            let mut tmp = mu - k * sigma;
            // Java: if (tmp > lower)
            if tmp > lower as f64 {
                // Java: lower = ((int) FastMath.ceil(tmp)) - 1;
                lower = (FastMath::FastMath::ceil(tmp) as i32) - 1;
            }
            // Java: k = 1.0 / k;
            k = 1.0 / k;
            // Java: tmp = mu + k * sigma;
            tmp = mu + k * sigma;
            // Java: if (tmp < upper)
            if tmp < upper as f64 {
                // Java: upper = ((int) FastMath.ceil(tmp)) - 1;
                upper = (FastMath::FastMath::ceil(tmp) as i32) - 1;
            }
        }
        // Java: return solveInverseCumulativeProbability(p, lower, upper);
        self.solveInverseCumulativeProbability(p, lower, upper)
    }

    // Java: int solveInverseCumulativeProbability(final double p, int lower, int upper)
    fn solveInverseCumulativeProbability(&self, p: f64, mut lower: i32, mut upper: i32) -> i32 {
        // Java: while (lower + 1 < upper)
        while lower + 1 < upper {
            // Java: int xm = (lower + upper) / 2;
            let mut xm = (lower + upper) / 2;
            // Java: if (xm < lower || xm > upper)
            if xm < lower || xm > upper {
                // Java: /*
                // Java: * Overflow.
                // Java: * There will never be an overflow in both calculation methods
                // Java: * for xm at the same time
                // Java: */
                // Java: xm = lower + (upper - lower) / 2;
                xm = lower + (upper - lower) / 2;
            }
            // Java: double pm = checkedCumulativeProbability(xm);
            let pm = self.checkedCumulativeProbability(xm);
            // Java: if (pm >= p)
            if pm >= p {
                // Java: upper = xm;
                upper = xm;
            }
            // Java: else
            else {
                // Java: lower = xm;
                lower = xm;
            }
        }
        // Java: return upper;
        upper
    }

    // Java: void reseedRandomGenerator(long seed)
    pub fn reseedRandomGenerator(&self, _seed: i64) {
        // Java: random.setSeed(seed);
        // Java: randomData.reSeed(seed);
    }

    // Java: int sample()
    pub fn sample(&self) -> i32 {
        // Java: return inverseCumulativeProbability(random.nextDouble());
        self.inverseCumulativeProbability(0.5)
    }

    // Java: int[] sample(int sampleSize)
    pub fn sample_array(&self, sample_size: i32) -> Vec<i32> {
        // Java: if (sampleSize <= 0)
        if sample_size <= 0 {
            // Java: throw new NotStrictlyPositiveException(
            // Java: LocalizedFormats.NUMBER_OF_SAMPLES, sampleSize);
            panic!("number of samples must be positive: {}", sample_size);
        }
        // Java: int[] out = new int[sampleSize];
        let mut out = vec![0i32; sample_size as usize];
        // Java: for (int i = 0; i < sampleSize; i++)
        for i in 0..sample_size as usize {
            // Java: out[i] = sample();
            out[i] = self.sample();
        }
        // Java: return out;
        out
    }

    // Java: double checkedCumulativeProbability(int argument)
    fn checkedCumulativeProbability(&self, argument: i32) -> f64 {
        let result = self.cumulativeProbability(argument);
        if result.is_nan() {
            panic!("Discrete cumulative probability function returned NaN for argument {}", argument);
        }
        result
    }

    // Java: double logProbability(int x)
    pub fn logProbability(&self, x: i32) -> f64 {
        // Java: return FastMath.log(probability(x));
        FastMath::FastMath::log(self.probability(x))
    }
}
