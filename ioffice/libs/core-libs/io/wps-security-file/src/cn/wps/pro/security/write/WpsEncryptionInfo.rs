#![allow(non_snake_case, dead_code, unused_variables, non_upper_case_globals, unreachable_code)]

// Java: import cn.wps.base.assertion.Assert
// use util::cn::wps::base::assertion::Assert;
// Java: import cn.wps.encrypt.EncryptionInfoInit
// use io_poi_compact::cn::wps::encrypt::EncryptionInfoInit;
// Java: import java.io.ByteArrayInputStream
use std::io::Cursor;
// Java: import java.io.IOException
// Java: import org.apache.poi.util.LittleEndianByteArrayOutputStream
// use util::org::apache::poi::util::LittleEndianByteArrayOutputStream;

// Java: public class WpsEncryptionInfo
pub struct WpsEncryptionInfo {
    // Java field: LittleEndianByteArrayOutputStream outputStream
    outputStream: Option<Box<dyn std::any::Any>>,
    // Java field: byte[] data
    data: Option<Vec<u8>>,
    // Java field: int versionMajor
    versionMajor: i32,
    // Java field: int versionMinor
    versionMinor: i32,
    // Java field: int encryptionFlags
    encryptionFlags: i32,
    // Java field: int saltSize
    saltSize: i32,
    // Java field: int verifySaltSize
    verifySaltSize: i32,
    // Java field: int encryptedVerifierHashInputSize
    encryptedVerifierHashInputSize: i32,
    // Java field: int encryptedVerifierHashValueSize
    encryptedVerifierHashValueSize: i32,
    // Java field: int encryptedKeyValueSize
    encryptedKeyValueSize: i32,
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
    saltValue: Option<Vec<u8>>,
    // Java field: byte[] verifySaltValue
    verifySaltValue: Option<Vec<u8>>,
    // Java field: byte[] encryptedKeyValue
    encryptedKeyValue: Option<Vec<u8>>,
    // Java field: byte[] encryptedVerifierHashValue
    encryptedVerifierHashValue: Option<Vec<u8>>,
    // Java field: byte[] encryptedVerifierHashInput
    encryptedVerifierHashInput: Option<Vec<u8>>,
}

// Java: public class WpsEncryptionInfo
impl WpsEncryptionInfo {
    // Java: WpsEncryptionInfo(EncryptionInfoInit value)
    pub fn new(_value: &dyn std::any::Any) -> Self {
      // Java: super();
      // Java: this.init(value);
      // Java: this.data = new byte[this.getSize()];
      // Java: this.outputStream = new LittleEndianByteArrayOutputStream(this.data, 0);
      // Java: this.write();
      todo!()
    }

    // Java: void write()
    fn write(&mut self) {
      // Java: this.outputStream.writeShort(this.versionMajor);
      // Java: this.outputStream.writeShort(this.versionMinor);
      // Java: this.outputStream.writeInt(this.encryptionFlags);
      // Java: this.writeHeader();
      todo!()
    }

    // Java: void writeHeader()
    fn writeHeader(&mut self) {
      // Java: this.outputStream.writeInt(this.saltSize);
      // Java: if (null != this.saltValue)
      if self.saltValue.is_some() {
         // Java: this.outputStream.write(this.saltValue);
      }
      // Java: this.outputStream.writeInt(this.verifySaltSize);
      // Java: this.outputStream.write(this.verifySaltValue);
      // Java: this.outputStream.writeInt(this.hashSize);
      // Java: this.outputStream.writeInt(this.hashAlgorithm);
      // Java: this.outputStream.writeInt(this.spinCount);
      // Java: this.outputStream.writeInt(this.cipherAlgorithm);
      // Java: this.outputStream.writeInt(this.cipherChaining);
      // Java: this.outputStream.writeInt(this.keyBits);
      // Java: this.outputStream.writeInt(this.blockSize);
      // Java: this.outputStream.writeInt(this.encryptedVerifierHashInputSize);
      // Java: this.outputStream.write(this.encryptedVerifierHashInput);
      // Java: this.outputStream.writeInt(this.encryptedVerifierHashValueSize);
      // Java: this.outputStream.write(this.encryptedVerifierHashValue);
      // Java: this.outputStream.writeInt(this.encryptedKeyValueSize);
      // Java: if (null != this.encryptedKeyValue)
      if self.encryptedKeyValue.is_some() {
         // Java: this.outputStream.write(this.encryptedKeyValue);
      }
      todo!()
    }

    // Java: int getSize()
    fn getSize(&self) -> i32 {
      // Java: return 8 + this.getHeaderSize();
      8 + self.getHeaderSize()
    }

    // Java: int getHeaderSize()
    fn getHeaderSize(&self) -> i32 {
      // Java: return 4 + this.saltSize + 4 + this.verifySaltSize + 4 + 4 + 4 + 4 + 4 + 4 + 4 + 4 + this.encryptedVerifierHashInputSize + 4 + this.encryptedVerifierHashValueSize + 4 + this.encryptedKeyValueSize;
      4 + self.saltSize + 4 + self.verifySaltSize + 4 + 4 + 4 + 4 + 4 + 4 + 4 + 4 + self.encryptedVerifierHashInputSize + 4 + self.encryptedVerifierHashValueSize + 4 + self.encryptedKeyValueSize
    }

