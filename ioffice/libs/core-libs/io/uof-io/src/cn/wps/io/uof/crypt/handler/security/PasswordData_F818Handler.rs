// Java: import cn.wps.io.uof.crypt.UofEncryptInfo
use crate::cn::wps::io::uof::crypt::UofEncryptInfo::UofEncryptInfo;
// Java: import cn.wps.io.uof.crypt.handler.SecurityHandlerBase
use crate::cn::wps::io::uof::crypt::handler::SecurityHandlerBase::SecurityHandlerBase;
// Java: import cn.wps.kfc.xpxml.reader.ElementHandler
use core_libs_kfc::kfc::xpxml::reader::ElementHandler::ElementHandler;
// Java: import cn.wps.kfc.xpxml.reader.XmlRoAttr
use core_libs_kfc::kfc::xpxml::reader::XmlRoAttr::XmlRoAttr;

use super::PasswordSaltValue_F816Handler::PasswordSaltValue_F816Handler;
use super::AlgorithmName_F813Handler::AlgorithmName_F813Handler;
use super::PasswordHashValue_F815Handler::PasswordHashValue_F815Handler;
use super::IteratorOperationsNumber_F814Handler::IteratorOperationsNumber_F814Handler;

// Java: public class PasswordData_F818Handler extends SecurityHandlerBase
pub struct PasswordData_F818Handler {
    pub(crate) base: SecurityHandlerBase,
    // Java field: PasswordSaltValue_F816Handler mPasswordSaltValue_F816Handler
    mPasswordSaltValue_F816Handler: Option<PasswordSaltValue_F816Handler>,
    // Java field: AlgorithmName_F813Handler mAlgorithmName_F813Handler
    mAlgorithmName_F813Handler: Option<AlgorithmName_F813Handler>,
    // Java field: PasswordHashValue_F815Handler mPasswordHashValue_F815Handler
    mPasswordHashValue_F815Handler: Option<PasswordHashValue_F815Handler>,
    // Java field: IteratorOperationsNumber_F814Handler mIteratorOperationsNumber_F814Handler
    mIteratorOperationsNumber_F814Handler: Option<IteratorOperationsNumber_F814Handler>,
}

// Java: public class PasswordData_F818Handler extends SecurityHandlerBase
impl PasswordData_F818Handler {
    // Java: PasswordData_F818Handler(UofEncryptInfo uofEncryptInfo)
    pub fn PasswordData_F818Handler(uofEncryptInfo: UofEncryptInfo) -> Self {
      // Java: super(uofEncryptInfo);
      PasswordData_F818Handler {
          base: SecurityHandlerBase::SecurityHandlerBase(uofEncryptInfo),
          mPasswordSaltValue_F816Handler: None,
          mAlgorithmName_F813Handler: None,
          mPasswordHashValue_F815Handler: None,
          mIteratorOperationsNumber_F814Handler: None,
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
         // Java: case 63507:
         63507i32 =>
            // Java: return this.getAlgorithmName_F813Handler();
            Some(Box::new(AlgorithmName_F813Handler::AlgorithmName_F813Handler(self.base.mUofEncryptInfo.clone()))),
         // Java: case 63508:
         63508i32 =>
            // Java: return this.getIteratorOperationsNumber_F814Handler();
            Some(Box::new(IteratorOperationsNumber_F814Handler::IteratorOperationsNumber_F814Handler(self.base.mUofEncryptInfo.clone()))),
         // Java: case 63509:
         63509i32 =>
            // Java: return this.getPasswordHashValue_F815Handler();
            Some(Box::new(PasswordHashValue_F815Handler::PasswordHashValue_F815Handler(self.base.mUofEncryptInfo.clone()))),
         // Java: case 63510:
         63510i32 =>
            // Java: return this.getPasswordSaltValue_F816Handler();
            Some(Box::new(PasswordSaltValue_F816Handler::PasswordSaltValue_F816Handler(self.base.mUofEncryptInfo.clone()))),
         // Java: default:
         _ =>
            // Java: return null;
            None,
      }
    }

