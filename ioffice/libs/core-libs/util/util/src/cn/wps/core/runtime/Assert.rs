#[allow(non_camel_case_types)]
pub struct cn { _private: () }

// Java: public class Assert extends cn
pub struct Assert {
    _cn: cn,
}

impl std::ops::Deref for Assert {
    type Target = cn;
    fn deref(&self) -> &cn { &self._cn }
}

impl std::ops::DerefMut for Assert {
    fn deref_mut(&mut self) -> &mut cn { &mut self._cn }
}

// Java: public class Assert extends cn
impl Assert {
    // Java: Assert()
    pub fn new() -> Self {
        // Java: super();
        Assert { _cn: cn { _private: () } }
    }

    // Java: static void assertNotNull(Object object)
    pub fn assertNotNull<T>(_object: &T) {}

    // Java: static void assertTrue(String message, boolean condition)
    pub fn assertTrue(_message: &str, _condition: bool) {}

    // Java: static void assertTrue(boolean condition)
    pub fn assertTrue_cond(_condition: bool) {}
}
