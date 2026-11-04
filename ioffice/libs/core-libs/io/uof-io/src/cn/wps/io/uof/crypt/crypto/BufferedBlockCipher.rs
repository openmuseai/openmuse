use super::BlockCipher::BlockCipher;
use super::CipherParameters::CipherParameters;

// Java: public class BufferedBlockCipher
pub struct BufferedBlockCipher {
    // Java field: byte[] mBuf
    pub(crate) mBuf: Vec<u8>,
    // Java field: int mBufOff
    pub(crate) mBufOff: i32,
    // Java field: boolean mForEncryption
    pub(crate) mForEncryption: bool,
    // Java field: BlockCipher mCipher
    pub(crate) mCipher: Option<Box<dyn BlockCipher>>,
    // Java field: boolean mPartialBlockOkay
    pub(crate) mPartialBlockOkay: bool,
    // Java field: boolean mPgpCFB
    pub(crate) mPgpCFB: bool,
}

// Java: public class BufferedBlockCipher
impl BufferedBlockCipher {
    // Java: BufferedBlockCipher()
    pub fn BufferedBlockCipher() -> Self {
      // Java: super();
      BufferedBlockCipher {
          mBuf: Vec::new(),
          mBufOff: 0,
          mForEncryption: false,
          mCipher: None,
          mPartialBlockOkay: false,
          mPgpCFB: false,
      }
    }

    // Java: BufferedBlockCipher(BlockCipher cipher)
    pub fn BufferedBlockCipher_with_cipher(mut cipher: Box<dyn BlockCipher>) -> Self {
      // Java: super();
      // Java: this.mCipher = cipher;
      // Java: this.mBuf = new byte[cipher.getBlockSize()];
      let block_size = cipher.getBlockSize() as usize;
      let mBuf = vec![0u8; block_size];
      // Java: this.mBufOff = 0;
      // Java: String name = cipher.getAlgorithmName();
      let name = cipher.getAlgorithmName();
      // Java: int idx = name.indexOf(47) + 1;
      let idx = (name.find('/').map(|i| i + 1).unwrap_or(0)) as i32;
      // Java: this.mPgpCFB = idx > 0 && name.startsWith("PGP", idx);
      let mPgpCFB = idx > 0 && name[idx as usize..].starts_with("PGP");
      let mPartialBlockOkay;
      // Java: if (!this.mPgpCFB && !(cipher instanceof StreamCipher))
      if !mPgpCFB {
         // Java: this.mPartialBlockOkay = idx > 0 && name.startsWith("OpenPGP", idx);
         mPartialBlockOkay = idx > 0 && name[idx as usize..].starts_with("OpenPGP");
      }
      // Java: else
      else {
         // Java: this.mPartialBlockOkay = true;
         mPartialBlockOkay = true;
      }
      BufferedBlockCipher {
          mBuf,
          mBufOff: 0,
          mForEncryption: false,
          mCipher: Some(cipher),
          mPartialBlockOkay,
          mPgpCFB,
      }
    }

    fn cipher(&self) -> &dyn BlockCipher {
        self.mCipher.as_ref().unwrap().as_ref()
    }

    fn cipher_mut(&mut self) -> &mut dyn BlockCipher {
        self.mCipher.as_mut().unwrap().as_mut()
    }

    // Java: BlockCipher getUnderlyingCipher()
    pub fn getUnderlyingCipher(&self) -> &dyn BlockCipher {
      // Java: return this.mCipher;
      self.cipher()
    }

    // Java: void init(boolean forEncryption, CipherParameters params)
    pub fn init(&mut self, forEncryption: bool, params: &dyn CipherParameters) {
      // Java: this.mForEncryption = forEncryption;
      self.mForEncryption = forEncryption;
      // Java: this.reset();
      self.mBuf.fill(0);
      self.mBufOff = 0;
      // Java: this.mCipher.init(forEncryption, params);
      self.cipher_mut().init(forEncryption, params);
    }

