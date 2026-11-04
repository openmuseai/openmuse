// Java: import cn.wps.base.assertion.KSAssert
// Java: import cn.wps.pdfExporter.KStream
use crate::cn::wps::pdfExporter::KStream::KStream;
// Java: import java.io.EOFException
// Java: import java.io.IOException
// Java: import java.io.InputStream
use std::io;

// Java: public class BufferInputStream extends InputStream
pub struct BufferInputStream {
    // Java field: KStream mStream
    pub(crate) mStream: KStream,
    // Java field: int mPos
    mPos: i32,
}

// Java: public class BufferInputStream extends InputStream
impl BufferInputStream {
    // Java: BufferInputStream(KStream stream)
    pub fn new(stream: KStream) -> Self {
      // Java: super();
      // Java: this.mStream = stream;
      // Java: this.mPos = 0;
      Self {
          mStream: stream,
          mPos: 0,
      }
    }

    // Java: int read()
    pub fn read_byte(&mut self) -> io::Result<i32> {
      // Java: if (this.mPos < this.mStream.getLength())
      if self.mPos < self.mStream.getLength()
      {
         // Java: return this.mStream.read(this.mPos);
         let val = self.mStream.read_at(self.mPos);
         self.mPos += 1;
         return Ok(val as i32);
      }
      // Java: else
      {
         // Java: throw new EOFException();
         return Err(io::Error::new(io::ErrorKind::UnexpectedEof, "EOF"));
      }
    }

    // Java: int read(byte[] buffer, int byteOffset, int byteCount)
    pub fn read_range(&mut self, buffer: &mut [u8], byte_offset: i32, mut byte_count: i32) -> i32 {
      // Java: KSAssert.assertTrue(byteOffset == 0);
      assert!(byte_offset == 0);
      // Java: int avail = this.available();
      let avail = self.available();
      // Java: if (byteCount > avail)
      if byte_count > avail
      {
         // Java: byteCount = avail;
         byte_count = avail;
      }
      // Java: if (byteCount == 0)
      if byte_count == 0
      {
         // Java: return -1;
         return -1;
      }
      // Java: else
      {
         // Java: int read = this.mStream.read(buffer, byteOffset, this.mPos, byteCount);
         let read = self.mStream.read_to_buf_range(buffer, byte_offset, self.mPos, byte_count);
         // Java: this.mPos += read;
         self.mPos += read;
         // Java: return read;
         return read;
      }
    }

    // Java: int available()
    pub fn available(&self) -> i32 {
      // Java: return this.mStream.getLength() - this.mPos;
      self.mStream.getLength() - self.mPos
    }
}
