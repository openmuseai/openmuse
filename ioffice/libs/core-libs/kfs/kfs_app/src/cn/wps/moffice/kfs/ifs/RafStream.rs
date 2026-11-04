#![allow(non_snake_case, dead_code, unused_variables, unused_imports)]

// Java: import cn.wps.moffice.kfs.RandomAccessFile
use crate::kfs::mfs::RandomAccessFile::RandomAccessFile;

// Java: public class RafStream extends InputStream
pub struct RafStream {
    // Java field:    private final RandomAccessFile mRaf;
    mRaf: RandomAccessFile,
}

// Java: public class RafStream extends InputStream
impl RafStream {
    // Java: public RafStream(RandomAccessFile raf) throws IOException
    pub fn new(mut raf: RandomAccessFile) -> RafStream {
        // Java: raf.seek(0L);
        raf.seek(0);
        RafStream { mRaf: raf }
    }

    // Java: public int read() throws IOException
    pub fn read(&mut self) -> i32 {
        // Java: return this.mRaf.read();
        self.mRaf.read()
    }

    // Java: public int read(byte[] b) throws IOException
    pub fn read__bytearr(&mut self, b: &mut [u8]) -> i32 {
        // Java: return this.mRaf.read(b);
        self.mRaf.read__bytearr(b)
    }

    // Java: public int read(byte[] b, int off, int len) throws IOException
    pub fn read__bytearr_int_int(&mut self, b: &mut [u8], off: i32, len: i32) -> i32 {
        // Java: return this.mRaf.read(b, off, len);
        self.mRaf.read__bytearr_int_int(b, off, len)
    }

    // Java: public int available() throws IOException
    pub fn available(&mut self) -> i32 {
        // Java: return (int)(this.mRaf.length() - this.mRaf.getFilePointer());
        (self.mRaf.length() - self.mRaf.getFilePointer()) as i32
    }

    // Java: public void close() throws IOException
    pub fn close(&mut self) {
        // Java: this.mRaf.close();
        self.mRaf.close();
    }
}