    // Java: int getBlockSize()
    pub fn getBlockSize(&self) -> i32 {
      // Java: return this.mCipher.getBlockSize();
      self.cipher().getBlockSize()
    }

    // Java: int getUpdateOutputSize(int len)
    pub fn getUpdateOutputSize(&self, len: i32) -> i32 {
      // Java: int total = len + this.mBufOff;
      let total = len + self.mBufOff;
      // Java: int leftOver;
      let leftOver;
      // Java: if (this.mPgpCFB)
      if self.mPgpCFB {
         // Java: if (this.mForEncryption)
         if self.mForEncryption {
            // Java: leftOver = total % this.mBuf.length - (this.mCipher.getBlockSize() + 2);
            leftOver = total % self.mBuf.len() as i32 - (self.cipher().getBlockSize() + 2);
         }
         // Java: else
         else {
            // Java: leftOver = total % this.mBuf.length;
            leftOver = total % self.mBuf.len() as i32;
         }
      }
      // Java: else
      else {
         // Java: leftOver = total % this.mBuf.length;
         leftOver = total % self.mBuf.len() as i32;
      }
      // Java: return total - leftOver;
      total - leftOver
    }

    // Java: int getOutputSize(int length)
    pub fn getOutputSize(&self, length: i32) -> i32 {
      // Java: return length + this.mBufOff;
      length + self.mBufOff
    }

    // Java: int processByte(byte in, byte[] out, int outOff)
    pub fn processByte(&mut self, r#in: u8, out: &mut [u8], outOff: i32) -> i32 {
      // Java: int resultLen = 0;
      let mut resultLen = 0;
      // Java: this.mBuf[this.mBufOff++] = in;
      self.mBuf[self.mBufOff as usize] = r#in;
      self.mBufOff += 1;
      // Java: if (this.mBufOff == this.mBuf.length)
      if self.mBufOff == self.mBuf.len() as i32 {
         // Java: resultLen = this.mCipher.processBlock(this.mBuf, 0, out, outOff);
         let buf_clone = self.mBuf.clone();
         resultLen = self.cipher_mut().processBlock(&buf_clone, 0, out, outOff);
         // Java: this.mBufOff = 0;
         self.mBufOff = 0;
      }
      // Java: return resultLen;
      resultLen
    }

