#![allow(non_snake_case, dead_code, unused_variables)]

pub mod IPicture;
pub mod IPictureDrawer;
pub mod IPicturePool;

// Re-export Bitmap content that was previously in Bitmap.rs
use std::io::Write;

// Java: public interface Bitmap
pub trait Bitmap {
    // Java: abstract boolean compress(CompressFormat var1, int var2, OutputStream var3)
    fn compress(&self, format: CompressFormat, quality: i32, out: &mut dyn Write) -> bool;

    // Java: abstract int getHeight()
    fn getHeight(&self) -> i32;

    // Java: abstract int getWidth()
    fn getWidth(&self) -> i32;

    // Java: abstract boolean isRecycled()
    fn isRecycled(&self) -> bool;

    // Java: abstract void recycle()
    fn recycle(&mut self);

    // Java: abstract void setFromType(FromType var1)
    fn setFromType(&mut self, from_type: FromType);

    // Java: abstract boolean isInMemory()
    fn isInMemory(&self) -> bool;

    // Java: abstract Bitmap createScaleBitmap(int var1, int var2)
    fn createScaleBitmap(&self, width: i32, height: i32) -> Box<dyn Bitmap>;

    // Java: abstract boolean isMaxSize()
    fn isMaxSize(&self) -> bool;

    // Java: abstract int getAllocationByteCount()
    fn getAllocationByteCount(&self) -> i32;
}

// Java: public enum CompressFormat
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CompressFormat {
    // Java field: CompressFormat JPEG
    JPEG,
    // Java field: CompressFormat PNG
    PNG,
}

// Java: public enum CompressFormat
impl CompressFormat {
    // Java field: int nativeInt

    // Java: CompressFormat(int nativeInt)
    pub fn nativeInt(&self) -> i32 {
        // Java: this.nativeInt = nativeInt;
        match self {
            CompressFormat::JPEG => 0,
            CompressFormat::PNG => 1,
        }
    }
}

// Java: public enum FromType
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FromType {
    // Java field: FromType MEMORY
    MEMORY,
    // Java field: FromType DISK
    DISK,
}

// Java: public enum FromType
impl FromType {
    // Java: FromType()
    pub fn new() -> Self {
        FromType::MEMORY
    }
}
pub mod IBitmapRef;
// pub mod BitmapRef;
