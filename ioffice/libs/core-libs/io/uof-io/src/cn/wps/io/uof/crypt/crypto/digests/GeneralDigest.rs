// Java: import cn.wps.io.uof.crypt.crypto.ExtendedDigest
// Java: import cn.wps.io.uof.crypt.util.Memoable
// Java: import cn.wps.io.uof.crypt.util.Pack
use crate::cn::wps::io::uof::crypt::util::Pack::Pack;

// Java field: int BYTE_LENGTH
pub(crate) const BYTE_LENGTH: i32 = 64;

// Java: public class GeneralDigest implements ExtendedDigest, Memoable
pub struct GeneralDigest {
    // Java field: byte[] mXBuf
    pub(crate) mXBuf: Vec<u8>,
    // Java field: int mXBufOff
    pub(crate) mXBufOff: i32,
    // Java field: long mByteCount
    pub(crate) mByteCount: i64,
    pub(crate) processWordFn: fn(&mut GeneralDigest, &[u8], i32),
    pub(crate) processLengthFn: fn(&mut GeneralDigest, i64),
    pub(crate) processBlockFn: fn(&mut GeneralDigest),
}

// Java: public class GeneralDigest implements ExtendedDigest, Memoable
impl GeneralDigest {
    // Java: GeneralDigest()
    pub fn GeneralDigest(
        pw: fn(&mut GeneralDigest, &[u8], i32),
        pl: fn(&mut GeneralDigest, i64),
        pb: fn(&mut GeneralDigest),
    ) -> Self {
      // Java: super();
      // Java: this.mXBufOff = 0;
      GeneralDigest {
          mXBuf: vec![0u8; 4],
          mXBufOff: 0,
          mByteCount: 0,
          processWordFn: pw,
          processLengthFn: pl,
          processBlockFn: pb,
      }
    }

    // Java: GeneralDigest(GeneralDigest var1)
    pub fn GeneralDigest_copy(var1: &GeneralDigest,
        pw: fn(&mut GeneralDigest, &[u8], i32),
        pl: fn(&mut GeneralDigest, i64),
        pb: fn(&mut GeneralDigest),
    ) -> Self {
      // Java: super();
      let mut result = Self::GeneralDigest(pw, pl, pb);
      // Java: this.copyIn(var1);
      result.copyIn(var1);
      result
    }

    // Java: GeneralDigest(byte[] var1)
    pub fn GeneralDigest_from_bytes(var1: &[u8],
        pw: fn(&mut GeneralDigest, &[u8], i32),
        pl: fn(&mut GeneralDigest, i64),
        pb: fn(&mut GeneralDigest),
    ) -> Self {
      // Java: super();
      let mut result = Self::GeneralDigest(pw, pl, pb);
      // Java: System.arraycopy(var1, 0, this.mXBuf, 0, this.mXBuf.length);
      result.mXBuf[..4].copy_from_slice(&var1[..4]);
      // Java: this.mXBufOff = Pack.bigEndianToInt(var1, 4);
      result.mXBufOff = Pack::bigEndianToInt(var1, 4);
      // Java: this.mByteCount = Pack.bigEndianToLong(var1, 8);
      result.mByteCount = Pack::bigEndianToLong(var1, 8);
      result
    }

    // Java: void copyIn(GeneralDigest var1)
    pub fn copyIn(&mut self, var1: &GeneralDigest) {
      // Java: System.arraycopy(var1.mXBuf, 0, this.mXBuf, 0, var1.mXBuf.length);
      let len = var1.mXBuf.len();
      self.mXBuf[..len].copy_from_slice(&var1.mXBuf[..len]);
      // Java: this.mXBufOff = var1.mXBufOff;
      self.mXBufOff = var1.mXBufOff;
      // Java: this.mByteCount = var1.mByteCount;
      self.mByteCount = var1.mByteCount;
    }

    // Java: void update(byte var1)
    pub fn update_byte(&mut self, var1: u8) {
      // Java: this.mXBuf[this.mXBufOff++] = var1;
      self.mXBuf[self.mXBufOff as usize] = var1;
      self.mXBufOff += 1;
      // Java: if (this.mXBufOff == this.mXBuf.length)
      if self.mXBufOff == self.mXBuf.len() as i32 {
         // Java: this.processWord(this.mXBuf, 0);
         let buf = self.mXBuf.clone();
         (self.processWordFn)(self, &buf, 0);
         // Java: this.mXBufOff = 0;
         self.mXBufOff = 0;
      }
      // Java: ++this.mByteCount;
      self.mByteCount += 1;
    }

