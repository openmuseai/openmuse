// Java: import cn.wps.io.uof.crypt.UofEncryptInfo
use crate::cn::wps::io::uof::crypt::UofEncryptInfo::UofEncryptInfo;
// Java: import cn.wps.io.uof.crypt.handler.SecurityHandlerBase
use crate::cn::wps::io::uof::crypt::handler::SecurityHandlerBase::SecurityHandlerBase;
// Java: import cn.wps.kfc.xpxml.reader.ElementHandler
use core_libs_kfc::kfc::xpxml::reader::ElementHandler::ElementHandler;
// Java: import cn.wps.kfc.xpxml.reader.XmlRoAttr
use core_libs_kfc::kfc::xpxml::reader::XmlRoAttr::XmlRoAttr;

use super::PgenCounterHandler::PgenCounterHandler;
use super::QHandler::QHandler;
use super::PHandler::PHandler;
use super::GHandler::GHandler;
use super::YHandler::YHandler;
use super::SeedHandler::SeedHandler;
use super::JHandler::JHandler;

// Java: public class DSAKeyValueHandler extends SecurityHandlerBase
pub struct DSAKeyValueHandler {
    pub(crate) base: SecurityHandlerBase,
    // Java field: PgenCounterHandler mPgenCounterHandler
    mPgenCounterHandler: Option<PgenCounterHandler>,
    // Java field: QHandler mQHandler
    mQHandler: Option<QHandler>,
    // Java field: PHandler mPHandler
    mPHandler: Option<PHandler>,
    // Java field: GHandler mGHandler
    mGHandler: Option<GHandler>,
    // Java field: YHandler mYHandler
    mYHandler: Option<YHandler>,
    // Java field: SeedHandler mSeedHandler
    mSeedHandler: Option<SeedHandler>,
    // Java field: JHandler mJHandler
    mJHandler: Option<JHandler>,
}

// Java: public class DSAKeyValueHandler extends SecurityHandlerBase
impl DSAKeyValueHandler {
    // Java: DSAKeyValueHandler(UofEncryptInfo uofEncryptInfo)
    pub fn DSAKeyValueHandler(uofEncryptInfo: UofEncryptInfo) -> Self {
      // Java: super(uofEncryptInfo);
      DSAKeyValueHandler {
          base: SecurityHandlerBase::SecurityHandlerBase(uofEncryptInfo),
          mPgenCounterHandler: None,
          mQHandler: None,
          mPHandler: None,
          mGHandler: None,
          mYHandler: None,
          mSeedHandler: None,
          mJHandler: None,
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
         // Java: case -1894424356:
         -1894424356i32 =>
            // Java: return this.getPgenCounterHandler();
            Some(Box::new(PgenCounterHandler::PgenCounterHandler(self.base.mUofEncryptInfo.clone()))),
         // Java: case 71:
         71i32 =>
            // Java: return this.getGHandler();
            Some(Box::new(GHandler::GHandler(self.base.mUofEncryptInfo.clone()))),
         // Java: case 74:
         74i32 =>
            // Java: return this.getJHandler();
            Some(Box::new(JHandler::JHandler(self.base.mUofEncryptInfo.clone()))),
         // Java: case 80:
         80i32 =>
            // Java: return this.getPHandler();
            Some(Box::new(PHandler::PHandler(self.base.mUofEncryptInfo.clone()))),
         // Java: case 81:
         81i32 =>
            // Java: return this.getQHandler();
            Some(Box::new(QHandler::QHandler(self.base.mUofEncryptInfo.clone()))),
         // Java: case 89:
         89i32 =>
            // Java: return this.getYHandler();
            Some(Box::new(YHandler::YHandler(self.base.mUofEncryptInfo.clone()))),
         // Java: case 2572945:
         2572945i32 =>
            // Java: return this.getSeedHandler();
            Some(Box::new(SeedHandler::SeedHandler(self.base.mUofEncryptInfo.clone()))),
         // Java: default:
         _ =>
            // Java: return null;
            None,
      }
    }

