#![allow(unused_imports, dead_code, non_snake_case, unused_variables, unused_mut, unused_assignments)]

// Java: import java.io.Serializable
// Java: import org.apache.commons.math3.exception.NullArgumentException
use crate::org::apache::commons::math3::exception::NullArgumentException;
// Java: import org.apache.commons.math3.util.MathUtils
use crate::org::apache::commons::math3::util::MathUtils;

use std::any::Any;

// Java field: long serialVersionUID
pub(crate) const serialVersionUID: i64 = -7818711964045118679;

// Java: public class concurrently
pub struct concurrently {}

// Java: public class ThirdMoment
pub struct ThirdMoment {
    // Java field: double m3
    pub(crate) m3: f64,
    // Java field: double nDevSq
    pub(crate) nDevSq: f64,
    pub(crate) n: i64,
    pub(crate) m1: f64,
    pub(crate) m2: f64,
    pub(crate) dev: f64,
    pub(crate) nDev: f64,
}

// Java: public class concurrently
impl concurrently {}

// Java: public class ThirdMoment
impl ThirdMoment {
    pub fn new() -> Self {
        ThirdMoment {
            m3: f64::NAN,
            nDevSq: f64::NAN,
            n: 0,
            m1: f64::NAN,
            m2: f64::NAN,
            dev: f64::NAN,
            nDev: f64::NAN,
        }
    }

    // Java: void increment(final double d)
    pub fn increment(&mut self, d: f64) {
        // Java: if (n < 1)
        if self.n < 1 {
            // Java: m3 = m2 = m1 = 0.0;
            self.m3 = 0.0;
            self.m2 = 0.0;
            self.m1 = 0.0;
        }
        // Java: double prevM2 = m2;
        let prev_m2 = self.m2;
        // Java: super.increment(d);
        self.n += 1;
        let n0 = self.n as f64;
        self.dev = d - self.m1;
        self.nDev = self.dev / n0;
        self.m1 += self.nDev;
        self.m2 += (self.n - 1) as f64 * self.dev * self.nDev;
        // Java: nDevSq = nDev * nDev;
        self.nDevSq = self.nDev * self.nDev;
        // Java: double n0 = n;
        // Java: m3 = m3 - 3.0 * nDev * prevM2 + (n0 - 1) * (n0 - 2) * nDevSq * dev;
        self.m3 = self.m3 - 3.0 * self.nDev * prev_m2 + (n0 - 1.0) * (n0 - 2.0) * self.nDevSq * self.dev;
    }

    // Java: double getResult()
    pub fn getResult(&self) -> f64 {
        // Java: return m3;
        self.m3
    }

    // Java: void clear()
    pub fn clear(&mut self) {
        // Java: super.clear();
        self.n = 0;
        self.m1 = f64::NAN;
        self.m2 = f64::NAN;
        self.dev = f64::NAN;
        self.nDev = f64::NAN;
        // Java: m3 = Double.NaN;
        self.m3 = f64::NAN;
        // Java: nDevSq = Double.NaN;
        self.nDevSq = f64::NAN;
    }

    // Java: ThirdMoment copy()
    pub fn copy(&self) -> ThirdMoment {
        // Java: ThirdMoment result = new ThirdMoment();
        // Java: // No try-catch or advertised exception because args are guaranteed non-null
        // Java: copy(this, result);
        // Java: return result;
        ThirdMoment {
            m3: self.m3,
            nDevSq: self.nDevSq,
            n: self.n,
            m1: self.m1,
            m2: self.m2,
            dev: self.dev,
            nDev: self.nDev,
        }
    }

    // Java: static void copy(ThirdMoment source, ThirdMoment dest)
    pub fn copy_to(source: &ThirdMoment, dest: &mut ThirdMoment) {
        // Java: dest.m3 = source.m3;
        dest.m3 = source.m3;
        // Java: dest.nDevSq = source.nDevSq;
        dest.nDevSq = source.nDevSq;
    }
}
