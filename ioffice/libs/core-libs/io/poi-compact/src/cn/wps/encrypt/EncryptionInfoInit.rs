#![allow(non_snake_case, dead_code, unused_variables, non_upper_case_globals, unreachable_code)]

// Java: import java.io.IOException
// Java: import java.util.HashMap
use std::collections::HashMap;
// Java: import org.apache.poi.poifs.filesystem.DocumentInputStream
// use crate::org::apache::poi::poifs::filesystem::DocumentInputStream;

// Java: public class EncryptionInfoInit
pub struct EncryptionInfoInit {
    // Java field: HashMap<String, Object> mKeyValue
    mKeyValue: HashMap<String, Box<dyn std::any::Any>>,
    // Java field: int keyBits
    keyBits: i32,
    // Java field: int blockSize
    blockSize: i32,
    // Java field: int cipherChaining
    cipherChaining: i32,
    // Java field: int cipherAlgorithm
    cipherAlgorithm: i32,
    // Java field: int hashSize
    hashSize: i32,
    // Java field: int hashAlgorithm
    hashAlgorithm: i32,
    // Java field: int spinCount
    spinCount: i32,
    // Java field: byte[] saltValue
    saltValue: Vec<u8>,
    // Java field: byte[] verifySaltValue
    verifySaltValue: Vec<u8>,
    // Java field: byte[] encryptedKeyValue
    encryptedKeyValue: Vec<u8>,
    // Java field: byte[] encryptedVerifierHashValue
    encryptedVerifierHashValue: Vec<u8>,
    // Java field: byte[] encryptedVerifierHashInput
    encryptedVerifierHashInput: Vec<u8>,
}

// Java: public class EncryptionInfoInit
impl EncryptionInfoInit {
    // Java: EncryptionInfoInit()
    pub fn new() -> Self {
      // Java: super();
      EncryptionInfoInit {
          mKeyValue: HashMap::new(),
          // Java: this.saltValue = new byte[]{51, -119, -23, 16, -48, 37, -31, 65, -8, -107, -79, -76, 25, -117, 109, 87};
          saltValue: vec![51, 137, 233, 16, 208, 37, 225, 65, 248, 149, 177, 180, 25, 139, 109, 87],
          // Java: this.verifySaltValue = new byte[]{-71, -56, -57, 34, 90, -78, 45, -18, -3, -28, 76, 104, -128, -68, 52, -40};
          verifySaltValue: vec![185, 200, 199, 34, 90, 178, 45, 238, 253, 228, 76, 104, 128, 188, 52, 216],
          // Java: this.hashSize = 20;
          hashSize: 20,
          // Java: this.hashAlgorithm = 4096;
          hashAlgorithm: 4096,
          // Java: this.spinCount = 100000;
          spinCount: 100000,
          // Java: this.cipherAlgorithm = 1;
          cipherAlgorithm: 1,
          // Java: this.cipherChaining = 2;
          cipherChaining: 2,
          // Java: this.keyBits = 128;
          keyBits: 128,
          // Java: this.blockSize = 16;
          blockSize: 16,
          // Java: this.encryptedVerifierHashInput = new byte[]{37, 47, -107, -41, 74, -5, 115, -55, 0, -11, 60, -76, 47, -10, -124, 4};
          encryptedVerifierHashInput: vec![37, 47, 149, 215, 74, 251, 115, 201, 0, 245, 60, 180, 47, 246, 132, 4],
          // Java: this.encryptedVerifierHashValue = new byte[]{53, 105, 13, 49, 1, 36, -12, -62, -80, -112, 99, -73, 40, -19, 7, 40, 83, 12, -16, 116, -80, 70, 125, -81, -14, -26, -99, 119, -124, -51, 125, -51};
          encryptedVerifierHashValue: vec![53, 105, 13, 49, 1, 36, 244, 194, 176, 144, 99, 183, 40, 237, 7, 40, 83, 12, 240, 116, 176, 70, 125, 175, 242, 230, 157, 119, 132, 205, 125, 205],
          // Java: this.encryptedKeyValue = new byte[]{-72, -68, 101, 86, 73, -79, -48, -60, 25, -116, 28, -123, 72, -2, 30, -24};
          encryptedKeyValue: vec![184, 188, 101, 86, 73, 177, 208, 196, 25, 140, 28, 133, 72, 254, 30, 232],
      }
    }

