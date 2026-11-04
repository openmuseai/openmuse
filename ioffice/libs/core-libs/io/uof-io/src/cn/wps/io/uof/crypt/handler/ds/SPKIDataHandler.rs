// Java: import cn.wps.io.uof.crypt.UofEncryptInfo
use crate::cn::wps::io::uof::crypt::UofEncryptInfo::UofEncryptInfo;
// Java: import cn.wps.io.uof.crypt.handler.SecurityHandlerBase
use crate::cn::wps::io::uof::crypt::handler::SecurityHandlerBase::SecurityHandlerBase;
// Java: import cn.wps.kfc.xpxml.reader.ElementHandler
use core_libs_kfc::kfc::xpxml::reader::ElementHandler::ElementHandler;
// Java: import cn.wps.kfc.xpxml.reader.XmlRoAttr
use core_libs_kfc::kfc::xpxml::reader::XmlRoAttr::XmlRoAttr;

use super::SPKISexpHandler::SPKISexpHandler;

// Java: public class SPKIDataHandler extends SecurityHandlerBase
pub struct SPKIDataHandler {
    pub(crate) base: SecurityHandlerBase,
    // Java field: SPKISexpHandler mSPKISexpHandler
    mSPKISexpHandler: Option<SPKISexpHandler>,
}

// Java: public class SPKIDataHandler extends SecurityHandlerBase
impl SPKIDataHandler {
    // Java: SPKIDataHandler(UofEncryptInfo uofEncryptInfo)
    pub fn SPKIDataHandler(uofEncryptInfo: UofEncryptInfo) -> Self {
      // Java: super(uofEncryptInfo);
      SPKIDataHandler {
          base: SecurityHandlerBase::SecurityHandlerBase(uofEncryptInfo),
          mSPKISexpHandler: None,
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
         // Java: case -1172602907:
         -1172602907i32 =>
            // Java: return this.getSPKISexpHandler();
            Some(Box::new(SPKISexpHandler::SPKISexpHandler(self.base.mUofEncryptInfo.clone()))),
         // Java: default:
         _ =>
            // Java: return null;
            None,
      }
    }

    // Java: SPKISexpHandler getSPKISexpHandler()
    fn getSPKISexpHandler(&mut self) -> &SPKISexpHandler {
      // Java: if (null == this.mSPKISexpHandler)
      if self.mSPKISexpHandler.is_none() {
         // Java: this.mSPKISexpHandler = new SPKISexpHandler(this.mUofEncryptInfo);
         self.mSPKISexpHandler = Some(SPKISexpHandler::SPKISexpHandler(self.base.mUofEncryptInfo.clone()));
      }
      // Java: return this.mSPKISexpHandler;
      self.mSPKISexpHandler.as_ref().unwrap()
    }

}

impl ElementHandler for SPKIDataHandler {
    fn startElement(&self, var1: i32, var2: &dyn XmlRoAttr) { self.startElement(var1, var2); }
    fn endElement(&self, var1: i32) { self.endElement(var1); }
    fn characters(&self, var1: &str) { self.characters(var1); }
    fn enterSubElement(&self, var1: i32) -> Option<Box<dyn ElementHandler>> { self.enterSubElement(var1) }
}
