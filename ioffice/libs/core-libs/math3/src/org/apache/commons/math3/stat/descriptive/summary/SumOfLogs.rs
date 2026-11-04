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

// Java: public class SumOfLogs
pub struct SumOfLogs {
    // Java field: int n
    pub(crate) n: i32,
    // Java field: double value
    pub(crate) value: f64,
}

// Java: public class concurrently
impl concurrently {
}

// Java: public class SumOfLogs
impl SumOfLogs {
    // Java: SumOfLogs()
    pub fn SumOfLogs(&self) {
       // Java: value = 0d;
       // Java: n = 0;
    }

    // Java: SumOfLogs(SumOfLogs original)
    pub fn SumOfLogs_2(&self) {
        // Java: copy(original, this);
    }

    // Java: void increment(final double d)
    pub fn increment(&self) {
        // Java: value += FastMath.log(d);
        // Java: n++;
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

    // Java: void clear()
    pub fn clear(&self) {
        // Java: value = 0d;
        // Java: n = 0;
    }

    // Java: double evaluate(final double[] values, final int begin, final int length)
    pub fn evaluate(&self) -> f64 {
        todo!()
    }

    // Java: SumOfLogs copy()
    pub fn copy(&self) -> SumOfLogs {
        // Java: SumOfLogs result = new SumOfLogs();
        // Java: // No try-catch or advertised exception here because args are valid
        // Java: copy(this, result);
        // Java: return result;
        todo!()
    }

    // Java: static void copy(SumOfLogs source, SumOfLogs dest)
    pub fn copy_2(&self) {
    }

}
