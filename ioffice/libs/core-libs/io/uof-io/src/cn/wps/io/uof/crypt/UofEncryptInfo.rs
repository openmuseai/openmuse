// Java: public class UofEncryptInfo
#[derive(Clone)]
pub struct UofEncryptInfo {
    // Java field: int mSaltSize
    pub(crate) mSaltSize: i32,
    // Java field: int mBlockSize
    pub(crate) mBlockSize: i32,
    // Java field: int mKeyBits
    pub(crate) mKeyBits: i32,
    // Java field: int mHashValSize
    pub(crate) mHashValSize: i32,
    // Java field: int mSpinCount
    pub(crate) mSpinCount: i32,
    // Java field: String mCipherAlgo
    pub(crate) mCipherAlgo: String,
    // Java field: String mChainMode
    pub(crate) mChainMode: String,
    // Java field: String mHashAlgo
    pub(crate) mHashAlgo: String,
    // Java field: byte[] mSaltValue
    pub(crate) mSaltValue: Vec<u8>,
    // Java field: byte[] mVerifiedSaltValue
    pub(crate) mVerifiedSaltValue: Vec<u8>,
    // Java field: byte[] mVerifiedEncPassword
    pub(crate) mVerifiedEncPassword: Vec<u8>,
    // Java field: byte[] mCryptedKey
    pub(crate) mCryptedKey: Vec<u8>,
    // Java field: byte[] mCryptedCRC
    pub(crate) mCryptedCRC: Vec<u8>,
    // Java field: byte[] mSessionKey
    pub(crate) mSessionKey: Vec<u8>,
    // Java field: String mCipherReference
    pub(crate) mCipherReference: String,
}

// Java: public class UofEncryptInfo
impl UofEncryptInfo {
    // Java: UofEncryptInfo()
    pub fn new() -> Self {
      // Java: super();
      UofEncryptInfo {
          mSaltSize: 0,
          mBlockSize: 0,
          mKeyBits: 0,
          mHashValSize: 0,
          mSpinCount: 0,
          mCipherAlgo: String::new(),
          mChainMode: String::new(),
          mHashAlgo: String::new(),
          mSaltValue: Vec::new(),
          mVerifiedSaltValue: Vec::new(),
          mVerifiedEncPassword: Vec::new(),
          mCryptedKey: Vec::new(),
          mCryptedCRC: Vec::new(),
          mSessionKey: Vec::new(),
          mCipherReference: String::new(),
      }
    }

    // Java: byte[] getSessionKey()
    pub fn getSessionKey(&self) -> &[u8] {
      // Java: return this.mSessionKey;
      &self.mSessionKey
    }

    // Java: void setSessionKey(byte[] mSessionKey)
    pub fn setSessionKey(&mut self, mSessionKey: Vec<u8>) {
      // Java: this.mSessionKey = mSessionKey;
      self.mSessionKey = mSessionKey;
    }

    // Java: int getSaltSize()
    pub fn getSaltSize(&self) -> i32 {
      // Java: return this.mSaltSize;
      self.mSaltSize
    }

    // Java: void setSaltSize(int mSaltSize)
    pub fn setSaltSize(&mut self, mSaltSize: i32) {
      // Java: this.mSaltSize = mSaltSize;
      self.mSaltSize = mSaltSize;
    }

    // Java: int getBlockSize()
    pub fn getBlockSize(&self) -> i32 {
      // Java: return this.mBlockSize;
      self.mBlockSize
    }

    // Java: void setBlockSize(int mBlockSize)
    pub fn setBlockSize(&mut self, mBlockSize: i32) {
      // Java: this.mBlockSize = mBlockSize;
      self.mBlockSize = mBlockSize;
    }

    // Java: void setCipherReference(String cipherReference)
    pub fn setCipherReference(&mut self, cipherReference: String) {
      // Java: this.mCipherReference = cipherReference;
      self.mCipherReference = cipherReference;
    }

    // Java: String getCipherReference()
    pub fn getCipherReference(&self) -> &str {
      // Java: return this.mCipherReference;
      &self.mCipherReference
    }

    // Java: int getKeyBits()
    pub fn getKeyBits(&self) -> i32 {
      // Java: return this.mKeyBits;
      self.mKeyBits
    }

    // Java: void setKeyBits(int keyBits)
    pub fn setKeyBits(&mut self, keyBits: i32) {
      // Java: this.mKeyBits = keyBits;
      self.mKeyBits = keyBits;
    }

    // Java: int getHashValSize()
    pub fn getHashValSize(&self) -> i32 {
      // Java: return this.mHashValSize;
      self.mHashValSize
    }

