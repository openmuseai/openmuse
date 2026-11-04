#![allow(non_snake_case, dead_code, unused_variables, non_upper_case_globals, unreachable_code)]

// Java: import java.io.ByteArrayInputStream
use std::io::Cursor;
// Java: import org.apache.poi.util.LittleEndianByteArrayOutputStream
// use util::org::apache::poi::util::LittleEndianByteArrayOutputStream;

// Java: public class SecurityWriterBase
pub struct SecurityWriterBase {
    // Java field: LittleEndianByteArrayOutputStream outputStream
    outputStream: Option<Box<dyn std::any::Any>>,
    // Java field: byte[] data
    data: Option<Vec<u8>>,
}

// Java: public class SecurityWriterBase
impl SecurityWriterBase {
    // Java: SecurityWriterBase()
    pub fn new() -> Self {
      // Java: super();
      SecurityWriterBase {
          outputStream: None,
          data: None,
      }
    }

    // Java: abstract abstract int getSize()
    // Abstract method - to be implemented by subclasses via trait

    // Java: abstract abstract void writeValue()
    // Abstract method - to be implemented by subclasses via trait

    // Java: void write()
    pub fn write(&mut self, get_size: fn(&Self) -> i32, write_value: fn(&mut Self)) {
      // Java: int size = this.getSize();
      let size = get_size(self);
      // Java: this.data = new byte[size];
      self.data = Some(vec![0u8; size as usize]);
      // Java: this.outputStream = new LittleEndianByteArrayOutputStream(this.data, 0);
      // Java: this.writeValue();
      write_value(self);
    }

    // Java: ByteArrayInputStream getInputStream()
    pub fn getInputStream(&mut self) -> Cursor<Vec<u8>> {
      // Java: this.write();
      // Java: return new ByteArrayInputStream(this.data);
      todo!()
    }

}
