// Java: import java.io.IOException
// Java: import java.io.InputStream
// Java: import java.security.GeneralSecurityException
// Java: import org.apache.poi.EncryptedDocumentException
use crate::org::apache::poi::EncryptedDocumentException::EncryptedDocumentException;
// Java: import org.apache.poi.poifs.filesystem.DirectoryNode
use crate::org::apache::poi::poifs::filesystem::DirectoryNode::DirectoryNode;
// Java: import org.apache.poi.poifs.filesystem.POIFSFileSystem
use crate::org::apache::poi::poifs::filesystem::POIFSFileSystem::POIFSFileSystem;

use crate::cn::wps::encrypt::EncryptionInfo::EncryptionInfo;

// Java field: String DEFAULT_PASSWORD
pub const DEFAULT_PASSWORD: &str = "VelvetSweatshop";

pub trait DecryptorTrait {
    fn getDataStream_from_dir(&mut self, dir: &mut DirectoryNode) -> std::io::Result<Box<dyn std::io::Read>>;
    fn verifyPassword(&mut self, password: &str) -> bool;
    fn getDataStream_from_fs(&mut self, fs: &mut POIFSFileSystem) -> std::io::Result<Box<dyn std::io::Read>> {
        self.getDataStream_from_dir(fs.getRoot())
    }
}

// Java: public class Decryptor
pub struct Decryptor;

// Java: public class Decryptor
impl Decryptor {
    // Java: Decryptor()
    pub fn new() -> Self {
        // Java: super();
        Decryptor
    }

    // Java: abstract InputStream getDataStream(DirectoryNode var1)

    // Java: abstract boolean verifyPassword(String var1)

    // Java: static Decryptor getInstance(EncryptionInfo info)
    pub fn getInstance(info: &EncryptionInfo) -> Box<dyn DecryptorTrait> {
        // Java: int major = info.getVersionMajor();
        let major = info.getVersionMajor();
        // Java: int minor = info.getVersionMinor();
        let minor = info.getVersionMinor();
        // Java: if (major == 4 && minor == 4)
        if major == 4 && minor == 4 {
            // Java: return new AgileDecryptor(info);
            Box::new(crate::cn::wps::encrypt::AgileDecryptor::AgileDecryptor::new(info))
        }
        // Java: else if (minor != 2 || major != 3 && major != 4)
        else if minor != 2 || (major != 3 && major != 4) {
            // Java: throw new EncryptedDocumentException("Unsupported version");
            panic!("Unsupported version");
        }
        // Java: else
        else {
            // Java: return new EcmaDecryptor(info);
            Box::new(crate::cn::wps::encrypt::EcmaDecryptor::EcmaDecryptor::new(info))
        }
    }

    // Java: InputStream getDataStream(POIFSFileSystem fs)
    pub fn getDataStream_from_fs(decryptor: &mut dyn DecryptorTrait, fs: &mut POIFSFileSystem) -> std::io::Result<Box<dyn std::io::Read>> {
        // Java: return this.getDataStream(fs.getRoot());
        decryptor.getDataStream_from_dir(fs.getRoot())
    }

    // Java: static int getBlockSize(int algorithm)
    pub fn getBlockSize(algorithm: i32) -> i32 {
        // Java: switch (algorithm)
        match algorithm {
            // Java: case 26126:
            26126 => {
                // Java: return 16;
                16
            }
            // Java: case 26127:
            26127 => {
                // Java: return 24;
                24
            }
            // Java: case 26128:
            26128 => {
                // Java: return 32;
                32
            }
            // Java: default:
            _ => {
                // Java: throw new EncryptedDocumentException("Unknown block size");
                panic!("Unknown block size");
            }
        }
    }
}
