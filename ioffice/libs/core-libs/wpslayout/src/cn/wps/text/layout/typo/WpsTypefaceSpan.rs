// Java: import android.graphics.Paint
pub struct Paint {
    typeface: Option<Typeface>,
}
impl Paint {
    pub fn setTypeface(&mut self, tf: Typeface) { self.typeface = Some(tf); }
}
// Java: import android.graphics.Typeface
use crate::cn::wps::text::layout::typo::layout::style::Typeface::Typeface;
// Java: import android.text.TextPaint
use crate::cn::wps::text::layout::typo::layout::style::CharacterStyle::TextPaint;
// Java: import cn.wps.text.layout.typo.layout.style.TypefaceSpan
use crate::cn::wps::text::layout::typo::layout::style::TypefaceSpan::TypefaceSpan;

// Java: public class WpsTypefaceSpan extends TypefaceSpan
pub struct WpsTypefaceSpan {
    // Java field: Typeface newType
    newType: Option<Typeface>,
    // Java field: boolean isTopBottomFont
    pub(crate) isTopBottomFont: bool,
    // Java field: String fontName
    pub(crate) fontName: String,
    // Java field: boolean isBold
    pub(crate) isBold: bool,
    // Java field: boolean isItalic
    pub(crate) isItalic: bool,
    base: TypefaceSpan,
}

// Java: public class WpsTypefaceSpan extends TypefaceSpan
impl WpsTypefaceSpan {
    // Java: WpsTypefaceSpan(String family, String fontName, boolean isBold, boolean isItalic, Typeface type, boolean isTopBottomFont)
    pub fn new(family: Option<String>, fontName: String, isBold: bool, isItalic: bool, r#type: Option<Typeface>, isTopBottomFont: bool) -> Self {
      // Java: super(family);
      let base = TypefaceSpan::new_with_family(family);
      // Java: this.newType = type;
      // Java: this.isTopBottomFont = isTopBottomFont;
      // Java: this.fontName = fontName;
      // Java: this.isBold = isBold;
      // Java: this.isItalic = isItalic;
      WpsTypefaceSpan {
          newType: r#type,
          isTopBottomFont,
          fontName,
          isBold,
          isItalic,
          base,
      }
    }

    // Java: boolean isTopBottomFont()
    pub fn isTopBottomFont(&self) -> bool {
      // Java: return this.isTopBottomFont;
      self.isTopBottomFont
    }

    // Java: void updateDrawState(TextPaint paint)
    pub fn updateDrawState(&self, _paint: &mut TextPaint) {
      // Java: applyTypeFace(paint, this.newType);
      // applyTypeFace handled via stub Paint
    }

    // Java: void updateMeasureState(TextPaint paint)
    pub fn updateMeasureState(&self, _paint: &mut TextPaint) {
      // Java: applyTypeFace(paint, this.newType);
      // applyTypeFace handled via stub Paint
    }

    // Java: static void applyTypeFace(Paint paint, Typeface tf)
    pub fn applyTypeFace(paint: &mut Paint, tf: Option<Typeface>) {
      // Java: if (tf != null)
      if let Some(typeface) = tf
      {
         // Java: paint.setTypeface(tf);
         paint.setTypeface(typeface);
      }
    }

    // Java: String getFontName()
    pub fn getFontName(&self) -> &str {
      // Java: return this.fontName;
      &self.fontName
    }

    // Java: boolean isBold()
    pub fn isBold(&self) -> bool {
      // Java: return this.isBold;
      self.isBold
    }

    // Java: boolean isItalic()
    pub fn isItalic(&self) -> bool {
      // Java: return this.isItalic;
      self.isItalic
    }

}
