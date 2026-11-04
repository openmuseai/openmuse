// Java: import java.io.Serializable
// Java: import java.security.MessageDigest
// Java: import java.security.NoSuchAlgorithmException
// Java: import java.security.NoSuchProviderException
// Java: import java.security.SecureRandom
// Java: import java.util.Collection
// Java: import org.apache.commons.math3.distribution.BetaDistribution
use crate::org::apache::commons::math3::distribution::BetaDistribution;
// Java: import org.apache.commons.math3.distribution.BinomialDistribution
use crate::org::apache::commons::math3::distribution::BinomialDistribution;
// Java: import org.apache.commons.math3.distribution.CauchyDistribution
use crate::org::apache::commons::math3::distribution::CauchyDistribution;
// Java: import org.apache.commons.math3.distribution.ChiSquaredDistribution
use crate::org::apache::commons::math3::distribution::ChiSquaredDistribution;
// Java: import org.apache.commons.math3.distribution.ExponentialDistribution
use crate::org::apache::commons::math3::distribution::ExponentialDistribution;
// Java: import org.apache.commons.math3.distribution.FDistribution
use crate::org::apache::commons::math3::distribution::FDistribution;
// Java: import org.apache.commons.math3.distribution.GammaDistribution
use crate::org::apache::commons::math3::distribution::GammaDistribution;
// Java: import org.apache.commons.math3.distribution.HypergeometricDistribution
use crate::org::apache::commons::math3::distribution::HypergeometricDistribution;
// Java: import org.apache.commons.math3.distribution.PascalDistribution
use crate::org::apache::commons::math3::distribution::PascalDistribution;
// Java: import org.apache.commons.math3.distribution.PoissonDistribution
use crate::org::apache::commons::math3::distribution::PoissonDistribution;
// Java: import org.apache.commons.math3.distribution.TDistribution
use crate::org::apache::commons::math3::distribution::TDistribution;
// Java: import org.apache.commons.math3.distribution.WeibullDistribution
use crate::org::apache::commons::math3::distribution::WeibullDistribution;
// Java: import org.apache.commons.math3.distribution.ZipfDistribution
use crate::org::apache::commons::math3::distribution::ZipfDistribution;
// Java: import org.apache.commons.math3.distribution.UniformIntegerDistribution
use crate::org::apache::commons::math3::distribution::UniformIntegerDistribution;
// Java: import org.apache.commons.math3.exception.MathInternalError
use crate::org::apache::commons::math3::exception::MathInternalError;
// Java: import org.apache.commons.math3.exception.NotANumberException
use crate::org::apache::commons::math3::exception::NotANumberException;
// Java: import org.apache.commons.math3.exception.NotFiniteNumberException
use crate::org::apache::commons::math3::exception::NotFiniteNumberException;
// Java: import org.apache.commons.math3.exception.NotPositiveException
use crate::org::apache::commons::math3::exception::NotPositiveException;
// Java: import org.apache.commons.math3.exception.NotStrictlyPositiveException
use crate::org::apache::commons::math3::exception::NotStrictlyPositiveException;
// Java: import org.apache.commons.math3.exception.NumberIsTooLargeException
use crate::org::apache::commons::math3::exception::NumberIsTooLargeException;
// Java: import org.apache.commons.math3.exception.OutOfRangeException
use crate::org::apache::commons::math3::exception::OutOfRangeException;
// Java: import org.apache.commons.math3.exception.util.LocalizedFormats
use crate::org::apache::commons::math3::exception::util::LocalizedFormats;
// Java: import org.apache.commons.math3.util.MathArrays
use crate::org::apache::commons::math3::util::MathArrays;

// Java field: long serialVersionUID

// Java: public class RandomDataGenerator implements RandomData, Serializable
pub struct RandomDataGenerator {
    // Java field: RandomGenerator rand
    pub(crate) rand: Box<dyn std::any::Any>,
    // Java field: RandomGenerator secRand
    secRand: Option<Box<dyn std::any::Any>>,
}

// Java: public interface using
pub trait using {
}

// Java: public class RandomDataGenerator implements RandomData, Serializable
impl RandomDataGenerator {
    // Java: RandomDataGenerator()
    pub fn RandomDataGenerator(&self) {
    }

    // Java: RandomDataGenerator(RandomGenerator rand)
    pub fn RandomDataGenerator_2(&self) {
        // Java: this.rand = rand;
    }

