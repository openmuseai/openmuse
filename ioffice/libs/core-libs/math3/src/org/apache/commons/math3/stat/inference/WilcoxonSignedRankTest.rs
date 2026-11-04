// Java: import org.apache.commons.math3.distribution.NormalDistribution
use crate::org::apache::commons::math3::distribution::NormalDistribution;
// Java: import org.apache.commons.math3.exception.ConvergenceException
use crate::org::apache::commons::math3::exception::ConvergenceException;
// Java: import org.apache.commons.math3.exception.DimensionMismatchException
use crate::org::apache::commons::math3::exception::DimensionMismatchException;
// Java: import org.apache.commons.math3.exception.MaxCountExceededException
use crate::org::apache::commons::math3::exception::MaxCountExceededException;
// Java: import org.apache.commons.math3.exception.NoDataException
use crate::org::apache::commons::math3::exception::NoDataException;
// Java: import org.apache.commons.math3.exception.NullArgumentException
use crate::org::apache::commons::math3::exception::NullArgumentException;
// Java: import org.apache.commons.math3.exception.NumberIsTooLargeException
use crate::org::apache::commons::math3::exception::NumberIsTooLargeException;
// Java: import org.apache.commons.math3.stat.ranking.NaNStrategy
use crate::org::apache::commons::math3::stat::ranking::NaNStrategy;
// Java: import org.apache.commons.math3.stat.ranking.NaturalRanking
use crate::org::apache::commons::math3::stat::ranking::NaturalRanking;
// Java: import org.apache.commons.math3.stat.ranking.TiesStrategy
use crate::org::apache::commons::math3::stat::ranking::TiesStrategy;
// Java: import org.apache.commons.math3.util.FastMath
use crate::org::apache::commons::math3::util::FastMath;

// Java: public class WilcoxonSignedRankTest
pub struct WilcoxonSignedRankTest {
    // Java field: NaturalRanking naturalRanking
    naturalRanking: Option<Box<dyn std::any::Any>>,
}

// Java: public class WilcoxonSignedRankTest
impl WilcoxonSignedRankTest {
    // Java: WilcoxonSignedRankTest()
    pub fn WilcoxonSignedRankTest(&self) {
        // Java: naturalRanking = new NaturalRanking(NaNStrategy.FIXED,
                // Java: TiesStrategy.AVERAGE);
    }

    // Java: WilcoxonSignedRankTest()
    pub fn WilcoxonSignedRankTest_2(&self) {
    }

    // Java: void ensureDataConformance(final double[] x, final double[] y)
    pub fn ensureDataConformance(&self) {
    }

    // Java: double[] calculateDifferences(final double[] x, final double[] y)
    pub fn calculateDifferences(&self) -> Vec<f64> {
        // Java: final double[] z = new double[x.length];
        // Java: for (int i = 0; i < x.length; ++i)
        {
            // Java: z[i] = y[i] - x[i];
        }
        // Java: return z;
        todo!()
    }

    // Java: double[] calculateAbsoluteDifferences(final double[] z)
    pub fn calculateAbsoluteDifferences(&self) -> Vec<f64> {
        todo!()
    }

    // Java: double wilcoxonSignedRank(final double[] x, final double[] y)
    pub fn wilcoxonSignedRank(&self) -> f64 {
        todo!()
    }

    // Java: double calculateExactPValue(final double Wmax, final int N)
    pub fn calculateExactPValue(&self) -> f64 {
        // Java: // Total number of outcomes (equal to 2^N but a lot faster)
        // Java: final int m = 1 << N;
        // Java: int largerRankSums = 0;
        // Java: for (int i = 0; i < m; ++i)
        {
            // Java: int rankSum = 0;
            // Java: // Generate all possible rank sums
            // Java: for (int j = 0; j < N; ++j)
            {
                // Java: // (i >> j) & 1 extract i's j-th bit from the right
                // Java: if (((i >> j) & 1) == 1)
                {
                    // Java: rankSum += j + 1;
                }
            }
            // Java: if (rankSum >= Wmax)
            {
                // Java: ++largerRankSums;
            }
        }
        // Java: /*
         // Java: * largerRankSums / m gives the one-sided p-value, so it's multiplied
         // Java: * with 2 to get the two-sided p-value
         // Java: */
        // Java: return 2 * ((double) largerRankSums) / ((double) m);
        todo!()
    }

    // Java: double calculateAsymptoticPValue(final double Wmin, final int N)
    pub fn calculateAsymptoticPValue(&self) -> f64 {
        // Java: final double ES = (double) (N * (N + 1)) / 4.0;
        // Java: /* Same as (but saves computations):
         // Java: * final double VarW = ((double) (N * (N + 1) * (2*N + 1))) / 24;
         // Java: */
        // Java: final double VarS = ES * ((double) (2 * N + 1) / 6.0);
        // Java: // - 0.5 is a continuity correction
        // Java: final double z = (Wmin - ES - 0.5) / FastMath.sqrt(VarS);
        // Java: // No try-catch or advertised exception because args are valid
        // Java: // pass a null rng to avoid unneeded overhead as we will not sample from this distribution
        // Java: final NormalDistribution standardNormal = new NormalDistribution(null, 0, 1);
        // Java: return 2*standardNormal.cumulativeProbability(z);
        todo!()
    }

    // Java: double wilcoxonSignedRankTest()
    pub fn wilcoxonSignedRankTest(&self) -> f64 {
        todo!()
    }

}
