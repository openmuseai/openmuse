// Java: import org.apache.commons.math3.distribution.ChiSquaredDistribution
use crate::org::apache::commons::math3::distribution::ChiSquaredDistribution;
// Java: import org.apache.commons.math3.exception.DimensionMismatchException
use crate::org::apache::commons::math3::exception::DimensionMismatchException;
// Java: import org.apache.commons.math3.exception.MaxCountExceededException
use crate::org::apache::commons::math3::exception::MaxCountExceededException;
// Java: import org.apache.commons.math3.exception.NotPositiveException
use crate::org::apache::commons::math3::exception::NotPositiveException;
// Java: import org.apache.commons.math3.exception.NotStrictlyPositiveException
use crate::org::apache::commons::math3::exception::NotStrictlyPositiveException;
// Java: import org.apache.commons.math3.exception.OutOfRangeException
use crate::org::apache::commons::math3::exception::OutOfRangeException;
// Java: import org.apache.commons.math3.exception.ZeroException
use crate::org::apache::commons::math3::exception::ZeroException;
// Java: import org.apache.commons.math3.exception.util.LocalizedFormats
use crate::org::apache::commons::math3::exception::util::LocalizedFormats;
// Java: import org.apache.commons.math3.util.FastMath
use crate::org::apache::commons::math3::util::FastMath;
// Java: import org.apache.commons.math3.util.MathArrays
use crate::org::apache::commons::math3::util::MathArrays;

// Java: public class GTest
pub struct GTest {
}

// Java: public class GTest
impl GTest {
    // Java: double g(final double[] expected, final long[] observed)
    pub fn g(&self) -> f64 {
        todo!()
    }

    // Java: double gTest(final double[] expected, final long[] observed)
    pub fn gTest(&self) -> f64 {
        todo!()
    }

    // Java: double gTestIntrinsic(final double[] expected, final long[] observed)
    pub fn gTestIntrinsic(&self) -> f64 {
        todo!()
    }

    // Java: boolean gTest()
    pub fn gTest_2(&self) -> bool {
        todo!()
    }

    // Java: double entropy(final long[][] k)
    pub fn entropy(&self) -> f64 {
        // Java: double h = 0d;
        // Java: double sum_k = 0d;
        // Java: for (int i = 0; i < k.length; i++)
        {
            // Java: for (int j = 0; j < k[i].length; j++)
            {
                // Java: sum_k += (double) k[i][j];
            }
        }
        // Java: for (int i = 0; i < k.length; i++)
        {
            // Java: for (int j = 0; j < k[i].length; j++)
            {
                // Java: if (k[i][j] != 0)
                {
                    // Java: final double p_ij = (double) k[i][j] / sum_k;
                    // Java: h += p_ij * FastMath.log(p_ij);
                }
            }
        }
        // Java: return -h;
        todo!()
    }

    // Java: double entropy(final long[] k)
    pub fn entropy_2(&self) -> f64 {
        // Java: double h = 0d;
        // Java: double sum_k = 0d;
        // Java: for (int i = 0; i < k.length; i++)
        {
            // Java: sum_k += (double) k[i];
        }
        // Java: for (int i = 0; i < k.length; i++)
        {
            // Java: if (k[i] != 0)
            {
                // Java: final double p_i = (double) k[i] / sum_k;
                // Java: h += p_i * FastMath.log(p_i);
            }
        }
        // Java: return -h;
        todo!()
    }

    // Java: double gDataSetsComparison(final long[] observed1, final long[] observed2)
    pub fn gDataSetsComparison(&self) -> f64 {
        todo!()
    }

    // Java: double rootLogLikelihoodRatio()
    pub fn rootLogLikelihoodRatio(&self) -> f64 {
        todo!()
    }

    // Java: double gTestDataSetsComparison()
    pub fn gTestDataSetsComparison(&self) -> f64 {
        todo!()
    }

    // Java: boolean gTestDataSetsComparison()
    pub fn gTestDataSetsComparison_2(&self) -> bool {
        todo!()
    }

}
