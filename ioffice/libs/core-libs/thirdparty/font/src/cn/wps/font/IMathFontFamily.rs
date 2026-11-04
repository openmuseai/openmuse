use std::sync::Arc;
use crate::cn::wps::font::ITypeface::ITypeface;

// Java field: int NORMAL
pub const NORMAL: i32 = 0;
// Java field: int MATH_PLAIN
pub const MATH_PLAIN: i32 = 1;
// Java field: int MATH_BLOD
pub const MATH_BLOD: i32 = 2;
// Java field: int MATH_ITALIC
pub const MATH_ITALIC: i32 = 3;
// Java field: int MATH_BLOD_ITALIC
pub const MATH_BLOD_ITALIC: i32 = 4;

// Java: public interface IMathFontFamily
pub trait IMathFontFamily: Send + Sync {
    // Java: abstract ITypeface getTypeface(int var1, int var2)
    fn getTypeface(&self, var1: i32, var2: i32) -> Option<Arc<dyn ITypeface>>;
}
