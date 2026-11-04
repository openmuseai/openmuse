// java package cn.wps.et.ss.formula.eval;
use super::IValueEval;
// java public final class StringEval implements IStringValueEval
pub struct StringEval {
    _value: Vec<u16>,
}
static EMPTY: StringEval = StringEval { _value: Vec::new() };
impl StringEval {
    // java public static final StringEval EMPTY_INSTANCE = new StringEval("");
    pub const EMPTY_INSTANCE: &'static Self = &EMPTY;
    // java public StringEval(String value) { this._value = value; }
    pub fn StringEval(value: &str) -> Self {
        Self::from_utf16(value.encode_utf16().collect())
    }
    // Nullable transport of the same original String constructor.
    // java public StringEval(String value){if(value==null){throw new IllegalArgumentException("value must not be null");}else{this._value=value;}}
    pub fn StringEval__nullable(value:Option<&str>)->Self{let value=value.expect("IllegalArgumentException value must not be null");Self::from_utf16(value.encode_utf16().collect())}
    // java public StringEval(Ptg ptg) { this(((StringPtg)ptg).getValue()); }
    pub fn StringEval__ptg(ptg: &crate::cn::wps::et::ss::formula::ptg::Ptg::Ptg) -> Self {
        use crate::cn::wps::et::ss::formula::ptg::Ptg::PtgKind;
        match &ptg.kind {
            PtgKind::Str(s) => Self::StringEval(s),
            PtgKind::StrRaw { units, .. } => Self::from_utf16(units.clone()),
            _ => panic!("ClassCastException StringPtg"),
        }
    }
    // java getClass().getName()+" ["+_value+"]"; all UTF-16 units remain intact.
    pub fn toString(&self) -> Vec<u16> {
        let mut text: Vec<u16> = "cn.wps.et.ss.formula.eval.StringEval ["
            .encode_utf16()
            .collect();
        text.extend_from_slice(&self._value);
        text.push(']' as u16);
        text
    }
    // rust: Java String's UTF-16 storage also retains unpaired surrogates.
    pub fn from_utf16(value: Vec<u16>) -> Self {
        Self { _value: value }
    }
    // java public String getStringValue() { return this._value; }
    pub fn getStringValue(&self) -> &[u16] {
        &self._value
    }
    // java return o instanceof StringEval && ((StringEval)o)._value.equals(this._value);
    pub fn equals(&self, other: &dyn IValueEval) -> bool {
        other
            .as_any()
            .downcast_ref::<Self>()
            .is_some_and(|v| v._value == self._value)
    }
    // java return this._value.hashCode();
    pub fn hashCode(&self) -> i32 {
        self._value
            .iter()
            .fold(0i32, |h, c| h.wrapping_mul(31).wrapping_add(*c as i32))
    }
}
impl IValueEval for StringEval {
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
}

impl super::IStringValueEval::IStringValueEval for StringEval {
    fn getStringValue(&self) -> Vec<u16> {
        self._value.clone()
    }
}
