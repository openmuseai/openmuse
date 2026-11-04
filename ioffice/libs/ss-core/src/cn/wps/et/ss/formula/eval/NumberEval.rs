// java package cn.wps.et.ss.formula.eval;
use super::IValueEval;
// java public final class NumberEval implements INumericValueEval, IStringValueEval
pub struct NumberEval {
    _value: f64,
    _stringValue: std::sync::OnceLock<String>,
}
static ZERO: NumberEval = NumberEval {
    _value: 0.0,
    _stringValue: std::sync::OnceLock::new(),
};
impl NumberEval {
    // java public static final NumberEval ZERO = new NumberEval(0.0);
    pub const ZERO: &'static Self = &ZERO;
    // java public NumberEval(double value) { this._value = value; }
    pub fn NumberEval(value: f64) -> Self {
        Self {
            _value: value,
            _stringValue: std::sync::OnceLock::new(),
        }
    }
    // java public NumberEval(Ptg ptg): null rejects, sid 30/31 only, then concrete token cast.
    pub fn NumberEval__ptg(ptg: Option<&crate::cn::wps::et::ss::formula::ptg::Ptg::Ptg>) -> Self {
        use crate::cn::wps::et::ss::formula::ptg::Ptg::PtgKind;
        let ptg = ptg.expect("ptg must not be null");
        match ptg.kind {
            PtgKind::Int(n) => Self::NumberEval(n as f64),
            PtgKind::Number(n) => Self::NumberEval(n),
            _ => panic!("bad argument type"),
        }
    }
    // java public double getNumberValue() { return this._value; }
    pub fn getNumberValue(&self) -> f64 {
        self._value
    }
    // java if (_stringValue==null) _stringValue=NumberToTextConverter.toText(_value,'.'); return _stringValue;
    pub fn getStringValue(&self) -> &str {
        self._stringValue.get_or_init(|| {
            crate::org::apache::poi::ss::util::NumberToTextConverter::NumberToTextConverter::toText(
                self._value,
                '.',
            )
        })
    }
    // java toString(): getClass().getSimpleName()+" ["+getStringValue()+"]";
    pub fn toString(&self) -> String {
        format!("NumberEval [{}]", self.getStringValue())
    }
    // java public boolean equals(Object o)
    pub fn equals(&self, other: &dyn IValueEval) -> bool {
        other
            .as_any()
            .downcast_ref::<Self>()
            .is_some_and(|v| v._value == self._value)
    }
    // java long bits = Double.doubleToLongBits(this._value); return (int)(bits ^ bits >>> 32);
    pub fn hashCode(&self) -> i32 {
        let bits = if self._value.is_nan() {
            0x7ff8000000000000
        } else {
            self._value.to_bits()
        };
        (bits ^ (bits >> 32)) as i32
    }
}
impl IValueEval for NumberEval {
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
    fn as_numeric_eval(&self) -> Option<&dyn super::INumericValueEval::INumericValueEval> {
        Some(self)
    }
}

impl super::INumericValueEval::INumericValueEval for NumberEval {
    fn getNumberValue(&self) -> f64 {
        self.getNumberValue()
    }
}
impl super::IStringValueEval::IStringValueEval for NumberEval {
    fn getStringValue(&self) -> Vec<u16> {
        self.getStringValue().encode_utf16().collect()
    }
}
