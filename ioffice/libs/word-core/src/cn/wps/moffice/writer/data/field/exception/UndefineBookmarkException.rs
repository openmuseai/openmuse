#![allow(non_snake_case, dead_code, unused_imports)]
use stubs::java::lang::RuntimeException::*;

// TYPE_ORACLE / java2rust: UndefineBookmarkException extends RuntimeException

// java public class UndefineBookmarkException extends RuntimeException
pub struct UndefineBookmarkException {
    // java RuntimeException super_
    pub(crate) super_: RuntimeException,
    pub(crate) message: String,
}

// java public class UndefineBookmarkException extends RuntimeException
impl UndefineBookmarkException {
    // java public UndefineBookmarkException(String message)
    pub fn new(message: &str) -> Self {
        Self {
            super_: RuntimeException::new__string(message),
            message: message.to_string(),
        }
    }
}

impl std::fmt::Display for UndefineBookmarkException {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.message)
    }
}

impl std::ops::Deref for UndefineBookmarkException {
    type Target = RuntimeException;
    fn deref(&self) -> &Self::Target {
        &self.super_
    }
}

impl std::ops::DerefMut for UndefineBookmarkException {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.super_
    }
}
