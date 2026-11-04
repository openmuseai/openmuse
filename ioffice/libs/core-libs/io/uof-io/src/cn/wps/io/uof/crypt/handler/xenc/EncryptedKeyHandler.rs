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

use super::ReferenceListHandler::ReferenceListHandler;
use super::CipherDataHandler::CipherDataHandler;
use super::EncryptionMethodHandler::EncryptionMethodHandler;
use super::EncryptionPropertiesHandler::EncryptionPropertiesHandler;
use super::CarriedKeyNameHandler::CarriedKeyNameHandler;

// Java: public class EncryptedKeyHandler extends SecurityHandlerBase
pub struct EncryptedKeyHandler {
    pub(crate) base: SecurityHandlerBase,
    // Java field: ReferenceListHandler mReferenceListHandler
    mReferenceListHandler: Option<ReferenceListHandler>,
    // Java field: CipherDataHandler mCipherDataHandler
    mCipherDataHandler: Option<CipherDataHandler>,
    // Java field: EncryptionMethodHandler mEncryptionMethodHandler
    mEncryptionMethodHandler: Option<EncryptionMethodHandler>,
    // Java field: EncryptionPropertiesHandler mEncryptionPropertiesHandler
    mEncryptionPropertiesHandler: Option<EncryptionPropertiesHandler>,
    // Java field: KeyInfoHandler mKeyInfoHandler
    mKeyInfoHandler: Option<KeyInfoHandler>,
    // Java field: CarriedKeyNameHandler mCarriedKeyNameHandler
    mCarriedKeyNameHandler: Option<CarriedKeyNameHandler>,
}

// Java: public class EncryptedKeyHandler extends SecurityHandlerBase
impl EncryptedKeyHandler {
    // Java: EncryptedKeyHandler(UofEncryptInfo uofEncryptInfo)
    pub fn EncryptedKeyHandler(uofEncryptInfo: UofEncryptInfo) -> Self {
      // Java: super(uofEncryptInfo);
      EncryptedKeyHandler {
          base: SecurityHandlerBase::SecurityHandlerBase(uofEncryptInfo),
          mReferenceListHandler: None, mCipherDataHandler: None,
          mEncryptionMethodHandler: None, mEncryptionPropertiesHandler: None,
          mKeyInfoHandler: None, mCarriedKeyNameHandler: None,
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
         // Java: case -1895303575:
         -1895303575i32 =>
            // Java: return this.getReferenceListHandler();
            Some(Box::new(ReferenceListHandler::ReferenceListHandler(self.base.mUofEncryptInfo.clone()))),
         // Java: case -1353445536:
         -1353445536i32 =>
            // Java: return this.getCarriedKeyNameHandler();
            Some(Box::new(CarriedKeyNameHandler::CarriedKeyNameHandler(self.base.mUofEncryptInfo.clone()))),
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

    // Java: ReferenceListHandler getReferenceListHandler()
    fn getReferenceListHandler(&mut self) -> &ReferenceListHandler {
      // Java: if (null == this.mReferenceListHandler)
      if self.mReferenceListHandler.is_none() {
         // Java: this.mReferenceListHandler = new ReferenceListHandler(this.mUofEncryptInfo);
         self.mReferenceListHandler = Some(ReferenceListHandler::ReferenceListHandler(self.base.mUofEncryptInfo.clone()));
      }
      // Java: return this.mReferenceListHandler;
      self.mReferenceListHandler.as_ref().unwrap()
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

    // Java: CarriedKeyNameHandler getCarriedKeyNameHandler()
    fn getCarriedKeyNameHandler(&mut self) -> &CarriedKeyNameHandler {
      // Java: if (null == this.mCarriedKeyNameHandler)
      if self.mCarriedKeyNameHandler.is_none() {
         // Java: this.mCarriedKeyNameHandler = new CarriedKeyNameHandler(this.mUofEncryptInfo);
         self.mCarriedKeyNameHandler = Some(CarriedKeyNameHandler::CarriedKeyNameHandler(self.base.mUofEncryptInfo.clone()));
      }
      // Java: return this.mCarriedKeyNameHandler;
      self.mCarriedKeyNameHandler.as_ref().unwrap()
    }

}

impl ElementHandler for EncryptedKeyHandler {
    fn startElement(&self, var1: i32, var2: &dyn XmlRoAttr) { self.startElement(var1, var2); }
    fn endElement(&self, var1: i32) { self.endElement(var1); }
    fn characters(&self, var1: &str) { self.characters(var1); }
    fn enterSubElement(&self, var1: i32) -> Option<Box<dyn ElementHandler>> { self.enterSubElement(var1) }
}
