// Java: import cn.wps.io.uof.crypt.UofEncryptInfo
use crate::cn::wps::io::uof::crypt::UofEncryptInfo::UofEncryptInfo;
// Java: import cn.wps.io.uof.crypt.handler.SecurityHandlerBase
use crate::cn::wps::io::uof::crypt::handler::SecurityHandlerBase::SecurityHandlerBase;
// Java: import cn.wps.kfc.xpxml.reader.ElementHandler
use core_libs_kfc::kfc::xpxml::reader::ElementHandler::ElementHandler;
// Java: import cn.wps.kfc.xpxml.reader.XmlRoAttr
use core_libs_kfc::kfc::xpxml::reader::XmlRoAttr::XmlRoAttr;

use super::Permission_F803Handler::Permission_F803Handler;

// Java: public class PermissionSetup_F802Handler extends SecurityHandlerBase
pub struct PermissionSetup_F802Handler {
    pub(crate) base: SecurityHandlerBase,
    // Java field: Permission_F803Handler mPermission_F803Handler
    mPermission_F803Handler: Option<Permission_F803Handler>,
}

// Java: public class PermissionSetup_F802Handler extends SecurityHandlerBase
impl PermissionSetup_F802Handler {
    // Java: PermissionSetup_F802Handler(UofEncryptInfo uofEncryptInfo)
    pub fn PermissionSetup_F802Handler(uofEncryptInfo: UofEncryptInfo) -> Self {
      // Java: super(uofEncryptInfo);
      PermissionSetup_F802Handler {
          base: SecurityHandlerBase::SecurityHandlerBase(uofEncryptInfo),
          mPermission_F803Handler: None,
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
         // Java: case 63491:
         63491i32 =>
            // Java: return this.getPermission_F803Handler();
            Some(Box::new(Permission_F803Handler::Permission_F803Handler(self.base.mUofEncryptInfo.clone()))),
         // Java: default:
         _ =>
            // Java: return null;
            None,
      }
    }

    // Java: Permission_F803Handler getPermission_F803Handler()
    fn getPermission_F803Handler(&mut self) -> &Permission_F803Handler {
      // Java: if (null == this.mPermission_F803Handler)
      if self.mPermission_F803Handler.is_none() {
         // Java: this.mPermission_F803Handler = new Permission_F803Handler(this.mUofEncryptInfo);
         self.mPermission_F803Handler = Some(Permission_F803Handler::Permission_F803Handler(self.base.mUofEncryptInfo.clone()));
      }
      // Java: return this.mPermission_F803Handler;
      self.mPermission_F803Handler.as_ref().unwrap()
    }

}

impl ElementHandler for PermissionSetup_F802Handler {
    fn startElement(&self, var1: i32, var2: &dyn XmlRoAttr) { self.startElement(var1, var2); }
    fn endElement(&self, var1: i32) { self.endElement(var1); }
    fn characters(&self, var1: &str) { self.characters(var1); }
    fn enterSubElement(&self, var1: i32) -> Option<Box<dyn ElementHandler>> { self.enterSubElement(var1) }
}
