// Java: import cn.wps.encrypt.CryptTool
use io_poi_compact::cn::wps::encrypt::CryptTool::*;
// Java: import cn.wps.encrypt.ModifyVerifier
use io_poi_compact::cn::wps::encrypt::ModifyVerifier::*;
// Java: import cn.wps.moffice.writer.core.TextDocument
use crate::cn::wps::moffice::writer::core::TextDocument::*;

// java public class KDocumentWriteProtection
#[derive(Default)]
pub struct KDocumentWriteProtection {
    // java private TextDocument mTextDocument;
    // rust: Java 对象引用**别名/回指**宿主文档 ⇒ 裸指针（先例 `InnerIOListener.mDoc: Option<*mut TextDocument>`）。
    //   按值持有 `Option<TextDocument>` 会与 `TextDocument.mDocumentCrypt` 构成 `E0072` 无限大小类型。
    mTextDocument: Option<*mut TextDocument>,
    // java private boolean isReadOnly;
    isReadOnly: bool,
    // java private ModifyVerifier mModifyVerifier;
    mModifyVerifier: Option<ModifyVerifier>,
    // java private String mWritePassword;
    mWritePassword: Option<String>,
}

// java public class KDocumentWriteProtection
impl KDocumentWriteProtection {
    // java public  KDocumentWriteProtection(TextDocument textDocument)
    pub fn KDocumentWriteProtection(textDocument: &TextDocument) -> Self {
      // java this.mTextDocument = textDocument;
      Self { mTextDocument: Some(textDocument as *const TextDocument as *mut TextDocument), isReadOnly: false, mModifyVerifier: None, mWritePassword: None }
      // java this.isReadOnly = false;
      // java this.mModifyVerifier = null;
      // java this.mWritePassword = null;
    }

    // java public String writePassword()
    pub fn writePassword(&self) -> Option<String> {
      // java return this.mWritePassword;
      return self.mWritePassword.clone();
    }

    // java public boolean isReadOnly()
    pub fn isReadOnly(&self) -> bool {
      // java return this.isReadOnly;
      return self.isReadOnly;
    }

    // java public void setReadOnly(boolean readOnly)
    pub fn setReadOnly(&mut self, readOnly: bool) {
      // java this.isReadOnly = readOnly;
      self.isReadOnly = readOnly;
    }

    // java public boolean hasWritePassword()
    pub fn hasWritePassword(&self) -> bool {
      // java return this.mModifyVerifier != null;
      return self.mModifyVerifier.is_some();
    }

    // java public ModifyVerifier getModifyVerifier()
    pub fn getModifyVerifier(&self) -> Option<&ModifyVerifier> {
      // java return this.mModifyVerifier;
      return self.mModifyVerifier.as_ref();
    }

    // java public void setWritePassword(String writePassword)
    pub fn setWritePassword__string(&mut self, writePassword: &str) {
      // java this.setWritePassword(writePassword, true);
      self.setWritePassword__string_boolean(Some(writePassword), true);
    }

    // java public void setWritePassword(String writePassword, boolean dirty)
    pub fn setWritePassword__string_boolean(&mut self, mut writePassword: Option<&str>, dirty: bool) {
      // java if (writePassword == null || writePassword.length() == 0)
      if writePassword.is_none() || (writePassword.as_ref().map(|__a| __a.len()).unwrap_or(0) as i32) == 0 {
        // java writePassword = null;
        writePassword = None;
      }
      // java this.mWritePassword = writePassword;
      self.mWritePassword = writePassword.map(|s| s.to_string());
      // java if (writePassword != null || this.mModifyVerifier != null)
      if writePassword.is_some() || self.mModifyVerifier.is_some() {
        // java if (dirty)
        if dirty {
          // java if (writePassword != null)
          if let Some(writePassword) = writePassword {
            // java this.mTextDocument.setModified(true);
            unsafe { (*self.mTextDocument.unwrap()).setModified__boolean(true); }
          }
          // java else if (writePassword == null && this.mModifyVerifier != null)
          else if writePassword.is_none() && self.mModifyVerifier.is_some() {
            // java this.mTextDocument.setModified(true);
            unsafe { (*self.mTextDocument.unwrap()).setModified__boolean(true); }
          }
        }
        // java if (writePassword != null)
        if let Some(writePassword) = writePassword {
          // java this.mModifyVerifier = CryptTool.createModifyVerifier(writePassword);
          self.mModifyVerifier = Some(CryptTool::createModifyVerifier(writePassword));
        }
        // java else
        else {
          // java this.mModifyVerifier = null;
          self.mModifyVerifier = None;
        }
      }
    }

    // java public void setModifyVerifier(ModifyVerifier modifyVerifier)
    pub fn setModifyVerifier(&mut self, modifyVerifier: &ModifyVerifier) {
      // java this.mModifyVerifier = modifyVerifier;
      self.mModifyVerifier = Some(modifyVerifier.clone());
    }

    // java public boolean verifyWritePassword(String writePassword)
    pub fn verifyWritePassword(&mut self, writePassword: Option<&str>) -> bool {
      // java if (this.mModifyVerifier == null)
      if self.mModifyVerifier.is_none() {
        // java return false;
        return false;
      }
      // java else if (writePassword != null && writePassword.length() != 0)
      else if let Some(writePassword) = writePassword {
        if (writePassword.len() as i32) != 0 {
          // java if (CryptTool.checkModifyPassword(this.mModifyVerifier, writePassword))
          if CryptTool::checkModifyPassword(self.mModifyVerifier.as_ref().unwrap(), writePassword) {
            // java this.isReadOnly = false;
            self.isReadOnly = false;
            // java return true;
            return true;
          }
          // java else
          else {
            // java return false;
            return false;
          }
        }
        // java else
        else {
          // java return false;
          return false;
        }
      }
      // java else
      else {
        // java return false;
        return false;
      }
    }

}
