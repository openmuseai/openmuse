// Java: import java.io.Serializable
// Java: import java.security.NoSuchAlgorithmException
// Java: import java.security.NoSuchProviderException
// Java: import java.util.Collection
// Java: import org.apache.commons.math3.distribution.IntegerDistribution
use crate::org::apache::commons::math3::distribution::IntegerDistribution;
// Java: import org.apache.commons.math3.distribution.RealDistribution
use crate::org::apache::commons::math3::distribution::RealDistribution;
// Java: import org.apache.commons.math3.exception.NotANumberException
use crate::org::apache::commons::math3::exception::NotANumberException;
// Java: import org.apache.commons.math3.exception.NotFiniteNumberException
use crate::org::apache::commons::math3::exception::NotFiniteNumberException;
// Java: import org.apache.commons.math3.exception.NotPositiveException
use crate::org::apache::commons::math3::exception::NotPositiveException;
// Java: import org.apache.commons.math3.exception.NotStrictlyPositiveException
use crate::org::apache::commons::math3::exception::NotStrictlyPositiveException;
// Java: import org.apache.commons.math3.exception.MathIllegalArgumentException
use crate::org::apache::commons::math3::exception::MathIllegalArgumentException;
// Java: import org.apache.commons.math3.exception.NumberIsTooLargeException
use crate::org::apache::commons::math3::exception::NumberIsTooLargeException;
// Java: import org.apache.commons.math3.exception.OutOfRangeException
use crate::org::apache::commons::math3::exception::OutOfRangeException;

// Java field: long serialVersionUID

// Java: public class RandomDataImpl implements RandomData, Serializable
pub struct RandomDataImpl {
    // Java field: RandomDataGenerator delegate
    pub(crate) delegate: Box<dyn std::any::Any>,
}

// Java: public class RandomDataImpl implements RandomData, Serializable
impl RandomDataImpl {
    // Java: RandomDataImpl()
    pub fn RandomDataImpl(&self) {
        // Java: delegate = new RandomDataGenerator();
    }

    // Java: RandomDataImpl(RandomGenerator rand)
    pub fn RandomDataImpl_2(&self) {
        // Java: delegate = new RandomDataGenerator(rand);
    }

    // Java: RandomDataGenerator getDelegate()
    pub fn getDelegate(&self) -> Box<dyn std::any::Any> {
        // Java: return delegate;
        todo!()
    }

    // Java: String nextHexString(int len)
    pub fn nextHexString(&self) -> String {
        // Java: return delegate.nextHexString(len);
        todo!()
    }

    // Java: int nextInt(int lower, int upper)
    pub fn nextInt(&self) -> i32 {
       // Java: return delegate.nextInt(lower, upper);
        todo!()
    }

    // Java: long nextLong(long lower, long upper)
    pub fn nextLong(&self) -> i64 {
        // Java: return delegate.nextLong(lower, upper);
        todo!()
    }

    // Java: String nextSecureHexString(int len)
    pub fn nextSecureHexString(&self) -> String {
        // Java: return delegate.nextSecureHexString(len);
        todo!()
    }

    // Java: int nextSecureInt(int lower, int upper)
    pub fn nextSecureInt(&self) -> i32 {
        // Java: return delegate.nextSecureInt(lower, upper);
        todo!()
    }

    // Java: long nextSecureLong(long lower, long upper)
    pub fn nextSecureLong(&self) -> i64 {
        // Java: return delegate.nextSecureLong(lower,upper);
        todo!()
    }

    // Java: long nextPoisson(double mean)
    pub fn nextPoisson(&self) -> i64 {
        // Java: return delegate.nextPoisson(mean);
        todo!()
    }

    // Java: double nextGaussian(double mu, double sigma)
    pub fn nextGaussian(&self) -> f64 {
        // Java: return delegate.nextGaussian(mu,sigma);
        todo!()
    }

    // Java: double nextExponential(double mean)
    pub fn nextExponential(&self) -> f64 {
        // Java: return delegate.nextExponential(mean);
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

    // Java: double nextBeta(double alpha, double beta)
    pub fn nextBeta(&self) -> f64 {
        // Java: return delegate.nextBeta(alpha, beta);
        todo!()
    }

    // Java: int nextBinomial(int numberOfTrials, double probabilityOfSuccess)
    pub fn nextBinomial(&self) -> i32 {
        // Java: return delegate.nextBinomial(numberOfTrials, probabilityOfSuccess);
        todo!()
    }

    // Java: double nextCauchy(double median, double scale)
    pub fn nextCauchy(&self) -> f64 {
        // Java: return delegate.nextCauchy(median, scale);
        todo!()
    }

    // Java: double nextChiSquare(double df)
    pub fn nextChiSquare(&self) -> f64 {
       // Java: return delegate.nextChiSquare(df);
        todo!()
    }

    // Java: double nextF(double numeratorDf, double denominatorDf)
    pub fn nextF(&self) -> f64 {
        // Java: return delegate.nextF(numeratorDf, denominatorDf);
        todo!()
    }

    // Java: double nextGamma(double shape, double scale)
    pub fn nextGamma(&self) -> f64 {
        // Java: return delegate.nextGamma(shape, scale);
        todo!()
    }

    // Java: int nextHypergeometric(int populationSize, int numberOfSuccesses, int sampleSize)
    pub fn nextHypergeometric(&self) -> i32 {
        todo!()
    }

    // Java: int nextPascal(int r, double p)
    pub fn nextPascal(&self) -> i32 {
        todo!()
    }

    // Java: double nextT(double df)
    pub fn nextT(&self) -> f64 {
        // Java: return delegate.nextT(df);
        todo!()
    }

    // Java: double nextWeibull(double shape, double scale)
    pub fn nextWeibull(&self) -> f64 {
        // Java: return delegate.nextWeibull(shape, scale);
        todo!()
    }

    // Java: int nextZipf(int numberOfElements, double exponent)
    pub fn nextZipf(&self) -> i32 {
        // Java: return delegate.nextZipf(numberOfElements, exponent);
        todo!()
    }

    // Java: void reSeed(long seed)
    pub fn reSeed(&self) {
        // Java: delegate.reSeed(seed);
    }

    // Java: void reSeedSecure()
    pub fn reSeedSecure(&self) {
        // Java: delegate.reSeedSecure();
    }

    // Java: void reSeedSecure(long seed)
    pub fn reSeedSecure_2(&self) {
        // Java: delegate.reSeedSecure(seed);
    }

    // Java: void reSeed()
    pub fn reSeed_2(&self) {
        // Java: delegate.reSeed();
    }

    // Java: void setSecureAlgorithm(String algorithm, String provider)
    pub fn setSecureAlgorithm(&self) {
    }

    // Java: int[] nextPermutation(int n, int k)
    pub fn nextPermutation(&self) -> Vec<i32> {
        todo!()
    }

    // Java: Object[] nextSample(Collection<?> c, int k)
    pub fn nextSample(&self) -> Box<dyn std::any::Any> {
        todo!()
    }

    // Java: double nextInversionDeviate(RealDistribution distribution)
    pub fn nextInversionDeviate(&self) -> f64 {
        todo!()
    }

    // Java: int nextInversionDeviate(IntegerDistribution distribution)
    pub fn nextInversionDeviate_2(&self) -> i32 {
        todo!()
    }

}


