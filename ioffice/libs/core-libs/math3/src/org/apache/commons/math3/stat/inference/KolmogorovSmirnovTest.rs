// Java: import java.math.BigDecimal
// Java: import java.util.Arrays
// Java: import java.util.HashSet
// Java: import org.apache.commons.math3.distribution.EnumeratedRealDistribution
use crate::org::apache::commons::math3::distribution::EnumeratedRealDistribution;
// Java: import org.apache.commons.math3.distribution.RealDistribution
use crate::org::apache::commons::math3::distribution::RealDistribution;
// Java: import org.apache.commons.math3.distribution.UniformRealDistribution
use crate::org::apache::commons::math3::distribution::UniformRealDistribution;
// Java: import org.apache.commons.math3.exception.InsufficientDataException
use crate::org::apache::commons::math3::exception::InsufficientDataException;
// Java: import org.apache.commons.math3.exception.MathArithmeticException
use crate::org::apache::commons::math3::exception::MathArithmeticException;
// Java: import org.apache.commons.math3.exception.MathInternalError
use crate::org::apache::commons::math3::exception::MathInternalError;
// Java: import org.apache.commons.math3.exception.NullArgumentException
use crate::org::apache::commons::math3::exception::NullArgumentException;
// Java: import org.apache.commons.math3.exception.NumberIsTooLargeException
use crate::org::apache::commons::math3::exception::NumberIsTooLargeException;
// Java: import org.apache.commons.math3.exception.OutOfRangeException
use crate::org::apache::commons::math3::exception::OutOfRangeException;
// Java: import org.apache.commons.math3.exception.TooManyIterationsException
use crate::org::apache::commons::math3::exception::TooManyIterationsException;
// Java: import org.apache.commons.math3.exception.util.LocalizedFormats
use crate::org::apache::commons::math3::exception::util::LocalizedFormats;
// Java: import org.apache.commons.math3.fraction.BigFraction
use crate::org::apache::commons::math3::fraction::BigFraction;
// Java: import org.apache.commons.math3.fraction.BigFractionField
use crate::org::apache::commons::math3::fraction::BigFractionField;
// Java: import org.apache.commons.math3.fraction.FractionConversionException
use crate::org::apache::commons::math3::fraction::FractionConversionException;
// Java: import org.apache.commons.math3.linear.Array2DRowFieldMatrix
use crate::org::apache::commons::math3::linear::Array2DRowFieldMatrix;
// Java: import org.apache.commons.math3.linear.FieldMatrix
use crate::org::apache::commons::math3::linear::FieldMatrix;
// Java: import org.apache.commons.math3.linear.MatrixUtils
use crate::org::apache::commons::math3::linear::MatrixUtils;
// Java: import org.apache.commons.math3.linear.RealMatrix
use crate::org::apache::commons::math3::linear::RealMatrix;
// Java: import org.apache.commons.math3.random.JDKRandomGenerator
use crate::org::apache::commons::math3::random::JDKRandomGenerator;
// Java: import org.apache.commons.math3.random.RandomGenerator
use crate::org::apache::commons::math3::random::RandomGenerator;
// Java: import org.apache.commons.math3.random.Well19937c
use crate::org::apache::commons::math3::random::Well19937c;
// Java: import org.apache.commons.math3.util.CombinatoricsUtils
use crate::org::apache::commons::math3::util::CombinatoricsUtils;
// Java: import org.apache.commons.math3.util.FastMath
use crate::org::apache::commons::math3::util::FastMath;
// Java: import org.apache.commons.math3.util.MathArrays
use crate::org::apache::commons::math3::util::MathArrays;
// Java: import org.apache.commons.math3.util.MathUtils
use crate::org::apache::commons::math3::util::MathUtils;

// Java field: int MAXIMUM_PARTIAL_SUM_COUNT
pub(crate) const MAXIMUM_PARTIAL_SUM_COUNT: i32 = 0;
// Java field: double KS_SUM_CAUCHY_CRITERION
pub(crate) const KS_SUM_CAUCHY_CRITERION: f64 = 0.0;

// Java: public class KolmogorovSmirnovTest
pub struct KolmogorovSmirnovTest {
}

// Java: public class javadoc
pub struct javadoc {
}

// Java: public class javadoc
pub struct javadoc_2 {
}

// Java: public class javadoc
pub struct javadoc_3 {
    // Java field: double x
    x: f64,
    // Java field: int sign
    sign: i32,
    // Java field: long i
    i: i64,
    // Java field: double partialSum
    partialSum: f64,
    // Java field: double delta
    delta: f64,
}

