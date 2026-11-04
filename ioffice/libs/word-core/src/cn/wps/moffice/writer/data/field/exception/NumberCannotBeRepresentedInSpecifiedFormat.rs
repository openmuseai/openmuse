#![allow(non_snake_case, dead_code, unused_imports)]
use stubs::java::lang::RuntimeException::*;

// java public class NumberCannotBeRepresentedInSpecifiedFormat extends RuntimeException
pub struct NumberCannotBeRepresentedInSpecifiedFormat {
    // java RuntimeException super_
    pub(crate) super_: RuntimeException,
}

// java public class NumberCannotBeRepresentedInSpecifiedFormat extends RuntimeException
impl NumberCannotBeRepresentedInSpecifiedFormat {
    pub fn new() -> Self {
        Self {
            super_: RuntimeException::new(),
        }
    }
}

impl std::ops::Deref for NumberCannotBeRepresentedInSpecifiedFormat {
    type Target = RuntimeException;

    fn deref(&self) -> &Self::Target {
        &self.super_
    }
}

impl std::ops::DerefMut for NumberCannotBeRepresentedInSpecifiedFormat {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.super_
    }
}
