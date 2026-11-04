#![allow(non_snake_case, dead_code, unused_variables, non_upper_case_globals, unreachable_code)]

// Java: import java.io.ByteArrayInputStream
// Java: import java.io.IOException
// Java: import org.apache.poi.util.LittleEndianByteArrayOutputStream

use std::io::Cursor;

// Java: public class WpsVersion
pub struct WpsVersion {
    // Java field: int mVersionMajor
    mVersionMajor: i32,
    // Java field: int mVersioMinor
    mVersioMinor: i32,
    // Java field: byte[] mIdentifier
    mIdentifier: Vec<u8>,
    // Java field: LittleEndianByteArrayOutputStream outputStream
    outputStream: Option<Box<dyn std::any::Any>>,
    // Java field: byte[] data
    data: Vec<u8>,
}

// Java: public class WpsVersion
impl WpsVersion {
    // Java: WpsVersion()
    pub fn new() -> Self {
      // Java: super();
      let mVersionMajor = 1;
      let mVersioMinor = 0;
      // Java: byte[] identifier = this.getIdentifier();
      let identifier: Vec<u8> = "Wps.Transform.Data".encode_utf16()
          .flat_map(|c| c.to_le_bytes())
          .collect();
      // Java: int size = 8 + identifier.length;
      let size = 8 + identifier.len();
      // Java: this.data = new byte[size];
      let mut data = vec![0u8; size];
      // Java: this.outputStream = new LittleEndianByteArrayOutputStream(this.data, 0);
      let mut offset = 0usize;
      // Java: this.outputStream.writeShort(this.getVersioMajor());
      data[offset] = (mVersionMajor & 0xFF) as u8;
      data[offset + 1] = ((mVersionMajor >> 8) & 0xFF) as u8;
      offset += 2;
      // Java: this.outputStream.writeShort(this.getVersioMinor());
      data[offset] = (mVersioMinor & 0xFF) as u8;
      data[offset + 1] = ((mVersioMinor >> 8) & 0xFF) as u8;
      offset += 2;
      // Java: this.outputStream.writeInt(identifier.length);
      let id_len = identifier.len() as i32;
      data[offset] = (id_len & 0xFF) as u8;
      data[offset + 1] = ((id_len >> 8) & 0xFF) as u8;
      data[offset + 2] = ((id_len >> 16) & 0xFF) as u8;
      data[offset + 3] = ((id_len >> 24) & 0xFF) as u8;
      offset += 4;
      // Java: this.outputStream.write(identifier);
      data[offset..offset + identifier.len()].copy_from_slice(&identifier);

      WpsVersion {
          mVersionMajor,
          mVersioMinor,
          mIdentifier: identifier,
          outputStream: None,
          data,
      }
    }

    // Java: ByteArrayInputStream getInputStream()
    pub fn getInputStream(&self) -> Cursor<Vec<u8>> {
      // Java: return new ByteArrayInputStream(this.data);
      Cursor::new(self.data.clone())
    }

    // Java: int getVersioMajor()
    fn getVersioMajor(&self) -> i32 {
      // Java: return 1;
      1
    }

    // Java: int getVersioMinor()
    fn getVersioMinor(&self) -> i32 {
      // Java: return 0;
      0
    }

    // Java: int getLength()
    fn getLength(&self) -> i32 {
      // Java: return this.mIdentifier.length;
      self.mIdentifier.len() as i32
    }

    // Java: byte[] getIdentifier()
    fn getIdentifier(&self) -> &[u8] {
      // Java: return this.mIdentifier;
      &self.mIdentifier
    }

}
