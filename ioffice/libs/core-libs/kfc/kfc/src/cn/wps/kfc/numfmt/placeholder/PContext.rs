#![allow(non_snake_case)]
#![allow(dead_code)]

// Java field: short CONTEXT_WORD
pub(crate) const CONTEXT_WORD: i16 = 1;
// Java field: short CONTEXT_EXCEL
pub(crate) const CONTEXT_EXCEL: i16 = 2;

// Java: public class PContext
pub struct PContext {
    // Java field: short mContextType
    pub(crate) mContextType: i16,
    // Java field: boolean mAddMinus
    pub(crate) mAddMinus: bool,
}

// Java: public class PContext
impl PContext {
    // Java: PContext()
    pub fn new() -> Self {
        // Java: super();
        PContext {
            mContextType: 0,
            mAddMinus: false,
        }
    }

    // Java: void clear()
    pub fn clear(&mut self) {
        // Java: this.mContextType = 2;
        self.mContextType = 2;
        // Java: this.mAddMinus = false;
        self.mAddMinus = false;
    }
}
