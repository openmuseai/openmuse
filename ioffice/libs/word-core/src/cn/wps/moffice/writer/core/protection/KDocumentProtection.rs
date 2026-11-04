// Java: import cn.wps.moffice.io.IDocProtection
use util_adapter::cn::wps::moffice::io::IDocProtection::*;
// Java: import cn.wps.moffice.writer.core.TextDocument
use crate::cn::wps::moffice::writer::core::TextDocument::TextDocument as WriterTextDocument;
// Java: import java.util.ArrayList
// Java: import java.util.List
// Java: import cn.wps.moffice.writer.core.protection.ProtectionType
use crate::cn::wps::moffice::writer::core::protection::ProtectionType::*;
// Java: import cn.wps.moffice.writer.core.protection.ProtectionPassword
use crate::cn::wps::moffice::writer::core::protection::ProtectionPassword::*;
// Java: import cn.wps.moffice.writer.core.protection.ProtectionListener
use crate::cn::wps::moffice::writer::core::protection::ProtectionListener::*;
// Java: import cn.wps.moffice.writer.core.protection.DocumentProtectionException
use crate::cn::wps::moffice::writer::core::protection::DocumentProtectionException::*;
// Java: import cn.wps.moffice.writer.core.protection.IncorrectPasswordException
use crate::cn::wps::moffice::writer::core::protection::IncorrectPasswordException::*;

// java public class KDocumentProtection implements IDocProtection
#[derive(Default)]
pub struct KDocumentProtection {
    // java TextDocument document;
    pub(crate) document: *mut WriterTextDocument,
    // java boolean protect = false;
    pub(crate) protect: bool,
    // java boolean enforced = false;
    pub(crate) enforced: bool,
    // java boolean styleLocked = false;
    pub(crate) styleLocked: bool,
    // java boolean styleLockEnforced = false;
    pub(crate) styleLockEnforced: bool,
    // java boolean fLockAtn = false;
    pub(crate) fLockAtn: bool,
    // java ProtectionType protectionType;
    pub(crate) protectionType: Option<ProtectionType>,
    // java ProtectionPassword password;
    pub(crate) password: Option<ProtectionPassword>,
    // java List<ProtectionListener> listeners;
    pub(crate) listeners: Option<Vec<Box<dyn ProtectionListener>>>,
    // java private boolean fThrowDocumentProtectionException;
    fThrowDocumentProtectionException: bool,
}

// java public interface IDocProtection
impl IDocProtection for KDocumentProtection {
    // java public boolean isReadOnlyProtection()
    fn isReadOnlyProtection(&self) -> bool {
      // java return (this.fLockAtn || this.isProtectOn()) && this.protectionType == ProtectionType.READONLY;
      return self.fLockAtn || self.isProtectOn() && self.protectionType == Some(ProtectionType::READONLY);
    }

    // java public boolean isFormProtection()
    fn isFormProtection(&self) -> bool {
      // java return this.isProtectOn() && this.protectionType == ProtectionType.FORMS && this.document.getProperty().getBoolean(631, false);
      // rust: 真实 `PropertySet::getBoolean` 无 default 参数（Java 的 `false` 即 Rust `bool` 缺省）⇒ 单参调用。
      return self.isProtectOn() && self.protectionType == Some(ProtectionType::FORMS) && self.document().getProperty().getBoolean(631);
    }

    // java public boolean isCommentProtection()
    fn isCommentProtection(&self) -> bool {
      // java return this.isProtectOn() && this.protectionType == ProtectionType.COMMENTS;
      return self.isProtectOn() && self.protectionType == Some(ProtectionType::COMMENTS);
    }

    // java public boolean isTrackedchangesProtection()
    fn isTrackedchangesProtection(&self) -> bool {
      // java return this.isProtectOn() && this.protectionType == ProtectionType.TRACKEDCHANGES;
      return self.isProtectOn() && self.protectionType == Some(ProtectionType::TRACKEDCHANGES);
    }

