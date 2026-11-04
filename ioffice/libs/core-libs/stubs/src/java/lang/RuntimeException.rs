#![allow(non_snake_case, dead_code, unused_imports)]

// Java: public class RuntimeException extends Exception
#[derive(Default, Clone)]
pub struct RuntimeException {
    pub message: Option<String>,
}

impl RuntimeException {
    pub fn new() -> Self {
        Self { message: None }
    }

    // java public RuntimeException(String message)
    pub fn new__string(message: &str) -> Self {
        Self {
            message: Some(message.to_string()),
        }
    }

    // java public RuntimeException(String message, Throwable cause)
    // rust: ★M-STAGE1 P2 —— 补此重载。`DocumentProtectionException`（Java）有
    //   `(String, Throwable)` 构造器，此前本桩缺该重载 ⇒ 引用方无法编译。
    //   与 `new__string` 一致，仅保留 message（`Throwable` 侧本桩无状态）。
    pub fn new__string_throwable(message: &str, _cause: &str) -> Self {
        Self {
            message: Some(message.to_string()),
        }
    }

    // java public RuntimeException(Throwable cause)
    // rust: ★M-STAGE1 P2 —— 补此重载（同 `new__string_throwable` 的缺失原因）。
    //   `DocumentProtectionException(Throwable)` 引用它。
    pub fn new__throwable(_cause: &str) -> Self {
        Self { message: None }
    }

    pub fn getMessage(&self) -> Option<&str> {
        self.message.as_deref()
    }
}

impl std::fmt::Display for RuntimeException {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.message.as_deref().unwrap_or(""))
    }
}
