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

// Java: public class Skewness
pub struct Skewness {
    // Java field: ThirdMoment moment
    pub(crate) moment: Box<dyn std::any::Any>,
    // Java field: boolean incMoment
    pub(crate) incMoment: bool,
}

// Java: public class concurrently
impl concurrently {
}

// Java: public class Skewness
impl Skewness {
    // Java: Skewness()
    pub fn Skewness(&self) {
        // Java: incMoment = true;
        // Java: moment = new ThirdMoment();
    }

    // Java: Skewness(final ThirdMoment m3)
    pub fn Skewness_2(&self) {
        // Java: incMoment = false;
        // Java: this.moment = m3;
    }

    // Java: Skewness(Skewness original)
    pub fn Skewness_3(&self) {
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
        // Java: if (moment.n < 3)
        {
            // Java: return Double.NaN;
        }
        // Java: double variance = moment.m2 / (moment.n - 1);
        // Java: if (variance < 10E-20)
        {
            // Java: return 0.0d;
        }
        // Java: else
        {
            // Java: double n0 = moment.getN();
            // Java: return  (n0 * moment.m3) /
            // Java: ((n0 - 1) * (n0 -2) * FastMath.sqrt(variance) * variance);
        }
        todo!()
    }

    // Java: long getN()
    pub fn getN(&self) -> i64 {
        // Java: return moment.getN();
        todo!()
    }

    // Java: void clear()
    pub fn clear(&self) {
        // Java: if (incMoment)
        {
            // Java: moment.clear();
        }
    }

    // Java: double evaluate()
    pub fn evaluate(&self) -> f64 {
        todo!()
    }

    // Java: Skewness copy()
    pub fn copy(&self) -> Skewness {
        // Java: Skewness result = new Skewness();
        // Java: // No try-catch or advertised exception because args are guaranteed non-null
        // Java: copy(this, result);
        // Java: return result;
        todo!()
    }

    // Java: static void copy(Skewness source, Skewness dest)
    pub fn copy_2(&self) {
    }

}
