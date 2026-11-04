// Java: import cn.wps.io.uof.crypt.UofEncryptInfo
use crate::cn::wps::io::uof::crypt::UofEncryptInfo::UofEncryptInfo;
// Java: import cn.wps.io.uof.crypt.handler.SecurityHandlerBase
use crate::cn::wps::io::uof::crypt::handler::SecurityHandlerBase::SecurityHandlerBase;
// Java: import cn.wps.kfc.xpxml.reader.ElementHandler
use core_libs_kfc::kfc::xpxml::reader::ElementHandler::ElementHandler;
// Java: import cn.wps.kfc.xpxml.reader.XmlRoAttr
use core_libs_kfc::kfc::xpxml::reader::XmlRoAttr::XmlRoAttr;

use super::SignaturePropertyHandler::SignaturePropertyHandler;

// Java: public class SignaturePropertiesHandler extends SecurityHandlerBase
pub struct SignaturePropertiesHandler {
    pub(crate) base: SecurityHandlerBase,
    // Java field: SignaturePropertyHandler mSignaturePropertyHandler
    mSignaturePropertyHandler: Option<SignaturePropertyHandler>,
}

// Java: public class SignaturePropertiesHandler extends SecurityHandlerBase
impl SignaturePropertiesHandler {
    // Java: SignaturePropertiesHandler(UofEncryptInfo uofEncryptInfo)
    pub fn SignaturePropertiesHandler(uofEncryptInfo: UofEncryptInfo) -> Self {
      // Java: super(uofEncryptInfo);
      SignaturePropertiesHandler {
          base: SecurityHandlerBase::SecurityHandlerBase(uofEncryptInfo),
          mSignaturePropertyHandler: None,
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
         // Java: case -645799283:
         -645799283i32 =>
            // Java: return this.getSignaturePropertyHandler();
            Some(Box::new(SignaturePropertyHandler::SignaturePropertyHandler(self.base.mUofEncryptInfo.clone()))),
         // Java: default:
         _ =>
            // Java: return null;
            None,
      }
    }

    // Java: SignaturePropertyHandler getSignaturePropertyHandler()
    fn getSignaturePropertyHandler(&mut self) -> &SignaturePropertyHandler {
      // Java: if (null == this.mSignaturePropertyHandler)
      if self.mSignaturePropertyHandler.is_none() {
         // Java: this.mSignaturePropertyHandler = new SignaturePropertyHandler(this.mUofEncryptInfo);
         self.mSignaturePropertyHandler = Some(SignaturePropertyHandler::SignaturePropertyHandler(self.base.mUofEncryptInfo.clone()));
      }
      // Java: return this.mSignaturePropertyHandler;
      self.mSignaturePropertyHandler.as_ref().unwrap()
    }

}

impl ElementHandler for SignaturePropertiesHandler {
    fn startElement(&self, var1: i32, var2: &dyn XmlRoAttr) { self.startElement(var1, var2); }
    fn endElement(&self, var1: i32) { self.endElement(var1); }
    fn characters(&self, var1: &str) { self.characters(var1); }
    fn enterSubElement(&self, var1: i32) -> Option<Box<dyn ElementHandler>> { self.enterSubElement(var1) }
}