    // Java: String nextHexString(int len)
    pub fn nextHexString(&self) -> String {
        // Java: if (len <= 0)
        {
            // Java: throw new NotStrictlyPositiveException(LocalizedFormats.LENGTH, len);
        }
        // Java: // Get a random number generator
        // Java: RandomGenerator ran = getRandomGenerator();
        // Java: // Initialize output buffer
        // Java: StringBuilder outBuffer = new StringBuilder();
        // Java: // Get int(len/2)+1 random bytes
        // Java: byte[] randomBytes = new byte[(len / 2) + 1];
        // Java: ran.nextBytes(randomBytes);
        // Java: // Convert each byte to 2 hex digits
        // Java: for (int i = 0; i < randomBytes.length; i++)
        {
            // Java: Integer c = Integer.valueOf(randomBytes[i]);
            // Java: /*
             // Java: * Add 128 to byte value to make interval 0-255 before doing hex
             // Java: * conversion. This guarantees <= 2 hex digits from toHexString()
             // Java: * toHexString would otherwise add 2^32 to negative arguments.
             // Java: */
            // Java: String hex = Integer.toHexString(c.intValue() + 128);
            // Java: // Make sure we add 2 hex digits for each byte
            // Java: if (hex.length() == 1)
            {
                // Java: hex = "0" + hex;
            }
            // Java: outBuffer.append(hex);
        }
        // Java: return outBuffer.toString().substring(0, len);
        todo!()
    }

    // Java: int nextInt(final int lower, final int upper)
    pub fn nextInt(&self) -> i32 {
        // Java: return new UniformIntegerDistribution(getRandomGenerator(), lower, upper).sample();
        todo!()
    }

    // Java: long nextLong(final long lower, final long upper)
    pub fn nextLong(&self) -> i64 {
        // Java: if (lower >= upper)
        {
            // Java: throw new NumberIsTooLargeException(LocalizedFormats.LOWER_BOUND_NOT_BELOW_UPPER_BOUND,
                                                // Java: lower, upper, false);
        }
        // Java: final long max = (upper - lower) + 1;
        // Java: if (max <= 0)
        {
            // Java: // the range is too wide to fit in a positive long (larger than 2^63); as it covers
            // Java: // more than half the long range, we use directly a simple rejection method
            // Java: final RandomGenerator rng = getRandomGenerator();
            // Java: while (true)
            {
                // Java: final long r = rng.nextLong();
                // Java: if (r >= lower && r <= upper)
                {
                    // Java: return r;
                }
            }
        }
        // Java: else if (max < Integer.MAX_VALUE)
        {
            // Java: // we can shift the range and generate directly a positive int
            // Java: return lower + getRandomGenerator().nextInt((int) max);
        }
        // Java: else
        {
            // Java: // we can shift the range and generate directly a positive long
            // Java: return lower + nextLong(getRandomGenerator(), max);
        }
        todo!()
    }

    // Java: static long nextLong(final RandomGenerator rng, final long n)
    pub fn nextLong_2(&self) -> i64 {
        // Java: if (n > 0)
        {
            // Java: final byte[] byteArray = new byte[8];
            // Java: long bits;
            // Java: long val;
            // Java: do
            {
                // Java: rng.nextBytes(byteArray);
                // Java: bits = 0;
                // Java: for (final byte b : byteArray)
                {
                    // Java: bits = (bits << 8) | (((long) b) & 0xffL);
                }
                // Java: bits &= 0x7fffffffffffffffL;
                // Java: val  = bits % n;
            }
            // Java: return val;
        }
        // Java: throw new NotStrictlyPositiveException(n);
        todo!()
    }

    // Java: String nextSecureHexString(int len)
    pub fn nextSecureHexString(&self) -> String {
        // Java: if (len <= 0)
        {
            // Java: throw new NotStrictlyPositiveException(LocalizedFormats.LENGTH, len);
        }
        // Java: // Get SecureRandom and setup Digest provider
        // Java: final RandomGenerator secRan = getSecRan();
        // Java: MessageDigest alg = null;
        // Java: try
        {
            // Java: alg = MessageDigest.getInstance("SHA-1");
        }
        // Java: catch (NoSuchAlgorithmException ex)
        {
            // Java: // this should never happen
            // Java: throw new MathInternalError(ex);
        }
        // Java: alg.reset();
        // Java: // Compute number of iterations required (40 bytes each)
        // Java: int numIter = (len / 40) + 1;
        // Java: StringBuilder outBuffer = new StringBuilder();
        // Java: for (int iter = 1; iter < numIter + 1; iter++)
        {
            // Java: byte[] randomBytes = new byte[40];
            // Java: secRan.nextBytes(randomBytes);
            // Java: alg.update(randomBytes);
            // Java: // Compute hash -- will create 20-byte binary hash
            // Java: byte[] hash = alg.digest();
            // Java: // Loop over the hash, converting each byte to 2 hex digits
            // Java: for (int i = 0; i < hash.length; i++)
            {
                // Java: Integer c = Integer.valueOf(hash[i]);
                // Java: /*
                 // Java: * Add 128 to byte value to make interval 0-255 This guarantees
                 // Java: * <= 2 hex digits from toHexString() toHexString would
                 // Java: * otherwise add 2^32 to negative arguments
                 // Java: */
                // Java: String hex = Integer.toHexString(c.intValue() + 128);
                // Java: // Keep strings uniform length -- guarantees 40 bytes
                // Java: if (hex.length() == 1)
                {
                    // Java: hex = "0" + hex;
                }
                // Java: outBuffer.append(hex);
            }
        }
        // Java: return outBuffer.toString().substring(0, len);
        todo!()
    }

