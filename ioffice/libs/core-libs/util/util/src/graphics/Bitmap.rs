// Java: import java.io.OutputStream
// TODO_RUST_CODE use ???; // java.io.OutputStream

// Java: public interface Bitmap
pub trait Bitmap {
    // Java: abstract boolean compress(CompressFormat var1, int var2, OutputStream var3)
    fn compress(&self, CompressFormat var1, int var2, OutputStream var3) -> bool;

    // Java: abstract int getHeight()
    fn getHeight(&self) -> i32;

    // Java: abstract int getWidth()
    fn getWidth(&self) -> i32;

    // Java: abstract boolean isRecycled()
    fn isRecycled(&self) -> bool;

    // Java: abstract void recycle()
    fn recycle(&self);

    // Java: abstract void setFromType(FromType var1)
    fn setFromType(&self, FromType var1);

    // Java: abstract boolean isInMemory()
    fn isInMemory(&self) -> bool;

    // Java: abstract Bitmap createScaleBitmap(int var1, int var2)
    fn createScaleBitmap(&self, int var1, int var2) -> Bitmap;

    // Java: abstract boolean isMaxSize()
    fn isMaxSize(&self) -> bool;

    // Java: abstract int getAllocationByteCount()
    fn getAllocationByteCount(&self) -> i32;

}
