#![allow(dead_code, non_snake_case, unused_variables, unused_mut, unused_imports)]

use super::KStream::KStream;
use super::KPdfObject::KPdfObject;
use super::KPdfUtils::KPdfUtils;

// Java: public class KPdfString extends KPdfObject
pub struct KPdfString {
    // Java field: String val
    val: String,
}

// Java: public class KPdfString extends KPdfObject
#[allow(non_snake_case)]
impl KPdfString {
    // Java: KPdfString(String val)
    pub fn new(val: &str) -> Self {
      // Java: super();
      let mut instance = Self { val: String::new() };
      // Java: this.val = this.formatString(val, false);
      instance.val = Self::formatString_static(val, false);
      instance
    }

    // Java: long getOutputSize(KPdfCatalog catalog, boolean indirect)
    pub fn getOutputSize(&self, _catalog: &KPdfObject, indirect: bool) -> i64 {
      // Java: return indirect ? this.getIndirectOutputSize(catalog) : (long)this.val.length();
      if indirect {
          todo!("getIndirectOutputSize requires KPdfCatalog")
      } else {
          self.val.len() as i64
      }
    }

    // Java: void emitObject(KStream stream, KPdfCatalog catalog, boolean indirect)
    pub fn emitObject(&self, stream: &mut KStream, _catalog: &KPdfObject, indirect: bool) {
      // Java: if (indirect)
      if indirect
      {
         // Java: this.emitIndirectObject(stream, catalog);
         todo!("emitIndirectObject requires KPdfCatalog")
      }
      // Java: else
      else
      {
         // Java: stream.write(this.val);
         stream.write_str(&self.val);
      }
    }

    // Java: String formatString(String text, boolean bWide)
    fn formatString_static(text: &str, bWide: bool) -> String {
      // Java: StringBuffer buf = new StringBuffer();
      let mut buf = String::new();
      // Java: if (bWide)
      if bWide
      {
         // Java: buf.append("<");
         buf.push('<');
         // Java: int len = text.length();
         let len = text.len();
         // Java: for(int i = 0; i < len; ++i)
         for ch in text.chars()
         {
            // Java: buf.append(KPdfUtils.getHex((char)text.charAt(i), 4));
            buf.push_str(&KPdfUtils::getHex_char(ch as u16, 4));
         }
         // Java: buf.append(">");
         buf.push('>');
      }
      // Java: else
      else
      {
         // Java: buf.append("(");
         buf.push('(');
         // Java: int len = text.length();
         let len = text.len();
         // Java: for(int i = 0; i < len; ++i)
         for ch in text.chars()
         {
            // Java: char c = text.charAt(i);
            let c = ch;
            // Java: if (c == '\\' || c == '(' || c == ')')
            if c == '\\' || c == '(' || c == ')'
            {
               // Java: buf.append("\\");
               buf.push('\\');
            }
            // Java: buf.append(c);
            buf.push(c);
         }
         // Java: buf.append(")");
         buf.push(')');
      }
      // Java: return buf.toString();
      buf
    }

    pub fn formatString(&self, text: &str, bWide: bool) -> String {
      Self::formatString_static(text, bWide)
    }

}
