// Java: import cn.wps.io.uof.crypt.UofEncryptInfo
use crate::cn::wps::io::uof::crypt::UofEncryptInfo::UofEncryptInfo;
// Java: import cn.wps.io.uof.crypt.handler.SecurityHandlerBase
use crate::cn::wps::io::uof::crypt::handler::SecurityHandlerBase::SecurityHandlerBase;
// Java: import cn.wps.kfc.xpxml.reader.ElementHandler
use core_libs_kfc::kfc::xpxml::reader::ElementHandler::ElementHandler;
// Java: import cn.wps.kfc.xpxml.reader.XmlRoAttr
use core_libs_kfc::kfc::xpxml::reader::XmlRoAttr::XmlRoAttr;

use super::IteratorOperationsNumberHandler::IteratorOperationsNumberHandler;
use super::BlockSizeHandler::BlockSizeHandler;
use super::PasswordSaltValueHandler::PasswordSaltValueHandler;
use super::EncryptionSaltValueHandler::EncryptionSaltValueHandler;
use super::PasswordTipsInfoHandler::PasswordTipsInfoHandler;
use super::PasswordCheckValueHandler::PasswordCheckValueHandler;

// Java: public class EncryptionPropertyHandler extends SecurityHandlerBase
pub struct EncryptionPropertyHandler {
    pub(crate) base: SecurityHandlerBase,
    // Java field: IteratorOperationsNumberHandler mIteratorOperationsNumberHandler
    mIteratorOperationsNumberHandler: Option<IteratorOperationsNumberHandler>,
    // Java field: BlockSizeHandler mBlockSizeHandler
    mBlockSizeHandler: Option<BlockSizeHandler>,
    // Java field: PasswordSaltValueHandler mPasswordSaltValueHandler
    mPasswordSaltValueHandler: Option<PasswordSaltValueHandler>,
    // Java field: EncryptionSaltValueHandler mEncryptionSaltValueHandler
    mEncryptionSaltValueHandler: Option<EncryptionSaltValueHandler>,
    // Java field: PasswordTipsInfoHandler mPasswordTipsInfoHandler
    mPasswordTipsInfoHandler: Option<PasswordTipsInfoHandler>,
    // Java field: PasswordCheckValueHandler mPasswordCheckValueHandler
    mPasswordCheckValueHandler: Option<PasswordCheckValueHandler>,
}

// Java: public class EncryptionPropertyHandler extends SecurityHandlerBase
impl EncryptionPropertyHandler {
    // Java: EncryptionPropertyHandler(UofEncryptInfo uofEncryptInfo)
    pub fn EncryptionPropertyHandler(uofEncryptInfo: UofEncryptInfo) -> Self {
      // Java: super(uofEncryptInfo);
      EncryptionPropertyHandler {
          base: SecurityHandlerBase::SecurityHandlerBase(uofEncryptInfo),
          mIteratorOperationsNumberHandler: None,
          mBlockSizeHandler: None,
          mPasswordSaltValueHandler: None,
          mEncryptionSaltValueHandler: None,
          mPasswordTipsInfoHandler: None,
          mPasswordCheckValueHandler: None,
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
         // Java: case 647769497:
         647769497i32 =>
            // Java: return this.getBlockSizeHandler();
            Some(Box::new(BlockSizeHandler::BlockSizeHandler(self.base.mUofEncryptInfo.clone()))),
         // Java: case 653680402:
         653680402i32 =>
            // Java: return this.getEncryptionSaltValueHandler();
            Some(Box::new(EncryptionSaltValueHandler::EncryptionSaltValueHandler(self.base.mUofEncryptInfo.clone()))),
         // Java: case 730396071:
         730396071i32 =>
            // Java: return this.getPasswordSaltValueHandler();
            Some(Box::new(PasswordSaltValueHandler::PasswordSaltValueHandler(self.base.mUofEncryptInfo.clone()))),
         // Java: case 877817868:
         877817868i32 =>
            // Java: return this.getIteratorOperationsNumberHandler();
            Some(Box::new(IteratorOperationsNumberHandler::IteratorOperationsNumberHandler(self.base.mUofEncryptInfo.clone()))),
         // Java: case 1164439606:
         1164439606i32 =>
            // Java: return this.getPasswordCheckValueHandler();
            Some(Box::new(PasswordCheckValueHandler::PasswordCheckValueHandler(self.base.mUofEncryptInfo.clone()))),
         // Java: case 1696794899:
         1696794899i32 =>
            // Java: return this.getPasswordTipsInfoHandler();
            Some(Box::new(PasswordTipsInfoHandler::PasswordTipsInfoHandler(self.base.mUofEncryptInfo.clone()))),
         // Java: default:
         _ =>
            // Java: return null;
            None,
      }
    }

