#![allow(non_snake_case, dead_code, unused_variables, non_upper_case_globals, unreachable_code)]

// Java: import cn.wps.encrypt.EncryptionInfoInit
use crate::cn::wps::encrypt::EncryptionInfoInit::EncryptionInfoInit;
// Java: import java.io.ByteArrayInputStream
// Java: import java.nio.charset.StandardCharsets
// Java: import javax.xml.parsers.DocumentBuilderFactory
// Java: import org.apache.poi.EncryptedDocumentException
// use crate::org::apache::poi::EncryptedDocumentException;
// Java: import org.apache.poi.poifs.filesystem.DocumentInputStream
// use crate::org::apache::poi::poifs::filesystem::DocumentInputStream;
// Java: import org.apache.poi.util.base64.Base64
// Java: import org.w3c.dom.NamedNodeMap
// Java: import org.w3c.dom.Node
// Java: import org.w3c.dom.NodeList

// Java: public class EncryptionVerifier
pub struct EncryptionVerifier {
    // Java field: byte[] salt
    pub salt: Option<Vec<u8>>,
    // Java field: byte[] verifier
    pub verifier: Option<Vec<u8>>,
    // Java field: byte[] verifierHash
    pub verifierHash: Option<Vec<u8>>,
    // Java field: byte[] encryptedKey
    pub encryptedKey: Option<Vec<u8>>,
    // Java field: int verifierHashSize
    pub verifierHashSize: i32,
    // Java field: int spinCount
    pub spinCount: i32,
    // Java field: int algorithm
    pub algorithm: i32,
    // Java field: int cipherMode
    pub cipherMode: i32,
}

// Java: public class EncryptionVerifier
impl EncryptionVerifier {
    // Java: EncryptionVerifier()
    pub fn new() -> Self {
      // Java: super();
      EncryptionVerifier {
          salt: None,
          verifier: None,
          verifierHash: None,
          encryptedKey: None,
          verifierHashSize: 0,
          spinCount: 0,
          algorithm: 0,
          cipherMode: 0,
      }
    }

