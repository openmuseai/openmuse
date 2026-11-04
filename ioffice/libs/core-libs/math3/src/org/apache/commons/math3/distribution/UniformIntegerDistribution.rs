#![allow(unused_imports, dead_code, non_snake_case)]
// Java: import org.apache.commons.math3.exception.NumberIsTooLargeException
use crate::org::apache::commons::math3::exception::NumberIsTooLargeException;
// Java: import org.apache.commons.math3.exception.util.LocalizedFormats
use crate::org::apache::commons::math3::exception::util::LocalizedFormats;
// Java: import org.apache.commons.math3.random.RandomGenerator
use crate::org::apache::commons::math3::random::RandomGenerator;
// Java: import org.apache.commons.math3.random.Well19937c
use crate::org::apache::commons::math3::random::Well19937c;

// Java field: long serialVersionUID
pub(crate) const SERIAL_VERSION_UID: i64 = 20120109;

// Java: public class UniformIntegerDistribution extends AbstractIntegerDistribution
pub struct UniformIntegerDistribution {
    // Java field: int lower
    pub(crate) lower: i32,
    // Java field: int upper
    pub(crate) upper: i32,
    rng: Well19937c::Well19937c,
}

// Java: public class UniformIntegerDistribution extends AbstractIntegerDistribution
impl UniformIntegerDistribution {
    // Java: UniformIntegerDistribution(int lower, int upper)
    pub fn new(lower: i32, upper: i32) -> Self {
        if lower > upper {
            panic!("lower ({}) > upper ({})", lower, upper);
        }
        UniformIntegerDistribution { lower, upper, rng: Well19937c::Well19937c::new() }
    }

    // Java: UniformIntegerDistribution()
    pub fn new_with_rng(_rng: Well19937c::Well19937c, lower: i32, upper: i32) -> Self {
        if lower > upper {
            panic!("lower ({}) > upper ({})", lower, upper);
        }
        UniformIntegerDistribution { lower, upper, rng: _rng }
    }

    // Java: double probability(int x)
    pub fn probability(&self, x: i32) -> f64 {
        // Java: if (x < lower || x > upper)
        if x < self.lower || x > self.upper {
            // Java: return 0;
            return 0.0;
        }
        // Java: return 1.0 / (upper - lower + 1);
        1.0 / (self.upper - self.lower + 1) as f64
    }

    // Java: double cumulativeProbability(int x)
    pub fn cumulativeProbability(&self, x: i32) -> f64 {
        // Java: if (x < lower)
        if x < self.lower {
            // Java: return 0;
            return 0.0;
        }
        // Java: if (x > upper)
        if x > self.upper {
            // Java: return 1;
            return 1.0;
        }
        // Java: return (x - lower + 1.0) / (upper - lower + 1.0);
        (x - self.lower + 1) as f64 / (self.upper - self.lower + 1) as f64
    }

    // Java: double getNumericalMean()
    pub fn getNumericalMean(&self) -> f64 {
        // Java: return 0.5 * (lower + upper);
        0.5 * (self.lower + self.upper) as f64
    }

    // Java: double getNumericalVariance()
    pub fn getNumericalVariance(&self) -> f64 {
        // Java: double n = upper - lower + 1;
        let n = (self.upper - self.lower + 1) as f64;
        // Java: return (n * n - 1) / 12.0;
        (n * n - 1.0) / 12.0
    }

    // Java: int getSupportLowerBound()
    pub fn getSupportLowerBound(&self) -> i32 {
        // Java: return lower;
        self.lower
    }

    // Java: int getSupportUpperBound()
    pub fn getSupportUpperBound(&self) -> i32 {
        // Java: return upper;
        self.upper
    }

    // Java: boolean isSupportConnected()
    pub fn isSupportConnected(&self) -> bool {
        // Java: return true;
        true
    }

    // Java: int sample()
    pub fn sample(&mut self) -> i32 {
        // Java: final int max = (upper - lower) + 1;
        let max = (self.upper - self.lower) + 1;
        // Java: if (max <= 0)
        if max <= 0 {
            // Java: // The range is too wide to fit in a positive int (larger
            // Java: // than 2^31); as it covers more than half the integer range,
            // Java: // we use a simple rejection method.
            // Java: while (true)
            loop {
                // Java: final int r = random.nextInt();
                let r = self.rng.next(32);
                // Java: if (r >= lower &&
                // Java: r <= upper)
                if r >= self.lower && r <= self.upper {
                    // Java: return r;
                    return r;
                }
            }
        }
        // Java: else
        else {
            // Java: // We can shift the range and directly generate a positive int.
            // Java: return lower + random.nextInt(max);
            self.lower + (self.rng.next(31).abs() % max)
        }
    }
}
