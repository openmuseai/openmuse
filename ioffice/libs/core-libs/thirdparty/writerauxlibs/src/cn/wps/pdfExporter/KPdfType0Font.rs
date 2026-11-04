#![allow(dead_code, non_snake_case, unused_variables, unused_mut, unused_imports)]

use super::KPdfFont::KPdfFont;
use super::KPdfObject::KPdfObject;
use super::KPdfArray::KPdfArray;
use super::KPdfTypeface::KPdfTypeface;
use super::KPdfUtils::KPdfUtils;
use super::KPdfCIDFont::KPdfCIDFont;
use super::KPdfDict::KPdfDict;

// Java: public class KPdfType0Font extends KPdfFont
pub struct KPdfType0Font {
    pub(crate) font: KPdfFont,
}

// Java: public class KPdfType0Font extends KPdfFont
#[allow(non_snake_case)]
impl KPdfType0Font {
    // Java: KPdfType0Font(KAdvancedTypefaceMetrics info, KPdfTypeface typeface)
    pub fn new(info: (), typeface: KPdfTypeface) -> Self {
      // Java: super(info, typeface);
      Self {
          font: KPdfFont::new(info, typeface),
      }
    }

    // Java: KPdfType0Font getFontSubset()
    pub fn getFontSubset(&self) -> KPdfType0Font {
      // Java: KPdfType0Font newSubset = new KPdfType0Font(this.mFontInfo, this.mTypeface);
      // Java: newSubset.populate();
      // Java: return newSubset;
      todo!("getFontSubset requires cloning KPdfTypeface")
    }

    // Java: boolean populate()
    pub fn populate(&mut self) -> bool {
      // Java: String name = this.mTypeface.getFamilyName() == null ? "宋体" : this.mTypeface.getFamilyName();
      let name = {
          let family = self.font.mTypeface.getFamilyName();
          if family.is_empty() { "宋体".to_string() } else { family.to_string() }
      };
      // Java: byte[] font = KPdfUtils.getGBKBytes(name);
      let _font_bytes = KPdfUtils::getGBKBytes(&name);
      // Java: this.insertName("Subtype", "Type0");
      // Java: this.insertName("BaseFont", font);
      // Java: this.insertName("Encoding", "Identity-H");
      // Java: KPdfCIDFont newCIDFont = new KPdfCIDFont(this.mFontInfo, this.mTypeface);
      // Java: this.addResource(newCIDFont);
      self.font.addResource(KPdfObject::new());
      // Java: KPdfArray descendantFonts = new KPdfArray();
      let mut descendantFonts = KPdfArray::new();
      // Java: descendantFonts.append((KPdfObject)(new KPdfObjectRef(newCIDFont)));
      descendantFonts.append(KPdfObject::new());
      // Java: this.insert("DescendantFonts", descendantFonts);
      // Java: this.populateToUnicodeTable();
      self.font.populateToUnicodeTable();
      // Java: return true;
      true
    }

}
