// Java: import java.io.IOException
// Java: import java.io.OutputStream

use super::OLEEntry::OLEEntry;

// Java: public class OLEOutputStream extends OutputStream
pub struct OLEOutputStream {
    // Java field: IOLEStream mStream
    mStream: Option<Box<dyn std::any::Any>>,
    // Java field: OLEEntry mEntry
    mEntry: Option<OLEEntry>,
}

// Java: public class OLEOutputStream extends OutputStream
impl OLEOutputStream {
    // Java: OLEOutputStream(IOLEStream pStream)
    pub fn new(_pStream: Box<dyn std::any::Any>) -> Self {
      // Java: super();
      // Java: this.mStream = pStream;
      // Java: this.mEntry = new OLEEntry();
      // Java: this.mStream.stat(this.mEntry);
      OLEOutputStream {
          mStream: Some(_pStream),
          mEntry: Some(OLEEntry::new()),
      }
    }

    // Java: void write(int b)
    pub fn write_byte(&mut self, _b: i32) {
      // Java: byte[] buf = new byte[]{(byte)b};
      // Java: this.mStream.write(buf, 0, 1);
    }

    // Java: void write(byte[] b, int off, int len)
    pub fn write_buf(&mut self, _b: &[u8], _off: i32, _len: i32) {
      // Java: this.mStream.write(b, off, len);
    }

    // Java: void flush()
    pub fn flush(&self) {
    }

    // Java: void close()
    pub fn close(&mut self) {
      // Java: this.mStream.close();
    }

}
