#![allow(non_snake_case, dead_code, unused_imports)]
use stubs::java::lang::RuntimeException::*;

// java public class UnExpectedEndFormatException extends RuntimeException
pub struct UnExpectedEndFormatException {
    // java RuntimeException super_
    pub(crate) super_: RuntimeException,
}

// java public class UnExpectedEndFormatException extends RuntimeException
impl UnExpectedEndFormatException {
    pub fn new() -> Self {
        Self {
            super_: RuntimeException::new(),
        }
    }
}

impl std::fmt::Display for UnExpectedEndFormatException {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "UnExpectedEndFormatException")
    }
}

impl std::ops::Deref for UnExpectedEndFormatException {
    type Target = RuntimeException;

    fn deref(&self) -> &Self::Target {
        &self.super_
    }
}

impl std::ops::DerefMut for UnExpectedEndFormatException {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.super_
    }
}
