// Java: import cn.wps.pdfExporter.KDynamicMemoryStream
use crate::cn::wps::pdfExporter::KDynamicMemoryStream::KDynamicMemoryStream;
// Java: import java.io.IOException
// Java: import java.io.OutputStream

// Java: public class BufferOutputStream extends OutputStream
pub struct BufferOutputStream {
    // Java field: KDynamicMemoryStream mStream
    pub(crate) mStream: KDynamicMemoryStream,
}

// Java: public class BufferOutputStream extends OutputStream
impl BufferOutputStream {
    // Java: BufferOutputStream()
    pub fn new() -> Self {
      // Java: super();
      Self {
          mStream: KDynamicMemoryStream::new(),
      }
    }

    // Java: void write(int oneByte)
    pub fn write_byte(&mut self, one_byte: i32) {
      // Java: byte[] buffer = new byte[]{(byte)oneByte};
      let buffer = [one_byte as u8];
      // Java: this.write(buffer);
      self.write_range(&buffer, 0, 1);
    }

    // Java: void write(byte[] buffer, int offset, int count)
    pub fn write_range(&mut self, buffer: &[u8], offset: i32, count: i32) {
      // Java: this.mStream.write(buffer, offset, count);
      self.mStream.write_bytes_range(buffer, offset, count);
    }

    // Java: int length()
    pub fn length(&self) -> i32 {
      // Java: return this.mStream.getLength();
      self.mStream.length
    }

    // Java: KDynamicMemoryStream getStream()
    pub fn getStream(&mut self) -> &mut KDynamicMemoryStream {
      // Java: return this.mStream;
      &mut self.mStream
    }
}
