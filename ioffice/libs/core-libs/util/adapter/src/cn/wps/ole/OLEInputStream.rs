// Java: import java.io.IOException
// Java: import java.io.InputStream

use super::OLEEntry::OLEEntry;

// Java: public class OLEInputStream extends InputStream
pub struct OLEInputStream {
    // Java field: OLEEntry mEntry
    pub(crate) mEntry: Option<OLEEntry>,
    // Java field: IOLEStream mStream
    pub(crate) mStream: Option<Box<dyn std::any::Any>>,
    // Java field: int mMark
    mMark: i32,
}

// Java: public class OLEInputStream extends InputStream
impl OLEInputStream {
    // Java: OLEInputStream(IOLEStream pStream)
    pub fn new(_pStream: Box<dyn std::any::Any>) -> Self {
      // Java: super();
      // Java: this.mStream = pStream;
      // Java: this.mEntry = new OLEEntry();
      // Java: this.mStream.stat(this.mEntry);
      OLEInputStream {
          mEntry: Some(OLEEntry::new()),
          mStream: Some(_pStream),
          mMark: 0,
      }
    }

    // Java: int read()
    pub fn read(&mut self) -> i32 {
      // Java: byte[] buf = new byte[2];
      // Java: this.mStream.read(buf, 0, 1);
      // Java: return buf[0];
      todo!()
    }

    // Java: int read(byte[] b, int off, int len)
    pub fn read_buf(&mut self, _b: &mut [u8], _off: i32, _len: i32) -> i32 {
      // Java: return this.mStream.read(b, off, len);
      todo!()
    }

    // Java: long skip(long n)
    pub fn skip(&mut self, _n: i64) -> i64 {
      // Java: int pos = this.mStream.tell();
      // Java: this.mStream.seek(pos + (int)n);
      // Java: return n;
      todo!()
    }

    // Java: int available()
    pub fn available(&self) -> i32 {
      // Java: return this.mEntry.size - this.mStream.tell();
      todo!()
    }

    // Java: void close()
    pub fn close(&mut self) {
      // Java: this.mStream.close();
      // Java: this.mEntry = null;
      self.mEntry = None;
      // Java: this.mStream = null;
      self.mStream = None;
    }

    // Java: void mark(int readlimit)
    pub fn mark(&mut self, _readlimit: i32) {
      // Java: try
      {
         // Java: this.mMark = this.mStream.tell();
      }
      // Java: catch (IOException var3)
      {
         // Java: this.mMark = -1;
      }
    }

    // Java: void reset()
    pub fn reset(&mut self) {
      // Java: this.mStream.seek(this.mMark);
    }

    // Java: boolean markSupported()
    pub fn markSupported(&self) -> bool {
      // Java: return true;
      true
    }

}
