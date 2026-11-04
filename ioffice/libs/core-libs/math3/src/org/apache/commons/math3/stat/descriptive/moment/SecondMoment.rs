// Java: import java.io.Serializable
// Java: import org.apache.commons.math3.exception.NullArgumentException
use crate::org::apache::commons::math3::exception::NullArgumentException;
// Java: import org.apache.commons.math3.util.MathUtils
use crate::org::apache::commons::math3::util::MathUtils;

// Java field: long serialVersionUID

// Java: public class concurrently
pub struct concurrently {
}

// Java: public class SecondMoment
pub struct SecondMoment {
    // Java field: double m2
    pub(crate) m2: f64,
}

// Java: public class concurrently
impl concurrently {
}

// Java: public class SecondMoment
impl SecondMoment {
    // Java: SecondMoment()
    pub fn SecondMoment(&self) {
        // Java: super();
        // Java: m2 = Double.NaN;
    }

    // Java: SecondMoment(SecondMoment original)
    pub fn SecondMoment_2(&self) {
    }

    // Java: void increment(final double d)
    pub fn increment(&self) {
        // Java: if (n < 1)
        {
            // Java: m1 = m2 = 0.0;
        }
        // Java: super.increment(d);
        // Java: m2 += ((double) n - 1) * dev * nDev;
    }

    // Java: void clear()
    pub fn clear(&self) {
        // Java: super.clear();
        // Java: m2 = Double.NaN;
    }

    // Java: double getResult()
    pub fn getResult(&self) -> f64 {
        // Java: return m2;
        todo!()
    }

    // Java: SecondMoment copy()
    pub fn copy(&self) -> SecondMoment {
        // Java: SecondMoment result = new SecondMoment();
        // Java: // no try-catch or advertised NAE because args are guaranteed non-null
        // Java: copy(this, result);
        // Java: return result;
        todo!()
    }

    // Java: static void copy(SecondMoment source, SecondMoment dest)
    pub fn copy_2(&self) {
    }

}
