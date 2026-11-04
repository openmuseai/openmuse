#![allow(non_snake_case, dead_code, unused_variables, non_upper_case_globals, unreachable_code)]

// Java: import java.io.IOException
// Java: import org.apache.poi.poifs.filesystem.DirectoryNode
// use crate::org::apache::poi::poifs::filesystem::DirectoryNode;
// Java: import org.apache.poi.poifs.filesystem.DocumentInputStream
// use crate::org::apache::poi::poifs::filesystem::DocumentInputStream;
// Java: import org.apache.poi.poifs.filesystem.POIFSFileSystem
// use crate::org::apache::poi::poifs::filesystem::POIFSFileSystem;

use super::EncryptionHeader::EncryptionHeader;
use super::EncryptionVerifier::EncryptionVerifier;

// Java: public class EncryptionInfo
pub struct EncryptionInfo {
    // Java field: int versionMajor
    pub versionMajor: i32,
    // Java field: int versionMinor
    pub versionMinor: i32,
    // Java field: int encryptionFlags
    pub encryptionFlags: i32,
    // Java field: EncryptionHeader header
    pub header: Option<EncryptionHeader>,
    // Java field: EncryptionVerifier verifier
    pub verifier: Option<EncryptionVerifier>,
}

// Java: public class EncryptionInfo
impl EncryptionInfo {
    // Java: EncryptionInfo()
    pub fn new() -> Self {
      // Java: super();
      EncryptionInfo {
          versionMajor: 0,
          versionMinor: 0,
          encryptionFlags: 0,
          header: None,
          verifier: None,
      }
    }

    // Java: EncryptionInfo(POIFSFileSystem fs)
    pub fn new_from_fs(_fs: &dyn std::any::Any) -> Self {
      // Java: this(fs.getRoot());
      todo!()
    }

    // Java: EncryptionInfo(DirectoryNode dir)
    pub fn new_from_dir(_dir: &mut dyn std::any::Any) -> Self {
      // Java: super();
      // Java: DocumentInputStream dis = dir.createDocumentInputStream("EncryptionInfo");
      // Java: this.versionMajor = dis.readShort();
      // Java: this.versionMinor = dis.readShort();
      // Java: this.encryptionFlags = dis.readInt();
      // Java: if (this.versionMajor == 4 && this.versionMinor == 4 && this.encryptionFlags == 64)
      {
         // Java: StringBuilder builder = new StringBuilder();
         // Java: byte[] xmlDescriptor = new byte[dis.available()];
         // Java: dis.read(xmlDescriptor);
         // Java: for(byte b : xmlDescriptor)
         {
            // Java: builder.append((char)b);
         }
         // Java: String descriptor = builder.toString();
         // Java: this.header = new EncryptionHeader(descriptor);
         // Java: this.verifier = new EncryptionVerifier(descriptor);
      }
      // Java: else
      {
         // Java: dis.readInt();
         // Java: this.header = new EncryptionHeader(dis);
         // Java: if (this.header.getAlgorithm() == 26625)
         {
            // Java: this.verifier = new EncryptionVerifier(dis, 20);
         }
         // Java: else
         {
            // Java: this.verifier = new EncryptionVerifier(dis, 32);
         }
      }
      todo!()
    }

    // Java: int getVersionMajor()
    pub fn getVersionMajor(&self) -> i32 {
      // Java: return this.versionMajor;
      self.versionMajor
    }

    // Java: int getVersionMinor()
    pub fn getVersionMinor(&self) -> i32 {
      // Java: return this.versionMinor;
      self.versionMinor
    }

    // Java: int getEncryptionFlags()
    pub fn getEncryptionFlags(&self) -> i32 {
      // Java: return this.encryptionFlags;
      self.encryptionFlags
    }

    // Java: EncryptionHeader getHeader()
    pub fn getHeader(&self) -> &Option<EncryptionHeader> {
      // Java: return this.header;
      &self.header
    }

    // Java: EncryptionVerifier getVerifier()
    pub fn getVerifier(&self) -> &Option<EncryptionVerifier> {
      // Java: return this.verifier;
      &self.verifier
    }

}
