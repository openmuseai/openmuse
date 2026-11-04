// Java: import cn.wps.io.uof.crypt.UofEncryptInfo
use crate::cn::wps::io::uof::crypt::UofEncryptInfo::UofEncryptInfo;
// Java: import cn.wps.io.uof.crypt.handler.SecurityHandlerBase
use crate::cn::wps::io::uof::crypt::handler::SecurityHandlerBase::SecurityHandlerBase;
// Java: import cn.wps.kfc.xpxml.reader.ElementHandler
use core_libs_kfc::kfc::xpxml::reader::ElementHandler::ElementHandler;
// Java: import cn.wps.kfc.xpxml.reader.XmlRoAttr
use core_libs_kfc::kfc::xpxml::reader::XmlRoAttr::XmlRoAttr;

use super::OriginatorKeyInfoHandler::OriginatorKeyInfoHandler;
use super::KA_NonceHandler::KA_NonceHandler;
use super::RecipientKeyInfoHandler::RecipientKeyInfoHandler;

// Java: public class AgreementMethodHandler extends SecurityHandlerBase
pub struct AgreementMethodHandler {
    pub(crate) base: SecurityHandlerBase,
    // Java field: OriginatorKeyInfoHandler mOriginatorKeyInfoHandler
    mOriginatorKeyInfoHandler: Option<OriginatorKeyInfoHandler>,
    // Java field: KA_NonceHandler mKA_NonceHandler
    mKA_NonceHandler: Option<KA_NonceHandler>,
    // Java field: RecipientKeyInfoHandler mRecipientKeyInfoHandler
    mRecipientKeyInfoHandler: Option<RecipientKeyInfoHandler>,
}

// Java: public class AgreementMethodHandler extends SecurityHandlerBase
impl AgreementMethodHandler {
    // Java: AgreementMethodHandler(UofEncryptInfo uofEncryptInfo)
    pub fn AgreementMethodHandler(uofEncryptInfo: UofEncryptInfo) -> Self {
      // Java: super(uofEncryptInfo);
      AgreementMethodHandler {
          base: SecurityHandlerBase::SecurityHandlerBase(uofEncryptInfo),
          mOriginatorKeyInfoHandler: None,
          mKA_NonceHandler: None,
          mRecipientKeyInfoHandler: None,
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
         // Java: case -1420510348:
         -1420510348i32 =>
            // Java: return this.getRecipientKeyInfoHandler();
            Some(Box::new(RecipientKeyInfoHandler::RecipientKeyInfoHandler(self.base.mUofEncryptInfo.clone()))),
         // Java: case 254254705:
         254254705i32 =>
            // Java: return this.getOriginatorKeyInfoHandler();
            Some(Box::new(OriginatorKeyInfoHandler::OriginatorKeyInfoHandler(self.base.mUofEncryptInfo.clone()))),
         // Java: case 783715480:
         783715480i32 =>
            // Java: return this.getKA_NonceHandler();
            Some(Box::new(KA_NonceHandler::KA_NonceHandler(self.base.mUofEncryptInfo.clone()))),
         // Java: default:
         _ =>
            // Java: return null;
            None,
      }
    }

    // Java: OriginatorKeyInfoHandler getOriginatorKeyInfoHandler()
    fn getOriginatorKeyInfoHandler(&mut self) -> &OriginatorKeyInfoHandler {
      // Java: if (null == this.mOriginatorKeyInfoHandler)
      if self.mOriginatorKeyInfoHandler.is_none() {
         // Java: this.mOriginatorKeyInfoHandler = new OriginatorKeyInfoHandler(this.mUofEncryptInfo);
         self.mOriginatorKeyInfoHandler = Some(OriginatorKeyInfoHandler::OriginatorKeyInfoHandler(self.base.mUofEncryptInfo.clone()));
      }
      // Java: return this.mOriginatorKeyInfoHandler;
      self.mOriginatorKeyInfoHandler.as_ref().unwrap()
    }

    // Java: KA_NonceHandler getKA_NonceHandler()
    fn getKA_NonceHandler(&mut self) -> &KA_NonceHandler {
      // Java: if (null == this.mKA_NonceHandler)
      if self.mKA_NonceHandler.is_none() {
         // Java: this.mKA_NonceHandler = new KA_NonceHandler(this.mUofEncryptInfo);
         self.mKA_NonceHandler = Some(KA_NonceHandler::KA_NonceHandler(self.base.mUofEncryptInfo.clone()));
      }
      // Java: return this.mKA_NonceHandler;
      self.mKA_NonceHandler.as_ref().unwrap()
    }

    // Java: RecipientKeyInfoHandler getRecipientKeyInfoHandler()
    fn getRecipientKeyInfoHandler(&mut self) -> &RecipientKeyInfoHandler {
      // Java: if (null == this.mRecipientKeyInfoHandler)
      if self.mRecipientKeyInfoHandler.is_none() {
         // Java: this.mRecipientKeyInfoHandler = new RecipientKeyInfoHandler(this.mUofEncryptInfo);
         self.mRecipientKeyInfoHandler = Some(RecipientKeyInfoHandler::RecipientKeyInfoHandler(self.base.mUofEncryptInfo.clone()));
      }
      // Java: return this.mRecipientKeyInfoHandler;
      self.mRecipientKeyInfoHandler.as_ref().unwrap()
    }

}

impl ElementHandler for AgreementMethodHandler {
    fn startElement(&self, var1: i32, var2: &dyn XmlRoAttr) { self.startElement(var1, var2); }
    fn endElement(&self, var1: i32) { self.endElement(var1); }
    fn characters(&self, var1: &str) { self.characters(var1); }
    fn enterSubElement(&self, var1: i32) -> Option<Box<dyn ElementHandler>> { self.enterSubElement(var1) }
}
