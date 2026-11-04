// Java: cn.wps.moffice.writer.core.protection.DocumentProtectionIO
//
// ★M-STAGE1 P2 —— 新建类型。全仓此前缺失（0 hit），是 `SettingsImporter`
//   依赖闭包的**最后一环**（`SettingsImporter.java:16 import ...DocumentProtectionIO`）。
//
// Java 原文 61 行，是一个**纯委托包装器**：持有一个 `KDocumentProtection`，
// 12 个方法全部直接转发到其字段。**自身无任何逻辑**。
//
// ★依赖条件（按 §24.4 判据核验）：
//   `KDocumentProtection`（264 行）已存在，本类所需的 7 个字段
//   （`protect`/`enforced`/`styleLocked`/`styleLockEnforced`/`fLockAtn`/
//    `protectionType`/`password`）**全部就位且为 `pub(crate)`**，
//   其唯一 1 处 `todo!()` 位于本类不触及的方法内。
//   闭包其余成员（`ProtectionType`/`ProtectionPassword`/`ProtectionListener`/
//   两个异常类）均已做实（`ProtectionPassword` 见 §27）。

// Java: cn.wps.moffice.writer.core.protection.KDocumentProtection  (同包，无需 import)
use crate::cn::wps::moffice::writer::core::protection::KDocumentProtection::KDocumentProtection;
// Java: import cn.wps.moffice.writer.core.protection.ProtectionType
use crate::cn::wps::moffice::writer::core::protection::ProtectionType::ProtectionType;
// Java: import cn.wps.moffice.writer.core.protection.ProtectionPassword
use crate::cn::wps::moffice::writer::core::protection::ProtectionPassword::ProtectionPassword;

// java public class DocumentProtectionIO
pub struct DocumentProtectionIO {
    // java KDocumentProtection protection;
    // rust: Java 为包内可见的可变引用目标；Rust 侧以 `Option<Box<..>>` 持有所有权。
    protection: Option<Box<KDocumentProtection>>,
    protectionRef: *mut KDocumentProtection,
}

// java public class DocumentProtectionIO
impl DocumentProtectionIO {
    // java public DocumentProtectionIO(KDocumentProtection protection)
    pub fn DocumentProtectionIO(protection: KDocumentProtection) -> Self {
        // java this.protection = protection;
        DocumentProtectionIO {
            protection: Some(Box::new(protection)),
            protectionRef: std::ptr::null_mut(),
        }
    }

    // Java: DocumentProtectionIO(KDocumentProtection protection) retains the same object.
    pub fn from_ref(protection: &mut KDocumentProtection) -> Self {
        Self { protection: None, protectionRef: protection }
    }

    // rust: 内部取用辅助（Java 直接字段访问 `this.protection.xxx`，
    //   Rust 需处理 `Option`；`DocumentProtectionIO` 一经构造即非空）。
    fn protection(&self) -> &KDocumentProtection {
        if !self.protectionRef.is_null() { unsafe { &*self.protectionRef } }
        else { self.protection.as_ref().expect("protection should not be null") }
    }

    fn protection_mut(&mut self) -> &mut KDocumentProtection {
        if !self.protectionRef.is_null() { unsafe { &mut *self.protectionRef } }
        else { self.protection.as_mut().expect("protection should not be null") }
    }

    // java public boolean isProtect()
    pub fn isProtect(&self) -> bool {
        // java return this.protection.protect;
        return self.protection().protect;
    }

    // java public void setProtect(boolean protect)
    pub fn setProtect(&mut self, protect: bool) {
        // java this.protection.protect = protect;
        self.protection_mut().protect = protect;
    }

    // java public boolean isEnforced()
    pub fn isEnforced(&self) -> bool {
        // java return this.protection.enforced;
        return self.protection().enforced;
    }

    // java public void setEnforced(boolean enforced)
    pub fn setEnforced(&mut self, enforced: bool) {
        // java this.protection.enforced = enforced;
        self.protection_mut().enforced = enforced;
    }

    // java public boolean isStyleLocked()
    pub fn isStyleLocked(&self) -> bool {
        // java return this.protection.styleLocked;
        return self.protection().styleLocked;
    }

    // java public void setStyleLocked(boolean styleLocked)
    pub fn setStyleLocked(&mut self, styleLocked: bool) {
        // java this.protection.styleLocked = styleLocked;
        self.protection_mut().styleLocked = styleLocked;
    }

    // java public boolean isStyleLockEnforced()
    pub fn isStyleLockEnforced(&self) -> bool {
        // java return this.protection.styleLockEnforced;
        return self.protection().styleLockEnforced;
    }

    // java public void setStyleLockEnforced(boolean styleLockEnforced)
    pub fn setStyleLockEnforced(&mut self, styleLockEnforced: bool) {
        // java this.protection.styleLockEnforced = styleLockEnforced;
        self.protection_mut().styleLockEnforced = styleLockEnforced;
    }

    // java public ProtectionType getProtectionType()
    pub fn getProtectionType(&self) -> Option<ProtectionType> {
        // java return this.protection.protectionType;
        return self.protection().protectionType.clone();
    }

    // java public void setProtectionType(ProtectionType protectionType)
    pub fn setProtectionType(&mut self, protectionType: ProtectionType) {
        // java this.protection.protectionType = protectionType;
        self.protection_mut().protectionType = Some(protectionType);
    }

    // java public ProtectionPassword getPassword()
    pub fn getPassword(&self) -> Option<ProtectionPassword> {
        // java return this.protection.password;
        return self.protection().password.clone();
    }

    // java public void setPassword(ProtectionPassword password)
    pub fn setPassword(&mut self, password: ProtectionPassword) {
        // java this.protection.password = password;
        self.protection_mut().password = Some(password);
    }

    // java public void setfLockAtn(boolean fLockAtn)
    pub fn setfLockAtn(&mut self, fLockAtn: bool) {
        // java this.protection.fLockAtn = fLockAtn;
        self.protection_mut().fLockAtn = fLockAtn;
    }
}
