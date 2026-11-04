#![allow(non_snake_case, dead_code, unused_variables, non_upper_case_globals)]

// Java: import java.io.IOException
// Java: import org.apache.poi.poifs.filesystem.DocumentInputStream
// use io_poi_compact::org::apache::poi::poifs::filesystem::DocumentInputStream;

// Java: public class WpsVersion
pub struct WpsVersion {
    // Java field: DocumentInputStream mVersion
    mVersion: Option<Box<dyn std::any::Any>>,
    // Java field: int mVersionMajor
    pub(crate) mVersionMajor: i32,
    // Java field: int mVersioMinor
    pub(crate) mVersioMinor: i32,
    // Java field: String mIdentifier
    mIdentifier: String,
}

// Java: public class WpsVersion
impl WpsVersion {
    // Java: WpsVersion(DocumentInputStream version)
    pub fn new(_version: Box<dyn std::any::Any>) -> Self {
      // Java: super();
      // Java: this.mVersion = version;
      // Java: this.mVersionMajor = this.mVersion.readShort();
      // Java: this.mVersioMinor = this.mVersion.readShort();
      // Java: this.mIdentifier = this.readIdentifier();
      todo!()
    }

    // Java: int getVersionMajor()
    pub fn getVersionMajor(&self) -> i32 {
      // Java: return this.mVersionMajor;
      self.mVersionMajor
    }

    // Java: int getVersionMinor()
    pub fn getVersionMinor(&self) -> i32 {
      // Java: return this.mVersioMinor;
      self.mVersioMinor
    }

    // Java: int getLength()
    fn getLength(&self) -> i32 {
      // Java: return this.mVersion.readInt();
      todo!()
    }

    // Java: String readIdentifier()
    fn readIdentifier(&self) -> String {
      // Java: int len = this.getLength();
      let len = self.getLength();
      // Java: byte[] data = new byte[len];
      let mut data = vec![0u8; len as usize];
      // Java: this.mVersion.read(data, 0, len);
      // Java: return Tool.parsedByteToStr(data, len);
      todo!()
    }

}
