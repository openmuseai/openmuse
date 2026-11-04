#![allow(unused_imports, dead_code, non_snake_case, unused_variables, unused_mut, unused_assignments)]

// Java: import java.io.Serializable
// Java: import org.apache.commons.math3.exception.NullArgumentException
use crate::org::apache::commons::math3::exception::NullArgumentException;
// Java: import org.apache.commons.math3.stat.descriptive.AbstractStorelessUnivariateStatistic
use crate::org::apache::commons::math3::stat::descriptive::AbstractStorelessUnivariateStatistic;
// Java: import org.apache.commons.math3.util.MathUtils
use crate::org::apache::commons::math3::util::MathUtils;

use std::any::Any;

// Java field: long serialVersionUID
pub(crate) const serialVersionUID: i64 = 6112755307178490473;

// Java: public class concurrently
pub struct concurrently {}

// Java: public class concurrently
impl concurrently {}

// Java field: long n
// Java field: double m1
// Java field: double dev
// Java field: double nDev

// Java: public class FirstMoment
pub struct FirstMoment {
    pub n: i64,
    pub m1: f64,
    pub dev: f64,
    pub nDev: f64,
}

// Java: public class FirstMoment
impl FirstMoment {
    // Java: FirstMoment()
    pub fn new() -> Self {
        // Java: n = 0;
        // Java: m1 = Double.NaN;
        // Java: dev = Double.NaN;
        // Java: nDev = Double.NaN;
        FirstMoment {
            n: 0,
            m1: f64::NAN,
            dev: f64::NAN,
            nDev: f64::NAN,
        }
    }

    // Java: FirstMoment(FirstMoment original)
    pub fn new_copy(original: &FirstMoment) -> Self {
        // Java: copy(original, this);
        FirstMoment {
            n: original.n,
            m1: original.m1,
            dev: original.dev,
            nDev: original.nDev,
        }
    }

    // Java: void increment(final double d)
    pub fn increment(&mut self, d: f64) {
        // Java: if (n == 0)
        if self.n == 0 {
            // Java: m1 = 0.0;
            self.m1 = 0.0;
        }
        // Java: n++;
        self.n += 1;
        // Java: double n0 = n;
        let n0 = self.n as f64;
        // Java: dev = d - m1;
        self.dev = d - self.m1;
        // Java: nDev = dev / n0;
        self.nDev = self.dev / n0;
        // Java: m1 += nDev;
        self.m1 += self.nDev;
    }

    // Java: void clear()
    pub fn clear(&mut self) {
        // Java: m1 = Double.NaN;
        self.m1 = f64::NAN;
        // Java: n = 0;
        self.n = 0;
        // Java: dev = Double.NaN;
        self.dev = f64::NAN;
        // Java: nDev = Double.NaN;
        self.nDev = f64::NAN;
    }

    // Java: double getResult()
    pub fn getResult(&self) -> f64 {
        // Java: return m1;
        self.m1
    }

    // Java: long getN()
    pub fn getN(&self) -> i64 {
        // Java: return n;
        self.n
    }

    // Java: FirstMoment copy()
    pub fn copy(&self) -> FirstMoment {
        // Java: FirstMoment result = new FirstMoment();
        // Java: copy(this, result);
        // Java: return result;
        FirstMoment::new_copy(self)
    }

    // Java: static void copy(FirstMoment source, FirstMoment dest)
    pub fn copy_to(source: &FirstMoment, dest: &mut FirstMoment) {
        // Java: dest.n = source.n;
        dest.n = source.n;
        // Java: dest.m1 = source.m1;
        dest.m1 = source.m1;
        // Java: dest.dev = source.dev;
        dest.dev = source.dev;
        // Java: dest.nDev = source.nDev;
        dest.nDev = source.nDev;
    }
}
