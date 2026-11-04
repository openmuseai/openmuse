// Java: import cn.wps.io.uof.crypt.UofEncryptInfo
use crate::cn::wps::io::uof::crypt::UofEncryptInfo::UofEncryptInfo;
// Java: import cn.wps.io.uof.crypt.handler.SecurityHandlerBase
use crate::cn::wps::io::uof::crypt::handler::SecurityHandlerBase::SecurityHandlerBase;
// Java: import cn.wps.kfc.xpxml.reader.ElementHandler
use core_libs_kfc::kfc::xpxml::reader::ElementHandler::ElementHandler;
// Java: import cn.wps.kfc.xpxml.reader.XmlRoAttr
use core_libs_kfc::kfc::xpxml::reader::XmlRoAttr::XmlRoAttr;

use super::HMACOutputLengthHandler::HMACOutputLengthHandler;

// Java: public class SignatureMethodHandler extends SecurityHandlerBase
pub struct SignatureMethodHandler {
    pub(crate) base: SecurityHandlerBase,
    // Java field: HMACOutputLengthHandler mHMACOutputLengthHandler
    mHMACOutputLengthHandler: Option<HMACOutputLengthHandler>,
}

// Java: public class SignatureMethodHandler extends SecurityHandlerBase
impl SignatureMethodHandler {
    // Java: SignatureMethodHandler(UofEncryptInfo uofEncryptInfo)
    pub fn SignatureMethodHandler(uofEncryptInfo: UofEncryptInfo) -> Self {
      // Java: super(uofEncryptInfo);
      SignatureMethodHandler {
          base: SecurityHandlerBase::SecurityHandlerBase(uofEncryptInfo),
          mHMACOutputLengthHandler: None,
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
         // Java: case 853496014:
         853496014i32 =>
            // Java: return this.getHMACOutputLengthHandler();
            Some(Box::new(HMACOutputLengthHandler::HMACOutputLengthHandler(self.base.mUofEncryptInfo.clone()))),
         // Java: default:
         _ =>
            // Java: return null;
            None,
      }
    }

    // Java: HMACOutputLengthHandler getHMACOutputLengthHandler()
    fn getHMACOutputLengthHandler(&mut self) -> &HMACOutputLengthHandler {
      // Java: if (null == this.mHMACOutputLengthHandler)
      if self.mHMACOutputLengthHandler.is_none() {
         // Java: this.mHMACOutputLengthHandler = new HMACOutputLengthHandler(this.mUofEncryptInfo);
         self.mHMACOutputLengthHandler = Some(HMACOutputLengthHandler::HMACOutputLengthHandler(self.base.mUofEncryptInfo.clone()));
      }
      // Java: return this.mHMACOutputLengthHandler;
      self.mHMACOutputLengthHandler.as_ref().unwrap()
    }

}

impl ElementHandler for SignatureMethodHandler {
    fn startElement(&self, var1: i32, var2: &dyn XmlRoAttr) { self.startElement(var1, var2); }
    fn endElement(&self, var1: i32) { self.endElement(var1); }
    fn characters(&self, var1: &str) { self.characters(var1); }
    fn enterSubElement(&self, var1: i32) -> Option<Box<dyn ElementHandler>> { self.enterSubElement(var1) }
}
