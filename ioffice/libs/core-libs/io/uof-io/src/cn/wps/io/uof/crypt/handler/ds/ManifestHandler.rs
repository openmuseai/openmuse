// Java: import cn.wps.io.uof.crypt.UofEncryptInfo
use crate::cn::wps::io::uof::crypt::UofEncryptInfo::UofEncryptInfo;
// Java: import cn.wps.io.uof.crypt.handler.SecurityHandlerBase
use crate::cn::wps::io::uof::crypt::handler::SecurityHandlerBase::SecurityHandlerBase;
// Java: import cn.wps.kfc.xpxml.reader.ElementHandler
use core_libs_kfc::kfc::xpxml::reader::ElementHandler::ElementHandler;
// Java: import cn.wps.kfc.xpxml.reader.XmlRoAttr
use core_libs_kfc::kfc::xpxml::reader::XmlRoAttr::XmlRoAttr;

use super::ReferenceHandler::ReferenceHandler;

// Java: public class ManifestHandler extends SecurityHandlerBase
pub struct ManifestHandler {
    pub(crate) base: SecurityHandlerBase,
    // Java field: ReferenceHandler mReferenceHandler
    mReferenceHandler: Option<ReferenceHandler>,
}

// Java: public class ManifestHandler extends SecurityHandlerBase
impl ManifestHandler {
    // Java: ManifestHandler(UofEncryptInfo uofEncryptInfo)
    pub fn ManifestHandler(uofEncryptInfo: UofEncryptInfo) -> Self {
      // Java: super(uofEncryptInfo);
      ManifestHandler {
          base: SecurityHandlerBase::SecurityHandlerBase(uofEncryptInfo),
          mReferenceHandler: None,
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
         // Java: case 1078812459:
         1078812459i32 =>
            // Java: return this.getReferenceHandler();
            Some(Box::new(ReferenceHandler::ReferenceHandler(self.base.mUofEncryptInfo.clone()))),
         // Java: default:
         _ =>
            // Java: return null;
            None,
      }
    }

    // Java: ReferenceHandler getReferenceHandler()
    fn getReferenceHandler(&mut self) -> &ReferenceHandler {
      // Java: if (null == this.mReferenceHandler)
      if self.mReferenceHandler.is_none() {
         // Java: this.mReferenceHandler = new ReferenceHandler(this.mUofEncryptInfo);
         self.mReferenceHandler = Some(ReferenceHandler::ReferenceHandler(self.base.mUofEncryptInfo.clone()));
      }
      // Java: return this.mReferenceHandler;
      self.mReferenceHandler.as_ref().unwrap()
    }

}

impl ElementHandler for ManifestHandler {
    fn startElement(&self, var1: i32, var2: &dyn XmlRoAttr) { self.startElement(var1, var2); }
    fn endElement(&self, var1: i32) { self.endElement(var1); }
    fn characters(&self, var1: &str) { self.characters(var1); }
    fn enterSubElement(&self, var1: i32) -> Option<Box<dyn ElementHandler>> { self.enterSubElement(var1) }
}
