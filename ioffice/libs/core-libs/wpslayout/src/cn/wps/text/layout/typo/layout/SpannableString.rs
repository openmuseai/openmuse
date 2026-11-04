#![allow(non_snake_case)]
#![allow(unused_variables)]
#![allow(dead_code)]
#![allow(unused_imports)]

// Java: public class SpannableString extends SpannableStringInternal implements CharSequence, GetChars, Spannable
pub struct SpannableString {
}

// Java: public class SpannableString extends SpannableStringInternal implements CharSequence, GetChars, Spannable
impl SpannableString {
    // Java: SpannableString(CharSequence source, boolean ignoreNoCopySpan)
    pub fn new(_source: i32, _ignoreNoCopySpan: bool) -> Self {
      // Java: super(source, 0, source.length(), ignoreNoCopySpan);
      SpannableString {}
    }

    // Java: SpannableString(CharSequence source)
    pub fn new_from_source(_source: i32) -> Self {
      // Java: this(source, false);
      Self::new(_source, false)
    }

    // Java: SpannableString(CharSequence source, int start, int end)
    pub fn new_range(_source: i32, _start: i32, _end: i32) -> Self {
      // Java: super(source, start, end, false);
      SpannableString {}
    }

    // Java: static SpannableString valueOf(CharSequence source)
    pub fn valueOf(_source: i32) -> SpannableString {
      // Java: return source instanceof SpannableString ? (SpannableString)source : new SpannableString(source);
      SpannableString::new_from_source(_source)
    }

    // Java: void setSpan(Object what, int start, int end, int flags)
    pub fn setSpan(&self, _what: i32, _start: i32, _end: i32, _flags: i32) {
      // Java: super.setSpan(what, start, end, flags);
    }

    // Java: void removeSpan(Object what)
    pub fn removeSpan(&self, _what: i32) {
      // Java: super.removeSpan(what);
    }

    // Java: CharSequence subSequence(int start, int end)
    pub fn subSequence(&self, _start: i32, _end: i32) -> i32 {
      // Java: return new SpannableString(this, start, end);
      0
    }
}
