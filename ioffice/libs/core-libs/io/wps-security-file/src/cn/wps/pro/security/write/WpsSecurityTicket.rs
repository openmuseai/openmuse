#![allow(non_snake_case, dead_code, unused_variables, non_upper_case_globals, unreachable_code)]

// Java: import cn.wps.base.utils.Base64
// Java: import java.io.ByteArrayInputStream
// Java: import org.apache.poi.util.LittleEndianByteArrayOutputStream

use std::io::Cursor;

// Java: public class WpsSecurityTicket
pub struct WpsSecurityTicket {
    // Java field: LittleEndianByteArrayOutputStream outputStream
    outputStream: Option<Box<dyn std::any::Any>>,
    // Java field: byte[] mDatabyte
    mDatabyte: Vec<u8>,
}

// Java: public class WpsSecurityTicket
impl WpsSecurityTicket {
    // Java: WpsSecurityTicket(String docId, String data)
    pub fn new(_docId: &str, _data: &str) -> Self {
      // Java: super();
      // Java: byte[] dataBase64 = Base64.decode(data);
      // Java: int size = 4 + docId.length() + 4 + dataBase64.length;
      // Java: this.mDatabyte = new byte[size];
      // Java: this.outputStream = new LittleEndianByteArrayOutputStream(this.mDatabyte, 0);
      // Java: this.writeString(docId);
      // Java: this.write(dataBase64);
      todo!()
    }

    // Java: void writeString(String value)
    fn writeString(&mut self, _value: &str) {
      // Java: this.outputStream.writeInt(value.length());
      // Java: this.outputStream.write(value.getBytes());
      todo!()
    }

    // Java: void write(byte[] value)
    fn write_bytes(&mut self, _value: &[u8]) {
      // Java: this.outputStream.writeInt(value.length);
      // Java: this.outputStream.write(value);
      todo!()
    }

    // Java: ByteArrayInputStream getInputStream()
    pub fn getInputStream(&self) -> Cursor<Vec<u8>> {
      // Java: return new ByteArrayInputStream(this.mDatabyte);
      Cursor::new(self.mDatabyte.clone())
    }

}
