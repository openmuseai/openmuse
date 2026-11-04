// Java: cn.wps.moffice.writer.core.protection.IncorrectPasswordException
//
// ★M-STAGE1 P2 —— 从 auto-generated 空壳（`new()` 为 `todo!()`、无字段）
//   **做实**。闭包链：
//     SettingsImporter → DocumentProtectionIO → ProtectionPassword
//                       → **IncorrectPasswordException**
//
// Java 原文：`extends DocumentProtectionException`，4 个构造器全部直接
// `super(...)` 转发，**自身无额外字段/逻辑**。

// Java: import cn.wps.moffice.writer.core.protection.DocumentProtectionException
use crate::cn::wps::moffice::writer::core::protection::DocumentProtectionException::DocumentProtectionException;

// java public class IncorrectPasswordException extends DocumentProtectionException
#[derive(Default)]
pub struct IncorrectPasswordException {
    // java DocumentProtectionException super_
    // rust: Rust 无继承 ⇒ 组合基类（与 `DocumentProtectionException` 自身
    //   组合 `RuntimeException` 的做法一致）。
    pub(crate) super_: DocumentProtectionException,
}

// java public class IncorrectPasswordException extends DocumentProtectionException
impl IncorrectPasswordException {
    // java public IncorrectPasswordException()
    pub fn IncorrectPasswordException() -> Self {
        // java super();
        Self {
            super_: DocumentProtectionException::new(),
        }
    }

    // java public IncorrectPasswordException(String message, Throwable cause)
    pub fn IncorrectPasswordException__string_throwable(message: &str, cause: &str) -> Self {
        // java super(message, cause);
        Self {
            super_: DocumentProtectionException::new__string_throwable(message, cause),
        }
    }

    // java public IncorrectPasswordException(String message)
    pub fn IncorrectPasswordException__string(message: &str) -> Self {
        // java super(message);
        Self {
            super_: DocumentProtectionException::new__string(message),
        }
    }

    // java public IncorrectPasswordException(Throwable cause)
    pub fn IncorrectPasswordException__throwable(cause: &str) -> Self {
        // java super(cause);
        Self {
            super_: DocumentProtectionException::new__throwable(cause),
        }
    }
}
