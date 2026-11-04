// Java: import cn.wps.io.uof.crypt.UofEncryptInfo
use crate::cn::wps::io::uof::crypt::UofEncryptInfo::UofEncryptInfo;
// Java: import cn.wps.io.uof.crypt.handler.SecurityHandlerBase
use crate::cn::wps::io::uof::crypt::handler::SecurityHandlerBase::SecurityHandlerBase;
// Java: import cn.wps.kfc.xpxml.reader.ElementHandler
use core_libs_kfc::kfc::xpxml::reader::ElementHandler::ElementHandler;
// Java: import cn.wps.kfc.xpxml.reader.XmlRoAttr
use core_libs_kfc::kfc::xpxml::reader::XmlRoAttr::XmlRoAttr;
// Java: import org.apache.poi.util.base64.Base64
use crate::compat::base64_util;

// Java: public class DigestValueHandler extends SecurityHandlerBase
pub struct DigestValueHandler {
    pub(crate) base: SecurityHandlerBase,
}

// Java: public class DigestValueHandler extends SecurityHandlerBase
impl DigestValueHandler {
    // Java: DigestValueHandler(UofEncryptInfo uofEncryptInfo)
    pub fn DigestValueHandler(uofEncryptInfo: UofEncryptInfo) -> Self {
      // Java: super(uofEncryptInfo);
      DigestValueHandler {
          base: SecurityHandlerBase::SecurityHandlerBase(uofEncryptInfo),
      }
    }

    // Java: void startElement(int element, XmlRoAttr attrsNotCopy)
    pub fn startElement(&self, _element: i32, _attrsNotCopy: &dyn XmlRoAttr) {
    }

    // Java: void endElement(int elementId)
    pub fn endElement(&self, _elementId: i32) {
    }

    // Java: void characters(String chars)
    pub fn characters(&mut self, chars: &str) {
      // Java: if (null != chars)
      if !chars.is_empty()
      {
         // Java: this.mUofEncryptInfo.setVerifiedEncPassword(Base64.decode(chars.getBytes()));
         self.base.mUofEncryptInfo.setVerifiedEncPassword(base64_util::decode(chars));
      }
    }

    // Java: ElementHandler enterSubElement(int subElement)
    pub fn enterSubElement(&self, _subElement: i32) -> Option<Box<dyn ElementHandler>> {
      // Java: switch (subElement)
      match _subElement
      {
         // Java: default:
         _ =>
            // Java: return null;
            None,
      }
    }

}

impl ElementHandler for DigestValueHandler {
    fn startElement(&self, var1: i32, var2: &dyn XmlRoAttr) {
        self.startElement(var1, var2);
    }
    fn endElement(&self, var1: i32) {
        self.endElement(var1);
    }
    fn characters(&self, _var1: &str) {
    }
    fn enterSubElement(&self, var1: i32) -> Option<Box<dyn ElementHandler>> {
        self.enterSubElement(var1)
    }
}
