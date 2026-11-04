// Java: import cn.wps.io.uof.crypt.UofEncryptInfo
use crate::cn::wps::io::uof::crypt::UofEncryptInfo::UofEncryptInfo;
// Java: import cn.wps.io.uof.crypt.handler.SecurityHandlerBase
use crate::cn::wps::io::uof::crypt::handler::SecurityHandlerBase::SecurityHandlerBase;
// Java: import cn.wps.kfc.xpxml.reader.ElementHandler
use core_libs_kfc::kfc::xpxml::reader::ElementHandler::ElementHandler;
// Java: import cn.wps.kfc.xpxml.reader.XmlRoAttr
use core_libs_kfc::kfc::xpxml::reader::XmlRoAttr::XmlRoAttr;

use super::DocumentSignature_F80BHandler::DocumentSignature_F80BHandler;
use super::Stamp_F810Handler::Stamp_F810Handler;

// Java: public class Signature_F80FHandler extends SecurityHandlerBase
pub struct Signature_F80FHandler {
    pub(crate) base: SecurityHandlerBase,
    // Java field: DocumentSignature_F80BHandler mDocumentSignature_F80BHandler
    mDocumentSignature_F80BHandler: Option<DocumentSignature_F80BHandler>,
    // Java field: Stamp_F810Handler mStamp_F810Handler
    mStamp_F810Handler: Option<Stamp_F810Handler>,
}

// Java: public class Signature_F80FHandler extends SecurityHandlerBase
impl Signature_F80FHandler {
    // Java: Signature_F80FHandler(UofEncryptInfo uofEncryptInfo)
    pub fn Signature_F80FHandler(uofEncryptInfo: UofEncryptInfo) -> Self {
      // Java: super(uofEncryptInfo);
      Signature_F80FHandler {
          base: SecurityHandlerBase::SecurityHandlerBase(uofEncryptInfo),
          mDocumentSignature_F80BHandler: None,
          mStamp_F810Handler: None,
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
         // Java: case 63499:
         63499i32 =>
            // Java: return this.getDocumentSignature_F80BHandler();
            Some(Box::new(DocumentSignature_F80BHandler::DocumentSignature_F80BHandler(self.base.mUofEncryptInfo.clone()))),
         // Java: case 63504:
         63504i32 =>
            // Java: return this.getStamp_F810Handler();
            Some(Box::new(Stamp_F810Handler::Stamp_F810Handler(self.base.mUofEncryptInfo.clone()))),
         // Java: default:
         _ =>
            // Java: return null;
            None,
      }
    }

    // Java: DocumentSignature_F80BHandler getDocumentSignature_F80BHandler()
    fn getDocumentSignature_F80BHandler(&mut self) -> &DocumentSignature_F80BHandler {
      // Java: if (null == this.mDocumentSignature_F80BHandler)
      if self.mDocumentSignature_F80BHandler.is_none() {
         // Java: this.mDocumentSignature_F80BHandler = new DocumentSignature_F80BHandler(this.mUofEncryptInfo);
         self.mDocumentSignature_F80BHandler = Some(DocumentSignature_F80BHandler::DocumentSignature_F80BHandler(self.base.mUofEncryptInfo.clone()));
      }
      // Java: return this.mDocumentSignature_F80BHandler;
      self.mDocumentSignature_F80BHandler.as_ref().unwrap()
    }

    // Java: Stamp_F810Handler getStamp_F810Handler()
    fn getStamp_F810Handler(&mut self) -> &Stamp_F810Handler {
      // Java: if (null == this.mStamp_F810Handler)
      if self.mStamp_F810Handler.is_none() {
         // Java: this.mStamp_F810Handler = new Stamp_F810Handler(this.mUofEncryptInfo);
         self.mStamp_F810Handler = Some(Stamp_F810Handler::Stamp_F810Handler(self.base.mUofEncryptInfo.clone()));
      }
      // Java: return this.mStamp_F810Handler;
      self.mStamp_F810Handler.as_ref().unwrap()
    }

}

impl ElementHandler for Signature_F80FHandler {
    fn startElement(&self, var1: i32, var2: &dyn XmlRoAttr) { self.startElement(var1, var2); }
    fn endElement(&self, var1: i32) { self.endElement(var1); }
    fn characters(&self, var1: &str) { self.characters(var1); }
    fn enterSubElement(&self, var1: i32) -> Option<Box<dyn ElementHandler>> { self.enterSubElement(var1) }
}
