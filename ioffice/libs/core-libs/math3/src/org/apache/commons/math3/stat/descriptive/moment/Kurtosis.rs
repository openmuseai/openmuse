// Java: import java.io.Serializable
// Java: import org.apache.commons.math3.exception.MathIllegalArgumentException
use crate::org::apache::commons::math3::exception::MathIllegalArgumentException;
// Java: import org.apache.commons.math3.exception.NullArgumentException
use crate::org::apache::commons::math3::exception::NullArgumentException;
// Java: import org.apache.commons.math3.stat.descriptive.AbstractStorelessUnivariateStatistic
use crate::org::apache::commons::math3::stat::descriptive::AbstractStorelessUnivariateStatistic;
// Java: import org.apache.commons.math3.util.FastMath
use crate::org::apache::commons::math3::util::FastMath;
// Java: import org.apache.commons.math3.util.MathUtils
use crate::org::apache::commons::math3::util::MathUtils;

// Java field: long serialVersionUID

// Java: public class concurrently
pub struct concurrently {
}

// Java: public class Kurtosis
pub struct Kurtosis {
    // Java field: FourthMoment moment
    pub(crate) moment: Box<dyn std::any::Any>,
    // Java field: boolean incMoment
    pub(crate) incMoment: bool,
}

// Java: public class concurrently
impl concurrently {
}

// Java: public class Kurtosis
impl Kurtosis {
    // Java: Kurtosis()
    pub fn Kurtosis(&self) {
        // Java: incMoment = true;
        // Java: moment = new FourthMoment();
    }

    // Java: Kurtosis(final FourthMoment m4)
    pub fn Kurtosis_2(&self) {
        // Java: incMoment = false;
        // Java: this.moment = m4;
    }

    // Java: Kurtosis(Kurtosis original)
    pub fn Kurtosis_3(&self) {
        // Java: copy(original, this);
    }

    // Java: void increment(final double d)
    pub fn increment(&self) {
        // Java: if (incMoment)
        {
            // Java: moment.increment(d);
        }
    }

    // Java: double getResult()
    pub fn getResult(&self) -> f64 {
        // Java: double kurtosis = Double.NaN;
        // Java: if (moment.getN() > 3)
        {
            // Java: double variance = moment.m2 / (moment.n - 1);
                // Java: if (moment.n <= 3 || variance < 10E-20)
                {
                    // Java: kurtosis = 0.0;
                }
                // Java: else
                {
                    // Java: double n = moment.n;
                    // Java: kurtosis =
                        // Java: (n * (n + 1) * moment.getResult() -
                                // Java: 3 * moment.m2 * moment.m2 * (n - 1)) /
                                // Java: ((n - 1) * (n -2) * (n -3) * variance * variance);
                }
        }
        // Java: return kurtosis;
        todo!()
    }

    // Java: void clear()
    pub fn clear(&self) {
        // Java: if (incMoment)
        {
            // Java: moment.clear();
        }
    }

    // Java: long getN()
    pub fn getN(&self) -> i64 {
        // Java: return moment.getN();
        todo!()
    }

    // Java: double evaluate(final double[] values, final int begin, final int length)
    pub fn evaluate(&self) -> f64 {
        todo!()
    }

    // Java: Kurtosis copy()
    pub fn copy(&self) -> Kurtosis {
        // Java: Kurtosis result = new Kurtosis();
        // Java: // No try-catch because args are guaranteed non-null
        // Java: copy(this, result);
        // Java: return result;
        todo!()
    }

    // Java: static void copy(Kurtosis source, Kurtosis dest)
    pub fn copy_2(&self) {
    }

}
