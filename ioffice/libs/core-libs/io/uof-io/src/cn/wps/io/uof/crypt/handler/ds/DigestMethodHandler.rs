// Java: import cn.wps.io.uof.crypt.UofEncryptInfo
use crate::cn::wps::io::uof::crypt::UofEncryptInfo::UofEncryptInfo;
// Java: import cn.wps.io.uof.crypt.handler.SecurityHandlerBase
use crate::cn::wps::io::uof::crypt::handler::SecurityHandlerBase::SecurityHandlerBase;
// Java: import cn.wps.kfc.xpxml.reader.ElementHandler
use core_libs_kfc::kfc::xpxml::reader::ElementHandler::ElementHandler;
// Java: import cn.wps.kfc.xpxml.reader.XmlRoAttr
use core_libs_kfc::kfc::xpxml::reader::XmlRoAttr::XmlRoAttr;
use core_libs_kfc::kfc::xpxml::reader::XmlRoAttrValue::XmlRoAttrValue;

// Java: public class DigestMethodHandler extends SecurityHandlerBase
pub struct DigestMethodHandler {
    pub(crate) base: SecurityHandlerBase,
}

// Java: public class DigestMethodHandler extends SecurityHandlerBase
impl DigestMethodHandler {
    // Java: DigestMethodHandler(UofEncryptInfo uofEncryptInfo)
    pub fn DigestMethodHandler(uofEncryptInfo: UofEncryptInfo) -> Self {
      // Java: super(uofEncryptInfo);
      DigestMethodHandler {
          base: SecurityHandlerBase::SecurityHandlerBase(uofEncryptInfo),
      }
    }

    // Java: void startElement(int element, XmlRoAttr attrsNotCopy)
    pub fn startElement(&mut self, _element: i32, attrsNotCopy: &dyn XmlRoAttr) {
      // Java: XmlRoAttr tmp;
      // Java: if ((tmp = attrsNotCopy.getByName(-2065509297)) != null)
      if let Some(tmp) = attrsNotCopy.get_by_name(-2065509297)
      {
         // Java: String algorithm = tmp.toString();
         let _algorithm = tmp.to_string();
         // Java: this.mUofEncryptInfo.setHashAlgo("SM3");
         self.base.mUofEncryptInfo.setHashAlgo("SM3".to_string());
      }
    }

    // Java: void endElement(int elementId)
    pub fn endElement(&self, _elementId: i32) {
    }

    // Java: void characters(String chars)
    pub fn characters(&self, _chars: &str) {
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

impl ElementHandler for DigestMethodHandler {
    fn startElement(&self, _var1: i32, _var2: &dyn XmlRoAttr) {
    }
    fn endElement(&self, var1: i32) {
        self.endElement(var1);
    }
    fn characters(&self, var1: &str) {
        self.characters(var1);
    }
    fn enterSubElement(&self, var1: i32) -> Option<Box<dyn ElementHandler>> {
        self.enterSubElement(var1)
    }
}