    // Java: void update(byte[] var1, int var2, int var3)
    pub fn update(&mut self, var1: &[u8], var2: i32, var3: i32) {
      // Java: var3 = Math.max(0, var3);
      let mut var3 = var3.max(0);
      // Java: int var4 = 0;
      let mut var4: i32 = 0;
      let var2 = var2;
      // Java: if (this.mXBufOff != 0)
      if self.mXBufOff != 0 {
         // Java: while(var4 < var3)
         while var4 < var3 {
            // Java: this.mXBuf[this.mXBufOff++] = var1[var2 + var4++];
            self.mXBuf[self.mXBufOff as usize] = var1[(var2 + var4) as usize];
            self.mXBufOff += 1;
            var4 += 1;
            // Java: if (this.mXBufOff == 4)
            if self.mXBufOff == 4 {
               // Java: this.processWord(this.mXBuf, 0);
               let buf = self.mXBuf.clone();
               (self.processWordFn)(self, &buf, 0);
               // Java: this.mXBufOff = 0;
               self.mXBufOff = 0;
               // Java: break;
               break;
            }
         }
      }
      // Java: for(int var5 = (var3 - var4 & -4) + var4; var4 < var5; var4 += 4)
      let var5 = ((var3 - var4) & !3) + var4;
      while var4 < var5 {
         // Java: this.processWord(var1, var2 + var4);
         (self.processWordFn)(self, var1, var2 + var4);
         var4 += 4;
      }
      // Java: while(var4 < var3)
      while var4 < var3 {
         // Java: this.mXBuf[this.mXBufOff++] = var1[var2 + var4++];
         self.mXBuf[self.mXBufOff as usize] = var1[(var2 + var4) as usize];
         self.mXBufOff += 1;
         var4 += 1;
      }
      // Java: this.mByteCount += (long)var3;
      self.mByteCount += var3 as i64;
    }

    // Java: void finish()
    pub fn finish(&mut self) {
      // Java: long var1 = this.mByteCount << 3;
      let var1 = self.mByteCount << 3;
      // Java: this.update((byte)-128);
      self.update_byte(0x80u8);
      // Java: while(this.mXBufOff != 0)
      while self.mXBufOff != 0 {
         // Java: this.update((byte)0);
         self.update_byte(0);
      }
      // Java: this.processLength(var1);
      (self.processLengthFn)(self, var1);
      // Java: this.processBlock();
      (self.processBlockFn)(self);
    }

    // Java: void reset()
    pub fn reset(&mut self) {
      // Java: this.mByteCount = 0L;
      self.mByteCount = 0;
      // Java: this.mXBufOff = 0;
      self.mXBufOff = 0;
      // Java: for(int var1 = 0; var1 < this.mXBuf.length; ++var1)
      for var1 in 0..self.mXBuf.len() {
         // Java: this.mXBuf[var1] = 0;
         self.mXBuf[var1] = 0;
      }
    }

    // Java: void populateState(byte[] var1)
    pub fn populateState(&self, var1: &mut [u8]) {
      // Java: System.arraycopy(this.mXBuf, 0, var1, 0, this.mXBufOff);
      var1[..self.mXBufOff as usize].copy_from_slice(&self.mXBuf[..self.mXBufOff as usize]);
      // Java: Pack.intToBigEndian(this.mXBufOff, var1, 4);
      Pack::intToBigEndian_into(self.mXBufOff, var1, 4);
      // Java: Pack.longToBigEndian(this.mByteCount, var1, 8);
      Pack::longToBigEndian_into(self.mByteCount, var1, 8);
    }

    // Java: int getByteLength()
    pub fn getByteLength(&self) -> i32 {
      // Java: return 64;
      64
    }

    // Java: abstract void processWord(byte[] var1, int var2)
    pub fn processWord(&mut self, var1: &[u8], var2: i32) {
      (self.processWordFn)(self, var1, var2);
    }

    // Java: abstract void processLength(long var1)
    pub fn processLength(&mut self, var1: i64) {
      (self.processLengthFn)(self, var1);
    }

    // Java: abstract void processBlock()
    pub fn processBlock(&mut self) {
      (self.processBlockFn)(self);
    }

}
