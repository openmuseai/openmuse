// Java: import cn.wps.io.uof.crypt.UofEncryptInfo
use crate::cn::wps::io::uof::crypt::UofEncryptInfo::UofEncryptInfo;
// Java: import cn.wps.io.uof.crypt.handler.SecurityHandlerBase
use crate::cn::wps::io::uof::crypt::handler::SecurityHandlerBase::SecurityHandlerBase;
// Java: import cn.wps.kfc.xpxml.reader.ElementHandler
use core_libs_kfc::kfc::xpxml::reader::ElementHandler::ElementHandler;
// Java: import cn.wps.kfc.xpxml.reader.XmlRoAttr
use core_libs_kfc::kfc::xpxml::reader::XmlRoAttr::XmlRoAttr;

use super::Role_F804Handler::Role_F804Handler;
use super::Object_F805Handler::Object_F805Handler;

// Java: public class Permission_F803Handler extends SecurityHandlerBase
pub struct Permission_F803Handler {
    pub(crate) base: SecurityHandlerBase,
    // Java field: Role_F804Handler mRole_F804Handler
    mRole_F804Handler: Option<Role_F804Handler>,
    // Java field: Object_F805Handler mObject_F805Handler
    mObject_F805Handler: Option<Object_F805Handler>,
}

// Java: public class Permission_F803Handler extends SecurityHandlerBase
impl Permission_F803Handler {
    // Java: Permission_F803Handler(UofEncryptInfo uofEncryptInfo)
    pub fn Permission_F803Handler(uofEncryptInfo: UofEncryptInfo) -> Self {
      // Java: super(uofEncryptInfo);
      Permission_F803Handler {
          base: SecurityHandlerBase::SecurityHandlerBase(uofEncryptInfo),
          mRole_F804Handler: None,
          mObject_F805Handler: None,
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
         // Java: case 63492:
         63492i32 =>
            // Java: return this.getRole_F804Handler();
            Some(Box::new(Role_F804Handler::Role_F804Handler(self.base.mUofEncryptInfo.clone()))),
         // Java: case 63493:
         63493i32 =>
            // Java: return this.getObject_F805Handler();
            Some(Box::new(Object_F805Handler::Object_F805Handler(self.base.mUofEncryptInfo.clone()))),
         // Java: default:
         _ =>
            // Java: return null;
            None,
      }
    }

    // Java: Role_F804Handler getRole_F804Handler()
    fn getRole_F804Handler(&mut self) -> &Role_F804Handler {
      // Java: if (null == this.mRole_F804Handler)
      if self.mRole_F804Handler.is_none() {
         // Java: this.mRole_F804Handler = new Role_F804Handler(this.mUofEncryptInfo);
         self.mRole_F804Handler = Some(Role_F804Handler::Role_F804Handler(self.base.mUofEncryptInfo.clone()));
      }
      // Java: return this.mRole_F804Handler;
      self.mRole_F804Handler.as_ref().unwrap()
    }

    // Java: Object_F805Handler getObject_F805Handler()
    fn getObject_F805Handler(&mut self) -> &Object_F805Handler {
      // Java: if (null == this.mObject_F805Handler)
      if self.mObject_F805Handler.is_none() {
         // Java: this.mObject_F805Handler = new Object_F805Handler(this.mUofEncryptInfo);
         self.mObject_F805Handler = Some(Object_F805Handler::Object_F805Handler(self.base.mUofEncryptInfo.clone()));
      }
      // Java: return this.mObject_F805Handler;
      self.mObject_F805Handler.as_ref().unwrap()
    }

}

impl ElementHandler for Permission_F803Handler {
    fn startElement(&self, var1: i32, var2: &dyn XmlRoAttr) { self.startElement(var1, var2); }
    fn endElement(&self, var1: i32) { self.endElement(var1); }
    fn characters(&self, var1: &str) { self.characters(var1); }
    fn enterSubElement(&self, var1: i32) -> Option<Box<dyn ElementHandler>> { self.enterSubElement(var1) }
}
