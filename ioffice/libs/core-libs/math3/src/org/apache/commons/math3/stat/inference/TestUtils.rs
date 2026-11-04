// Java: import java.util.Collection
// Java: import org.apache.commons.math3.distribution.RealDistribution
use crate::org::apache::commons::math3::distribution::RealDistribution;
// Java: import org.apache.commons.math3.exception.ConvergenceException
use crate::org::apache::commons::math3::exception::ConvergenceException;
// Java: import org.apache.commons.math3.exception.DimensionMismatchException
use crate::org::apache::commons::math3::exception::DimensionMismatchException;
// Java: import org.apache.commons.math3.exception.InsufficientDataException
use crate::org::apache::commons::math3::exception::InsufficientDataException;
// Java: import org.apache.commons.math3.exception.MaxCountExceededException
use crate::org::apache::commons::math3::exception::MaxCountExceededException;
// Java: import org.apache.commons.math3.exception.NoDataException
use crate::org::apache::commons::math3::exception::NoDataException;
// Java: import org.apache.commons.math3.exception.NotPositiveException
use crate::org::apache::commons::math3::exception::NotPositiveException;
// Java: import org.apache.commons.math3.exception.NotStrictlyPositiveException
use crate::org::apache::commons::math3::exception::NotStrictlyPositiveException;
// Java: import org.apache.commons.math3.exception.NullArgumentException
use crate::org::apache::commons::math3::exception::NullArgumentException;
// Java: import org.apache.commons.math3.exception.NumberIsTooSmallException
use crate::org::apache::commons::math3::exception::NumberIsTooSmallException;
// Java: import org.apache.commons.math3.exception.OutOfRangeException
use crate::org::apache::commons::math3::exception::OutOfRangeException;
// Java: import org.apache.commons.math3.exception.ZeroException
use crate::org::apache::commons::math3::exception::ZeroException;
// Java: import org.apache.commons.math3.stat.descriptive.StatisticalSummary
use crate::org::apache::commons::math3::stat::descriptive::StatisticalSummary;

// Java field: TTest T_TEST
static T_TEST: std::sync::OnceLock<Box<dyn std::any::Any + Send + Sync>> = std::sync::OnceLock::new();
// Java field: ChiSquareTest CHI_SQUARE_TEST
static CHI_SQUARE_TEST: std::sync::OnceLock<Box<dyn std::any::Any + Send + Sync>> = std::sync::OnceLock::new();
// Java field: OneWayAnova ONE_WAY_ANANOVA
static ONE_WAY_ANANOVA: std::sync::OnceLock<Box<dyn std::any::Any + Send + Sync>> = std::sync::OnceLock::new();
// Java field: GTest G_TEST
static G_TEST: std::sync::OnceLock<Box<dyn std::any::Any + Send + Sync>> = std::sync::OnceLock::new();
// Java field: KolmogorovSmirnovTest KS_TEST
static KS_TEST: std::sync::OnceLock<Box<dyn std::any::Any + Send + Sync>> = std::sync::OnceLock::new();

// Java: public class TestUtils
pub struct TestUtils {
}

// Java: public class TestUtils
impl TestUtils {
    // Java: TestUtils()
    pub fn TestUtils(&self) {
        // Java: super();
    }

    // Java: static double homoscedasticT(final double[] sample1, final double[] sample2)
    pub fn homoscedasticT(&self) -> f64 {
        todo!()
    }

    // Java: static double homoscedasticT()
    pub fn homoscedasticT_2(&self) -> f64 {
        todo!()
    }

    // Java: static boolean homoscedasticTTest()
    pub fn homoscedasticTTest(&self) -> bool {
        todo!()
    }

    // Java: static double homoscedasticTTest(final double[] sample1, final double[] sample2)
    pub fn homoscedasticTTest_2(&self) -> f64 {
        todo!()
    }

    // Java: static double homoscedasticTTest()
    pub fn homoscedasticTTest_3(&self) -> f64 {
        todo!()
    }

    // Java: static double pairedT(final double[] sample1, final double[] sample2)
    pub fn pairedT(&self) -> f64 {
        todo!()
    }

    // Java: static boolean pairedTTest()
    pub fn pairedTTest(&self) -> bool {
        todo!()
    }

    // Java: static double pairedTTest(final double[] sample1, final double[] sample2)
    pub fn pairedTTest_2(&self) -> f64 {
        todo!()
    }

    // Java: static double t(final double mu, final double[] observed)
    pub fn t(&self) -> f64 {
        todo!()
    }

    // Java: static double t(final double mu, final StatisticalSummary sampleStats)
    pub fn t_2(&self) -> f64 {
        todo!()
    }

    // Java: static double t(final double[] sample1, final double[] sample2)
    pub fn t_3(&self) -> f64 {
        todo!()
    }

    // Java: static double t()
    pub fn t_4(&self) -> f64 {
        todo!()
    }

    // Java: static boolean tTest(final double mu, final double[] sample, final double alpha)
    pub fn tTest(&self) -> bool {
        todo!()
    }

    // Java: static double tTest(final double mu, final double[] sample)
    pub fn tTest_2(&self) -> f64 {
        todo!()
    }

    // Java: static boolean tTest()
    pub fn tTest_3(&self) -> bool {
        todo!()
    }

    // Java: static double tTest(final double mu, final StatisticalSummary sampleStats)
    pub fn tTest_4(&self) -> f64 {
        todo!()
    }

