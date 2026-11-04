// Java: import cn.wps.io.uof.crypt.UofEncryptInfo
use crate::cn::wps::io::uof::crypt::UofEncryptInfo::UofEncryptInfo;
// Java: import cn.wps.io.uof.crypt.handler.SecurityHandlerBase
use crate::cn::wps::io::uof::crypt::handler::SecurityHandlerBase::SecurityHandlerBase;
// Java: import cn.wps.io.uof.crypt.handler.ds.KeyInfoHandler
use crate::cn::wps::io::uof::crypt::handler::ds::KeyInfoHandler::KeyInfoHandler;
// Java: import cn.wps.kfc.xpxml.reader.ElementHandler
use core_libs_kfc::kfc::xpxml::reader::ElementHandler::ElementHandler;
// Java: import cn.wps.kfc.xpxml.reader.XmlRoAttr
use core_libs_kfc::kfc::xpxml::reader::XmlRoAttr::XmlRoAttr;

use super::CipherDataHandler::CipherDataHandler;
use super::EncryptionMethodHandler::EncryptionMethodHandler;
use super::EncryptionPropertiesHandler::EncryptionPropertiesHandler;

// Java: public class EncryptedDataHandler extends SecurityHandlerBase
pub struct EncryptedDataHandler {
    pub(crate) base: SecurityHandlerBase,
    // Java field: CipherDataHandler mCipherDataHandler
    mCipherDataHandler: Option<CipherDataHandler>,
    // Java field: EncryptionMethodHandler mEncryptionMethodHandler
    mEncryptionMethodHandler: Option<EncryptionMethodHandler>,
    // Java field: EncryptionPropertiesHandler mEncryptionPropertiesHandler
    mEncryptionPropertiesHandler: Option<EncryptionPropertiesHandler>,
    // Java field: KeyInfoHandler mKeyInfoHandler
    mKeyInfoHandler: Option<KeyInfoHandler>,
}

// Java: public class EncryptedDataHandler extends SecurityHandlerBase
impl EncryptedDataHandler {
    // Java: EncryptedDataHandler(UofEncryptInfo uofEncryptInfo)
    pub fn EncryptedDataHandler(uofEncryptInfo: UofEncryptInfo) -> Self {
      // Java: super(uofEncryptInfo);
      EncryptedDataHandler {
          base: SecurityHandlerBase::SecurityHandlerBase(uofEncryptInfo),
          mCipherDataHandler: None,
          mEncryptionMethodHandler: None,
          mEncryptionPropertiesHandler: None,
          mKeyInfoHandler: None,
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
         // Java: case -1899027868:
         -1899027868i32 =>
            // Java: return this.getEncryptionMethodHandler();
            Some(Box::new(EncryptionMethodHandler::EncryptionMethodHandler(self.base.mUofEncryptInfo.clone()))),
         // Java: case 848873357:
         848873357i32 =>
            // Java: return this.getKeyInfoHandler();
            Some(Box::new(KeyInfoHandler::KeyInfoHandler(self.base.mUofEncryptInfo.clone()))),
         // Java: case 1645619317:
         1645619317i32 =>
            // Java: return this.getCipherDataHandler();
            Some(Box::new(CipherDataHandler::CipherDataHandler(self.base.mUofEncryptInfo.clone()))),
         // Java: case 1744594230:
         1744594230i32 =>
            // Java: return this.getEncryptionPropertiesHandler();
            Some(Box::new(EncryptionPropertiesHandler::EncryptionPropertiesHandler(self.base.mUofEncryptInfo.clone()))),
         // Java: default:
         _ =>
            // Java: return null;
            None,
      }
    }

    // Java: CipherDataHandler getCipherDataHandler()
    fn getCipherDataHandler(&mut self) -> &CipherDataHandler {
      // Java: if (null == this.mCipherDataHandler)
      if self.mCipherDataHandler.is_none() {
         // Java: this.mCipherDataHandler = new CipherDataHandler(this.mUofEncryptInfo);
         self.mCipherDataHandler = Some(CipherDataHandler::CipherDataHandler(self.base.mUofEncryptInfo.clone()));
      }
      // Java: return this.mCipherDataHandler;
      self.mCipherDataHandler.as_ref().unwrap()
    }

    // Java: EncryptionMethodHandler getEncryptionMethodHandler()
    fn getEncryptionMethodHandler(&mut self) -> &EncryptionMethodHandler {
      // Java: if (null == this.mEncryptionMethodHandler)
      if self.mEncryptionMethodHandler.is_none() {
         // Java: this.mEncryptionMethodHandler = new EncryptionMethodHandler(this.mUofEncryptInfo);
         self.mEncryptionMethodHandler = Some(EncryptionMethodHandler::EncryptionMethodHandler(self.base.mUofEncryptInfo.clone()));
      }
      // Java: return this.mEncryptionMethodHandler;
      self.mEncryptionMethodHandler.as_ref().unwrap()
    }

    // Java: EncryptionPropertiesHandler getEncryptionPropertiesHandler()
    fn getEncryptionPropertiesHandler(&mut self) -> &EncryptionPropertiesHandler {
      // Java: if (null == this.mEncryptionPropertiesHandler)
      if self.mEncryptionPropertiesHandler.is_none() {
         // Java: this.mEncryptionPropertiesHandler = new EncryptionPropertiesHandler(this.mUofEncryptInfo);
         self.mEncryptionPropertiesHandler = Some(EncryptionPropertiesHandler::EncryptionPropertiesHandler(self.base.mUofEncryptInfo.clone()));
      }
      // Java: return this.mEncryptionPropertiesHandler;
      self.mEncryptionPropertiesHandler.as_ref().unwrap()
    }

    // Java: KeyInfoHandler getKeyInfoHandler()
    fn getKeyInfoHandler(&mut self) -> &KeyInfoHandler {
      // Java: if (null == this.mKeyInfoHandler)
      if self.mKeyInfoHandler.is_none() {
         // Java: this.mKeyInfoHandler = new KeyInfoHandler(this.mUofEncryptInfo);
         self.mKeyInfoHandler = Some(KeyInfoHandler::KeyInfoHandler(self.base.mUofEncryptInfo.clone()));
      }
      // Java: return this.mKeyInfoHandler;
      self.mKeyInfoHandler.as_ref().unwrap()
    }

}

impl ElementHandler for EncryptedDataHandler {
    fn startElement(&self, var1: i32, var2: &dyn XmlRoAttr) { self.startElement(var1, var2); }
    fn endElement(&self, var1: i32) { self.endElement(var1); }
    fn characters(&self, var1: &str) { self.characters(var1); }
    fn enterSubElement(&self, var1: i32) -> Option<Box<dyn ElementHandler>> { self.enterSubElement(var1) }
}
