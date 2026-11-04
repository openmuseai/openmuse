#![allow(non_snake_case, dead_code, unused_variables)]

// Java: import cn.wps.graphics.PictureInfo
use crate::graphics::PictureInfo::PictureInfo;
// Java: import cn.wps.graphics.Size
use crate::graphics::Size::Size;

// Java field: int TYPE_NONE
pub const TYPE_NONE: i32 = -1;
// Java field: int TYPE_NORMAL
pub const TYPE_NORMAL: i32 = 0;
// Java field: int TYPE_METAFILE
pub const TYPE_METAFILE: i32 = 1;

// Java: public interface IPicture
pub trait IPicture {
    // Java: abstract void dispose()
    fn dispose(&mut self);

    // Java: abstract String getPath()
    fn getPath(&self) -> String;

    // Java: abstract int getType()
    fn getType(&self) -> i32;

    // Java: abstract boolean isSmall()
    fn isSmall(&self) -> bool;

    // Java: abstract boolean isFixedSize()
    fn isFixedSize(&self) -> bool;

    // Java: abstract void setDiskCache(IPicture var1)
    fn setDiskCache(&mut self, var1: Box<dyn IPicture>);

    // Java: abstract IPicture getDiskCache(int var1, int var2)
    fn getDiskCache(&self, var1: i32, var2: i32) -> Option<Box<dyn IPicture>>;

    // Java: abstract PictureInfo getPictureInfo()
    fn getPictureInfo(&self) -> PictureInfo;

    // Java: abstract Size calBitmapSize(Size var1, boolean var2)
    fn calBitmapSize(&self, var1: &Size, var2: bool) -> Size;

    // Java: abstract boolean isTooSmall(int var1, int var2, int var3, int var4)
    fn isTooSmall(&self, var1: i32, var2: i32, var3: i32, var4: i32) -> bool;

    // Java: abstract boolean isFastDraw()
    fn isFastDraw(&self) -> bool;

    // Java: abstract int getWidth()
    fn getWidth(&self) -> i32;

    // Java: abstract int getHeight()
    fn getHeight(&self) -> i32;
}