    // Java: int nextSecureInt(final int lower, final int upper)
    pub fn nextSecureInt(&self) -> i32 {
        // Java: return new UniformIntegerDistribution(getSecRan(), lower, upper).sample();
        todo!()
    }

    // Java: long nextSecureLong(final long lower, final long upper)
    pub fn nextSecureLong(&self) -> i64 {
        // Java: if (lower >= upper)
        {
            // Java: throw new NumberIsTooLargeException(LocalizedFormats.LOWER_BOUND_NOT_BELOW_UPPER_BOUND,
                                                // Java: lower, upper, false);
        }
        // Java: final RandomGenerator rng = getSecRan();
        // Java: final long max = (upper - lower) + 1;
        // Java: if (max <= 0)
        {
            // Java: // the range is too wide to fit in a positive long (larger than 2^63); as it covers
            // Java: // more than half the long range, we use directly a simple rejection method
            // Java: while (true)
            {
                // Java: final long r = rng.nextLong();
                // Java: if (r >= lower && r <= upper)
                {
                    // Java: return r;
                }
            }
        }
        // Java: else if (max < Integer.MAX_VALUE)
        {
            // Java: // we can shift the range and generate directly a positive int
            // Java: return lower + rng.nextInt((int) max);
        }
        // Java: else
        {
            // Java: // we can shift the range and generate directly a positive long
            // Java: return lower + nextLong(rng, max);
        }
        todo!()
    }

    // Java: long nextPoisson(double mean)
    pub fn nextPoisson(&self) -> i64 {
        // Java: return new PoissonDistribution(getRandomGenerator(), mean,
                // Java: PoissonDistribution.DEFAULT_EPSILON,
                // Java: PoissonDistribution.DEFAULT_MAX_ITERATIONS).sample();
        todo!()
    }

    // Java: double nextGaussian(double mu, double sigma)
    pub fn nextGaussian(&self) -> f64 {
        // Java: if (sigma <= 0)
        {
            // Java: throw new NotStrictlyPositiveException(LocalizedFormats.STANDARD_DEVIATION, sigma);
        }
        // Java: return sigma * getRandomGenerator().nextGaussian() + mu;
        todo!()
    }

    // Java: double nextExponential(double mean)
    pub fn nextExponential(&self) -> f64 {
        // Java: return new ExponentialDistribution(getRandomGenerator(), mean,
                // Java: ExponentialDistribution.DEFAULT_INVERSE_ABSOLUTE_ACCURACY).sample();
        todo!()
    }

    // Java: double nextGamma(double shape, double scale)
    pub fn nextGamma(&self) -> f64 {
        // Java: return new GammaDistribution(getRandomGenerator(),shape, scale,
                // Java: GammaDistribution.DEFAULT_INVERSE_ABSOLUTE_ACCURACY).sample();
        todo!()
    }

    // Java: int nextHypergeometric(int populationSize, int numberOfSuccesses, int sampleSize)
    pub fn nextHypergeometric(&self) -> i32 {
        // Java: return new HypergeometricDistribution(getRandomGenerator(),populationSize,
                // Java: numberOfSuccesses, sampleSize).sample();
        todo!()
    }

    // Java: int nextPascal(int r, double p)
    pub fn nextPascal(&self) -> i32 {
        // Java: return new PascalDistribution(getRandomGenerator(), r, p).sample();
        todo!()
    }

    // Java: double nextT(double df)
    pub fn nextT(&self) -> f64 {
        // Java: return new TDistribution(getRandomGenerator(), df,
                // Java: TDistribution.DEFAULT_INVERSE_ABSOLUTE_ACCURACY).sample();
        todo!()
    }

    // Java: double nextWeibull(double shape, double scale)
    pub fn nextWeibull(&self) -> f64 {
        // Java: return new WeibullDistribution(getRandomGenerator(), shape, scale,
                // Java: WeibullDistribution.DEFAULT_INVERSE_ABSOLUTE_ACCURACY).sample();
        todo!()
    }

