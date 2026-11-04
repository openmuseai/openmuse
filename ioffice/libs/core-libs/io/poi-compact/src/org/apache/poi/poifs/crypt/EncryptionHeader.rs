#![allow(non_snake_case, dead_code, unused_variables, non_upper_case_globals, unreachable_code)]

// Java: import cn.wps.base.assertion.IOAssert
// use util::cn::wps::base::assertion::IOAssert;
// Java: import cn.wps.encrypt.EncryptionInfoInit
use crate::cn::wps::encrypt::EncryptionInfoInit::EncryptionInfoInit;
// Java: import java.io.ByteArrayInputStream
// Java: import java.io.IOException
// Java: import java.nio.charset.StandardCharsets
// Java: import javax.xml.parsers.DocumentBuilderFactory
// Java: import org.apache.poi.EncryptedDocumentException
// use crate::org::apache::poi::EncryptedDocumentException;
// Java: import org.apache.poi.poifs.filesystem.DocumentInputStream
// use crate::org::apache::poi::poifs::filesystem::DocumentInputStream;
// Java: import org.apache.poi.util.base64.Base64
// Java: import org.w3c.dom.NamedNodeMap

// Java field: int ALGORITHM_RC4
pub const ALGORITHM_RC4: i32 = 26625;
// Java field: int ALGORITHM_AES_128
pub const ALGORITHM_AES_128: i32 = 26126;
// Java field: int ALGORITHM_AES_192
pub const ALGORITHM_AES_192: i32 = 26127;
// Java field: int ALGORITHM_AES_256
pub const ALGORITHM_AES_256: i32 = 26128;
// Java field: int HASH_SHA1
pub const HASH_SHA1: i32 = 32772;
// Java field: int HASH_SHA512
pub const HASH_SHA512: i32 = 32782;
// Java field: int PROVIDER_RC4
pub const PROVIDER_RC4: i32 = 1;
// Java field: int PROVIDER_AES
pub const PROVIDER_AES: i32 = 24;
// Java field: int MODE_ECB
pub const MODE_ECB: i32 = 1;
// Java field: int MODE_CBC
pub const MODE_CBC: i32 = 2;
// Java field: int MODE_CFB
pub const MODE_CFB: i32 = 3;

// Java: public class EncryptionHeader
pub struct EncryptionHeader {
    // Java field: int flags
    pub flags: i32,
    // Java field: int sizeExtra
    pub sizeExtra: i32,
    // Java field: int algorithm
    pub algorithm: i32,
    // Java field: int hashAlgorithm
    pub hashAlgorithm: i32,
    // Java field: int keySize
    pub keySize: i32,
    // Java field: int providerType
    pub providerType: i32,
    // Java field: int cipherMode
    pub cipherMode: i32,
    // Java field: byte[] keySalt
    pub keySalt: Option<Vec<u8>>,
    // Java field: String cspName
    pub cspName: Option<String>,
}

// Java: public class EncryptionHeader
impl EncryptionHeader {
    // Java: EncryptionHeader(DocumentInputStream is)
    pub fn new_from_stream(_is: &mut dyn std::any::Any) -> Self {
      // Java: super();
      // Java: this.flags = is.readInt();
      // Java: this.sizeExtra = is.readInt();
      // Java: this.algorithm = is.readInt();
      // Java: this.hashAlgorithm = is.readInt();
      // Java: this.keySize = is.readInt();
      // Java: this.providerType = is.readInt();
      // Java: is.readLong();
      // Java: StringBuilder builder = new StringBuilder();
      // Java: while(true)
      {
         // Java: char c = (char)is.readShort();
         // Java: if (c == 0)
         {
            // Java: this.cspName = builder.toString();
            // Java: this.cipherMode = 1;
            // Java: this.keySalt = null;
            // Java: return;
         }
         // Java: builder.append(c);
      }
      todo!()
    }

    // Java: EncryptionHeader(String descriptor)
    pub fn new_from_descriptor(_descriptor: &str) -> Self {
      // Java: super();
      // Java: NamedNodeMap keyData;
      // Java: try
      {
         // Java: ByteArrayInputStream is = new ByteArrayInputStream(descriptor.getBytes(StandardCharsets.UTF_8));
         // Java: keyData = DocumentBuilderFactory.newInstance().newDocumentBuilder().parse(is).getElementsByTagName("keyData").item(0).getAttributes();
      }
      // Java: catch (Exception var10)
      {
         // Java: throw new EncryptedDocumentException("Unable to parse keyData");
      }
      // Java: this.keySize = Integer.parseInt(keyData.getNamedItem("keyBits").getNodeValue());
      // Java: this.flags = 0;
      // Java: this.sizeExtra = 0;
      // Java: this.cspName = null;
      // Java: int blockSize = Integer.parseInt(keyData.getNamedItem("blockSize").getNodeValue());
      // Java: String cipher = keyData.getNamedItem("cipherAlgorithm").getNodeValue();
      // Java: if ("AES".equals(cipher))
      {
         // Java: this.providerType = 24;
         // Java: if (blockSize == 16)
         {
            // Java: this.algorithm = 26126;
         }
         // Java: else if (blockSize == 24)
         {
            // Java: this.algorithm = 26127;
         }
         // Java: else
         {
            // Java: if (blockSize != 32)
            {
               // Java: throw new EncryptedDocumentException("Unsupported key length");
            }
            // Java: this.algorithm = 26128;
         }
         // Java: String chaining = keyData.getNamedItem("cipherChaining").getNodeValue();
         // Java: if ("ChainingModeCBC".equals(chaining))
         {
            // Java: this.cipherMode = 2;
         }
         // Java: else
         {
            // Java: if (!"ChainingModeCFB".equals(chaining))
            {
               // Java: throw new EncryptedDocumentException("Unsupported chaining mode");
            }
            // Java: this.cipherMode = 3;
         }
         // Java: String hashAlg = keyData.getNamedItem("hashAlgorithm").getNodeValue();
         // Java: int hashSize = Integer.parseInt(keyData.getNamedItem("hashSize").getNodeValue());
         // Java: if ("SHA1".equals(hashAlg) && hashSize == 20)
         {
            // Java: this.hashAlgorithm = 32772;
         }
         // Java: else
         {
            // Java: if (!"SHA512".equals(hashAlg) || hashSize != 64)
            {
               // Java: throw new EncryptedDocumentException("Unsupported hash algorithm");
            }
            // Java: this.hashAlgorithm = 32782;
         }
         // Java: String salt = keyData.getNamedItem("saltValue").getNodeValue();
         // Java: int saltLength = Integer.parseInt(keyData.getNamedItem("saltSize").getNodeValue());
         // Java: this.keySalt = Base64.decode(salt.getBytes(StandardCharsets.UTF_8));
         // Java: if (this.keySalt.length != saltLength)
         {
            // Java: throw new EncryptedDocumentException("Invalid salt length");
         }
      }
      // Java: else
      {
         // Java: throw new EncryptedDocumentException("Unsupported cipher");
      }
      todo!()
    }

