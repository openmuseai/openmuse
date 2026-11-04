// Java: import org.apache.commons.math3.exception.NotStrictlyPositiveException
use crate::org::apache::commons::math3::exception::NotStrictlyPositiveException;
// Java: import org.apache.commons.math3.exception.util.LocalizedFormats
use crate::org::apache::commons::math3::exception::util::LocalizedFormats;
// Java: import org.apache.commons.math3.random.RandomGenerator
use crate::org::apache::commons::math3::random::RandomGenerator::RandomGenerator;
// Java: import org.apache.commons.math3.random.Well19937c
use crate::org::apache::commons::math3::random::Well19937c;
// Java: import org.apache.commons.math3.special.Gamma
use crate::org::apache::commons::math3::special::Gamma;
// Java: import org.apache.commons.math3.util.CombinatoricsUtils
use crate::org::apache::commons::math3::util::CombinatoricsUtils;
// Java: import org.apache.commons.math3.util.FastMath
use crate::org::apache::commons::math3::util::FastMath;
// Java: import org.apache.commons.math3.util.MathUtils
use crate::org::apache::commons::math3::util::MathUtils;

// Java field: int DEFAULT_MAX_ITERATIONS
pub(crate) const DEFAULT_MAX_ITERATIONS: i32 = 0;
// Java field: double DEFAULT_EPSILON
pub(crate) const DEFAULT_EPSILON: f64 = 0.0;
// Java field: long serialVersionUID
pub(crate) const serialVersionUID: i64 = 0;

// Java: public class PoissonDistribution extends AbstractIntegerDistribution
pub struct PoissonDistribution {
    // Java field: NormalDistribution normal
    pub(crate) normal: Box<dyn std::any::Any>,
    // Java field: ExponentialDistribution exponential
    exponential: Box<dyn std::any::Any>,
    // Java field: double mean
    pub(crate) mean: f64,
    // Java field: int maxIterations
    maxIterations: i32,
    // Java field: double epsilon
    epsilon: f64,
}

// Java: public class PoissonDistribution extends AbstractIntegerDistribution
impl PoissonDistribution {
    // Java: PoissonDistribution(double p)
    pub fn PoissonDistribution(&self, p: f64) {
        // Java: this(p, DEFAULT_EPSILON, DEFAULT_MAX_ITERATIONS);
    }

    // Java: PoissonDistribution(double p, double epsilon, int maxIterations)
    pub fn PoissonDistribution_1(&self, p: f64, epsilon: f64, maxIterations: i32) {
    }

    // Java: PoissonDistribution()
    pub fn PoissonDistribution_2(&self) {
    }

    // Java: PoissonDistribution(double p, double epsilon)
    pub fn PoissonDistribution_3(&self, p: f64, epsilon: f64) {
    }

    // Java: PoissonDistribution(double p, int maxIterations)
    pub fn PoissonDistribution_4(&self, p: f64, maxIterations: i32) {
        // Java: this(p, DEFAULT_EPSILON, maxIterations);
    }

    // Java: double getMean()
    pub fn getMean(&self) -> f64 {
        // Java: return mean;
        todo!();
    }

    // Java: double probability(int x)
    pub fn probability(&self, x: i32) -> f64 {
        // Java: final double logProbability = logProbability(x);
        // Java: return logProbability == Double.NEGATIVE_INFINITY ? 0 : FastMath.exp(logProbability);
        todo!();
    }

    // Java: double logProbability(int x)
    pub fn logProbability(&self, x: i32) -> f64 {
        // Java: double ret;
        // Java: if (x < 0 || x == Integer.MAX_VALUE)
        {
            // Java: ret = Double.NEGATIVE_INFINITY;
        }
        // Java: else if (x == 0)
        {
            // Java: ret = -mean;
        }
        // Java: else
        {
            // Java: ret = -SaddlePointExpansion.getStirlingError(x) -
                  // Java: SaddlePointExpansion.getDeviancePart(x, mean) -
                  // Java: 0.5 * FastMath.log(MathUtils.TWO_PI) - 0.5 * FastMath.log(x);
        }
        // Java: return ret;
        todo!();
    }

    // Java: double cumulativeProbability(int x)
    pub fn cumulativeProbability(&self, x: i32) -> f64 {
        // Java: if (x < 0)
        {
            // Java: return 0;
        }
        // Java: if (x == Integer.MAX_VALUE)
        {
            // Java: return 1;
        }
        // Java: return Gamma.regularizedGammaQ((double) x + 1, mean, epsilon,
                                       // Java: maxIterations);
        todo!();
    }

    // Java: double normalApproximateProbability(int x)
    pub fn normalApproximateProbability(&self, x: i32) -> f64 {
        // Java: // calculate the probability using half-correction
        // Java: return normal.cumulativeProbability(x + 0.5);
        todo!();
    }