    // Java: IteratorOperationsNumberHandler getIteratorOperationsNumberHandler()
    fn getIteratorOperationsNumberHandler(&mut self) -> &IteratorOperationsNumberHandler {
      // Java: if (null == this.mIteratorOperationsNumberHandler)
      if self.mIteratorOperationsNumberHandler.is_none() {
         // Java: this.mIteratorOperationsNumberHandler = new IteratorOperationsNumberHandler(this.mUofEncryptInfo);
         self.mIteratorOperationsNumberHandler = Some(IteratorOperationsNumberHandler::IteratorOperationsNumberHandler(self.base.mUofEncryptInfo.clone()));
      }
      // Java: return this.mIteratorOperationsNumberHandler;
      self.mIteratorOperationsNumberHandler.as_ref().unwrap()
    }

    // Java: BlockSizeHandler getBlockSizeHandler()
    fn getBlockSizeHandler(&mut self) -> &BlockSizeHandler {
      // Java: if (null == this.mBlockSizeHandler)
      if self.mBlockSizeHandler.is_none() {
         // Java: this.mBlockSizeHandler = new BlockSizeHandler(this.mUofEncryptInfo);
         self.mBlockSizeHandler = Some(BlockSizeHandler::BlockSizeHandler(self.base.mUofEncryptInfo.clone()));
      }
      // Java: return this.mBlockSizeHandler;
      self.mBlockSizeHandler.as_ref().unwrap()
    }

    // Java: PasswordSaltValueHandler getPasswordSaltValueHandler()
    fn getPasswordSaltValueHandler(&mut self) -> &PasswordSaltValueHandler {
      // Java: if (null == this.mPasswordSaltValueHandler)
      if self.mPasswordSaltValueHandler.is_none() {
         // Java: this.mPasswordSaltValueHandler = new PasswordSaltValueHandler(this.mUofEncryptInfo);
         self.mPasswordSaltValueHandler = Some(PasswordSaltValueHandler::PasswordSaltValueHandler(self.base.mUofEncryptInfo.clone()));
      }
      // Java: return this.mPasswordSaltValueHandler;
      self.mPasswordSaltValueHandler.as_ref().unwrap()
    }

    // Java: EncryptionSaltValueHandler getEncryptionSaltValueHandler()
    fn getEncryptionSaltValueHandler(&mut self) -> &EncryptionSaltValueHandler {
      // Java: if (null == this.mEncryptionSaltValueHandler)
      if self.mEncryptionSaltValueHandler.is_none() {
         // Java: this.mEncryptionSaltValueHandler = new EncryptionSaltValueHandler(this.mUofEncryptInfo);
         self.mEncryptionSaltValueHandler = Some(EncryptionSaltValueHandler::EncryptionSaltValueHandler(self.base.mUofEncryptInfo.clone()));
      }
      // Java: return this.mEncryptionSaltValueHandler;
      self.mEncryptionSaltValueHandler.as_ref().unwrap()
    }

    // Java: PasswordTipsInfoHandler getPasswordTipsInfoHandler()
    fn getPasswordTipsInfoHandler(&mut self) -> &PasswordTipsInfoHandler {
      // Java: if (null == this.mPasswordTipsInfoHandler)
      if self.mPasswordTipsInfoHandler.is_none() {
         // Java: this.mPasswordTipsInfoHandler = new PasswordTipsInfoHandler(this.mUofEncryptInfo);
         self.mPasswordTipsInfoHandler = Some(PasswordTipsInfoHandler::PasswordTipsInfoHandler(self.base.mUofEncryptInfo.clone()));
      }
      // Java: return this.mPasswordTipsInfoHandler;
      self.mPasswordTipsInfoHandler.as_ref().unwrap()
    }

    // Java: PasswordCheckValueHandler getPasswordCheckValueHandler()
    fn getPasswordCheckValueHandler(&mut self) -> &PasswordCheckValueHandler {
      // Java: if (null == this.mPasswordCheckValueHandler)
      if self.mPasswordCheckValueHandler.is_none() {
         // Java: this.mPasswordCheckValueHandler = new PasswordCheckValueHandler(this.mUofEncryptInfo);
         self.mPasswordCheckValueHandler = Some(PasswordCheckValueHandler::PasswordCheckValueHandler(self.base.mUofEncryptInfo.clone()));
      }
      // Java: return this.mPasswordCheckValueHandler;
      self.mPasswordCheckValueHandler.as_ref().unwrap()
    }

}

impl ElementHandler for EncryptionPropertyHandler {
    fn startElement(&self, var1: i32, var2: &dyn XmlRoAttr) { self.startElement(var1, var2); }
    fn endElement(&self, var1: i32) { self.endElement(var1); }
    fn characters(&self, var1: &str) { self.characters(var1); }
    fn enterSubElement(&self, var1: i32) -> Option<Box<dyn ElementHandler>> { self.enterSubElement(var1) }
}
