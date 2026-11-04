// Java: import java.io.Closeable
// Java: import java.io.IOException

// Java: public interface IOLEDocument extends Closeable
pub trait IOLEDocument {
    // Java: abstract IOLEStorage getRootStorage()
    fn getRootStorage(&self) -> Box<dyn std::any::Any>;

    // Java: abstract int getSectorSize()
    fn getSectorSize(&self) -> i32;

    // Java: abstract int getBufferSize()
    fn getBufferSize(&self) -> i32;

    // Java: abstract void setBufferSize(int var1)
    fn setBufferSize(&mut self, var1: i32);

    // Java: abstract byte[] getBuffers()
    fn getBuffers(&self) -> Vec<u8>;

}
