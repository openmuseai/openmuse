#![allow(non_snake_case)]
#![allow(dead_code)]

// Java field: int S_OK
pub(crate) const S_OK: i32 = 0;
// Java field: int S_FALSE
pub(crate) const S_FALSE: i32 = 1;
// Java field: int S_CONTINUE
pub(crate) const S_CONTINUE: i32 = 131073;
// Java field: int M_NOT_REDRAW
pub(crate) const M_NOT_REDRAW: i32 = 262145;
// Java field: int E_FAIL
pub(crate) const E_FAIL: i32 = -2147467259;
// Java field: int E_NOTIMPL
pub(crate) const E_NOTIMPL: i32 = -2147483647;
// Java field: int E_ABORT
pub(crate) const E_ABORT: i32 = -2147467260;
// Java field: int E_ACCESSDENIED
pub(crate) const E_ACCESSDENIED: i32 = -2147024891;
// Java field: int E_INVALIDARG
pub(crate) const E_INVALIDARG: i32 = -2147483645;
// Java field: int E_UNEXPECTED
pub(crate) const E_UNEXPECTED: i32 = -2147418113;

// Java: public class HResult
pub struct HResult {
}

// Java: public class HResult
impl HResult {
    // Java: HResult()
    pub fn new() -> Self {
      // Java: super();
      HResult {}
    }

}