// Java: public class javadoc
pub struct javadoc_4 {
}

// Java: public class javadoc
pub struct javadoc_5 {
}

// Java: public class javadoc
pub struct javadoc_6 {
}

// Java: public class KolmogorovSmirnovTest
impl KolmogorovSmirnovTest {
}

// Java: public class javadoc
impl javadoc {
}

// Java: public class javadoc
impl javadoc_2 {
    // Java: double pelzGood(double d, int n)
    pub fn pelzGood(&self) -> f64 {
        // Java: // Change the variable since approximation is for the distribution evaluated at d / sqrt(n)
        // Java: final double sqrtN = FastMath.sqrt(n);
        // Java: final double z = d * sqrtN;
        // Java: final double z2 = d * d * n;
        // Java: final double z4 = z2 * z2;
        // Java: final double z6 = z4 * z2;
        // Java: final double z8 = z4 * z4;
        // Java: // Eventual return value
        // Java: double ret = 0;
        // Java: // Compute K_0(z)
        // Java: double sum = 0;
        // Java: double increment = 0;
        // Java: double kTerm = 0;
        // Java: double z2Term = MathUtils.PI_SQUARED / (8 * z2);
        // Java: int k = 1;
        // Java: for (; k < MAXIMUM_PARTIAL_SUM_COUNT; k++)
        {
            // Java: kTerm = 2 * k - 1;
            // Java: increment = FastMath.exp(-z2Term * kTerm * kTerm);
            // Java: sum += increment;
            // Java: if (increment <= PG_SUM_RELATIVE_ERROR * sum)
            {
                // Java: break;
            }
        }
        // Java: if (k == MAXIMUM_PARTIAL_SUM_COUNT)
        {
            // Java: throw new TooManyIterationsException(MAXIMUM_PARTIAL_SUM_COUNT);
        }
        // Java: ret = sum * FastMath.sqrt(2 * FastMath.PI) / z;
        // Java: // K_1(z)
        // Java: // Sum is -inf to inf, but k term is always (k + 1/2) ^ 2, so really have
        // Java: // twice the sum from k = 0 to inf (k = -1 is same as 0, -2 same as 1, ...)
        // Java: final double twoZ2 = 2 * z2;
        // Java: sum = 0;
        // Java: kTerm = 0;
        // Java: double kTerm2 = 0;
        // Java: for (k = 0; k < MAXIMUM_PARTIAL_SUM_COUNT; k++)
        {
            // Java: kTerm = k + 0.5;
            // Java: kTerm2 = kTerm * kTerm;
            // Java: increment = (MathUtils.PI_SQUARED * kTerm2 - z2) * FastMath.exp(-MathUtils.PI_SQUARED * kTerm2 / twoZ2);
            // Java: sum += increment;
            // Java: if (FastMath.abs(increment) < PG_SUM_RELATIVE_ERROR * FastMath.abs(sum))
            {
                // Java: break;
            }
        }
        // Java: if (k == MAXIMUM_PARTIAL_SUM_COUNT)
        {
            // Java: throw new TooManyIterationsException(MAXIMUM_PARTIAL_SUM_COUNT);
        }
        // Java: final double sqrtHalfPi = FastMath.sqrt(FastMath.PI / 2);
        // Java: // Instead of doubling sum, divide by 3 instead of 6
        // Java: ret += sum * sqrtHalfPi / (3 * z4 * sqrtN);
        // Java: // K_2(z)
        // Java: // Same drill as K_1, but with two doubly infinite sums, all k terms are even powers.
        // Java: final double z4Term = 2 * z4;
        // Java: final double z6Term = 6 * z6;
        // Java: z2Term = 5 * z2;
        // Java: final double pi4 = MathUtils.PI_SQUARED * MathUtils.PI_SQUARED;
        // Java: sum = 0;
        // Java: kTerm = 0;
        // Java: kTerm2 = 0;
        // Java: for (k = 0; k < MAXIMUM_PARTIAL_SUM_COUNT; k++)
        {
            // Java: kTerm = k + 0.5;
            // Java: kTerm2 = kTerm * kTerm;
            // Java: increment =  (z6Term + z4Term + MathUtils.PI_SQUARED * (z4Term - z2Term) * kTerm2 +
                    // Java: pi4 * (1 - twoZ2) * kTerm2 * kTerm2) * FastMath.exp(-MathUtils.PI_SQUARED * kTerm2 / twoZ2);
            // Java: sum += increment;
            // Java: if (FastMath.abs(increment) < PG_SUM_RELATIVE_ERROR * FastMath.abs(sum))
            {
                // Java: break;
            }
        }
        // Java: if (k == MAXIMUM_PARTIAL_SUM_COUNT)
        {
            // Java: throw new TooManyIterationsException(MAXIMUM_PARTIAL_SUM_COUNT);
        }
        // Java: double sum2 = 0;
        // Java: kTerm2 = 0;
        // Java: for (k = 1; k < MAXIMUM_PARTIAL_SUM_COUNT; k++)
        {
            // Java: kTerm2 = k * k;
            // Java: increment = MathUtils.PI_SQUARED * kTerm2 * FastMath.exp(-MathUtils.PI_SQUARED * kTerm2 / twoZ2);
            // Java: sum2 += increment;
            // Java: if (FastMath.abs(increment) < PG_SUM_RELATIVE_ERROR * FastMath.abs(sum2))
            {
                // Java: break;
            }
        }
        // Java: if (k == MAXIMUM_PARTIAL_SUM_COUNT)
        {
            // Java: throw new TooManyIterationsException(MAXIMUM_PARTIAL_SUM_COUNT);
        }
        // Java: // Again, adjust coefficients instead of doubling sum, sum2
        // Java: ret += (sqrtHalfPi / n) * (sum / (36 * z2 * z2 * z2 * z) - sum2 / (18 * z2 * z));
        // Java: // K_3(z) One more time with feeling - two doubly infinite sums, all k powers even.
        // Java: // Multiply coefficient denominators by 2, so omit doubling sums.
        // Java: final double pi6 = pi4 * MathUtils.PI_SQUARED;
        // Java: sum = 0;
        // Java: double kTerm4 = 0;
        // Java: double kTerm6 = 0;
        // Java: for (k = 0; k < MAXIMUM_PARTIAL_SUM_COUNT; k++)
        {
            // Java: kTerm = k + 0.5;
            // Java: kTerm2 = kTerm * kTerm;
            // Java: kTerm4 = kTerm2 * kTerm2;
            // Java: kTerm6 = kTerm4 * kTerm2;
            // Java: increment = (pi6 * kTerm6 * (5 - 30 * z2) + pi4 * kTerm4 * (-60 * z2 + 212 * z4) +
                            // Java: MathUtils.PI_SQUARED * kTerm2 * (135 * z4 - 96 * z6) - 30 * z6 - 90 * z8) *
                    // Java: FastMath.exp(-MathUtils.PI_SQUARED * kTerm2 / twoZ2);
            // Java: sum += increment;
            // Java: if (FastMath.abs(increment) < PG_SUM_RELATIVE_ERROR * FastMath.abs(sum))
            {
                // Java: break;
            }
        }
        // Java: if (k == MAXIMUM_PARTIAL_SUM_COUNT)
        {
            // Java: throw new TooManyIterationsException(MAXIMUM_PARTIAL_SUM_COUNT);
        }
        // Java: sum2 = 0;
        // Java: for (k = 1; k < MAXIMUM_PARTIAL_SUM_COUNT; k++)
        {
            // Java: kTerm2 = k * k;
            // Java: kTerm4 = kTerm2 * kTerm2;
            // Java: increment = (-pi4 * kTerm4 + 3 * MathUtils.PI_SQUARED * kTerm2 * z2) *
                    // Java: FastMath.exp(-MathUtils.PI_SQUARED * kTerm2 / twoZ2);
            // Java: sum2 += increment;
            // Java: if (FastMath.abs(increment) < PG_SUM_RELATIVE_ERROR * FastMath.abs(sum2))
            {
                // Java: break;
            }
        }
        // Java: if (k == MAXIMUM_PARTIAL_SUM_COUNT)
        {
            // Java: throw new TooManyIterationsException(MAXIMUM_PARTIAL_SUM_COUNT);
        }
        // Java: return ret + (sqrtHalfPi / (sqrtN * n)) * (sum / (3240 * z6 * z4) +
                // Java: + sum2 / (108 * z6));
        todo!()
    }

}

// Java: public class javadoc
impl javadoc_3 {
}

// Java: public class javadoc
impl javadoc_4 {
}

// Java: public class javadoc
impl javadoc_5 {
}

// Java: public class javadoc
impl javadoc_6 {
}
