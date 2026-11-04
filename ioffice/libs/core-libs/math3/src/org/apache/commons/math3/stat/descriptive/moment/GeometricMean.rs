// Java: import java.io.Serializable
// Java: import org.apache.commons.math3.exception.MathIllegalArgumentException
use crate::org::apache::commons::math3::exception::MathIllegalArgumentException;
// Java: import org.apache.commons.math3.exception.MathIllegalStateException
use crate::org::apache::commons::math3::exception::MathIllegalStateException;
// Java: import org.apache.commons.math3.exception.NullArgumentException
use crate::org::apache::commons::math3::exception::NullArgumentException;
// Java: import org.apache.commons.math3.exception.util.LocalizedFormats
use crate::org::apache::commons::math3::exception::util::LocalizedFormats;
// Java: import org.apache.commons.math3.stat.descriptive.AbstractStorelessUnivariateStatistic
use crate::org::apache::commons::math3::stat::descriptive::AbstractStorelessUnivariateStatistic;
// Java: import org.apache.commons.math3.stat.descriptive.StorelessUnivariateStatistic
use crate::org::apache::commons::math3::stat::descriptive::StorelessUnivariateStatistic;
// Java: import org.apache.commons.math3.stat.descriptive.summary.SumOfLogs
use crate::org::apache::commons::math3::stat::descriptive::summary::SumOfLogs;
// Java: import org.apache.commons.math3.util.FastMath
use crate::org::apache::commons::math3::util::FastMath;
// Java: import org.apache.commons.math3.util.MathUtils
use crate::org::apache::commons::math3::util::MathUtils;

// Java field: long serialVersionUID

// Java: public class concurrently
pub struct concurrently {
}

// Java: public class GeometricMean
pub struct GeometricMean {
    // Java field: StorelessUnivariateStatistic sumOfLogs
    pub(crate) sumOfLogs: Box<dyn std::any::Any>,
}

// Java: public class concurrently
impl concurrently {
}

// Java: public class GeometricMean
impl GeometricMean {
    // Java: GeometricMean()
    pub fn GeometricMean(&self) {
        // Java: sumOfLogs = new SumOfLogs();
    }

    // Java: GeometricMean(GeometricMean original)
    pub fn GeometricMean_2(&self) {
        // Java: super();
        // Java: copy(original, this);
    }

    // Java: GeometricMean(SumOfLogs sumOfLogs)
    pub fn GeometricMean_3(&self) {
        // Java: this.sumOfLogs = sumOfLogs;
    }

    // Java: GeometricMean copy()
    pub fn copy(&self) -> GeometricMean {
        // Java: GeometricMean result = new GeometricMean();
        // Java: // no try-catch or advertised exception because args guaranteed non-null
        // Java: copy(this, result);
        // Java: return result;
        todo!()
    }

    // Java: void increment(final double d)
    pub fn increment(&self) {
        // Java: sumOfLogs.increment(d);
    }

    // Java: double getResult()
    pub fn getResult(&self) -> f64 {
        // Java: if (sumOfLogs.getN() > 0)
        {
            // Java: return FastMath.exp(sumOfLogs.getResult() / sumOfLogs.getN());
        }
        // Java: else
        {
            // Java: return Double.NaN;
        }
        todo!()
    }

    // Java: void clear()
    pub fn clear(&self) {
        // Java: sumOfLogs.clear();
    }

    // Java: double evaluate()
    pub fn evaluate(&self) -> f64 {
        todo!()
    }

    // Java: long getN()
    pub fn getN(&self) -> i64 {
        // Java: return sumOfLogs.getN();
        todo!()
    }

    // Java: void setSumLogImpl(StorelessUnivariateStatistic sumLogImpl)
    pub fn setSumLogImpl(&self) {
    }

    // Java: StorelessUnivariateStatistic getSumLogImpl()
    pub fn getSumLogImpl(&self) -> Box<dyn std::any::Any> {
        // Java: return sumOfLogs;
        todo!()
    }

    // Java: static void copy(GeometricMean source, GeometricMean dest)
    pub fn copy_2(&self) {
    }

    // Java: void checkEmpty()
    pub fn checkEmpty(&self) {
        // Java: if (getN() > 0)
        {
            // Java: throw new MathIllegalStateException(
                    // Java: LocalizedFormats.VALUES_ADDED_BEFORE_CONFIGURING_STATISTIC,
                    // Java: getN());
        }
    }

}
