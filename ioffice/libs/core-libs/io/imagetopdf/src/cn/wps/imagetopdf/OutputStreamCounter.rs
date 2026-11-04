// Java: import java.io.IOException
use std::io::{self, Write};
// Java: import java.io.OutputStream

// Java: public class OutputStreamCounter extends OutputStream
pub struct OutputStreamCounter {
    // Java field: OutputStream out
    pub(crate) out: Box<dyn Write>,
    // Java field: long position
    pub(crate) position: i64,
}

// Java: public class OutputStreamCounter extends OutputStream
impl OutputStreamCounter {
    // Java: OutputStreamCounter(OutputStream out)
    pub fn new(out: Box<dyn Write>) -> Self {
        // Java: super();
        // Java: this.out = out;
        OutputStreamCounter { out, position: 0 }
    }

    // Java: void close()
    pub fn close(&mut self) {
        // Java: this.out.close();
        let _ = self.out.flush();
    }

    // Java: void flush()
    pub fn flush(&mut self) {
        // Java: this.out.flush();
        let _ = self.out.flush();
    }

    // Java: void write(byte[] b)
    pub fn write_buf(&mut self, b: &[u8]) -> io::Result<()> {
        // Java: this.position += (long)b.length;
        self.position += b.len() as i64;
        // Java: this.out.write(b);
        self.out.write_all(b)
    }

    // Java: void write(int b)
    pub fn write_byte(&mut self, b: i32) -> io::Result<()> {
        // Java: ++this.position;
        self.position += 1;
        // Java: this.out.write(b);
        self.out.write_all(&[b as u8])
    }

    // Java: void write(byte[] b, int off, int len)
    pub fn write_buf_offset(&mut self, b: &[u8], off: i32, len: i32) -> io::Result<()> {
        // Java: this.position += (long)len;
        self.position += len as i64;
        // Java: this.out.write(b, off, len);
        self.out.write_all(&b[off as usize..(off + len) as usize])
    }

    // Java: long getPosition()
    pub fn getPosition(&self) -> i64 {
        // Java: return this.position;
        self.position
    }

    // Java: void resetPosition()
    pub fn resetPosition(&mut self) {
        // Java: this.position = 0L;
        self.position = 0;
    }
}
