#![allow(non_snake_case, dead_code)]

// Java: import cn.wps.graphics.Bitmap

pub struct Bitmap;
pub struct IPicture;

// Java: public interface IPictureMemoryPool
pub trait IPictureMemoryPool {
    // Java: abstract void onOutOfMemory()
    fn onOutOfMemory(&self);

    // Java: abstract boolean check(int var1)
    fn check(&self, var1: i32) -> bool;

    // Java: abstract boolean check(int var1, boolean var2)
    fn check_with_flag(&self, var1: i32, var2: bool) -> bool;

    // Java: abstract void clearMemory()
    fn clearMemory(&self);

    // Java: abstract void setMaxMemorySize(int var1)
    fn setMaxMemorySize(&mut self, var1: i32);

    // Java: abstract int getMaxMemorySize()
    fn getMaxMemorySize(&self) -> i32;

    // Java: abstract void dispose()
    fn dispose(&mut self);

    // Java: abstract void clear()
    fn clear(&mut self);

    // Java: abstract Bitmap getBitmap(IPicture var1, int var2, int var3, boolean var4, boolean var5)
    fn getBitmap(&self, var1: &IPicture, var2: i32, var3: i32, var4: bool, var5: bool) -> Option<Bitmap>;

    // Java: abstract boolean addPictureToMap(IPicture var1, Bitmap var2, int var3, int var4)
    fn addPictureToMap(&mut self, var1: &IPicture, var2: Bitmap, var3: i32, var4: i32) -> bool;

    // Java: abstract Bitmap getReuseBitmap(int var1, int var2, boolean var3)
    fn getReuseBitmap(&self, var1: i32, var2: i32, var3: bool) -> Option<Bitmap>;
}