    // Java: static boolean tTest()
    pub fn tTest_5(&self) -> bool {
        todo!()
    }

    // Java: static double tTest(final double[] sample1, final double[] sample2)
    pub fn tTest_6(&self) -> f64 {
        todo!()
    }

    // Java: static boolean tTest()
    pub fn tTest_7(&self) -> bool {
        todo!()
    }

    // Java: static double tTest()
    pub fn tTest_8(&self) -> f64 {
        todo!()
    }

    // Java: static double chiSquare(final double[] expected, final long[] observed)
    pub fn chiSquare(&self) -> f64 {
        todo!()
    }

    // Java: static double chiSquare(final long[][] counts)
    pub fn chiSquare_2(&self) -> f64 {
        todo!()
    }

    // Java: static boolean chiSquareTest()
    pub fn chiSquareTest(&self) -> bool {
        todo!()
    }

    // Java: static double chiSquareTest(final double[] expected, final long[] observed)
    pub fn chiSquareTest_2(&self) -> f64 {
        todo!()
    }

    // Java: static boolean chiSquareTest(final long[][] counts, final double alpha)
    pub fn chiSquareTest_3(&self) -> bool {
        todo!()
    }

    // Java: static double chiSquareTest(final long[][] counts)
    pub fn chiSquareTest_4(&self) -> f64 {
        todo!()
    }

    // Java: static double chiSquareDataSetsComparison()
    pub fn chiSquareDataSetsComparison(&self) -> f64 {
        todo!()
    }

    // Java: static double chiSquareTestDataSetsComparison()
    pub fn chiSquareTestDataSetsComparison(&self) -> f64 {
        todo!()
    }

    // Java: static boolean chiSquareTestDataSetsComparison()
    pub fn chiSquareTestDataSetsComparison_2(&self) -> bool {
        todo!()
    }

    // Java: static double oneWayAnovaFValue(final Collection<double[]> categoryData)
    pub fn oneWayAnovaFValue(&self) -> f64 {
        todo!()
    }

    // Java: static double oneWayAnovaPValue(final Collection<double[]> categoryData)
    pub fn oneWayAnovaPValue(&self) -> f64 {
        todo!()
    }

    // Java: static boolean oneWayAnovaTest()
    pub fn oneWayAnovaTest(&self) -> bool {
        todo!()
    }

    // Java: static double g(final double[] expected, final long[] observed)
    pub fn g(&self) -> f64 {
        todo!()
    }

    // Java: static double gTest(final double[] expected, final long[] observed)
    pub fn gTest(&self) -> f64 {
        todo!()
    }

    // Java: static double gTestIntrinsic(final double[] expected, final long[] observed)
    pub fn gTestIntrinsic(&self) -> f64 {
        todo!()
    }

    // Java: static boolean gTest()
    pub fn gTest_2(&self) -> bool {
        todo!()
    }

    // Java: static double gDataSetsComparison()
    pub fn gDataSetsComparison(&self) -> f64 {
        todo!()
    }

    // Java: static double rootLogLikelihoodRatio(final long k11, final long k12, final long k21, final long k22)
    pub fn rootLogLikelihoodRatio(&self) -> f64 {
        todo!()
    }

    // Java: static double gTestDataSetsComparison()
    pub fn gTestDataSetsComparison(&self) -> f64 {
        todo!()
    }

    // Java: static boolean gTestDataSetsComparison()
    pub fn gTestDataSetsComparison_2(&self) -> bool {
        todo!()
    }

    // Java: static double kolmogorovSmirnovStatistic(RealDistribution dist, double[] data)
    pub fn kolmogorovSmirnovStatistic(&self) -> f64 {
        todo!()
    }

    // Java: static double kolmogorovSmirnovTest(RealDistribution dist, double[] data)
    pub fn kolmogorovSmirnovTest(&self) -> f64 {
        todo!()
    }

    // Java: static double kolmogorovSmirnovTest(RealDistribution dist, double[] data, boolean strict)
    pub fn kolmogorovSmirnovTest_2(&self) -> f64 {
        todo!()
    }

    // Java: static boolean kolmogorovSmirnovTest(RealDistribution dist, double[] data, double alpha)
    pub fn kolmogorovSmirnovTest_3(&self) -> bool {
        todo!()
    }

    // Java: static double kolmogorovSmirnovStatistic(double[] x, double[] y)
    pub fn kolmogorovSmirnovStatistic_2(&self) -> f64 {
        todo!()
    }

    // Java: static double kolmogorovSmirnovTest(double[] x, double[] y)
    pub fn kolmogorovSmirnovTest_4(&self) -> f64 {
        todo!()
    }

    // Java: static double kolmogorovSmirnovTest(double[] x, double[] y, boolean strict)
    pub fn kolmogorovSmirnovTest_5(&self) -> f64 {
        todo!()
    }

    // Java: static double exactP(double d, int m, int n, boolean strict)
    pub fn exactP(&self) -> f64 {
        // Java: return KS_TEST.exactP(d, n, m, strict);
        todo!()
    }

    // Java: static double approximateP(double d, int n, int m)
    pub fn approximateP(&self) -> f64 {
        // Java: return KS_TEST.approximateP(d, n, m);
        todo!()
    }

    // Java: static double monteCarloP(double d, int n, int m, boolean strict, int iterations)
    pub fn monteCarloP(&self) -> f64 {
        // Java: return KS_TEST.monteCarloP(d, n, m, strict, iterations);
        todo!()
    }

}
