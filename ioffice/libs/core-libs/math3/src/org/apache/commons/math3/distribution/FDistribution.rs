// Java: import org.apache.commons.math3.exception.NotStrictlyPositiveException
use crate::org::apache::commons::math3::exception::NotStrictlyPositiveException;
// Java: import org.apache.commons.math3.exception.util.LocalizedFormats
use crate::org::apache::commons::math3::exception::util::LocalizedFormats;
// Java: import org.apache.commons.math3.random.RandomGenerator
use crate::org::apache::commons::math3::random::RandomGenerator::RandomGenerator;
// Java: import org.apache.commons.math3.random.Well19937c
use crate::org::apache::commons::math3::random::Well19937c;
// Java: import org.apache.commons.math3.special.Beta
use crate::org::apache::commons::math3::special::Beta;
// Java: import org.apache.commons.math3.util.FastMath
use crate::org::apache::commons::math3::util::FastMath;

// Java field: double DEFAULT_INVERSE_ABSOLUTE_ACCURACY
pub(crate) const DEFAULT_INVERSE_ABSOLUTE_ACCURACY: f64 = 0.0;
// Java field: long serialVersionUID
pub(crate) const serialVersionUID: i64 = 0;

// Java: public class FDistribution extends AbstractRealDistribution
pub struct FDistribution {
    // Java field: double numeratorDegreesOfFreedom
    pub(crate) numeratorDegreesOfFreedom: f64,
    // Java field: double denominatorDegreesOfFreedom
    pub(crate) denominatorDegreesOfFreedom: f64,
    // Java field: double solverAbsoluteAccuracy
    solverAbsoluteAccuracy: f64,
    // Java field: double numericalVariance
    pub(crate) numericalVariance: f64,
    // Java field: boolean numericalVarianceIsCalculated
    numericalVarianceIsCalculated: bool,
}

// Java: public class FDistribution extends AbstractRealDistribution
impl FDistribution {
    // Java: FDistribution()
    pub fn FDistribution(&self) {
    }

    // Java: FDistribution()
    pub fn FDistribution_1(&self) {
    }

    // Java: FDistribution()
    pub fn FDistribution_2(&self) {
    }

    // Java: FDistribution()
    pub fn FDistribution_3(&self) {
    }

    // Java: double density(double x)
    pub fn density(&self, x: f64) -> f64 {
        // Java: return FastMath.exp(logDensity(x));
        todo!();
    }

    // Java: double logDensity(double x)
    pub fn logDensity(&self, x: f64) -> f64 {
        // Java: final double nhalf = numeratorDegreesOfFreedom / 2;
        // Java: final double mhalf = denominatorDegreesOfFreedom / 2;
        // Java: final double logx = FastMath.log(x);
        // Java: final double logn = FastMath.log(numeratorDegreesOfFreedom);
        // Java: final double logm = FastMath.log(denominatorDegreesOfFreedom);
        // Java: final double lognxm = FastMath.log(numeratorDegreesOfFreedom * x +
                // Java: denominatorDegreesOfFreedom);
        // Java: return nhalf * logn + nhalf * logx - logx +
               // Java: mhalf * logm - nhalf * lognxm - mhalf * lognxm -
               // Java: Beta.logBeta(nhalf, mhalf);
        todo!();
    }

    // Java: double cumulativeProbability(double x)
    pub fn cumulativeProbability(&self, x: f64) -> f64 {
        // Java: double ret;
        // Java: if (x <= 0)
        {
            // Java: ret = 0;
        }
        // Java: else
        {
            // Java: double n = numeratorDegreesOfFreedom;
            // Java: double m = denominatorDegreesOfFreedom;
            // Java: ret = Beta.regularizedBeta((n * x) / (m + n * x),
                // Java: 0.5 * n,
                // Java: 0.5 * m);
        }
        // Java: return ret;
        todo!();
    }

    // Java: double getNumeratorDegreesOfFreedom()
    pub fn getNumeratorDegreesOfFreedom(&self) -> f64 {
        // Java: return numeratorDegreesOfFreedom;
        todo!();
    }

    // Java: double getDenominatorDegreesOfFreedom()
    pub fn getDenominatorDegreesOfFreedom(&self) -> f64 {
        // Java: return denominatorDegreesOfFreedom;
        todo!();
    }

    // Java: double getSolverAbsoluteAccuracy()
    pub fn getSolverAbsoluteAccuracy(&self) -> f64 {
        // Java: return solverAbsoluteAccuracy;
        todo!();
    }

    // Java: double getNumericalMean()
    pub fn getNumericalMean(&self) -> f64 {
        // Java: final double denominatorDF = getDenominatorDegreesOfFreedom();
        // Java: if (denominatorDF > 2)
        {
            // Java: return denominatorDF / (denominatorDF - 2);
        }
        // Java: return Double.NaN;
        todo!();
    }

    // Java: double getNumericalVariance()
    pub fn getNumericalVariance(&self) -> f64 {
        // Java: if (!numericalVarianceIsCalculated)
        {
            // Java: numericalVariance = calculateNumericalVariance();
            // Java: numericalVarianceIsCalculated = true;
        }
        // Java: return numericalVariance;
        todo!();
    }

    // Java: double calculateNumericalVariance()
    pub fn calculateNumericalVariance(&self) -> f64 {
        // Java: final double denominatorDF = getDenominatorDegreesOfFreedom();
        // Java: if (denominatorDF > 4)
        {
            // Java: final double numeratorDF = getNumeratorDegreesOfFreedom();
            // Java: final double denomDFMinusTwo = denominatorDF - 2;
            // Java: return ( 2 * (denominatorDF * denominatorDF) * (numeratorDF + denominatorDF - 2) ) /
                   // Java: ( (numeratorDF * (denomDFMinusTwo * denomDFMinusTwo) * (denominatorDF - 4)) );
        }
        // Java: return Double.NaN;
        todo!();
    }

    // Java: double getSupportLowerBound()
    pub fn getSupportLowerBound(&self) -> f64 {
        // Java: return 0;
        todo!();
    }

    // Java: double getSupportUpperBound()
    pub fn getSupportUpperBound(&self) -> f64 {
        // Java: return Double.POSITIVE_INFINITY;
        todo!();
    }

    // Java: boolean isSupportLowerBoundInclusive()
    pub fn isSupportLowerBoundInclusive(&self) -> bool {
        // Java: return false;
        todo!();
    }

    // Java: boolean isSupportUpperBoundInclusive()
    pub fn isSupportUpperBoundInclusive(&self) -> bool {
        // Java: return false;
        todo!();
    }

    // Java: boolean isSupportConnected()
    pub fn isSupportConnected(&self) -> bool {
        // Java: return true;
        todo!();
    }

}
