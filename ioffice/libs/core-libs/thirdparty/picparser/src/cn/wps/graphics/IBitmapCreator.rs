#![allow(non_snake_case, dead_code)]

// Java: import android.graphics.Bitmap

pub struct Bitmap;
pub struct PictureInfo;

// Java: public interface IBitmapCreator
pub trait IBitmapCreator {
    // Java: abstract Bitmap createBitmap(PictureInfo var1, int var2, int var3)
    fn createBitmap(&self, var1: &PictureInfo, var2: i32, var3: i32) -> Bitmap;
}
