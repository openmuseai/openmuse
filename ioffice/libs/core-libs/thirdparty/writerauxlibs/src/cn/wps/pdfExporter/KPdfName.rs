// Java: import cn.wps.base.assertion.KSAssert
// use util::cn::wps::base::assertion::KSAssert; // not available in this crate

use super::KPdfObject::KPdfObject;
use super::KStream::KStream;
use super::KPdfUtils::KPdfUtils;

// Java field: int MAXLEN
pub(crate) const MAXLEN: i32 = 127;
// Java field: String ESCAPED
pub(crate) const ESCAPED: &str = "#/%()<>[]{}";

// Java: public class KPdfName extends KPdfObject
pub struct KPdfName {
    // Java field: String name
    pub(crate) name: String,
}

// Java: public class KPdfName extends KPdfObject
#[allow(non_snake_case)]
impl KPdfName {
    // Java: KPdfName(String name)
    pub fn new(name: &str) -> Self {
      // Java: super();
      // Java: this.name = this.FormatName(name);
      let formatted = Self::FormatName_str(name);
      Self { name: formatted }
    }

    // Java: KPdfName(byte[] name)
    pub fn from_bytes(name: &[u8]) -> Self {
      // Java: super();
      // Java: this.name = this.FormatName(name);
      let formatted = Self::FormatName_bytes(name);
      Self { name: formatted }
    }

    // Java: long getOutputSize(KPdfCatalog catalog, boolean indirect)
    pub fn getOutputSize(&self, _catalog: &KPdfObject, _indirect: bool) -> i64 {
      // Java: return (long)this.name.length();
      self.name.len() as i64
    }

    // Java: void emitObject(KStream stream, KPdfCatalog catalog, boolean indirect)
    pub fn emitObject(&self, stream: &mut KStream, _catalog: &KPdfObject, _indirect: bool) {
      // Java: stream.write(this.name);
      stream.write_str(&self.name);
    }

    // Java: String FormatName(String name)
    fn FormatName_str(name: &str) -> String {
      // Java: KSAssert.assertTrue(name.length() < 127);
      debug_assert!(name.len() < 127);
      // Java: StringBuffer result = new StringBuffer("/");
      let mut result = String::from("/");
      // Java: int size = name.length();
      let chars: Vec<char> = name.chars().collect();
      let size = chars.len();
      // Java: for(int i = 0; i < size; ++i)
      for i in 0..size {
         // Java: char c = name.charAt(i);
         let c = chars[i];
         // Java: if ((c & 128) == 0 && c >= '!' && "#/%()<>[]{}".indexOf(c) < 0)
         if (c as u32 & 128) == 0 && c >= '!' && !ESCAPED.contains(c) {
            // Java: result.append(c);
            result.push(c);
         }
         // Java: else
         else {
            // Java: result.append("#");
            result.push('#');
            // Java: result.append(KPdfUtils.getHex((int)(c & 255), 2));
            result.push_str(&KPdfUtils::getHex_int((c as i32) & 255, 2));
         }
      }
      // Java: return result.toString();
      result
    }

    // Java: String FormatName(byte[] name)
    fn FormatName_bytes(name: &[u8]) -> String {
      // Java: KSAssert.assertTrue(name.length < 127);
      debug_assert!(name.len() < 127);
      // Java: StringBuffer result = new StringBuffer("/");
      let mut result = String::from("/");
      // Java: for(int i = 0; i < name.length; ++i)
      for i in 0..name.len() {
         // Java: char c = (char)name[i];
         let c = name[i] as char;
         // Java: if ((c & 128) == 0 && c >= '!' && "#/%()<>[]{}".indexOf(c) < 0)
         if (c as u32 & 128) == 0 && c >= '!' && !ESCAPED.contains(c) {
            // Java: result.append(c);
            result.push(c);
         }
         // Java: else
         else {
            // Java: result.append("#");
            result.push('#');
            // Java: result.append(KPdfUtils.getHex((int)(c & 255), 2));
            result.push_str(&KPdfUtils::getHex_int((c as i32) & 255, 2));
         }
      }
      // Java: return result.toString();
      result
    }

}