    // Java: double getNumericalMean()
    pub fn getNumericalMean(&self) -> f64 {
        // Java: return getMean();
        todo!();
    }

    // Java: double getNumericalVariance()
    pub fn getNumericalVariance(&self) -> f64 {
        // Java: return getMean();
        todo!();
    }

    // Java: int getSupportLowerBound()
    pub fn getSupportLowerBound(&self) -> i32 {
        // Java: return 0;
        todo!();
    }

    // Java: int getSupportUpperBound()
    pub fn getSupportUpperBound(&self) -> i32 {
        // Java: return Integer.MAX_VALUE;
        todo!();
    }

    // Java: boolean isSupportConnected()
    pub fn isSupportConnected(&self) -> bool {
        // Java: return true;
        todo!();
    }

    // Java: int sample()
    pub fn sample(&self) -> i32 {
        // Java: return (int) FastMath.min(nextPoisson(mean), Integer.MAX_VALUE);
        todo!();
    }

    // Java: long nextPoisson(double meanPoisson)
    pub fn nextPoisson(&self, meanPoisson: f64) -> i64 {
        // Java: final double pivot = 40.0d;
        // Java: if (meanPoisson < pivot)
        {
            // Java: double p = FastMath.exp(-meanPoisson);
            // Java: long n = 0;
            // Java: double r = 1.0d;
            // Java: double rnd = 1.0d;
            // Java: while (n < 1000 * meanPoisson)
            {
                // Java: rnd = random.nextDouble();
                // Java: r *= rnd;
                // Java: if (r >= p)
                {
                    // Java: n++;
                }
                // Java: else
                {
                    // Java: return n;
                }
            }
            // Java: return n;
        }
        // Java: else
        {
            // Java: final double lambda = FastMath.floor(meanPoisson);
            // Java: final double lambdaFractional = meanPoisson - lambda;
            // Java: final double logLambda = FastMath.log(lambda);
            // Java: final double logLambdaFactorial = CombinatoricsUtils.factorialLog((int) lambda);
            // Java: final long y2 = lambdaFractional < Double.MIN_VALUE ? 0 : nextPoisson(lambdaFractional);
            // Java: final double delta = FastMath.sqrt(lambda * FastMath.log(32 * lambda / FastMath.PI + 1));
            // Java: final double halfDelta = delta / 2;
            // Java: final double twolpd = 2 * lambda + delta;
            // Java: final double a1 = FastMath.sqrt(FastMath.PI * twolpd) * FastMath.exp(1 / (8 * lambda));
            // Java: final double a2 = (twolpd / delta) * FastMath.exp(-delta * (1 + delta) / twolpd);
            // Java: final double aSum = a1 + a2 + 1;
            // Java: final double p1 = a1 / aSum;
            // Java: final double p2 = a2 / aSum;
            // Java: final double c1 = 1 / (8 * lambda);
            // Java: double x = 0;
            // Java: double y = 0;
            // Java: double v = 0;
            // Java: int a = 0;
            // Java: double t = 0;
            // Java: double qr = 0;
            // Java: double qa = 0;
            // Java: for (;;)
            {
                // Java: final double u = random.nextDouble();
                // Java: if (u <= p1)
                {
                    // Java: final double n = random.nextGaussian();
                    // Java: x = n * FastMath.sqrt(lambda + halfDelta) - 0.5d;
                    // Java: if (x > delta || x < -lambda)
                    {
                        // Java: continue;
                    }
                    // Java: y = x < 0 ? FastMath.floor(x) : FastMath.ceil(x);
                    // Java: final double e = exponential.sample();
                    // Java: v = -e - (n * n / 2) + c1;
                }
                // Java: else
                {
                    // Java: if (u > p1 + p2)
                    {
                        // Java: y = lambda;
                        // Java: break;
                    }
                    // Java: else
                    {
                        // Java: x = delta + (twolpd / delta) * exponential.sample();
                        // Java: y = FastMath.ceil(x);
                        // Java: v = -exponential.sample() - delta * (x + 1) / twolpd;
                    }
                }
                // Java: a = x < 0 ? 1 : 0;
                // Java: t = y * (y + 1) / (2 * lambda);
                // Java: if (v < -t && a == 0)
                {
                    // Java: y = lambda + y;
                    // Java: break;
                }
                // Java: qr = t * ((2 * y + 1) / (6 * lambda) - 1);
                // Java: qa = qr - (t * t) / (3 * (lambda + a * (y + 1)));
                // Java: if (v < qa)
                {
                    // Java: y = lambda + y;
                    // Java: break;
                }
                // Java: if (v > qr)
                {
                    // Java: continue;
                }
                // Java: if (v < y * logLambda - CombinatoricsUtils.factorialLog((int) (y + lambda)) + logLambdaFactorial)
                {
                    // Java: y = lambda + y;
                    // Java: break;
                }
            }
            // Java: return y2 + (long) y;
        }
        todo!();
    }

}
