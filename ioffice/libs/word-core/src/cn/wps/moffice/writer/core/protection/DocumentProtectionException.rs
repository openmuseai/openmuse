// Java: import java.lang.RuntimeException
// rust: ★M-STAGE1 P2 —— 补缺失 import。该文件此前**从未被登记**，
//   故缺少基类 import 的缺陷一直未被编译器发现。
use stubs::java::lang::RuntimeException::RuntimeException;

// java public class DocumentProtectionException extends RuntimeException
#[derive(Default)]
pub struct DocumentProtectionException {
    // java RuntimeException super_
    pub(crate) super_: RuntimeException,
}

// java public class DocumentProtectionException extends RuntimeException
impl DocumentProtectionException {
    // java public  DocumentProtectionException()
    pub fn new() -> Self {
      Self { super_: RuntimeException::new() }
    }

    // java public  DocumentProtectionException(String message, Throwable cause)
    pub fn new__string_throwable(message: &str, cause: &str) -> Self {
      // java super(message, cause);
      Self { super_: RuntimeException::new__string_throwable(message, cause) }
    }

    // java public  DocumentProtectionException(String message)
    pub fn new__string(message: &str) -> Self {
      // java super(message);
      Self { super_: RuntimeException::new__string(message) }
    }

    // java public  DocumentProtectionException(Throwable cause)
    pub fn new__throwable(cause: &str) -> Self {
      // java super(cause);
      Self { super_: RuntimeException::new__throwable(cause) }
    }

}

impl std::ops::Deref for DocumentProtectionException {
    type Target = RuntimeException;

    fn deref(&self) -> &Self::Target {
        &self.super_
    }
}

impl std::ops::DerefMut for DocumentProtectionException {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.super_
    }
}