    // Java: int processBytes(byte[] in, int inOff, int len, byte[] out, int outOff)
    pub fn processBytes(&mut self, r#in: &[u8], inOff: i32, len: i32, out: &mut [u8], outOff: i32) -> i32 {
      // Java: if (len < 0)
      if len < 0 {
         // Java: throw new IllegalArgumentException("Can't have a negative input length!");
         panic!("Can't have a negative input length!");
      }
      // Java: else
      {
         // Java: int blockSize = this.getBlockSize();
         let blockSize = self.getBlockSize();
         // Java: int length = this.getUpdateOutputSize(len);
         let length = self.getUpdateOutputSize(len);
         // Java: if (length > 0 && outOff + length > out.length)
         if length > 0 && outOff + length > out.len() as i32 {
            // Java: throw new OutputLengthException("output buffer too short");
            panic!("output buffer too short");
         }
         // Java: else
         {
            // Java: int resultLen = 0;
            let mut resultLen: i32 = 0;
            // Java: int gapLen = this.mBuf.length - this.mBufOff;
            let gapLen = self.mBuf.len() as i32 - self.mBufOff;
            let mut inOff = inOff;
            let mut len = len;
            // Java: if (len > gapLen)
            if len > gapLen {
               // Java: System.arraycopy(in, inOff, this.mBuf, this.mBufOff, gapLen);
               self.mBuf[self.mBufOff as usize..(self.mBufOff + gapLen) as usize]
                   .copy_from_slice(&r#in[inOff as usize..(inOff + gapLen) as usize]);
               // Java: resultLen += this.mCipher.processBlock(this.mBuf, 0, out, outOff);
               let buf_clone = self.mBuf.clone();
               resultLen += self.cipher_mut().processBlock(&buf_clone, 0, out, outOff);
               // Java: this.mBufOff = 0;
               self.mBufOff = 0;
               // Java: len -= gapLen;
               len -= gapLen;
               // Java: for(inOff += gapLen; len > this.mBuf.length; inOff += blockSize)
               inOff += gapLen;
               while len > self.mBuf.len() as i32 {
                  // Java: resultLen += this.mCipher.processBlock(in, inOff, out, outOff + resultLen);
                  resultLen += self.cipher_mut().processBlock(r#in, inOff, out, outOff + resultLen);
                  // Java: len -= blockSize;
                  len -= blockSize;
                  inOff += blockSize;
               }
            }
            // Java: System.arraycopy(in, inOff, this.mBuf, this.mBufOff, len);
            if len > 0 {
                self.mBuf[self.mBufOff as usize..(self.mBufOff + len) as usize]
                    .copy_from_slice(&r#in[inOff as usize..(inOff + len) as usize]);
            }
            // Java: this.mBufOff += len;
            self.mBufOff += len;
            // Java: if (this.mBufOff == this.mBuf.length)
            if self.mBufOff == self.mBuf.len() as i32 {
               // Java: resultLen += this.mCipher.processBlock(this.mBuf, 0, out, outOff + resultLen);
               let buf_clone = self.mBuf.clone();
               resultLen += self.cipher_mut().processBlock(&buf_clone, 0, out, outOff + resultLen);
               // Java: this.mBufOff = 0;
               self.mBufOff = 0;
            }
            // Java: return resultLen;
            return resultLen;
         }
      }
    }

    // Java: int doFinal(byte[] out, int outOff)
    pub fn doFinal(&mut self, out: &mut [u8], outOff: i32) -> i32 {
      // Java: int var4;
      let var4;
      // Java: try
      {
         // Java: int resultLen = 0;
         let mut resultLen: i32 = 0;
         // Java: if (outOff + this.mBufOff > out.length)
         if outOff + self.mBufOff > out.len() as i32 {
            // Java: throw new OutputLengthException("output buffer too short for doFinal()");
            panic!("output buffer too short for doFinal()");
         }
         // Java: if (this.mBufOff != 0)
         if self.mBufOff != 0 {
            // Java: if (!this.mPartialBlockOkay)
            if !self.mPartialBlockOkay {
               // Java: throw new DataLengthException("data not block size aligned");
               panic!("data not block size aligned");
            }
            // Java: this.mCipher.processBlock(this.mBuf, 0, this.mBuf, 0);
            let mut buf_clone = self.mBuf.clone();
            self.cipher_mut().processBlock(&buf_clone.clone(), 0, &mut buf_clone, 0);
            self.mBuf = buf_clone;
            // Java: resultLen = this.mBufOff;
            resultLen = self.mBufOff;
            // Java: this.mBufOff = 0;
            self.mBufOff = 0;
            // Java: System.arraycopy(this.mBuf, 0, out, outOff, resultLen);
            out[outOff as usize..(outOff + resultLen) as usize]
                .copy_from_slice(&self.mBuf[0..resultLen as usize]);
         }
         // Java: var4 = resultLen;
         var4 = resultLen;
      }
      // Java: finally
      {
         // Java: this.reset();
         self.reset();
      }
      // Java: return var4;
      var4
    }

    // Java: void reset()
    pub fn reset(&mut self) {
      // Java: for(int i = 0; i < this.mBuf.length; ++i)
      for i in 0..self.mBuf.len() {
         // Java: this.mBuf[i] = 0;
         self.mBuf[i] = 0;
      }
      // Java: this.mBufOff = 0;
      self.mBufOff = 0;
      // Java: this.mCipher.reset();
      self.cipher_mut().reset();
    }

}
