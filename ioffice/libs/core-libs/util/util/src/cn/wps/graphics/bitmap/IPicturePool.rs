// Java: import cn.wps.graphics.Bitmap
// Java: import cn.wps.graphics.PictureInfo

use std::any::Any;
use std::sync::Arc;

type Element = Option<Arc<dyn Any + Send + Sync>>;

// Java: public interface IPicturePool
pub trait IPicturePool {
    // Java: abstract void onOutOfMemory()
    fn onOutOfMemory(&self);

    // Java: abstract void clearMemory()
    fn clearMemory(&self);

    // Java: abstract void dispose()
    fn dispose(&mut self);

    // Java: abstract void clear()
    fn clear(&mut self);

    // Java: abstract void setMaxMemorySize(int var1)
    fn setMaxMemorySize(&mut self, var1: i32);

    // Java: abstract int getMaxMemorySize()
    fn getMaxMemorySize(&self) -> i32;

    // Java: abstract PictureInfo getInfoFromPool(String var1)
    fn getInfoFromPool(&self, var1: &str) -> Element;

    // Java: abstract IPicture getPictureFromPool(Object var1)
    fn getPictureFromPool(&self, var1: Element) -> Element;

    // Java: abstract Bitmap getBitmapFromPool(Object var1, int var2, int var3, boolean var4, boolean var5)
    fn getBitmapFromPool_obj(&self, var1: Element, var2: i32, var3: i32, var4: bool, var5: bool) -> Element;

    // Java: abstract Bitmap getBitmapFromPool(IPicture var1, int var2, int var3, boolean var4, boolean var5)
    fn getBitmapFromPool(&self, var1: Element, var2: i32, var3: i32, var4: bool, var5: bool) -> Element;

    // Java: abstract Bitmap getBitmapFromPoolError(IPicture var1, int var2, int var3)
    fn getBitmapFromPoolError(&self, var1: Element, var2: i32, var3: i32) -> Element;

    // Java: abstract String createNewPath(IPicture var1, Bitmap var2, boolean var3)
    fn createNewPath(&self, var1: Element, var2: Element, var3: bool) -> String;

    // Java: abstract IPictureDrawer createPictureDrawer()
    fn createPictureDrawer(&self) -> Element;

    // Java: abstract Bitmap getTransparentBitmap(IPicture var1, int var2, int var3, int var4)
    fn getTransparentBitmap(&self, var1: Element, var2: i32, var3: i32, var4: i32) -> Element;
}
