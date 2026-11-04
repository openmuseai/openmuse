#![allow(non_snake_case, dead_code, unused_variables, non_upper_case_globals, unreachable_code)]

// Java: import cn.wps.base.assertion.IOAssert
// use util::cn::wps::base::assertion::IOAssert;
// Java: import cn.wps.crypt.sal.digest.DigestSHA1
use crate::cn::wps::crypt::sal::digest::DigestSHA1::DigestSHA1;
// Java: import cn.wps.crypt.sal.digest.DigestSHA512
use crate::cn::wps::crypt::sal::digest::DigestSHA512::DigestSHA512;
// Java: import cn.wps.crypt.sal.digest.IDigest
use crate::cn::wps::crypt::sal::digest::IDigest::IDigest;
// Java: import java.io.IOException
// Java: import java.io.InputStream
// Java: import java.io.UnsupportedEncodingException
// Java: import java.security.GeneralSecurityException
// Java: import java.security.NoSuchAlgorithmException
// Java: import org.apache.poi.EncryptedDocumentException
// use crate::org::apache::poi::EncryptedDocumentException;
// Java: import org.apache.poi.poifs.filesystem.DirectoryNode
// use crate::org::apache::poi::poifs::filesystem::DirectoryNode;
// Java: import org.apache.poi.poifs.filesystem.DocumentInputStream
// use crate::org::apache::poi::poifs::filesystem::DocumentInputStream;
// Java: import org.apache.poi.poifs.filesystem.POIFSFileSystem
// use crate::org::apache::poi::poifs::filesystem::POIFSFileSystem;

// Java field: String DEFAULT_PASSWORD
pub const DEFAULT_PASSWORD: &str = "VelvetSweatshop";

// Java: public class Decryptor
pub struct Decryptor {
}

// Java: public class Decryptor
impl Decryptor {
    // Java: Decryptor()
    pub fn new() -> Self {
      // Java: super();
      Decryptor {}
    }

    // Java: abstract InputStream getDataStream(DirectoryNode var1)
    // Abstract method - implemented by subclasses

    // Java: abstract InputStream getDataStream(DocumentInputStream var1)
    // Abstract method - implemented by subclasses

    // Java: abstract boolean verifyPassword(String var1)
    // Abstract method - implemented by subclasses

    // Java: static Decryptor getInstance(EncryptionInfo info)
    pub fn getInstance(_info: &dyn std::any::Any) -> Box<dyn std::any::Any> {
      // Java: int major = info.getVersionMajor();
      // Java: int minor = info.getVersionMinor();
      // Java: if (major == 4 && minor == 4)
      {
         // Java: return new AgileDecryptor(info);
      }
      // Java: else if (minor != 2 || major != 3 && major != 4)
      {
         // Java: throw new EncryptedDocumentException("Unsupported version");
      }
      // Java: else
      {
         // Java: return new EcmaDecryptor(info);
      }
      todo!()
    }

    // Java: static Decryptor getInstance(EncryptionInfoPro info)
    pub fn getInstance_pro(_info: &dyn std::any::Any) -> Box<dyn std::any::Any> {
      // Java: int major = info.getVersionMajor();
      // Java: int minor = info.getVersionMinor();
      // Java: if (major == 4 && minor == 4)
      {
         // Java: return new AgileDecryptor(info);
      }
      // Java: else if (minor != 2 || major != 3 && major != 4)
      {
         // Java: throw new EncryptedDocumentException("Unsupported version");
      }
      // Java: else
      {
         // Java: return new EcmaDecryptor(info);
      }
      todo!()
    }

    // Java: InputStream getDataStream(POIFSFileSystem fs)
    pub fn getDataStream_fs(&self, _fs: &dyn std::any::Any) -> Box<dyn std::any::Any> {
      // Java: return this.getDataStream(fs.getRoot());
      todo!()
    }

    // Java: InputStream getWpsContent(DocumentInputStream fs)
    pub fn getWpsContent(&self, _fs: &dyn std::any::Any) -> Box<dyn std::any::Any> {
      // Java: return this.getDataStream(fs);
      todo!()
    }

    // Java: abstract int getDataStreamSize()
    // Abstract method - implemented by subclasses

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
            panic!("EncryptedDocumentException: Unknown block size")
         }
      }
    }

    // Java: byte[] hashPassword(EncryptionInfo info, String hashAlgorithmName, String password)
    pub fn hashPassword(&self, _info: &dyn std::any::Any, hashAlgorithmName: &str, password: &str) -> Vec<u8> {
      // Java: IDigest digest = null;
      let mut digest: Box<dyn IDigest>;
      // Java: if (hashAlgorithmName.equals("SHA-1"))
      if hashAlgorithmName == "SHA-1" {
         // Java: digest = new DigestSHA1();
         digest = Box::new(DigestSHA1::new());
      }
      // Java: else
      else {
         // Java: if (!hashAlgorithmName.equals("SHA-512"))
         if hashAlgorithmName != "SHA-512" {
            // Java: IOAssert.fail("UnsupportedAlgorithm!");
            panic!("UnsupportedAlgorithm!");
            // Java: return null;
         }
         // Java: digest = new DigestSHA512();
         digest = Box::new(DigestSHA512::new());
      }

      // Java: byte[] bytes;
      // Java: try
      {
         // Java: bytes = password.getBytes("UTF-16LE");
      }
      // Java: catch (UnsupportedEncodingException var11)
      {
         // Java: throw new EncryptedDocumentException("UTF16 not supported");
      }
      let bytes: Vec<u8> = password.encode_utf16().flat_map(|c| c.to_le_bytes()).collect();

      // Java: int hashLen = digest.getLength();
      let hashLen = digest.get_length() as usize;
      // Java: byte[] hash = new byte[hashLen];
      let mut hash = vec![0u8; hashLen];
      // Java: digest.update(info.getVerifier().getSalt(), 0, info.getVerifier().getSalt().length);
      // Java: digest.update(bytes, 0, bytes.length);
      // Java: digest.get(hash);
      // Java: byte[] iterator = new byte[4];
      let mut iterator = [0u8; 4];
      // Java: int count = info.getVerifier().getSpinCount();
      // Java: for(int i = 0; i < count; ++i)
      {
         // Java: iterator[0] = (byte)(i >>> 0 & 255);
         // Java: iterator[1] = (byte)(i >>> 8 & 255);
         // Java: iterator[2] = (byte)(i >>> 16 & 255);
         // Java: iterator[3] = (byte)(i >>> 24 & 255);
         // Java: digest.update(iterator, 0, 4);
         // Java: digest.update(hash, 0, hashLen);
         // Java: digest.get(hash);
      }
      // Java: return hash;
      todo!()
    }

}
