#![allow(non_snake_case, dead_code)]

// Java field: int TYPE_COLOR
pub const TYPE_COLOR: i32 = 0;
// Java field: int TYPE_BITMAP
pub const TYPE_BITMAP: i32 = 1;

// Java: public interface IGdiPainter
pub trait IGdiPainter {
    // Java: abstract int getType()
    fn getType(&self) -> i32;
}