    // Java: EncryptionVerifier(String descriptor)
    pub fn new_from_descriptor(_descriptor: &str) -> Self {
      // Java: super();
      // Java: NamedNodeMap keyData = null;
      // Java: try
      {
         // Java: ByteArrayInputStream is = new ByteArrayInputStream(descriptor.getBytes(StandardCharsets.UTF_8));
         // Java: NodeList keyEncryptor = DocumentBuilderFactory.newInstance().newDocumentBuilder().parse(is).getElementsByTagName("keyEncryptor").item(0).getChildNodes();
         // Java: for(int i = 0; i < keyEncryptor.getLength(); ++i)
         {
            // Java: Node node = keyEncryptor.item(i);
            // Java: if (node.getNodeName().equals("p:encryptedKey"))
            {
               // Java: keyData = node.getAttributes();
               // Java: break;
            }
         }
         // Java: if (keyData == null)
         {
            // Java: throw new EncryptedDocumentException("");
         }
      }
      // Java: catch (Exception var7)
      {
         // Java: throw new EncryptedDocumentException("Unable to parse keyEncryptor");
      }
      // Java: this.spinCount = Integer.parseInt(keyData.getNamedItem("spinCount").getNodeValue());
      // Java: this.verifier = Base64.decode(keyData.getNamedItem("encryptedVerifierHashInput").getNodeValue().getBytes(StandardCharsets.UTF_8));
      // Java: this.salt = Base64.decode(keyData.getNamedItem("saltValue").getNodeValue().getBytes(StandardCharsets.UTF_8));
      // Java: this.encryptedKey = Base64.decode(keyData.getNamedItem("encryptedKeyValue").getNodeValue().getBytes(StandardCharsets.UTF_8));
      // Java: int saltSize = Integer.parseInt(keyData.getNamedItem("saltSize").getNodeValue());
      // Java: if (saltSize != this.salt.length)
      {
         // Java: throw new EncryptedDocumentException("Invalid salt size");
      }
      // Java: else
      {
         // Java: this.verifierHash = Base64.decode(keyData.getNamedItem("encryptedVerifierHashValue").getNodeValue().getBytes(StandardCharsets.UTF_8));
         // Java: int blockSize = Integer.parseInt(keyData.getNamedItem("blockSize").getNodeValue());
         // Java: String alg = keyData.getNamedItem("cipherAlgorithm").getNodeValue();
         // Java: if ("AES".equals(alg))
         {
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
                  // Java: throw new EncryptedDocumentException("Unsupported block size");
               }
               // Java: this.algorithm = 26128;
            }
            // Java: String chain = keyData.getNamedItem("cipherChaining").getNodeValue();
            // Java: if ("ChainingModeCBC".equals(chain))
            {
               // Java: this.cipherMode = 2;
            }
            // Java: else
            {
               // Java: if (!"ChainingModeCFB".equals(chain))
               {
                  // Java: throw new EncryptedDocumentException("Unsupported chaining mode");
               }
               // Java: this.cipherMode = 3;
            }
            // Java: this.verifierHashSize = Integer.parseInt(keyData.getNamedItem("hashSize").getNodeValue());
         }
         // Java: else
         {
            // Java: throw new EncryptedDocumentException("Unsupported cipher");
         }
      }
      todo!()
    }

    // Java: EncryptionVerifier(EncryptionInfoInit keyData)
    pub fn new_from_init(_keyData: &EncryptionInfoInit) -> Self {
      // Java: super();
      // Java: this.spinCount = (Integer)keyData.getNamedItem("spinCount");
      // Java: this.verifier = (byte[])keyData.getNamedItem("encryptedVerifierHashInput");
      // Java: this.salt = (byte[])keyData.getNamedItem("verifySaltValue");
      // Java: this.encryptedKey = (byte[])keyData.getNamedItem("encryptedKeyValue");
      // Java: int saltSize = (Integer)keyData.getNamedItem("verifySaltSize");
      // Java: if (saltSize != this.salt.length)
      {
         // Java: throw new EncryptedDocumentException("Invalid salt size");
      }
      // Java: else
      {
         // Java: this.verifierHash = (byte[])keyData.getNamedItem("encryptedVerifierHashValue");
         // Java: int blockSize = (Integer)keyData.getNamedItem("blockSize");
         // Java: Integer alg = (Integer)keyData.getNamedItem("cipherAlgorithm");
         // Java: if (alg == 1)
         {
            // Java: this.algorithm = 26126;
         }
         // Java: else if (alg == 2)
         {
            // Java: this.algorithm = 26127;
         }
         // Java: else
         {
            // Java: if (alg != 4)
            {
               // Java: throw new EncryptedDocumentException("Unsupported block size");
            }
            // Java: this.algorithm = 26128;
         }
         // Java: this.cipherMode = (Integer)keyData.getNamedItem("cipherChaining");
         // Java: this.verifierHashSize = (Integer)keyData.getNamedItem("hashSize");
      }
      todo!()
    }

    // Java: EncryptionVerifier(DocumentInputStream is, int encryptedLength)
    pub fn new_from_stream(_is: &mut dyn std::any::Any, encryptedLength: i32) -> Self {
      // Java: super();
      // Java: int saltSize = is.readInt();
      // Java: if (saltSize != 16)
      {
         // Java: throw new RuntimeException("Salt size != 16 !?");
      }
      // Java: else
      {
         // Java: this.salt = new byte[16];
         // Java: is.readFully(this.salt);
         // Java: this.verifier = new byte[16];
         // Java: is.readFully(this.verifier);
         // Java: this.verifierHashSize = is.readInt();
         // Java: this.verifierHash = new byte[encryptedLength];
         // Java: is.readFully(this.verifierHash);
         // Java: this.spinCount = 50000;
         // Java: this.algorithm = 26126;
         // Java: this.cipherMode = 1;
         // Java: this.encryptedKey = null;
      }
      todo!()
    }

    // Java: byte[] getSalt()
    pub fn getSalt(&self) -> &Option<Vec<u8>> {
      // Java: return this.salt;
      &self.salt
    }

    // Java: byte[] getVerifier()
    pub fn getVerifier(&self) -> &Option<Vec<u8>> {
      // Java: return this.verifier;
      &self.verifier
    }

    // Java: byte[] getVerifierHash()
    pub fn getVerifierHash(&self) -> &Option<Vec<u8>> {
      // Java: return this.verifierHash;
      &self.verifierHash
    }

    // Java: int getSpinCount()
    pub fn getSpinCount(&self) -> i32 {
      // Java: return this.spinCount;
      self.spinCount
    }

    // Java: int getCipherMode()
    pub fn getCipherMode(&self) -> i32 {
      // Java: return this.cipherMode;
      self.cipherMode
    }

    // Java: int getAlgorithm()
    pub fn getAlgorithm(&self) -> i32 {
      // Java: return this.algorithm;
      self.algorithm
    }

    // Java: byte[] getEncryptedKey()
    pub fn getEncryptedKey(&self) -> &Option<Vec<u8>> {
      // Java: return this.encryptedKey;
      &self.encryptedKey
    }

}