    // Java: void setHashValSize(int mHashValSize)
    pub fn setHashValSize(&mut self, mHashValSize: i32) {
      // Java: this.mHashValSize = mHashValSize;
      self.mHashValSize = mHashValSize;
    }

    // Java: int getSpinCount()
    pub fn getSpinCount(&self) -> i32 {
      // Java: return this.mSpinCount;
      self.mSpinCount
    }

    // Java: void setSpinCount(int mSpinCount)
    pub fn setSpinCount(&mut self, mSpinCount: i32) {
      // Java: this.mSpinCount = mSpinCount;
      self.mSpinCount = mSpinCount;
    }

    // Java: String getCipherAlgo()
    pub fn getCipherAlgo(&self) -> &str {
      // Java: return this.mCipherAlgo;
      &self.mCipherAlgo
    }

    // Java: void setCipherAlgo(String mCipherAlgo)
    pub fn setCipherAlgo(&mut self, mCipherAlgo: String) {
      // Java: this.mCipherAlgo = mCipherAlgo;
      self.mCipherAlgo = mCipherAlgo;
    }

    // Java: String getChainMode()
    pub fn getChainMode(&self) -> &str {
      // Java: return this.mChainMode;
      &self.mChainMode
    }

    // Java: void setChainMode(String mChainMode)
    pub fn setChainMode(&mut self, mChainMode: String) {
      // Java: this.mChainMode = mChainMode;
      self.mChainMode = mChainMode;
    }

    // Java: String getHashAlgo()
    pub fn getHashAlgo(&self) -> &str {
      // Java: return this.mHashAlgo;
      &self.mHashAlgo
    }

    // Java: void setHashAlgo(String mHashAlgo)
    pub fn setHashAlgo(&mut self, mHashAlgo: String) {
      // Java: this.mHashAlgo = mHashAlgo;
      self.mHashAlgo = mHashAlgo;
    }

    // Java: byte[] getSaltValue()
    pub fn getSaltValue(&self) -> &[u8] {
      // Java: return this.mSaltValue;
      &self.mSaltValue
    }

    // Java: void setSaltValue(byte[] mSaltValue)
    pub fn setSaltValue(&mut self, mSaltValue: Vec<u8>) {
      // Java: this.mSaltValue = mSaltValue;
      self.mSaltValue = mSaltValue;
    }

    // Java: byte[] getVerifiedSaltValue()
    pub fn getVerifiedSaltValue(&self) -> &[u8] {
      // Java: return this.mVerifiedSaltValue;
      &self.mVerifiedSaltValue
    }

    // Java: void setVerifiedSaltValue(byte[] mVerifiedSaltValue)
    pub fn setVerifiedSaltValue(&mut self, mVerifiedSaltValue: Vec<u8>) {
      // Java: this.mVerifiedSaltValue = mVerifiedSaltValue;
      self.mVerifiedSaltValue = mVerifiedSaltValue;
    }

    // Java: byte[] getVerifiedEncPassword()
    pub fn getVerifiedEncPassword(&self) -> &[u8] {
      // Java: return this.mVerifiedEncPassword;
      &self.mVerifiedEncPassword
    }

    // Java: void setVerifiedEncPassword(byte[] mVerifiedEncPassword)
    pub fn setVerifiedEncPassword(&mut self, mVerifiedEncPassword: Vec<u8>) {
      // Java: this.mVerifiedEncPassword = mVerifiedEncPassword;
      self.mVerifiedEncPassword = mVerifiedEncPassword;
    }

    // Java: byte[] getCryptedKey()
    pub fn getCryptedKey(&self) -> &[u8] {
      // Java: return this.mCryptedKey;
      &self.mCryptedKey
    }

    // Java: void setCryptedKey(byte[] mCryptedKey)
    pub fn setCryptedKey(&mut self, mCryptedKey: Vec<u8>) {
      // Java: this.mCryptedKey = mCryptedKey;
      self.mCryptedKey = mCryptedKey;
    }

    // Java: byte[] getCryptedCRC()
    pub fn getCryptedCRC(&self) -> &[u8] {
      // Java: return this.mCryptedCRC;
      &self.mCryptedCRC
    }

    // Java: void setCryptedCRC(byte[] mCryptedCRC)
    pub fn setCryptedCRC(&mut self, mCryptedCRC: Vec<u8>) {
      // Java: this.mCryptedCRC = mCryptedCRC;
      self.mCryptedCRC = mCryptedCRC;
    }

}