    // Java: PgenCounterHandler getPgenCounterHandler()
    fn getPgenCounterHandler(&mut self) -> &PgenCounterHandler {
      // Java: if (null == this.mPgenCounterHandler)
      if self.mPgenCounterHandler.is_none() {
         // Java: this.mPgenCounterHandler = new PgenCounterHandler(this.mUofEncryptInfo);
         self.mPgenCounterHandler = Some(PgenCounterHandler::PgenCounterHandler(self.base.mUofEncryptInfo.clone()));
      }
      // Java: return this.mPgenCounterHandler;
      self.mPgenCounterHandler.as_ref().unwrap()
    }

    // Java: QHandler getQHandler()
    fn getQHandler(&mut self) -> &QHandler {
      // Java: if (null == this.mQHandler)
      if self.mQHandler.is_none() {
         // Java: this.mQHandler = new QHandler(this.mUofEncryptInfo);
         self.mQHandler = Some(QHandler::QHandler(self.base.mUofEncryptInfo.clone()));
      }
      // Java: return this.mQHandler;
      self.mQHandler.as_ref().unwrap()
    }

    // Java: PHandler getPHandler()
    fn getPHandler(&mut self) -> &PHandler {
      // Java: if (null == this.mPHandler)
      if self.mPHandler.is_none() {
         // Java: this.mPHandler = new PHandler(this.mUofEncryptInfo);
         self.mPHandler = Some(PHandler::PHandler(self.base.mUofEncryptInfo.clone()));
      }
      // Java: return this.mPHandler;
      self.mPHandler.as_ref().unwrap()
    }

    // Java: GHandler getGHandler()
    fn getGHandler(&mut self) -> &GHandler {
      // Java: if (null == this.mGHandler)
      if self.mGHandler.is_none() {
         // Java: this.mGHandler = new GHandler(this.mUofEncryptInfo);
         self.mGHandler = Some(GHandler::GHandler(self.base.mUofEncryptInfo.clone()));
      }
      // Java: return this.mGHandler;
      self.mGHandler.as_ref().unwrap()
    }

    // Java: YHandler getYHandler()
    fn getYHandler(&mut self) -> &YHandler {
      // Java: if (null == this.mYHandler)
      if self.mYHandler.is_none() {
         // Java: this.mYHandler = new YHandler(this.mUofEncryptInfo);
         self.mYHandler = Some(YHandler::YHandler(self.base.mUofEncryptInfo.clone()));
      }
      // Java: return this.mYHandler;
      self.mYHandler.as_ref().unwrap()
    }

    // Java: SeedHandler getSeedHandler()
    fn getSeedHandler(&mut self) -> &SeedHandler {
      // Java: if (null == this.mSeedHandler)
      if self.mSeedHandler.is_none() {
         // Java: this.mSeedHandler = new SeedHandler(this.mUofEncryptInfo);
         self.mSeedHandler = Some(SeedHandler::SeedHandler(self.base.mUofEncryptInfo.clone()));
      }
      // Java: return this.mSeedHandler;
      self.mSeedHandler.as_ref().unwrap()
    }

    // Java: JHandler getJHandler()
    fn getJHandler(&mut self) -> &JHandler {
      // Java: if (null == this.mJHandler)
      if self.mJHandler.is_none() {
         // Java: this.mJHandler = new JHandler(this.mUofEncryptInfo);
         self.mJHandler = Some(JHandler::JHandler(self.base.mUofEncryptInfo.clone()));
      }
      // Java: return this.mJHandler;
      self.mJHandler.as_ref().unwrap()
    }

}

impl ElementHandler for DSAKeyValueHandler {
    fn startElement(&self, var1: i32, var2: &dyn XmlRoAttr) { self.startElement(var1, var2); }
    fn endElement(&self, var1: i32) { self.endElement(var1); }
    fn characters(&self, var1: &str) { self.characters(var1); }
    fn enterSubElement(&self, var1: i32) -> Option<Box<dyn ElementHandler>> { self.enterSubElement(var1) }
}
