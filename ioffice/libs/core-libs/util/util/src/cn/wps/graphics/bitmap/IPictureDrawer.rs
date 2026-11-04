#![allow(non_snake_case, dead_code, unused_variables)]

use super::IPicture::IPicture;

// Java: public interface IPictureDrawer
pub trait IPictureDrawer {
    // Java: abstract boolean drawBitmap(Object var1, IPicture var2, Object var3, Object var4, boolean var5)
    fn drawBitmap(&self, var1: &dyn std::any::Any, var2: &dyn IPicture, var3: &dyn std::any::Any, var4: &dyn std::any::Any, var5: bool) -> bool;
}