    // Java: EncryptionInfoInit(DocumentInputStream is)
    pub fn new_from_stream(_is: &mut dyn std::any::Any) -> Self {
      // Java: super();
      // Java: int saltSize = is.readInt();
      // Java: this.saltValue = new byte[saltSize];
      // Java: is.read(this.saltValue);
      // Java: this.mKeyValue.put("saltSize", saltSize);
      // Java: this.mKeyValue.put("saltValue", this.saltValue);
      // Java: int verifySaltSize = is.readInt();
      // Java: this.verifySaltValue = new byte[verifySaltSize];
      // Java: is.read(this.verifySaltValue);
      // Java: this.mKeyValue.put("verifySaltSize", verifySaltSize);
      // Java: this.mKeyValue.put("verifySaltValue", this.verifySaltValue);
      // Java: this.hashSize = is.readInt();
      // Java: this.mKeyValue.put("hashSize", this.hashSize);
      // Java: this.hashAlgorithm = is.readInt();
      // Java: this.mKeyValue.put("hashAlgorithm", this.hashAlgorithm);
      // Java: this.spinCount = is.readInt();
      // Java: this.mKeyValue.put("spinCount", this.spinCount);
      // Java: this.cipherAlgorithm = is.readInt();
      // Java: this.mKeyValue.put("cipherAlgorithm", this.cipherAlgorithm);
      // Java: this.cipherChaining = is.readInt();
      // Java: this.mKeyValue.put("cipherChaining", this.cipherChaining);
      // Java: this.keyBits = is.readInt();
      // Java: this.mKeyValue.put("keyBits", this.keyBits);
      // Java: this.blockSize = is.readInt();
      // Java: this.mKeyValue.put("blockSize", this.blockSize);
      // Java: int encryptedVerifierHashInputSize = is.readInt();
      // Java: this.encryptedVerifierHashInput = new byte[encryptedVerifierHashInputSize];
      // Java: is.read(this.encryptedVerifierHashInput);
      // Java: this.mKeyValue.put("encryptedVerifierHashInput", this.encryptedVerifierHashInput);
      // Java: int encryptedVerifierHashValueSize = is.readInt();
      // Java: this.encryptedVerifierHashValue = new byte[encryptedVerifierHashValueSize];
      // Java: is.read(this.encryptedVerifierHashValue);
      // Java: this.mKeyValue.put("encryptedVerifierHashValue", this.encryptedVerifierHashValue);
      // Java: int encryptedKeyValueSize = is.readInt();
      // Java: this.encryptedKeyValue = new byte[encryptedKeyValueSize];
      // Java: is.read(this.encryptedKeyValue);
      // Java: this.mKeyValue.put("encryptedKeyValue", this.encryptedKeyValue);
      todo!()
    }

    // Java: Object getNamedItem(String key)
    pub fn getNamedItem(&self, key: &str) -> Option<&Box<dyn std::any::Any>> {
      // Java: return this.mKeyValue.get(key);
      self.mKeyValue.get(key)
    }

    // Java: void putNamedItem(String key, Object value)
    pub fn putNamedItem(&mut self, key: &str, value: Box<dyn std::any::Any>) {
      // Java: this.mKeyValue.put(key, value);
      self.mKeyValue.insert(key.to_string(), value);
    }

}

impl crate::cn::wps::encrypt::EncryptionHeader::EncryptionInfoInitTrait for EncryptionInfoInit {
    fn putNamedItem_int(&mut self, key: &str, value: i32) {
        self.mKeyValue.insert(key.to_string(), Box::new(value));
    }

    fn putNamedItem_bytes(&mut self, key: &str, value: &[u8]) {
        self.mKeyValue.insert(key.to_string(), Box::new(value.to_vec()));
    }
}
