// Java: import java.io.Closeable
// Java: import java.io.IOException

// Java: public interface IOLEStorage extends Closeable
pub trait IOLEStorage {
    // Java: abstract IOLEStream openStream(String var1)
    fn openStream(&self, var1: &str) -> Box<dyn std::any::Any>;

    // Java: abstract IOLEStorage openStorage(String var1)
    fn openStorage(&self, var1: &str) -> Box<dyn std::any::Any>;

    // Java: abstract IOLEStream createStream(String var1)
    fn createStream(&mut self, var1: &str) -> Box<dyn std::any::Any>;

    // Java: abstract IOLEStorage createStorage(String var1)
    fn createStorage(&mut self, var1: &str) -> Box<dyn std::any::Any>;

    // Java: abstract void renameElement(String var1, String var2)
    fn renameElement(&mut self, var1: &str, var2: &str);

    // Java: abstract int getElementCount()
    fn getElementCount(&self) -> i32;

    // Java: abstract void getElements(int var1, int var2, OLEEntry[] var3)
    fn getElements(&self, var1: i32, var2: i32, var3: &mut [Box<dyn std::any::Any>]);

    // Java: abstract byte[] getStorageClsid()
    fn getStorageClsid(&self) -> Vec<u8>;

    // Java: abstract void setStorageClsid(byte[] var1)
    fn setStorageClsid(&mut self, var1: &[u8]);

    // Java: abstract boolean destroyElement(String var1)
    fn destroyElement(&mut self, var1: &str) -> bool;

}
