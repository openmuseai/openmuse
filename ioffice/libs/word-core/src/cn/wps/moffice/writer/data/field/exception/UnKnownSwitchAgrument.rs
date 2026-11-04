#![allow(non_snake_case, dead_code, unused_imports)]
use stubs::java::lang::RuntimeException::*;

// java public class UnKnownSwitchAgrument extends RuntimeException
pub struct UnKnownSwitchAgrument {
    // java RuntimeException super_
    pub(crate) super_: RuntimeException,
}

impl UnKnownSwitchAgrument {
    pub fn new() -> Self {
        Self {
            super_: RuntimeException::new(),
        }
    }
}

impl std::ops::Deref for UnKnownSwitchAgrument {
    type Target = RuntimeException;
    fn deref(&self) -> &Self::Target {
        &self.super_
    }
}
