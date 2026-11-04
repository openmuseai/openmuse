// Java: import cn.wps.io.uof.crypt.UofEncryptInfo
use crate::cn::wps::io::uof::crypt::UofEncryptInfo::UofEncryptInfo;
// Java: import cn.wps.io.uof.crypt.handler.SecurityHandlerBase
use crate::cn::wps::io::uof::crypt::handler::SecurityHandlerBase::SecurityHandlerBase;
// Java: import cn.wps.kfc.xpxml.reader.ElementHandler
use core_libs_kfc::kfc::xpxml::reader::ElementHandler::ElementHandler;
// Java: import cn.wps.kfc.xpxml.reader.XmlRoAttr
use core_libs_kfc::kfc::xpxml::reader::XmlRoAttr::XmlRoAttr;

use super::EncryptionPropertyHandler::EncryptionPropertyHandler;

// Java: public class EncryptionPropertiesHandler extends SecurityHandlerBase
pub struct EncryptionPropertiesHandler {
    pub(crate) base: SecurityHandlerBase,
    // Java field: EncryptionPropertyHandler mEncryptionPropertyHandler
    mEncryptionPropertyHandler: Option<EncryptionPropertyHandler>,
}

// Java: public class EncryptionPropertiesHandler extends SecurityHandlerBase
impl EncryptionPropertiesHandler {
    // Java: EncryptionPropertiesHandler(UofEncryptInfo uofEncryptInfo)
    pub fn EncryptionPropertiesHandler(uofEncryptInfo: UofEncryptInfo) -> Self {
      // Java: super(uofEncryptInfo);
      EncryptionPropertiesHandler {
          base: SecurityHandlerBase::SecurityHandlerBase(uofEncryptInfo),
          mEncryptionPropertyHandler: None,
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
         // Java: case -154609000:
         -154609000i32 =>
            // Java: return this.getEncryptionPropertyHandler();
            Some(Box::new(EncryptionPropertyHandler::EncryptionPropertyHandler(self.base.mUofEncryptInfo.clone()))),
         // Java: default:
         _ =>
            // Java: return null;
            None,
      }
    }

    // Java: EncryptionPropertyHandler getEncryptionPropertyHandler()
    fn getEncryptionPropertyHandler(&mut self) -> &EncryptionPropertyHandler {
      // Java: if (null == this.mEncryptionPropertyHandler)
      if self.mEncryptionPropertyHandler.is_none() {
         // Java: this.mEncryptionPropertyHandler = new EncryptionPropertyHandler(this.mUofEncryptInfo);
         self.mEncryptionPropertyHandler = Some(EncryptionPropertyHandler::EncryptionPropertyHandler(self.base.mUofEncryptInfo.clone()));
      }
      // Java: return this.mEncryptionPropertyHandler;
      self.mEncryptionPropertyHandler.as_ref().unwrap()
    }

}

impl ElementHandler for EncryptionPropertiesHandler {
    fn startElement(&self, var1: i32, var2: &dyn XmlRoAttr) { self.startElement(var1, var2); }
    fn endElement(&self, var1: i32) { self.endElement(var1); }
    fn characters(&self, var1: &str) { self.characters(var1); }
    fn enterSubElement(&self, var1: i32) -> Option<Box<dyn ElementHandler>> { self.enterSubElement(var1) }
}
