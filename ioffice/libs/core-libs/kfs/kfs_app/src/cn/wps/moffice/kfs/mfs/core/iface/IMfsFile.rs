// Java: public interface IMfsFile 
pub trait IMfsFile {
    // Java: int read(long var1, byte[] var3, int var4, int var5);
    fn read(&mut self, var1: i64, var3: Vec<u8>, var4: i32, var5: i32) -> i32;

    // Java: void write(long var1, byte[] var3, int var4, int var5);
    fn write(&mut self, var1: i64, var3: Vec<u8>, var4: i32, var5: i32);

    // Java: void truncate(long var1);
    fn truncate(&mut self, var1: i64);

    // Java: long length();
    fn length(&mut self) -> i64;

    // Java: String[] list();
    fn list(&mut self) -> Vec<String>;

    // Java: boolean isFile();
    fn isFile(&mut self) -> bool;

    // Java: boolean setLastModified(long var1);
    fn setLastModified(&mut self, var1: i64) -> bool;

    // Java: long lastModified();
    fn lastModified(&mut self) -> i64;

    // Java: boolean delete();
    fn delete(&mut self) -> bool;

    // Java: boolean export(String var1);
    fn export(&mut self, var1: String) -> bool;

}
