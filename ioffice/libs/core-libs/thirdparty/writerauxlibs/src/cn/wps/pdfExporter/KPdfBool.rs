// Java: import cn.wps.base.assertion.KSAssert
// use util::cn::wps::base::assertion::KSAssert; // not available in this crate

use super::KPdfObject::KPdfObject;
use super::KStream::KStream;

// Java: public class KPdfBool extends KPdfObject
pub struct KPdfBool {
    // Java field: boolean fValue
    pub(crate) fValue: bool,
}

// Java: public class KPdfBool extends KPdfObject
#[allow(non_snake_case)]
impl KPdfBool {
    // Java: KPdfBool(boolean val)
    pub fn new(val: bool) -> Self {
      // Java: super();
      // Java: this.fValue = val;
      Self { fValue: val }
    }

    // Java: void emitObject(KStream stream, KPdfCatalog catalog, boolean indirect)
    pub fn emitObject(&self, stream: &mut KStream, _catalog: &KPdfObject, indirect: bool) {
      // Java: KSAssert.assertFalse(indirect);
      debug_assert!(!indirect);
      // Java: if (this.fValue)
      if self.fValue {
         // Java: stream.write("true");
         stream.write_str("true");
      }
      // Java: else
      else {
         // Java: stream.write("false");
         stream.write_str("false");
      }
    }

    // Java: long getOutputSize(KPdfCatalog catalog, boolean indirect)
    pub fn getOutputSize(&self, _catalog: &KPdfObject, indirect: bool) -> i64 {
      // Java: KSAssert.assertFalse(indirect);
      debug_assert!(!indirect);
      // Java: return this.fValue ? (long)"true".length() : (long)"false".length();
      if self.fValue { 4 } else { 5 }
    }

}