    // Java: PasswordSaltValue_F816Handler getPasswordSaltValue_F816Handler()
    fn getPasswordSaltValue_F816Handler(&mut self) -> &PasswordSaltValue_F816Handler {
      // Java: if (null == this.mPasswordSaltValue_F816Handler)
      if self.mPasswordSaltValue_F816Handler.is_none() {
         // Java: this.mPasswordSaltValue_F816Handler = new PasswordSaltValue_F816Handler(this.mUofEncryptInfo);
         self.mPasswordSaltValue_F816Handler = Some(PasswordSaltValue_F816Handler::PasswordSaltValue_F816Handler(self.base.mUofEncryptInfo.clone()));
      }
      // Java: return this.mPasswordSaltValue_F816Handler;
      self.mPasswordSaltValue_F816Handler.as_ref().unwrap()
    }

    // Java: AlgorithmName_F813Handler getAlgorithmName_F813Handler()
    fn getAlgorithmName_F813Handler(&mut self) -> &AlgorithmName_F813Handler {
      // Java: if (null == this.mAlgorithmName_F813Handler)
      if self.mAlgorithmName_F813Handler.is_none() {
         // Java: this.mAlgorithmName_F813Handler = new AlgorithmName_F813Handler(this.mUofEncryptInfo);
         self.mAlgorithmName_F813Handler = Some(AlgorithmName_F813Handler::AlgorithmName_F813Handler(self.base.mUofEncryptInfo.clone()));
      }
      // Java: return this.mAlgorithmName_F813Handler;
      self.mAlgorithmName_F813Handler.as_ref().unwrap()
    }

    // Java: PasswordHashValue_F815Handler getPasswordHashValue_F815Handler()
    fn getPasswordHashValue_F815Handler(&mut self) -> &PasswordHashValue_F815Handler {
      // Java: if (null == this.mPasswordHashValue_F815Handler)
      if self.mPasswordHashValue_F815Handler.is_none() {
         // Java: this.mPasswordHashValue_F815Handler = new PasswordHashValue_F815Handler(this.mUofEncryptInfo);
         self.mPasswordHashValue_F815Handler = Some(PasswordHashValue_F815Handler::PasswordHashValue_F815Handler(self.base.mUofEncryptInfo.clone()));
      }
      // Java: return this.mPasswordHashValue_F815Handler;
      self.mPasswordHashValue_F815Handler.as_ref().unwrap()
    }

    // Java: IteratorOperationsNumber_F814Handler getIteratorOperationsNumber_F814Handler()
    fn getIteratorOperationsNumber_F814Handler(&mut self) -> &IteratorOperationsNumber_F814Handler {
      // Java: if (null == this.mIteratorOperationsNumber_F814Handler)
      if self.mIteratorOperationsNumber_F814Handler.is_none() {
         // Java: this.mIteratorOperationsNumber_F814Handler = new IteratorOperationsNumber_F814Handler(this.mUofEncryptInfo);
         self.mIteratorOperationsNumber_F814Handler = Some(IteratorOperationsNumber_F814Handler::IteratorOperationsNumber_F814Handler(self.base.mUofEncryptInfo.clone()));
      }
      // Java: return this.mIteratorOperationsNumber_F814Handler;
      self.mIteratorOperationsNumber_F814Handler.as_ref().unwrap()
    }

}

impl ElementHandler for PasswordData_F818Handler {
    fn startElement(&self, var1: i32, var2: &dyn XmlRoAttr) { self.startElement(var1, var2); }
    fn endElement(&self, var1: i32) { self.endElement(var1); }
    fn characters(&self, var1: &str) { self.characters(var1); }
    fn enterSubElement(&self, var1: i32) -> Option<Box<dyn ElementHandler>> { self.enterSubElement(var1) }
}
