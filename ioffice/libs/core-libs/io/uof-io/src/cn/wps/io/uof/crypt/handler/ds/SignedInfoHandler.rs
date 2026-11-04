// Java: import cn.wps.io.uof.crypt.UofEncryptInfo
use crate::cn::wps::io::uof::crypt::UofEncryptInfo::UofEncryptInfo;
// Java: import cn.wps.io.uof.crypt.handler.SecurityHandlerBase
use crate::cn::wps::io::uof::crypt::handler::SecurityHandlerBase::SecurityHandlerBase;
// Java: import cn.wps.kfc.xpxml.reader.ElementHandler
use core_libs_kfc::kfc::xpxml::reader::ElementHandler::ElementHandler;
// Java: import cn.wps.kfc.xpxml.reader.XmlRoAttr
use core_libs_kfc::kfc::xpxml::reader::XmlRoAttr::XmlRoAttr;

use super::ReferenceHandler::ReferenceHandler;
use super::CanonicalizationMethodHandler::CanonicalizationMethodHandler;
use super::SignatureMethodHandler::SignatureMethodHandler;

// Java: public class SignedInfoHandler extends SecurityHandlerBase
pub struct SignedInfoHandler {
    pub(crate) base: SecurityHandlerBase,
    // Java field: ReferenceHandler mReferenceHandler
    mReferenceHandler: Option<ReferenceHandler>,
    // Java field: CanonicalizationMethodHandler mCanonicalizationMethodHandler
    mCanonicalizationMethodHandler: Option<CanonicalizationMethodHandler>,
    // Java field: SignatureMethodHandler mSignatureMethodHandler
    mSignatureMethodHandler: Option<SignatureMethodHandler>,
}

// Java: public class SignedInfoHandler extends SecurityHandlerBase
impl SignedInfoHandler {
    // Java: SignedInfoHandler(UofEncryptInfo uofEncryptInfo)
    pub fn SignedInfoHandler(uofEncryptInfo: UofEncryptInfo) -> Self {
      // Java: super(uofEncryptInfo);
      SignedInfoHandler {
          base: SecurityHandlerBase::SecurityHandlerBase(uofEncryptInfo),
          mReferenceHandler: None,
          mCanonicalizationMethodHandler: None,
          mSignatureMethodHandler: None,
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
         // Java: case -2047256047:
         -2047256047i32 =>
            // Java: return this.getCanonicalizationMethodHandler();
            Some(Box::new(CanonicalizationMethodHandler::CanonicalizationMethodHandler(self.base.mUofEncryptInfo.clone()))),
         // Java: case -1801215079:
         -1801215079i32 =>
            // Java: return this.getSignatureMethodHandler();
            Some(Box::new(SignatureMethodHandler::SignatureMethodHandler(self.base.mUofEncryptInfo.clone()))),
         // Java: case 1078812459:
         1078812459i32 =>
            // Java: return this.getReferenceHandler();
            Some(Box::new(ReferenceHandler::ReferenceHandler(self.base.mUofEncryptInfo.clone()))),
         // Java: default:
         _ =>
            // Java: return null;
            None,
      }
    }

    // Java: ReferenceHandler getReferenceHandler()
    fn getReferenceHandler(&mut self) -> &ReferenceHandler {
      // Java: if (null == this.mReferenceHandler)
      if self.mReferenceHandler.is_none() {
         // Java: this.mReferenceHandler = new ReferenceHandler(this.mUofEncryptInfo);
         self.mReferenceHandler = Some(ReferenceHandler::ReferenceHandler(self.base.mUofEncryptInfo.clone()));
      }
      // Java: return this.mReferenceHandler;
      self.mReferenceHandler.as_ref().unwrap()
    }

    // Java: CanonicalizationMethodHandler getCanonicalizationMethodHandler()
    fn getCanonicalizationMethodHandler(&mut self) -> &CanonicalizationMethodHandler {
      // Java: if (null == this.mCanonicalizationMethodHandler)
      if self.mCanonicalizationMethodHandler.is_none() {
         // Java: this.mCanonicalizationMethodHandler = new CanonicalizationMethodHandler(this.mUofEncryptInfo);
         self.mCanonicalizationMethodHandler = Some(CanonicalizationMethodHandler::CanonicalizationMethodHandler(self.base.mUofEncryptInfo.clone()));
      }
      // Java: return this.mCanonicalizationMethodHandler;
      self.mCanonicalizationMethodHandler.as_ref().unwrap()
    }

    // Java: SignatureMethodHandler getSignatureMethodHandler()
    fn getSignatureMethodHandler(&mut self) -> &SignatureMethodHandler {
      // Java: if (null == this.mSignatureMethodHandler)
      if self.mSignatureMethodHandler.is_none() {
         // Java: this.mSignatureMethodHandler = new SignatureMethodHandler(this.mUofEncryptInfo);
         self.mSignatureMethodHandler = Some(SignatureMethodHandler::SignatureMethodHandler(self.base.mUofEncryptInfo.clone()));
      }
      // Java: return this.mSignatureMethodHandler;
      self.mSignatureMethodHandler.as_ref().unwrap()
    }

}

impl ElementHandler for SignedInfoHandler {
    fn startElement(&self, var1: i32, var2: &dyn XmlRoAttr) { self.startElement(var1, var2); }
    fn endElement(&self, var1: i32) { self.endElement(var1); }
    fn characters(&self, var1: &str) { self.characters(var1); }
    fn enterSubElement(&self, var1: i32) -> Option<Box<dyn ElementHandler>> { self.enterSubElement(var1) }
}