    // Java: int nextZipf(int numberOfElements, double exponent)
    pub fn nextZipf(&self) -> i32 {
        // Java: return new ZipfDistribution(getRandomGenerator(), numberOfElements, exponent).sample();
        todo!()
    }

    // Java: double nextBeta(double alpha, double beta)
    pub fn nextBeta(&self) -> f64 {
        // Java: return new BetaDistribution(getRandomGenerator(), alpha, beta,
                // Java: BetaDistribution.DEFAULT_INVERSE_ABSOLUTE_ACCURACY).sample();
        todo!()
    }

    // Java: int nextBinomial(int numberOfTrials, double probabilityOfSuccess)
    pub fn nextBinomial(&self) -> i32 {
        // Java: return new BinomialDistribution(getRandomGenerator(), numberOfTrials, probabilityOfSuccess).sample();
        todo!()
    }

    // Java: double nextCauchy(double median, double scale)
    pub fn nextCauchy(&self) -> f64 {
        // Java: return new CauchyDistribution(getRandomGenerator(), median, scale,
                // Java: CauchyDistribution.DEFAULT_INVERSE_ABSOLUTE_ACCURACY).sample();
        todo!()
    }

    // Java: double nextChiSquare(double df)
    pub fn nextChiSquare(&self) -> f64 {
        // Java: return new ChiSquaredDistribution(getRandomGenerator(), df,
                // Java: ChiSquaredDistribution.DEFAULT_INVERSE_ABSOLUTE_ACCURACY).sample();
        todo!()
    }

    // Java: double nextF(double numeratorDf, double denominatorDf)
    pub fn nextF(&self) -> f64 {
        // Java: return new FDistribution(getRandomGenerator(), numeratorDf, denominatorDf,
                // Java: FDistribution.DEFAULT_INVERSE_ABSOLUTE_ACCURACY).sample();
        todo!()
    }

    // Java: double nextUniform(double lower, double upper)
    pub fn nextUniform(&self) -> f64 {
        todo!()
    }

    // Java: double nextUniform(double lower, double upper, boolean lowerInclusive)
    pub fn nextUniform_2(&self) -> f64 {
        todo!()
    }

    // Java: int[] nextPermutation(int n, int k)
    pub fn nextPermutation(&self) -> Vec<i32> {
        todo!()
    }

    // Java: Object[] nextSample(Collection<?> c, int k)
    pub fn nextSample(&self) -> Box<dyn std::any::Any> {
        // Java: int len = c.size();
        // Java: if (k > len)
        {
            // Java: throw new NumberIsTooLargeException(LocalizedFormats.SAMPLE_SIZE_EXCEEDS_COLLECTION_SIZE,
                                                // Java: k, len, true);
        }
        // Java: if (k <= 0)
        {
            // Java: throw new NotStrictlyPositiveException(LocalizedFormats.NUMBER_OF_SAMPLES, k);
        }
        // Java: Object[] objects = c.toArray();
        // Java: int[] index = nextPermutation(len, k);
        // Java: Object[] result = new Object[k];
        // Java: for (int i = 0; i < k; i++)
        {
            // Java: result[i] = objects[index[i]];
        }
        // Java: return result;
        todo!()
    }

    // Java: void reSeed(long seed)
    pub fn reSeed(&self) {
       // Java: getRandomGenerator().setSeed(seed);
    }

    // Java: void reSeedSecure()
    pub fn reSeedSecure(&self) {
        // Java: getSecRan().setSeed(System.currentTimeMillis());
    }

    // Java: void reSeedSecure(long seed)
    pub fn reSeedSecure_2(&self) {
        // Java: getSecRan().setSeed(seed);
    }

    // Java: void reSeed()
    pub fn reSeed_2(&self) {
        // Java: getRandomGenerator().setSeed(System.currentTimeMillis() + System.identityHashCode(this));
    }

    // Java: void setSecureAlgorithm(String algorithm, String provider)
    pub fn setSecureAlgorithm(&self) {
    }

    // Java: RandomGenerator getRandomGenerator()
    pub fn getRandomGenerator(&self) -> Box<dyn std::any::Any> {
        // Java: if (rand == null)
        {
            // Java: initRan();
        }
        // Java: return rand;
        todo!()
    }

    // Java: void initRan()
    pub fn initRan(&self) {
        // Java: rand = new Well19937c(System.currentTimeMillis() + System.identityHashCode(this));
    }

    // Java: RandomGenerator getSecRan()
    pub fn getSecRan(&self) -> Box<dyn std::any::Any> {
        // Java: if (secRand == null)
        {
            // Java: secRand = RandomGeneratorFactory.createRandomGenerator(new SecureRandom());
            // Java: secRand.setSeed(System.currentTimeMillis() + System.identityHashCode(this));
        }
        // Java: return secRand;
        todo!()
    }

}