    // Java: void init(EncryptionInfoInit value)
    fn init_from_value(&mut self, _value: &dyn std::any::Any) {
      // Java: this.versionMajor = (Integer)value.getNamedItem("versionMajor");
      // Java: this.versionMinor = (Integer)value.getNamedItem("versionMinor");
      // Java: this.encryptionFlags = (Integer)value.getNamedItem("encryptionFlags");
      // Java: this.saltValue = (byte[])value.getNamedItem("saltValue");
      // Java: this.saltSize = null == this.saltValue ? 0 : this.saltValue.length;
      // Java: this.verifySaltValue = (byte[])value.getNamedItem("verifySaltValue");
      // Java: this.verifySaltSize = null == this.verifySaltValue ? 0 : this.verifySaltValue.length;
      // Java: Assert.assertEquals(this.verifySaltSize, this.verifySaltValue.length);
      // Java: this.hashSize = 20;
      self.hashSize = 20;
      // Java: this.hashAlgorithm = 4096;
      self.hashAlgorithm = 4096;
      // Java: this.spinCount = 100000;
      self.spinCount = 100000;
      // Java: this.cipherAlgorithm = 1;
      self.cipherAlgorithm = 1;
      // Java: this.cipherChaining = 2;
      self.cipherChaining = 2;
      // Java: this.keyBits = 128;
      self.keyBits = 128;
      // Java: this.blockSize = 16;
      self.blockSize = 16;
      // Java: this.encryptedVerifierHashInput = (byte[])value.getNamedItem("encryptedVerifierHashInput");
      // Java: this.encryptedVerifierHashInputSize = null == this.encryptedVerifierHashInput ? 0 : this.encryptedVerifierHashInput.length;
      // Java: Assert.assertEquals(this.encryptedVerifierHashInputSize, this.encryptedVerifierHashInput.length);
      // Java: this.encryptedVerifierHashValue = (byte[])value.getNamedItem("encryptedVerifierHashValue");
      // Java: this.encryptedVerifierHashValueSize = null == this.encryptedVerifierHashValue ? 0 : this.encryptedVerifierHashValue.length;
      // Java: Assert.assertEquals(this.encryptedVerifierHashValueSize, this.encryptedVerifierHashValue.length);
      // Java: this.encryptedKeyValue = (byte[])value.getNamedItem("encryptedKeyValue");
      // Java: this.encryptedKeyValueSize = null == this.encryptedKeyValue ? 0 : this.encryptedKeyValue.length;
      todo!()
    }

    // Java: void init()
    fn init(&mut self) {
      // Java: this.versionMajor = 4;
      self.versionMajor = 4;
      // Java: this.versionMinor = 4;
      self.versionMinor = 4;
      // Java: this.encryptionFlags = 64;
      self.encryptionFlags = 64;
      // Java: this.saltSize = 16;
      self.saltSize = 16;
      // Java: this.saltValue = new byte[]{51, -119, -23, 16, -48, 37, -31, 65, -8, -107, -79, -76, 25, -117, 109, 87};
      self.saltValue = Some(vec![51, 137, 233, 16, 208, 37, 225, 65, 248, 149, 177, 180, 25, 139, 109, 87]);
      // Java: this.verifySaltSize = 16;
      self.verifySaltSize = 16;
      // Java: this.verifySaltValue = new byte[]{-71, -56, -57, 34, 90, -78, 45, -18, -3, -28, 76, 104, -128, -68, 52, -40};
      self.verifySaltValue = Some(vec![185, 200, 199, 34, 90, 178, 45, 238, 253, 228, 76, 104, 128, 188, 52, 216]);
      // Java: this.hashSize = 20;
      self.hashSize = 20;
      // Java: this.hashAlgorithm = 4096;
      self.hashAlgorithm = 4096;
      // Java: this.spinCount = 100000;
      self.spinCount = 100000;
      // Java: this.cipherAlgorithm = 1;
      self.cipherAlgorithm = 1;
      // Java: this.cipherChaining = 2;
      self.cipherChaining = 2;
      // Java: this.keyBits = 128;
      self.keyBits = 128;
      // Java: this.blockSize = 16;
      self.blockSize = 16;
      // Java: this.encryptedVerifierHashInputSize = 16;
      self.encryptedVerifierHashInputSize = 16;
      // Java: this.encryptedVerifierHashInput = new byte[]{37, 47, -107, -41, 74, -5, 115, -55, 0, -11, 60, -76, 47, -10, -124, 4};
      self.encryptedVerifierHashInput = Some(vec![37, 47, 149, 215, 74, 251, 115, 201, 0, 245, 60, 180, 47, 246, 132, 4]);
      // Java: this.encryptedVerifierHashValueSize = 32;
      self.encryptedVerifierHashValueSize = 32;
      // Java: this.encryptedVerifierHashValue = new byte[]{53, 105, 13, 49, 1, 36, -12, -62, -80, -112, 99, -73, 40, -19, 7, 40, 83, 12, -16, 116, -80, 70, 125, -81, -14, -26, -99, 119, -124, -51, 125, -51};
      self.encryptedVerifierHashValue = Some(vec![53, 105, 13, 49, 1, 36, 244, 194, 176, 144, 99, 183, 40, 237, 7, 40, 83, 12, 240, 116, 176, 70, 125, 175, 242, 230, 157, 119, 132, 205, 125, 205]);
      // Java: this.encryptedKeyValueSize = 16;
      self.encryptedKeyValueSize = 16;
      // Java: this.encryptedKeyValue = new byte[]{-72, -68, 101, 86, 73, -79, -48, -60, 25, -116, 28, -123, 72, -2, 30, -24};
      self.encryptedKeyValue = Some(vec![184, 188, 101, 86, 73, 177, 208, 196, 25, 140, 28, 133, 72, 254, 30, 232]);
    }

    // Java: ByteArrayInputStream getInputStream()
    pub fn getInputStream(&self) -> Cursor<Vec<u8>> {
      // Java: return new ByteArrayInputStream(this.data);
      Cursor::new(self.data.clone().unwrap_or_default())
    }

}
