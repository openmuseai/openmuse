// Java: import cn.wps.io.uof.crypt.UofEncryptInfo
use crate::cn::wps::io::uof::crypt::UofEncryptInfo::UofEncryptInfo;
// Java: import cn.wps.io.uof.crypt.handler.SecurityHandlerBase
use crate::cn::wps::io::uof::crypt::handler::SecurityHandlerBase::SecurityHandlerBase;
// Java: import cn.wps.kfc.xpxml.reader.ElementHandler
use core_libs_kfc::kfc::xpxml::reader::ElementHandler::ElementHandler;
// Java: import cn.wps.kfc.xpxml.reader.XmlRoAttr
use core_libs_kfc::kfc::xpxml::reader::XmlRoAttr::XmlRoAttr;

use super::PGPKeyPacketHandler::PGPKeyPacketHandler;
use super::PGPKeyIDHandler::PGPKeyIDHandler;

// Java: public class PGPDataHandler extends SecurityHandlerBase
pub struct PGPDataHandler {
    pub(crate) base: SecurityHandlerBase,
    // Java field: PGPKeyPacketHandler mPGPKeyPacketHandler
    mPGPKeyPacketHandler: Option<PGPKeyPacketHandler>,
    // Java field: PGPKeyIDHandler mPGPKeyIDHandler
    mPGPKeyIDHandler: Option<PGPKeyIDHandler>,
}

// Java: public class PGPDataHandler extends SecurityHandlerBase
impl PGPDataHandler {
    // Java: PGPDataHandler(UofEncryptInfo uofEncryptInfo)
    pub fn PGPDataHandler(uofEncryptInfo: UofEncryptInfo) -> Self {
      // Java: super(uofEncryptInfo);
      PGPDataHandler {
          base: SecurityHandlerBase::SecurityHandlerBase(uofEncryptInfo),
          mPGPKeyPacketHandler: None,
          mPGPKeyIDHandler: None,
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
         // Java: case -1358118399:
         -1358118399i32 =>
            // Java: return this.getPGPKeyIDHandler();
            Some(Box::new(PGPKeyIDHandler::PGPKeyIDHandler(self.base.mUofEncryptInfo.clone()))),
         // Java: case 77794958:
         77794958i32 =>
            // Java: return this.getPGPKeyPacketHandler();
            Some(Box::new(PGPKeyPacketHandler::PGPKeyPacketHandler(self.base.mUofEncryptInfo.clone()))),
         // Java: default:
         _ =>
            // Java: return null;
            None,
      }
    }

    // Java: PGPKeyPacketHandler getPGPKeyPacketHandler()
    fn getPGPKeyPacketHandler(&mut self) -> &PGPKeyPacketHandler {
      // Java: if (null == this.mPGPKeyPacketHandler)
      if self.mPGPKeyPacketHandler.is_none() {
         // Java: this.mPGPKeyPacketHandler = new PGPKeyPacketHandler(this.mUofEncryptInfo);
         self.mPGPKeyPacketHandler = Some(PGPKeyPacketHandler::PGPKeyPacketHandler(self.base.mUofEncryptInfo.clone()));
      }
      // Java: return this.mPGPKeyPacketHandler;
      self.mPGPKeyPacketHandler.as_ref().unwrap()
    }

    // Java: PGPKeyIDHandler getPGPKeyIDHandler()
    fn getPGPKeyIDHandler(&mut self) -> &PGPKeyIDHandler {
      // Java: if (null == this.mPGPKeyIDHandler)
      if self.mPGPKeyIDHandler.is_none() {
         // Java: this.mPGPKeyIDHandler = new PGPKeyIDHandler(this.mUofEncryptInfo);
         self.mPGPKeyIDHandler = Some(PGPKeyIDHandler::PGPKeyIDHandler(self.base.mUofEncryptInfo.clone()));
      }
      // Java: return this.mPGPKeyIDHandler;
      self.mPGPKeyIDHandler.as_ref().unwrap()
    }

}

impl ElementHandler for PGPDataHandler {
    fn startElement(&self, var1: i32, var2: &dyn XmlRoAttr) {
        self.startElement(var1, var2);
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
