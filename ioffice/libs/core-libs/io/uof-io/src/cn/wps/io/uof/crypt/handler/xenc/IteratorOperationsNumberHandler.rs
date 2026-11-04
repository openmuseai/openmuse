// Java: import cn.wps.base.io.TypeConversion
use crate::compat::type_conversion;
// Java: import cn.wps.io.uof.crypt.UofEncryptInfo
use crate::cn::wps::io::uof::crypt::UofEncryptInfo::UofEncryptInfo;
// Java: import cn.wps.io.uof.crypt.handler.SecurityHandlerBase
use crate::cn::wps::io::uof::crypt::handler::SecurityHandlerBase::SecurityHandlerBase;
// Java: import cn.wps.kfc.xpxml.reader.ElementHandler
use core_libs_kfc::kfc::xpxml::reader::ElementHandler::ElementHandler;
// Java: import cn.wps.kfc.xpxml.reader.XmlRoAttr
use core_libs_kfc::kfc::xpxml::reader::XmlRoAttr::XmlRoAttr;

// Java: public class IteratorOperationsNumberHandler extends SecurityHandlerBase
pub struct IteratorOperationsNumberHandler {
    pub(crate) base: SecurityHandlerBase,
}

// Java: public class IteratorOperationsNumberHandler extends SecurityHandlerBase
impl IteratorOperationsNumberHandler {
    // Java: IteratorOperationsNumberHandler(UofEncryptInfo uofEncryptInfo)
    pub fn IteratorOperationsNumberHandler(uofEncryptInfo: UofEncryptInfo) -> Self {
      // Java: super(uofEncryptInfo);
      IteratorOperationsNumberHandler {
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
      // Java: Integer val = TypeConversion.string2Integer(chars);
      let val = type_conversion::string2Integer(Some(chars));
      // Java: if (null != val)
      if let Some(v) = val
      {
         // Java: this.mUofEncryptInfo.setSpinCount(val);
         self.base.mUofEncryptInfo.setSpinCount(v);
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

impl ElementHandler for IteratorOperationsNumberHandler {
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
