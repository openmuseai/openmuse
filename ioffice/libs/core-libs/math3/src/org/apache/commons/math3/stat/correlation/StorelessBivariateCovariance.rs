// Java: import org.apache.commons.math3.exception.NumberIsTooSmallException
use crate::org::apache::commons::math3::exception::NumberIsTooSmallException;
// Java: import org.apache.commons.math3.exception.util.LocalizedFormats
use crate::org::apache::commons::math3::exception::util::LocalizedFormats;

// Java: public class is
pub struct is {
}

// Java: public class is
pub struct is_2 {
}

// Java: public class StorelessBivariateCovariance
pub struct StorelessBivariateCovariance {
    // Java field: double meanX
    meanX: f64,
    // Java field: double meanY
    meanY: f64,
    // Java field: double n
    pub(crate) n: f64,
    // Java field: double covarianceNumerator
    pub(crate) covarianceNumerator: f64,
    // Java field: boolean biasCorrected
    biasCorrected: bool,
}

// Java: public class is
impl is {
}

// Java: public class is
impl is_2 {
}

// Java: public class StorelessBivariateCovariance
impl StorelessBivariateCovariance {
    // Java: void increment(final double x, final double y)
    pub fn increment(&self) {
        // Java: n++;
        // Java: final double deltaX = x - meanX;
        // Java: final double deltaY = y - meanY;
        // Java: meanX += deltaX / n;
        // Java: meanY += deltaY / n;
        // Java: covarianceNumerator += ((n - 1.0) / n) * deltaX * deltaY;
    }

    // Java: void append(StorelessBivariateCovariance cov)
    pub fn append(&self) {
        // Java: double oldN = n;
        // Java: n += cov.n;
        // Java: final double deltaX = cov.meanX - meanX;
        // Java: final double deltaY = cov.meanY - meanY;
        // Java: meanX += deltaX * cov.n / n;
        // Java: meanY += deltaY * cov.n / n;
        // Java: covarianceNumerator += cov.covarianceNumerator + oldN * cov.n / n * deltaX * deltaY;
    }

    // Java: double getN()
    pub fn getN(&self) -> f64 {
        // Java: return n;
        todo!()
    }

    // Java: double getResult()
    pub fn getResult(&self) -> f64 {
        // Java: if (n < 2)
        {
            // Java: throw new NumberIsTooSmallException(LocalizedFormats.INSUFFICIENT_DIMENSION,
                                                // Java: n, 2, true);
        }
        // Java: if (biasCorrected)
        {
            // Java: return covarianceNumerator / (n - 1d);
        }
        // Java: else
        {
            // Java: return covarianceNumerator / n;
        }
        todo!()
    }

}
