#![allow(non_snake_case, dead_code)]

// Java: public interface IGraphicsException
pub trait IGraphicsException {
    // Java: abstract int getType()
    fn getType(&self) -> i32;

    // Java: abstract int getExpectedWidth()
    fn getExpectedWidth(&self) -> i32;

    // Java: abstract int getExpectedHeight()
    fn getExpectedHeight(&self) -> i32;
}

// Java field: int TYPE_BIGBITMAP
pub const TYPE_BIGBITMAP: i32 = 0;
// Java field: int TYPE_OUT_OF_MEMORY
pub const TYPE_OUT_OF_MEMORY: i32 = 1;
// Java field: int TYPE_PICTURE_DUMPING
pub const TYPE_PICTURE_DUMPING: i32 = 2;
// Java field: int TYPE_SHAPE_BMP_UNCREATED
pub const TYPE_SHAPE_BMP_UNCREATED: i32 = 3;
