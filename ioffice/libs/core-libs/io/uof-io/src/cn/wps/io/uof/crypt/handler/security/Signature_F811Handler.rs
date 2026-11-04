// Java: import cn.wps.io.uof.crypt.UofEncryptInfo
use crate::cn::wps::io::uof::crypt::UofEncryptInfo::UofEncryptInfo;
// Java: import cn.wps.io.uof.crypt.handler.SecurityHandlerBase
use crate::cn::wps::io::uof::crypt::handler::SecurityHandlerBase::SecurityHandlerBase;
// Java: import cn.wps.io.uof.crypt.handler.ds.KeyInfoHandler
use crate::cn::wps::io::uof::crypt::handler::ds::KeyInfoHandler::KeyInfoHandler;
// Java: import cn.wps.io.uof.crypt.handler.ds.ObjectHandler
use crate::cn::wps::io::uof::crypt::handler::ds::ObjectHandler::ObjectHandler;
// Java: import cn.wps.io.uof.crypt.handler.ds.SignatureValueHandler
use crate::cn::wps::io::uof::crypt::handler::ds::SignatureValueHandler::SignatureValueHandler;
// Java: import cn.wps.io.uof.crypt.handler.ds.SignedInfoHandler
use crate::cn::wps::io::uof::crypt::handler::ds::SignedInfoHandler::SignedInfoHandler;
// Java: import cn.wps.kfc.xpxml.reader.ElementHandler
use core_libs_kfc::kfc::xpxml::reader::ElementHandler::ElementHandler;
// Java: import cn.wps.kfc.xpxml.reader.XmlRoAttr
use core_libs_kfc::kfc::xpxml::reader::XmlRoAttr::XmlRoAttr;

// Java: public class Signature_F811Handler extends SecurityHandlerBase
pub struct Signature_F811Handler {
    pub(crate) base: SecurityHandlerBase,
    // Java field: ObjectHandler mObjectHandler
    mObjectHandler: Option<ObjectHandler>,
    // Java field: SignedInfoHandler mSignedInfoHandler
    mSignedInfoHandler: Option<SignedInfoHandler>,
    // Java field: SignatureValueHandler mSignatureValueHandler
    mSignatureValueHandler: Option<SignatureValueHandler>,
    // Java field: KeyInfoHandler mKeyInfoHandler
    mKeyInfoHandler: Option<KeyInfoHandler>,
}

// Java: public class Signature_F811Handler extends SecurityHandlerBase
impl Signature_F811Handler {
    // Java: Signature_F811Handler(UofEncryptInfo uofEncryptInfo)
    pub fn Signature_F811Handler(uofEncryptInfo: UofEncryptInfo) -> Self {
      // Java: super(uofEncryptInfo);
      Signature_F811Handler {
          base: SecurityHandlerBase::SecurityHandlerBase(uofEncryptInfo),
          mObjectHandler: None,
          mSignedInfoHandler: None,
          mSignatureValueHandler: None,
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
         // Java: case -1939501217:
         -1939501217i32 =>
            // Java: return this.getObjectHandler();
            Some(Box::new(ObjectHandler::ObjectHandler(self.base.mUofEncryptInfo.clone()))),
         // Java: case -1712486471:
         -1712486471i32 =>
            // Java: return this.getSignatureValueHandler();
            Some(Box::new(SignatureValueHandler::SignatureValueHandler(self.base.mUofEncryptInfo.clone()))),
         // Java: case 848873357:
         848873357i32 =>
            // Java: return this.getKeyInfoHandler();
            Some(Box::new(KeyInfoHandler::KeyInfoHandler(self.base.mUofEncryptInfo.clone()))),
         // Java: case 1013265930:
         1013265930i32 =>
            // Java: return this.getSignedInfoHandler();
            Some(Box::new(SignedInfoHandler::SignedInfoHandler(self.base.mUofEncryptInfo.clone()))),
         // Java: default:
         _ =>
            // Java: return null;
            None,
      }
    }

    // Java: ObjectHandler getObjectHandler()
    fn getObjectHandler(&mut self) -> &ObjectHandler {
      // Java: if (null == this.mObjectHandler)
      if self.mObjectHandler.is_none() {
         // Java: this.mObjectHandler = new ObjectHandler(this.mUofEncryptInfo);
         self.mObjectHandler = Some(ObjectHandler::ObjectHandler(self.base.mUofEncryptInfo.clone()));
      }
      // Java: return this.mObjectHandler;
      self.mObjectHandler.as_ref().unwrap()
    }

    // Java: SignedInfoHandler getSignedInfoHandler()
    fn getSignedInfoHandler(&mut self) -> &SignedInfoHandler {
      // Java: if (null == this.mSignedInfoHandler)
      if self.mSignedInfoHandler.is_none() {
         // Java: this.mSignedInfoHandler = new SignedInfoHandler(this.mUofEncryptInfo);
         self.mSignedInfoHandler = Some(SignedInfoHandler::SignedInfoHandler(self.base.mUofEncryptInfo.clone()));
      }
      // Java: return this.mSignedInfoHandler;
      self.mSignedInfoHandler.as_ref().unwrap()
    }

    // Java: SignatureValueHandler getSignatureValueHandler()
    fn getSignatureValueHandler(&mut self) -> &SignatureValueHandler {
      // Java: if (null == this.mSignatureValueHandler)
      if self.mSignatureValueHandler.is_none() {
         // Java: this.mSignatureValueHandler = new SignatureValueHandler(this.mUofEncryptInfo);
         self.mSignatureValueHandler = Some(SignatureValueHandler::SignatureValueHandler(self.base.mUofEncryptInfo.clone()));
      }
      // Java: return this.mSignatureValueHandler;
      self.mSignatureValueHandler.as_ref().unwrap()
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

impl ElementHandler for Signature_F811Handler {
    fn startElement(&self, var1: i32, var2: &dyn XmlRoAttr) { self.startElement(var1, var2); }
    fn endElement(&self, var1: i32) { self.endElement(var1); }
    fn characters(&self, var1: &str) { self.characters(var1); }
    fn enterSubElement(&self, var1: i32) -> Option<Box<dyn ElementHandler>> { self.enterSubElement(var1) }
}
