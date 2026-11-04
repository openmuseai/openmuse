#![allow(unused_imports, dead_code, non_snake_case, unused_variables, unused_mut, unused_assignments)]

// Java: import java.io.Serializable
// Java: import org.apache.commons.math3.exception.NullArgumentException
use crate::org::apache::commons::math3::exception::NullArgumentException;
// Java: import org.apache.commons.math3.util.MathUtils
use crate::org::apache::commons::math3::util::MathUtils;

// Java field: long serialVersionUID
pub(crate) const serialVersionUID: i64 = -6399246845653829380i64;

// Java: public class concurrently
pub struct concurrently {
}

// Java: public class FourthMoment
pub struct FourthMoment {
    // Java field: double m4
    pub(crate) m4: f64,
}

// Java: public class concurrently
impl concurrently {
}

// Java: public class FourthMoment
impl FourthMoment {
    // Java: void increment(final double d)
    pub fn increment(&mut self, _d: f64) {
        // Java: if (n < 1)
        // Java: m4 = 0.0;
        // Java: m3 = 0.0;
        // Java: m2 = 0.0;
        // Java: m1 = 0.0;
        // Java: double prevM3 = m3;
        // Java: double prevM2 = m2;
        // Java: super.increment(d);
        // Java: double n0 = n;
        // Java: m4 = m4 - 4.0 * nDev * prevM3 + 6.0 * nDevSq * prevM2 +
        // Java: ((n0 * n0) - 3 * (n0 -1)) * (nDevSq * nDevSq * (n0 - 1) * n0);
        todo!()
    }

    // Java: double getResult()
    pub fn getResult(&self) -> f64 {
        // Java: return m4;
        self.m4
    }

    // Java: void clear()
    pub fn clear(&mut self) {
        // Java: super.clear();
        // Java: m4 = Double.NaN;
        self.m4 = f64::NAN;
    }

    // Java: FourthMoment copy()
    pub fn copy_instance(&self) -> FourthMoment {
        // Java: FourthMoment result = new FourthMoment();
        // Java: // No try-catch or advertised exception because args are guaranteed non-null
        // Java: copy(this, result);
        // Java: return result;
        FourthMoment { m4: self.m4 }
    }

    // Java: static void copy(FourthMoment source, FourthMoment dest)
    pub fn copy_static(source: &FourthMoment, dest: &mut FourthMoment) {
        dest.m4 = source.m4;
    }

}
