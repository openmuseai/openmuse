// Java: import java.io.Serializable
// Java: import org.apache.commons.math3.exception.MathIllegalArgumentException
use crate::org::apache::commons::math3::exception::MathIllegalArgumentException;
// Java: import org.apache.commons.math3.exception.NullArgumentException
use crate::org::apache::commons::math3::exception::NullArgumentException;
// Java: import org.apache.commons.math3.stat.descriptive.AbstractStorelessUnivariateStatistic
use crate::org::apache::commons::math3::stat::descriptive::AbstractStorelessUnivariateStatistic;
// Java: import org.apache.commons.math3.util.MathUtils
use crate::org::apache::commons::math3::util::MathUtils;

// Java field: long serialVersionUID

// Java: public class concurrently
pub struct concurrently {
}

// Java: public class Min
pub struct Min {
    // Java field: long n
    pub(crate) n: i64,
    // Java field: double value
    pub(crate) value: f64,
}

// Java: public class concurrently
impl concurrently {
}

// Java: public class Min
impl Min {
    // Java: Min()
    pub fn Min(&self) {
        // Java: n = 0;
        // Java: value = Double.NaN;
    }

    // Java: Min(Min original)
    pub fn Min_2(&self) {
        // Java: copy(original, this);
    }

    // Java: void increment(final double d)
    pub fn increment(&self) {
        // Java: if (d < value || Double.isNaN(value))
        {
            // Java: value = d;
        }
        // Java: n++;
    }

    // Java: void clear()
    pub fn clear(&self) {
        // Java: value = Double.NaN;
        // Java: n = 0;
    }

    // Java: double getResult()
    pub fn getResult(&self) -> f64 {
        // Java: return value;
        todo!()
    }

    // Java: long getN()
    pub fn getN(&self) -> i64 {
        // Java: return n;
        todo!()
    }

    // Java: double evaluate(final double[] values, final int begin, final int length)
    pub fn evaluate(&self) -> f64 {
        todo!()
    }

    // Java: Min copy()
    pub fn copy(&self) -> Min {
        // Java: Min result = new Min();
        // Java: // No try-catch or advertised exception - args are non-null
        // Java: copy(this, result);
        // Java: return result;
        todo!()
    }

    // Java: static void copy(Min source, Min dest)
    pub fn copy_2(&self) {
    }

}