    // java public boolean isProtectOnEdit()
    fn isProtectOnEdit(&self) -> bool {
      // java return this.isReadOnlyProtection() || this.isFormProtection() || this.isCommentProtection() || this.isTrackedchangesProtection();
      return self.isReadOnlyProtection() || self.isFormProtection() || self.isCommentProtection() || self.isTrackedchangesProtection();
    }

}

// java public class KDocumentProtection implements IDocProtection
impl KDocumentProtection {
    fn document(&self) -> &WriterTextDocument {
      unsafe { &*self.document }
    }

    fn document_mut(&mut self) -> &mut WriterTextDocument {
      unsafe { &mut *self.document }
    }

    // java public  KDocumentProtection(TextDocument document)
    pub fn KDocumentProtection(document: &WriterTextDocument) -> Self {
      // java this.protectionType = ProtectionType.NONE;
      Self { document: document as *const WriterTextDocument as *mut WriterTextDocument, protect: Default::default(), enforced: Default::default(), styleLocked: Default::default(), styleLockEnforced: Default::default(), fLockAtn: Default::default(), protectionType: Some(ProtectionType::NONE), password: Some(ProtectionPassword::ProtectionPassword()), listeners: Some(Vec::new()), fThrowDocumentProtectionException: false }
      // java this.password = new ProtectionPassword();
      // java this.listeners = new ArrayList();
      // java this.fThrowDocumentProtectionException = false;
      // java this.document = document;
    }

    // java public TextDocument getDocument()
    pub fn getDocument(&self) -> &WriterTextDocument {
      // java return this.document;
      return self.document();
    }

    pub fn new(document: &WriterTextDocument) -> Self {
      Self::KDocumentProtection(document)
    }

    // java public void protect(String password, ProtectionType protectionType, boolean styleLockEnforced)
    pub fn protect(&mut self, password: &str, protectionType: &ProtectionType, styleLockEnforced: bool) {
      // java if (this.isProtectOn())
      if self.isProtectOn() {
        // java throw new DocumentProtectionException("Document protected!");
        // rust: Java `throw` ⇒ Rust `panic!`；异常对象本身不参与 panic 消息格式化
        panic!("Document protected!");
      }
      // java else if (protectionType != ProtectionType.NONE || styleLockEnforced)
      else if *protectionType != ProtectionType::NONE || styleLockEnforced {
        // java if (protectionType != ProtectionType.NONE)
        if *protectionType != ProtectionType::NONE {
          // java this.protectionType = protectionType;
          self.protectionType = Some(*protectionType);
          // java this.protect = true;
          self.protect = true;
          // java this.enforced = true;
          self.enforced = true;
          // java this.setupProtection(true);
          self.setupProtection(true);
        }
        // java if (styleLockEnforced)
        if styleLockEnforced {
          // java this.styleLockEnforced = true;
          self.styleLockEnforced = true;
          // java this.styleLocked = true;
          self.styleLocked = true;
        }
        // java if (this.isProtectOn() || styleLockEnforced)
        if self.isProtectOn() || styleLockEnforced {
          // java this.password.enforce(password);
          self.password.as_mut().expect("password").enforce(&password);
        }
        // java this.fireChanged();
        self.fireChanged();
      }
    }

    // java public boolean isProtectOn()
    pub fn isProtectOn(&self) -> bool {
      // java return this.protect && this.enforced;
      return self.protect && self.enforced;
    }

    // java public boolean isProtect()
    pub fn isProtect(&self) -> bool {
      // java return this.protect;
      return self.protect;
    }

    // java public ProtectionType getProtectionType()
    pub fn getProtectionType(&self) -> ProtectionType {
      // java return this.protectionType;
      // rust: 字段为 `Option`（Java 可空）；构造器已置 NONE ⇒ 缺省用 NONE 表达。
      return self.protectionType.unwrap_or(ProtectionType::NONE);
    }

