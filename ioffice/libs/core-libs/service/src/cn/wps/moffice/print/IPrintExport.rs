// Java: import android.graphics.Bitmap
use stubs::android::graphics::Bitmap::*;

// Java: public interface IPrintExport 
pub trait IPrintExport {
    // Java: boolean createDocument(String var1);
    fn createDocument(&mut self, var1: &str) -> bool;

    // Java: void closeDocument();
    fn closeDocument(&mut self);

    // Java: void destroy();
    fn destroy(&mut self);

    // Java: boolean addPage(Bitmap var1, int var2, int var3, boolean var4);
    fn addPage(&mut self, var1: Bitmap, var2: i32, var3: i32, var4: bool) -> bool;

}
