#![allow(non_snake_case, dead_code)]

// Java: import android.graphics.Bitmap
// Java: import android.graphics.Xfermode

pub struct Bitmap;
pub struct Xfermode;

// Java: public interface GdiImage
pub trait GdiImage {
    // Java: abstract void setColors(int[] var1)
    fn setColors(&mut self, var1: &[i32]);

    // Java: abstract Bitmap getBitmap()
    fn getBitmap(&self) -> Bitmap;

    // Java: abstract int getWidth()
    fn getWidth(&self) -> i32;

    // Java: abstract int getHeight()
    fn getHeight(&self) -> i32;

    // Java: abstract Xfermode getXferMode()
    fn getXferMode(&self) -> Xfermode;

    // Java: abstract void setRop(int var1)
    fn setRop(&mut self, var1: i32);

    // Java: abstract int getRop()
    fn getRop(&self) -> i32;

    // Java: abstract void transparent(int var1)
    fn transparent(&mut self, var1: i32);
}
