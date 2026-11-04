// Java: import cn.wps.io.uof.crypt.UofEncryptInfo
use crate::cn::wps::io::uof::crypt::UofEncryptInfo::UofEncryptInfo;
// Java: import cn.wps.io.uof.crypt.handler.SecurityHandlerBase
use crate::cn::wps::io::uof::crypt::handler::SecurityHandlerBase::SecurityHandlerBase;
// Java: import cn.wps.kfc.xpxml.reader.ElementHandler
use core_libs_kfc::kfc::xpxml::reader::ElementHandler::ElementHandler;
// Java: import cn.wps.kfc.xpxml.reader.XmlRoAttr
use core_libs_kfc::kfc::xpxml::reader::XmlRoAttr::XmlRoAttr;

use super::Path_F812Handler::Path_F812Handler;
use super::Signature_F811Handler::Signature_F811Handler;

// Java: public class Stamp_F810Handler extends SecurityHandlerBase
pub struct Stamp_F810Handler {
    pub(crate) base: SecurityHandlerBase,
    // Java field: Path_F812Handler mPath_F812Handler
    mPath_F812Handler: Option<Path_F812Handler>,
    // Java field: Signature_F811Handler mSignature_F811Handler
    mSignature_F811Handler: Option<Signature_F811Handler>,
}

// Java: public class Stamp_F810Handler extends SecurityHandlerBase
impl Stamp_F810Handler {
    // Java: Stamp_F810Handler(UofEncryptInfo uofEncryptInfo)
    pub fn Stamp_F810Handler(uofEncryptInfo: UofEncryptInfo) -> Self {
      // Java: super(uofEncryptInfo);
      Stamp_F810Handler {
          base: SecurityHandlerBase::SecurityHandlerBase(uofEncryptInfo),
          mPath_F812Handler: None,
          mSignature_F811Handler: None,
      }
    }

    // Java: void startElement(int element, XmlRoAttr attrsNotCopy)
    pub fn startElement(&self, _element: i32, _attrsNotCopy: &dyn XmlRoAttr) {
    }

    // Java: void endElement(int elementId)
    pub fn endElement(&self, _elementId: i32) {
    }

    // Java: void characters(String chars)
    pub fn characters(&self, _chars: &str) {
    }

    // Java: ElementHandler enterSubElement(int subElement)
    pub fn enterSubElement(&self, subElement: i32) -> Option<Box<dyn ElementHandler>> {
      // Java: switch (subElement)
      match subElement
      {
         // Java: case 63505:
         63505i32 =>
            // Java: return this.getSignature_F811Handler();
            Some(Box::new(Signature_F811Handler::Signature_F811Handler(self.base.mUofEncryptInfo.clone()))),
         // Java: case 63506:
         63506i32 =>
            // Java: return this.getPath_F812Handler();
            Some(Box::new(Path_F812Handler::Path_F812Handler(self.base.mUofEncryptInfo.clone()))),
         // Java: default:
         _ =>
            // Java: return null;
            None,
      }
    }

    // Java: Path_F812Handler getPath_F812Handler()
    fn getPath_F812Handler(&mut self) -> &Path_F812Handler {
      // Java: if (null == this.mPath_F812Handler)
      if self.mPath_F812Handler.is_none() {
         // Java: this.mPath_F812Handler = new Path_F812Handler(this.mUofEncryptInfo);
         self.mPath_F812Handler = Some(Path_F812Handler::Path_F812Handler(self.base.mUofEncryptInfo.clone()));
      }
      // Java: return this.mPath_F812Handler;
      self.mPath_F812Handler.as_ref().unwrap()
    }

    // Java: Signature_F811Handler getSignature_F811Handler()
    fn getSignature_F811Handler(&mut self) -> &Signature_F811Handler {
      // Java: if (null == this.mSignature_F811Handler)
      if self.mSignature_F811Handler.is_none() {
         // Java: this.mSignature_F811Handler = new Signature_F811Handler(this.mUofEncryptInfo);
         self.mSignature_F811Handler = Some(Signature_F811Handler::Signature_F811Handler(self.base.mUofEncryptInfo.clone()));
      }
      // Java: return this.mSignature_F811Handler;
      self.mSignature_F811Handler.as_ref().unwrap()
    }

}

impl ElementHandler for Stamp_F810Handler {
    fn startElement(&self, var1: i32, var2: &dyn XmlRoAttr) { self.startElement(var1, var2); }
    fn endElement(&self, var1: i32) { self.endElement(var1); }
    fn characters(&self, var1: &str) { self.characters(var1); }
    fn enterSubElement(&self, var1: i32) -> Option<Box<dyn ElementHandler>> { self.enterSubElement(var1) }
}