    // java public boolean isStyleLockEnforced()
    pub fn isStyleLockEnforced(&self) -> bool {
      // java return this.styleLockEnforced;
      return self.styleLockEnforced;
    }

    // java private void setupProtection(boolean enforce)
    pub fn setupProtection(&mut self, enforce: bool) {
      // java switch (this.protectionType)
      match self.protectionType {
        // java case TRACKEDCHANGES:
        TRACKEDCHANGES => {
          // java if (enforce != this.document.getTrackChange())
          if enforce != self.document().getTrackChange() {
            // java this.document.setTrackChange(enforce);
            self.document_mut().setTrackChange__boolean(enforce);
          }
          // java break;
        }
        // java default:
        _ => {
          // java if (!enforce && this.document.getTrackChange())
          if !enforce && self.document().getTrackChange() {
            // java this.document.setTrackChange(false);
            self.document_mut().setTrackChange__boolean(false);
          }
        }
        // java (default 分支无 break)`
      }
    }

    // java public void unprotect(String password)
    pub fn unprotect(&mut self, password: Option<&str>) {
      // java if (this.isProtectOn() || this.isStyleLockEnforced())
      if self.isProtectOn() || self.isStyleLockEnforced() {
        // java if (password != null && !password.isEmpty())
        if let Some(password) = password {
          if !password.is_empty() {
            // java this.password.clear(password);
            self.password.as_mut().expect("password").clear(password);
            // java this.enforced = false;
            self.enforced = false;
            // java this.styleLockEnforced = false;
            self.styleLockEnforced = false;
            // java this.setupProtection(false);
            self.setupProtection(false);
            // java this.fireChanged();
            self.fireChanged();
          }
        }
      }
    }

    // java public void addListener(ProtectionListener listener)
    pub fn addListener(&mut self, listener: Box<dyn ProtectionListener>) {
      // java this.listeners.add(listener);
      self.listeners.get_or_insert_with(Vec::new).push(listener);
    }

    // java public void removeListener(ProtectionListener listener)
    pub fn removeListener(&mut self, listener: &dyn ProtectionListener) {
      // java this.listeners.remove(listener);
      if let Some(listeners) = self.listeners.as_mut() {
        listeners.retain(|l| !std::ptr::eq(l.as_ref(), listener));
      }
    }

    // java private void fireChanged()
    pub fn fireChanged(&mut self) {
      // java this.document.start();
      self.document_mut().start();
          // java this.document.setModified(true);
          self.document_mut().setModified__boolean(true);
            // java this.document.commit("Protection");
            self.document_mut().commit__string(&"Protection");
      // java this.document.getTransaction().clearHistory();
      // rust: `getTransaction()` 返回 `Option<Rc<RefCell<Transaction>>>` ⇒ 解引用后调用
      self.document_mut().getTransaction().as_ref().expect("transaction").borrow_mut().clearHistory();
      // java int i = 0;
      let mut i: i32 = 0;
      let mut n: i32 = (self.listeners.as_ref().map(|__a| __a.len()).unwrap_or(0) as i32);
      // java for(int n = this.listeners.size(); i < n; ++i)
      while i < n {
        // java ProtectionListener listener = (ProtectionListener)this.listeners.get(i);
        let mut listener: Box<dyn ProtectionListener> = todo!("cast → Box<dyn ProtectionListener>");
            // java listener.onProtectionChanged(this);
            listener.onProtectionChanged(self);
            // java catch (Exception e)
            // java e.printStackTrace();
            let _ = ();
      }
    }

    // java public boolean isThrowDocumentProtectionException()
    pub fn isThrowDocumentProtectionException(&self) -> bool {
      // java return this.fThrowDocumentProtectionException;
      return self.fThrowDocumentProtectionException;
    }

    // java public void setThrowDocumentProtectionException(boolean fThrow)
    pub fn setThrowDocumentProtectionException(&mut self, fThrow: bool) {
      // java this.fThrowDocumentProtectionException = fThrow;
      self.fThrowDocumentProtectionException = fThrow;
    }

}
