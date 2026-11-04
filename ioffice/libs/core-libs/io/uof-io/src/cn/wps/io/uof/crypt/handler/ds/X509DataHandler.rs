// Java: import cn.wps.io.uof.crypt.UofEncryptInfo
use crate::cn::wps::io::uof::crypt::UofEncryptInfo::UofEncryptInfo;
// Java: import cn.wps.io.uof.crypt.handler.SecurityHandlerBase
use crate::cn::wps::io::uof::crypt::handler::SecurityHandlerBase::SecurityHandlerBase;
// Java: import cn.wps.kfc.xpxml.reader.ElementHandler
use core_libs_kfc::kfc::xpxml::reader::ElementHandler::ElementHandler;
// Java: import cn.wps.kfc.xpxml.reader.XmlRoAttr
use core_libs_kfc::kfc::xpxml::reader::XmlRoAttr::XmlRoAttr;

use super::X509SubjectNameHandler::X509SubjectNameHandler;
use super::X509IssuerSerialHandler::X509IssuerSerialHandler;
use super::X509SKIHandler::X509SKIHandler;
use super::X509CertificateHandler::X509CertificateHandler;
use super::X509CRLHandler::X509CRLHandler;

// Java: public class X509DataHandler extends SecurityHandlerBase
pub struct X509DataHandler {
    pub(crate) base: SecurityHandlerBase,
    // Java field: X509SubjectNameHandler mX509SubjectNameHandler
    mX509SubjectNameHandler: Option<X509SubjectNameHandler>,
    // Java field: X509IssuerSerialHandler mX509IssuerSerialHandler
    mX509IssuerSerialHandler: Option<X509IssuerSerialHandler>,
    // Java field: X509SKIHandler mX509SKIHandler
    mX509SKIHandler: Option<X509SKIHandler>,
    // Java field: X509CertificateHandler mX509CertificateHandler
    mX509CertificateHandler: Option<X509CertificateHandler>,
    // Java field: X509CRLHandler mX509CRLHandler
    mX509CRLHandler: Option<X509CRLHandler>,
}

// Java: public class X509DataHandler extends SecurityHandlerBase
impl X509DataHandler {
    // Java: X509DataHandler(UofEncryptInfo uofEncryptInfo)
    pub fn X509DataHandler(uofEncryptInfo: UofEncryptInfo) -> Self {
      // Java: super(uofEncryptInfo);
      X509DataHandler {
          base: SecurityHandlerBase::SecurityHandlerBase(uofEncryptInfo),
          mX509SubjectNameHandler: None,
          mX509IssuerSerialHandler: None,
          mX509SKIHandler: None,
          mX509CertificateHandler: None,
          mX509CRLHandler: None,
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
         // Java: case -1940615593:
         -1940615593i32 =>
            // Java: return this.getX509CRLHandler();
            Some(Box::new(X509CRLHandler::X509CRLHandler(self.base.mUofEncryptInfo.clone()))),
         // Java: case -1940600437:
         -1940600437i32 =>
            // Java: return this.getX509SKIHandler();
            Some(Box::new(X509SKIHandler::X509SKIHandler(self.base.mUofEncryptInfo.clone()))),
         // Java: case -314623983:
         -314623983i32 =>
            // Java: return this.getX509CertificateHandler();
            Some(Box::new(X509CertificateHandler::X509CertificateHandler(self.base.mUofEncryptInfo.clone()))),
         // Java: case 1270908627:
         1270908627i32 =>
            // Java: return this.getX509IssuerSerialHandler();
            Some(Box::new(X509IssuerSerialHandler::X509IssuerSerialHandler(self.base.mUofEncryptInfo.clone()))),
         // Java: case 1424540753:
         1424540753i32 =>
            // Java: return this.getX509SubjectNameHandler();
            Some(Box::new(X509SubjectNameHandler::X509SubjectNameHandler(self.base.mUofEncryptInfo.clone()))),
         // Java: default:
         _ =>
            // Java: return null;
            None,
      }
    }

    // Java: X509SubjectNameHandler getX509SubjectNameHandler()
    fn getX509SubjectNameHandler(&mut self) -> &X509SubjectNameHandler {
      // Java: if (null == this.mX509SubjectNameHandler)
      if self.mX509SubjectNameHandler.is_none() {
         // Java: this.mX509SubjectNameHandler = new X509SubjectNameHandler(this.mUofEncryptInfo);
         self.mX509SubjectNameHandler = Some(X509SubjectNameHandler::X509SubjectNameHandler(self.base.mUofEncryptInfo.clone()));
      }
      // Java: return this.mX509SubjectNameHandler;
      self.mX509SubjectNameHandler.as_ref().unwrap()
    }

    // Java: X509IssuerSerialHandler getX509IssuerSerialHandler()
    fn getX509IssuerSerialHandler(&mut self) -> &X509IssuerSerialHandler {
      // Java: if (null == this.mX509IssuerSerialHandler)
      if self.mX509IssuerSerialHandler.is_none() {
         // Java: this.mX509IssuerSerialHandler = new X509IssuerSerialHandler(this.mUofEncryptInfo);
         self.mX509IssuerSerialHandler = Some(X509IssuerSerialHandler::X509IssuerSerialHandler(self.base.mUofEncryptInfo.clone()));
      }
      // Java: return this.mX509IssuerSerialHandler;
      self.mX509IssuerSerialHandler.as_ref().unwrap()
    }

    // Java: X509SKIHandler getX509SKIHandler()
    fn getX509SKIHandler(&mut self) -> &X509SKIHandler {
      // Java: if (null == this.mX509SKIHandler)
      if self.mX509SKIHandler.is_none() {
         // Java: this.mX509SKIHandler = new X509SKIHandler(this.mUofEncryptInfo);
         self.mX509SKIHandler = Some(X509SKIHandler::X509SKIHandler(self.base.mUofEncryptInfo.clone()));
      }
      // Java: return this.mX509SKIHandler;
      self.mX509SKIHandler.as_ref().unwrap()
    }

    // Java: X509CertificateHandler getX509CertificateHandler()
    fn getX509CertificateHandler(&mut self) -> &X509CertificateHandler {
      // Java: if (null == this.mX509CertificateHandler)
      if self.mX509CertificateHandler.is_none() {
         // Java: this.mX509CertificateHandler = new X509CertificateHandler(this.mUofEncryptInfo);
         self.mX509CertificateHandler = Some(X509CertificateHandler::X509CertificateHandler(self.base.mUofEncryptInfo.clone()));
      }
      // Java: return this.mX509CertificateHandler;
      self.mX509CertificateHandler.as_ref().unwrap()
    }

    // Java: X509CRLHandler getX509CRLHandler()
    fn getX509CRLHandler(&mut self) -> &X509CRLHandler {
      // Java: if (null == this.mX509CRLHandler)
      if self.mX509CRLHandler.is_none() {
         // Java: this.mX509CRLHandler = new X509CRLHandler(this.mUofEncryptInfo);
         self.mX509CRLHandler = Some(X509CRLHandler::X509CRLHandler(self.base.mUofEncryptInfo.clone()));
      }
      // Java: return this.mX509CRLHandler;
      self.mX509CRLHandler.as_ref().unwrap()
    }

}

impl ElementHandler for X509DataHandler {
    fn startElement(&self, var1: i32, var2: &dyn XmlRoAttr) { self.startElement(var1, var2); }
    fn endElement(&self, var1: i32) { self.endElement(var1); }
    fn characters(&self, var1: &str) { self.characters(var1); }
    fn enterSubElement(&self, var1: i32) -> Option<Box<dyn ElementHandler>> { self.enterSubElement(var1) }
}
