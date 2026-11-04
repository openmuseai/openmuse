// Java: import cn.wps.io.uof.crypt.UofEncryptInfo
use crate::cn::wps::io::uof::crypt::UofEncryptInfo::UofEncryptInfo;
// Java: import cn.wps.io.uof.crypt.handler.SecurityHandlerBase
use crate::cn::wps::io::uof::crypt::handler::SecurityHandlerBase::SecurityHandlerBase;
// Java: import cn.wps.kfc.xpxml.reader.ElementHandler
use core_libs_kfc::kfc::xpxml::reader::ElementHandler::ElementHandler;
// Java: import cn.wps.kfc.xpxml.reader.XmlRoAttr
use core_libs_kfc::kfc::xpxml::reader::XmlRoAttr::XmlRoAttr;

use super::DataReferenceHandler::DataReferenceHandler;
use super::KeyReferenceHandler::KeyReferenceHandler;

// Java: public class ReferenceListHandler extends SecurityHandlerBase
pub struct ReferenceListHandler {
    pub(crate) base: SecurityHandlerBase,
    // Java field: DataReferenceHandler mDataReferenceHandler
    mDataReferenceHandler: Option<DataReferenceHandler>,
    // Java field: KeyReferenceHandler mKeyReferenceHandler
    mKeyReferenceHandler: Option<KeyReferenceHandler>,
}

// Java: public class ReferenceListHandler extends SecurityHandlerBase
impl ReferenceListHandler {
    // Java: ReferenceListHandler(UofEncryptInfo uofEncryptInfo)
    pub fn ReferenceListHandler(uofEncryptInfo: UofEncryptInfo) -> Self {
      // Java: super(uofEncryptInfo);
      ReferenceListHandler {
          base: SecurityHandlerBase::SecurityHandlerBase(uofEncryptInfo),
          mDataReferenceHandler: None,
          mKeyReferenceHandler: None,
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
         // Java: case -1931501919:
         -1931501919i32 =>
            // Java: return this.getDataReferenceHandler();
            Some(Box::new(DataReferenceHandler::DataReferenceHandler(self.base.mUofEncryptInfo.clone()))),
         // Java: case -1236676660:
         -1236676660i32 =>
            // Java: return this.getKeyReferenceHandler();
            Some(Box::new(KeyReferenceHandler::KeyReferenceHandler(self.base.mUofEncryptInfo.clone()))),
         // Java: default:
         _ =>
            // Java: return null;
            None,
      }
    }

    // Java: DataReferenceHandler getDataReferenceHandler()
    fn getDataReferenceHandler(&mut self) -> &DataReferenceHandler {
      // Java: if (null == this.mDataReferenceHandler)
      if self.mDataReferenceHandler.is_none() {
         // Java: this.mDataReferenceHandler = new DataReferenceHandler(this.mUofEncryptInfo);
         self.mDataReferenceHandler = Some(DataReferenceHandler::DataReferenceHandler(self.base.mUofEncryptInfo.clone()));
      }
      // Java: return this.mDataReferenceHandler;
      self.mDataReferenceHandler.as_ref().unwrap()
    }

    // Java: KeyReferenceHandler getKeyReferenceHandler()
    fn getKeyReferenceHandler(&mut self) -> &KeyReferenceHandler {
      // Java: if (null == this.mKeyReferenceHandler)
      if self.mKeyReferenceHandler.is_none() {
         // Java: this.mKeyReferenceHandler = new KeyReferenceHandler(this.mUofEncryptInfo);
         self.mKeyReferenceHandler = Some(KeyReferenceHandler::KeyReferenceHandler(self.base.mUofEncryptInfo.clone()));
      }
      // Java: return this.mKeyReferenceHandler;
      self.mKeyReferenceHandler.as_ref().unwrap()
    }

}

impl ElementHandler for ReferenceListHandler {
    fn startElement(&self, var1: i32, var2: &dyn XmlRoAttr) { self.startElement(var1, var2); }
    fn endElement(&self, var1: i32) { self.endElement(var1); }
    fn characters(&self, var1: &str) { self.characters(var1); }
    fn enterSubElement(&self, var1: i32) -> Option<Box<dyn ElementHandler>> { self.enterSubElement(var1) }
}
