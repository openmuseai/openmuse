// java package cn.wps.et.ss.formula.eval;
use super::IValueEval;
// java public final class BoolEval implements INumericValueEval, IStringValueEval
pub struct BoolEval {
    _value: bool,
}
static TRUE: BoolEval = BoolEval { _value: true };
static FALSE: BoolEval = BoolEval { _value: false };
impl BoolEval {
    // java public static final BoolEval FALSE = new BoolEval(false), TRUE = new BoolEval(true);
    pub const TRUE: &'static Self = &TRUE;
    pub const FALSE: &'static Self = &FALSE;
    // java return b ? TRUE : FALSE;
    pub fn valueOf(b: bool) -> &'static Self {
        if b {
            Self::TRUE
        } else {
            Self::FALSE
        }
    }
    // java return this._value;
    pub fn getBooleanValue(&self) -> bool {
        self._value
    }
    // java return this._value ? 1.0 : 0.0;
    pub fn getNumberValue(&self) -> f64 {
        if self._value {
            1.0
        } else {
            0.0
        }
    }
    // java return this._value ? "TRUE" : "FALSE";
    pub fn getStringValue(&self) -> &'static str {
        if self._value {
            "TRUE"
        } else {
            "FALSE"
        }
    }
    // java public static Boolean parseBoolean(String strRep)
    pub fn parseBoolean(s: &str) -> Option<bool> {
        if s.eq_ignore_ascii_case("TRUE") {
            Some(true)
        } else if s.eq_ignore_ascii_case("FALSE") {
            Some(false)
        } else {
            None
        }
    }
    // java return o instanceof BoolEval && ((BoolEval)o)._value == this._value;
    pub fn equals(&self, other: &dyn IValueEval) -> bool {
        other
            .as_any()
            .downcast_ref::<Self>()
            .is_some_and(|v| v._value == self._value)
    }
    // java return this._value ? 19 : 23;
    pub fn hashCode(&self) -> i32 {
        if self._value {
            19
        } else {
            23
        }
    }
}
impl IValueEval for BoolEval {
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
    fn as_numeric_eval(&self) -> Option<&dyn super::INumericValueEval::INumericValueEval> {
        Some(self)
    }
}

impl super::INumericValueEval::INumericValueEval for BoolEval {
    fn getNumberValue(&self) -> f64 {
        self.getNumberValue()
    }
}
impl super::IStringValueEval::IStringValueEval for BoolEval {
    fn getStringValue(&self) -> Vec<u16> {
        self.getStringValue().encode_utf16().collect()
    }
}
