// Java: import org.apache.commons.math3.exception.util.Localizable
use crate::org::apache::commons::math3::exception::util::Localizable;
// Java: import org.apache.commons.math3.exception.util.LocalizedFormats
use crate::org::apache::commons::math3::exception::util::LocalizedFormats;

// Java field: long serialVersionUID
pub(crate) const serialVersionUID: i64 = -3629324471511904459;

// Java: public class NoBracketingException extends MathIllegalArgumentException
#[derive(Debug, Clone)]
pub struct NoBracketingException {
    // Java field: double lo
    pub(crate) lo: f64,
    // Java field: double hi
    pub(crate) hi: f64,
    // Java field: double fLo
    pub(crate) fLo: f64,
    // Java field: double fHi
    pub(crate) fHi: f64,
}

// Java: public class NoBracketingException extends MathIllegalArgumentException
impl NoBracketingException {
    // Java: NoBracketingException()
    pub fn NoBracketingException(lo: f64, hi: f64, f_lo: f64, f_hi: f64) -> Self {
        NoBracketingException { lo, hi, fLo: f_lo, fHi: f_hi }
    }

    // Java: NoBracketingException()
    pub fn NoBracketingException_specific(_specific: &str, lo: f64, hi: f64, f_lo: f64, f_hi: f64) -> Self {
        NoBracketingException { lo, hi, fLo: f_lo, fHi: f_hi }
    }

    // Java: double getLo()
    pub fn getLo(&self) -> f64 {
        // Java: return lo;
        self.lo
    }

    // Java: double getHi()
    pub fn getHi(&self) -> f64 {
        // Java: return hi;
        self.hi
    }

    // Java: double getFLo()
    pub fn getFLo(&self) -> f64 {
        // Java: return fLo;
        self.fLo
    }

    // Java: double getFHi()
    pub fn getFHi(&self) -> f64 {
        // Java: return fHi;
        self.fHi
    }
}

impl std::fmt::Display for NoBracketingException {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "NoBracketingException: lo={}, hi={}, fLo={}, fHi={}", self.lo, self.hi, self.fLo, self.fHi)
    }
}

impl std::error::Error for NoBracketingException {}
