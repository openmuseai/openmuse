// Java: import java.io.Closeable
// Java: import java.io.DataInput
// Java: import java.io.DataOutput
// Java: import java.io.IOException

// Java: public interface IOLEStream extends DataInput
pub trait IOLEStream {
    // Java: abstract void seek(int var1)
    fn seek(&mut self, var1: i32);

    // Java: abstract int tell()
    fn tell(&self) -> i32;

    // Java: abstract void setSize(int var1)
    fn setSize(&mut self, var1: i32);

    // Java: abstract void stat(OLEEntry var1)
    fn stat(&self, var1: &dyn std::any::Any);

    // Java: abstract int read(byte[] var1, int var2, int var3)
    fn read(&mut self, var1: &mut [u8], var2: i32, var3: i32) -> i32;

}
