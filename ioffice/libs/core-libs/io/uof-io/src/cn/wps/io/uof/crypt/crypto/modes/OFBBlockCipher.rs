// Java: import cn.wps.io.uof.crypt.crypto.BlockCipher
use crate::cn::wps::io::uof::crypt::crypto::BlockCipher::BlockCipher;
// Java: import cn.wps.io.uof.crypt.crypto.CipherParameters
use crate::cn::wps::io::uof::crypt::crypto::CipherParameters::CipherParameters;
// Java: import cn.wps.io.uof.crypt.crypto.DataLengthException
// Java: import cn.wps.io.uof.crypt.crypto.StreamBlockCipher
// Java: import cn.wps.io.uof.crypt.crypto.params.ParametersWithIV
use crate::cn::wps::io::uof::crypt::crypto::params::ParametersWithIV::ParametersWithIV;

// Java: public class OFBBlockCipher extends StreamBlockCipher
pub struct OFBBlockCipher {
    // Java field: int mByteCount
    pub(crate) mByteCount: i32,
    // Java field: byte[] mIvArray
    pub(crate) mIvArray: Vec<u8>,
    // Java field: byte[] mOfbV
    pub(crate) mOfbV: Vec<u8>,
    // Java field: byte[] mOfbOutV
    pub(crate) mOfbOutV: Vec<u8>,
    // Java field: int mBlockSize
    pub(crate) mBlockSize: i32,
    // Java field: BlockCipher mCipher
    pub(crate) mCipher: Box<dyn BlockCipher>,
}

// Java: public class OFBBlockCipher extends StreamBlockCipher
impl OFBBlockCipher {
    // Java: OFBBlockCipher(BlockCipher var1, int var2)
    pub fn OFBBlockCipher(var1: Box<dyn BlockCipher>, var2: i32) -> Self {
      // Java: super(var1);
      let bs = var1.getBlockSize() as usize;
      // Java: this.mCipher = var1;
      // Java: this.mBlockSize = var2 / 8;
      // Java: this.mIvArray = new byte[var1.getBlockSize()];
      // Java: this.mOfbV = new byte[var1.getBlockSize()];
      // Java: this.mOfbOutV = new byte[var1.getBlockSize()];
      OFBBlockCipher {
          mByteCount: 0,
          mIvArray: vec![0u8; bs],
          mOfbV: vec![0u8; bs],
          mOfbOutV: vec![0u8; bs],
          mBlockSize: var2 / 8,
          mCipher: var1,
      }
    }

    // Java: void init(boolean var1, CipherParameters var2)
    pub fn init(&mut self, _var1: bool, var2: Option<&ParametersWithIV>, raw_params: Option<&dyn CipherParameters>) {
      // Java: if (var2 instanceof ParametersWithIV)
      if let Some(var3) = var2 {
         // Java: ParametersWithIV var3 = (ParametersWithIV)var2;
         // Java: byte[] var4 = var3.getIV();
         let var4 = var3.getIV();
         // Java: if (var4.length < this.mIvArray.length)
         if var4.len() < self.mIvArray.len() {
            // Java: System.arraycopy(var4, 0, this.mIvArray, this.mIvArray.length - var4.length, var4.length);
            let offset = self.mIvArray.len() - var4.len();
            self.mIvArray[offset..offset + var4.len()].copy_from_slice(var4);
            // Java: for(int var5 = 0; var5 < this.mIvArray.length - var4.length; ++var5)
            for var5 in 0..offset {
               // Java: this.mIvArray[var5] = 0;
               self.mIvArray[var5] = 0;
            }
         }
         // Java: else
         else {
            // Java: System.arraycopy(var4, 0, this.mIvArray, 0, this.mIvArray.length);
            let len = self.mIvArray.len();
            self.mIvArray[..len].copy_from_slice(&var4[..len]);
         }
         // Java: this.reset();
         self.reset();
         // Java: if (var3.getParameters() != null)
         {
            // Java: this.mCipher.init(true, var3.getParameters());
            self.mCipher.init(true, var3.getParameters());
         }
      }
      // Java: else
      else {
         // Java: this.reset();
         self.reset();
         // Java: if (var2 != null)
         if let Some(p) = raw_params {
            // Java: this.mCipher.init(true, var2);
            self.mCipher.init(true, p);
         }
      }
    }

