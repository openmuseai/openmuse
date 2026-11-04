// Java: import java.util.Collection
// Java: import org.apache.commons.math3.exception.NotANumberException
use crate::org::apache::commons::math3::exception::NotANumberException;
// Java: import org.apache.commons.math3.exception.NotFiniteNumberException
use crate::org::apache::commons::math3::exception::NotFiniteNumberException;
// Java: import org.apache.commons.math3.exception.NotStrictlyPositiveException
use crate::org::apache::commons::math3::exception::NotStrictlyPositiveException;
// Java: import org.apache.commons.math3.exception.NumberIsTooLargeException
use crate::org::apache::commons::math3::exception::NumberIsTooLargeException;

// Java field: throws NumberIsTooLargeException, NotFiniteNumberException, NotANumberException
// Java field: throws NumberIsTooLargeException, NotFiniteNumberException, NotANumberException
// Java field: throws NumberIsTooLargeException, NotStrictlyPositiveException
// Java field: throws NumberIsTooLargeException, NotStrictlyPositiveException
// Java: public interface RandomData
pub trait RandomData {
    // Java: abstract String nextHexString(int len)
    fn nextHexString(&self) -> String;

    // Java: abstract int nextInt(int lower, int upper)
    fn nextInt(&self) -> i32;

    // Java: abstract long nextLong(long lower, long upper)
    fn nextLong(&self) -> i64;

    // Java: abstract String nextSecureHexString(int len)
    fn nextSecureHexString(&self) -> String;

    // Java: abstract int nextSecureInt(int lower, int upper)
    fn nextSecureInt(&self) -> i32;

    // Java: abstract long nextSecureLong(long lower, long upper)
    fn nextSecureLong(&self) -> i64;

    // Java: abstract long nextPoisson(double mean)
    fn nextPoisson(&self) -> i64;

    // Java: abstract double nextGaussian(double mu, double sigma)
    fn nextGaussian(&self) -> f64;

    // Java: abstract double nextExponential(double mean)
    fn nextExponential(&self) -> f64;

    // Java: abstract double nextUniform(double lower, double upper)
    fn nextUniform(&self) -> f64;

    // Java: abstract double nextUniform(double lower, double upper, boolean lowerInclusive)
    fn nextUniform_2(&self) -> f64;

    // Java: abstract int[] nextPermutation(int n, int k)
    fn nextPermutation(&self) -> Vec<i32>;

    // Java: abstract Object[] nextSample(Collection<?> c, int k)
    fn nextSample(&self) -> Box<dyn std::any::Any>;

}
