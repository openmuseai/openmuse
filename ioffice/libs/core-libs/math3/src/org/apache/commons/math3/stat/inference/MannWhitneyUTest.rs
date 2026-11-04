// Java: import org.apache.commons.math3.distribution.NormalDistribution
use crate::org::apache::commons::math3::distribution::NormalDistribution;
// Java: import org.apache.commons.math3.exception.ConvergenceException
use crate::org::apache::commons::math3::exception::ConvergenceException;
// Java: import org.apache.commons.math3.exception.MaxCountExceededException
use crate::org::apache::commons::math3::exception::MaxCountExceededException;
// Java: import org.apache.commons.math3.exception.NoDataException
use crate::org::apache::commons::math3::exception::NoDataException;
// Java: import org.apache.commons.math3.exception.NullArgumentException
use crate::org::apache::commons::math3::exception::NullArgumentException;
// Java: import org.apache.commons.math3.stat.ranking.NaNStrategy
use crate::org::apache::commons::math3::stat::ranking::NaNStrategy;
// Java: import org.apache.commons.math3.stat.ranking.NaturalRanking
use crate::org::apache::commons::math3::stat::ranking::NaturalRanking;
// Java: import org.apache.commons.math3.stat.ranking.TiesStrategy
use crate::org::apache::commons::math3::stat::ranking::TiesStrategy;
// Java: import org.apache.commons.math3.util.FastMath
use crate::org::apache::commons::math3::util::FastMath;

// Java: public class MannWhitneyUTest
pub struct MannWhitneyUTest {
    // Java field: NaturalRanking naturalRanking
    naturalRanking: Option<Box<dyn std::any::Any>>,
}

// Java: public class MannWhitneyUTest
impl MannWhitneyUTest {
    // Java: MannWhitneyUTest()
    pub fn MannWhitneyUTest(&self) {
        // Java: naturalRanking = new NaturalRanking(NaNStrategy.FIXED,
                // Java: TiesStrategy.AVERAGE);
    }

    // Java: MannWhitneyUTest()
    pub fn MannWhitneyUTest_2(&self) {
    }

    // Java: void ensureDataConformance(final double[] x, final double[] y)
    pub fn ensureDataConformance(&self) {
    }

    // Java: double[] concatenateSamples(final double[] x, final double[] y)
    pub fn concatenateSamples(&self) -> Vec<f64> {
        // Java: final double[] z = new double[x.length + y.length];
        // Java: System.arraycopy(x, 0, z, 0, x.length);
        // Java: System.arraycopy(y, 0, z, x.length, y.length);
        // Java: return z;
        todo!()
    }

    // Java: double mannWhitneyU(final double[] x, final double[] y)
    pub fn mannWhitneyU(&self) -> f64 {
        todo!()
    }

    // Java: double calculateAsymptoticPValue()
    pub fn calculateAsymptoticPValue(&self) -> f64 {
        todo!()
    }

    // Java: double mannWhitneyUTest(final double[] x, final double[] y)
    pub fn mannWhitneyUTest(&self) -> f64 {
        todo!()
    }

}