    // Java: String getAlgorithmName()
    pub fn getAlgorithmName(&self) -> String {
      // Java: return this.mCipher.getAlgorithmName() + "/OFB" + this.mBlockSize * 8;
      format!("{}/OFB{}", self.mCipher.getAlgorithmName(), self.mBlockSize * 8)
    }

    // Java: int getBlockSize()
    pub fn getBlockSize(&self) -> i32 {
      // Java: return this.mBlockSize;
      self.mBlockSize
    }

    // Java: int processBlock(byte[] var1, int var2, byte[] var3, int var4)
    pub fn processBlock(&mut self, var1: &[u8], var2: i32, var3: &mut [u8], var4: i32) -> i32 {
      // Java: this.processBytes(var1, var2, this.mBlockSize, var3, var4);
      let bs = self.mBlockSize;
      for i in 0..bs {
          var3[(var4 + i) as usize] = self.calculateByte(var1[(var2 + i) as usize]);
      }
      // Java: return this.mBlockSize;
      self.mBlockSize
    }

    // Java: void reset()
    pub fn reset(&mut self) {
      // Java: System.arraycopy(this.mIvArray, 0, this.mOfbV, 0, this.mIvArray.length);
      let len = self.mIvArray.len();
      self.mOfbV[..len].copy_from_slice(&self.mIvArray[..len]);
      // Java: this.mByteCount = 0;
      self.mByteCount = 0;
      // Java: this.mCipher.reset();
      self.mCipher.reset();
    }

    // Java: byte calculateByte(byte var1)
    pub fn calculateByte(&mut self, var1: u8) -> u8 {
      // Java: if (this.mByteCount == 0)
      if self.mByteCount == 0 {
         // Java: this.mCipher.processBlock(this.mOfbV, 0, this.mOfbOutV, 0);
         let ofb_v = self.mOfbV.clone();
         self.mCipher.processBlock(&ofb_v, 0, &mut self.mOfbOutV, 0);
      }
      // Java: byte var2 = (byte)(this.mOfbOutV[this.mByteCount++] ^ var1);
      let var2 = self.mOfbOutV[self.mByteCount as usize] ^ var1;
      self.mByteCount += 1;
      // Java: if (this.mByteCount == this.mBlockSize)
      if self.mByteCount == self.mBlockSize {
         // Java: this.mByteCount = 0;
         self.mByteCount = 0;
         // Java: System.arraycopy(this.mOfbV, this.mBlockSize, this.mOfbV, 0, this.mOfbV.length - this.mBlockSize);
         let bs = self.mBlockSize as usize;
         let vlen = self.mOfbV.len();
         self.mOfbV.copy_within(bs..vlen, 0);
         // Java: System.arraycopy(this.mOfbOutV, 0, this.mOfbV, this.mOfbV.length - this.mBlockSize, this.mBlockSize);
         let dest_start = vlen - bs;
         let src = self.mOfbOutV[..bs].to_vec();
         self.mOfbV[dest_start..dest_start + bs].copy_from_slice(&src);
      }
      // Java: return var2;
      var2
    }

}

impl BlockCipher for OFBBlockCipher {
    fn init(&mut self, var1: bool, var2: &dyn CipherParameters) {
        let pwiv = var2.as_any().downcast_ref::<crate::cn::wps::io::uof::crypt::crypto::params::ParametersWithIV::ParametersWithIV>();
        self.init(var1, pwiv, Some(var2));
    }
    fn getAlgorithmName(&self) -> String { self.getAlgorithmName() }
    fn getBlockSize(&self) -> i32 { self.getBlockSize() }
    fn processBlock(&mut self, var1: &[u8], var2: i32, var3: &mut [u8], var4: i32) -> i32 {
        self.processBlock(var1, var2, var3, var4)
    }
    fn reset(&mut self) { self.reset() }
}
