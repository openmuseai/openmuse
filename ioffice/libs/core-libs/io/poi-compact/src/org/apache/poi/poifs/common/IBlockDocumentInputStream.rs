// Java: import cn.wps.moffice.kfs.File
// Java: import org.apache.poi.poifs.filesystem.DocumentInputStream

use std::path::PathBuf;

pub struct DocumentInputStream;

// Java: public interface IBlockDocumentInputStream
pub trait IBlockDocumentInputStream {
    // Java: abstract long seek(long var1)
    fn seek(&mut self, pos: i64) -> i64;

    // Java: abstract short readShort()
    fn readShort(&mut self) -> i16;

    // Java: abstract int readUShort()
    fn readUShort(&mut self) -> i32;

    // Java: abstract int readInt()
    fn readInt(&mut self) -> i32;

    // Java: abstract byte readByte()
    fn readByte(&mut self) -> i8;

    // Java: abstract int available()
    fn available(&self) -> i32;

    // Java: abstract void read(byte[] var1)
    fn read(&mut self, buf: &mut [u8]);

    // Java: abstract void dump(File var1, int var2)
    fn dump(&mut self, file: &PathBuf, size: i32);

    // Java: abstract DocumentInputStream getDocument()
    fn getDocument(&self) -> Option<&crate::org::apache::poi::poifs::filesystem::DocumentInputStream::DocumentInputStream>;

    // Java: abstract boolean isContinueNext()
    fn isContinueNext(&self) -> bool;
}