    // Java: EncryptionHeader(EncryptionInfoInit descriptor)
    pub fn new_from_init(_descriptor: &EncryptionInfoInit) -> Self {
      // Java: super();
      // Java: this.keySize = (Integer)descriptor.getNamedItem("keyBits");
      // Java: this.flags = 0;
      // Java: this.sizeExtra = 0;
      // Java: this.cspName = null;
      // Java: int blockSize = (Integer)descriptor.getNamedItem("blockSize");
      // Java: Integer cipher = (Integer)descriptor.getNamedItem("cipherAlgorithm");
      // Java: this.providerType = 24;
      // Java: if (cipher == 1)
      {
         // Java: this.algorithm = 26126;
      }
      // Java: else if (cipher == 2)
      {
         // Java: this.algorithm = 26127;
      }
      // Java: else
      {
         // Java: if (cipher != 4)
         {
            // Java: throw new EncryptedDocumentException("Unsupported key length");
         }
         // Java: this.algorithm = 26128;
      }
      // Java: this.cipherMode = (Integer)descriptor.getNamedItem("cipherChaining");
      // Java: Integer hashAlg = (Integer)descriptor.getNamedItem("hashAlgorithm");
      // Java: int hashSize = (Integer)descriptor.getNamedItem("hashSize");
      // Java: if (hashAlg == 4096 && hashSize == 20)
      {
         // Java: this.hashAlgorithm = 32772;
      }
      // Java: else
      {
         // Java: if (hashAlg != 8192 || hashSize != 64)
         {
            // Java: throw new EncryptedDocumentException("Unsupported hash algorithm");
         }
         // Java: this.hashAlgorithm = 32782;
      }
      // Java: byte[] salt = (byte[])descriptor.getNamedItem("saltValue");
      // Java: int saltLength = (Integer)descriptor.getNamedItem("saltSize");
      // Java: this.keySalt = salt;
      todo!()
    }

    // Java: int getCipherMode()
    pub fn getCipherMode(&self) -> i32 {
      // Java: return this.cipherMode;
      self.cipherMode
    }

    // Java: int getFlags()
    pub fn getFlags(&self) -> i32 {
      // Java: return this.flags;
      self.flags
    }

    // Java: int getSizeExtra()
    pub fn getSizeExtra(&self) -> i32 {
      // Java: return this.sizeExtra;
      self.sizeExtra
    }

    // Java: int getAlgorithm()
    pub fn getAlgorithm(&self) -> i32 {
      // Java: return this.algorithm;
      self.algorithm
    }

    // Java: int getHashAlgorithm()
    pub fn getHashAlgorithm(&self) -> i32 {
      // Java: return this.hashAlgorithm;
      self.hashAlgorithm
    }

    // Java: String getHashAlgorithmName()
    pub fn getHashAlgorithmName(&self) -> Option<String> {
      // Java: if (this.hashAlgorithm == 32772)
      if self.hashAlgorithm == HASH_SHA1 {
         // Java: return "SHA-1";
         return Some(String::from("SHA-1"));
      }
      // Java: else if (this.hashAlgorithm == 32782)
      else if self.hashAlgorithm == HASH_SHA512 {
         // Java: return "SHA-512";
         return Some(String::from("SHA-512"));
      }
      // Java: else
      {
         // Java: IOAssert.fail("NoSupportedHashAlgorithm!");
         panic!("NoSupportedHashAlgorithm!");
         // Java: return null;
      }
    }

    // Java: int getKeySize()
    pub fn getKeySize(&self) -> i32 {
      // Java: return this.keySize;
      self.keySize
    }

    // Java: byte[] getKeySalt()
    pub fn getKeySalt(&self) -> &Option<Vec<u8>> {
      // Java: return this.keySalt;
      &self.keySalt
    }

    // Java: int getProviderType()
    pub fn getProviderType(&self) -> i32 {
      // Java: return this.providerType;
      self.providerType
    }

    // Java: String getCspName()
    pub fn getCspName(&self) -> &Option<String> {
      // Java: return this.cspName;
      &self.cspName
    }

}
