// Java: import android.text.TextPaint
use crate::cn::wps::text::layout::typo::layout::style::CharacterStyle::TextPaint;
use crate::cn::wps::text::layout::typo::layout::style::Typeface::Typeface;

// Java: public class TypefaceSpan extends MetricAffectingSpan
pub struct TypefaceSpan {
    // Java field: String mFamily
    pub(crate) mFamily: Option<String>,
    // Java field: Typeface mTypeface
    pub(crate) mTypeface: Option<Typeface>,
}

// Java: public class TypefaceSpan extends MetricAffectingSpan
impl TypefaceSpan {
    // Java: TypefaceSpan(String family)
    pub fn new_with_family(family: Option<String>) -> Self {
      // Java: this(family, (Typeface)null);
      Self::new(family, None)
    }

    // Java: TypefaceSpan(Typeface typeface)
    pub fn new_with_typeface(typeface: Option<Typeface>) -> Self {
      // Java: this((String)null, typeface);
      Self::new(None, typeface)
    }

    // Java: TypefaceSpan(String family, Typeface typeface)
    pub fn new(family: Option<String>, typeface: Option<Typeface>) -> Self {
      // Java: super();
      // Java: this.mFamily = family;
      // Java: this.mTypeface = typeface;
      TypefaceSpan { mFamily: family, mTypeface: typeface }
    }

    // Java: String getFamily()
    pub fn getFamily(&self) -> &Option<String> {
      // Java: return this.mFamily;
      &self.mFamily
    }

    // Java: Typeface getTypeface()
    pub fn getTypeface(&self) -> &Option<Typeface> {
      // Java: return this.mTypeface;
      &self.mTypeface
    }

    // Java: void updateDrawState(TextPaint ds)
    pub fn updateDrawState(&self, _ds: &mut TextPaint) {
    }

    // Java: void updateMeasureState(TextPaint paint)
    pub fn updateMeasureState(&self, _paint: &mut TextPaint) {
    }

}
