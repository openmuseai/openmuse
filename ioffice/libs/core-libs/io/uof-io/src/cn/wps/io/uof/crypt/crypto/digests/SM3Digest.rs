// Java: import cn.wps.io.uof.crypt.util.Memoable
// Java: import cn.wps.io.uof.crypt.util.Pack
use crate::cn::wps::io::uof::crypt::util::Pack::Pack;
use super::GeneralDigest::GeneralDigest;

// Java field: int DIGEST_LENGTH
pub(crate) const DIGEST_LENGTH: i32 = 32;
// Java field: int BLOCK_SIZE
pub(crate) const BLOCK_SIZE: i32 = 16;
// Java: public class SM3Digest extends GeneralDigest
pub struct SM3Digest {
    pub(crate) base: GeneralDigest,
    // Java field: int[] mVArray
    pub(crate) mVArray: Vec<i32>,
    // Java field: int[] mInWords
    pub(crate) mInWords: Vec<i32>,
    // Java field: int mXOff
    pub(crate) mXOff: i32,
    // Java field: int[] mWArray
    pub(crate) mWArray: Vec<i32>,
}

// Java field: int[] T
static T: std::sync::OnceLock<Vec<i32>> = std::sync::OnceLock::new();

fn get_t() -> &'static Vec<i32> {
    T.get_or_init(|| {
        let mut t = vec![0i32; 64];
        for var0 in 0..16i32 {
            let var1: i32 = 0x79CC4519u32 as i32;
            t[var0 as usize] = (var1 as u32).rotate_left(var0 as u32) as i32;
        }
        for var3 in 16..64i32 {
            let var1_mod = var3 % 32;
            let var2: i32 = 0x7A879D8Au32 as i32;
            t[var3 as usize] = (var2 as u32).rotate_left(var1_mod as u32) as i32;
        }
        t
    })
}

fn sm3_process_word(gd: &mut GeneralDigest, _var1: &[u8], _var2: i32) {
    let _ = (gd, _var1, _var2);
}
fn sm3_process_length(gd: &mut GeneralDigest, _var1: i64) {
    let _ = (gd, _var1);
}
fn sm3_process_block(gd: &mut GeneralDigest) {
    let _ = gd;
}

// Java: public class SM3Digest extends GeneralDigest
impl SM3Digest {
    // Java: SM3Digest()
    pub fn SM3Digest() -> Self {
      // Java: super();
      let mut d = SM3Digest {
          base: GeneralDigest::GeneralDigest(sm3_process_word, sm3_process_length, sm3_process_block),
          mVArray: vec![0i32; 8],
          mInWords: vec![0i32; 16],
          mXOff: 0,
          mWArray: vec![0i32; 68],
      };
      // Java: this.reset();
      d.reset();
      d
    }

    // Java: SM3Digest(SM3Digest var1)
    pub fn SM3Digest_copy(var1: &SM3Digest) -> Self {
      // Java: super((GeneralDigest)var1);
      let mut d = SM3Digest {
          base: GeneralDigest::GeneralDigest(sm3_process_word, sm3_process_length, sm3_process_block),
          mVArray: vec![0i32; 8],
          mInWords: vec![0i32; 16],
          mXOff: 0,
          mWArray: vec![0i32; 68],
      };
      d.base.copyIn(&var1.base);
      // Java: this.copyIn(var1);
      d.copyIn(var1);
      d
    }

    // Java: void copyIn(SM3Digest var1)
    pub fn copyIn(&mut self, var1: &SM3Digest) {
      // Java: System.arraycopy(var1.mVArray, 0, this.mVArray, 0, this.mVArray.length);
      self.mVArray.copy_from_slice(&var1.mVArray);
      // Java: System.arraycopy(var1.mInWords, 0, this.mInWords, 0, this.mInWords.length);
      self.mInWords.copy_from_slice(&var1.mInWords);
      // Java: this.mXOff = var1.mXOff;
      self.mXOff = var1.mXOff;
    }

    // Java: String getAlgorithmName()
    pub fn getAlgorithmName(&self) -> String {
      // Java: return "SM3";
      "SM3".to_string()
    }

    // Java: int getDigestSize()
    pub fn getDigestSize(&self) -> i32 {
      // Java: return 32;
      32
    }

    // Java: Memoable copy()
    pub fn copy(&self) -> SM3Digest {
      // Java: return new SM3Digest(this);
      SM3Digest::SM3Digest_copy(self)
    }

    // Java: void reset(Memoable var1)
    pub fn reset_from(&mut self, var1: &SM3Digest) {
      // Java: SM3Digest var2 = (SM3Digest)var1;
      // Java: super.copyIn(var2);
      self.base.copyIn(&var1.base);
      // Java: this.copyIn(var2);
      self.copyIn(var1);
    }

    // Java: void reset()
    pub fn reset(&mut self) {
      // Java: super.reset();
      self.base.reset();
      // Java: this.mVArray[0] = 1937774191;
      self.mVArray[0] = 1937774191;
      // Java: this.mVArray[1] = 1226093241;
      self.mVArray[1] = 1226093241;
      // Java: this.mVArray[2] = 388252375;
      self.mVArray[2] = 388252375;
      // Java: this.mVArray[3] = -628488704;
      self.mVArray[3] = -628488704;
      // Java: this.mVArray[4] = -1452330820;
      self.mVArray[4] = -1452330820;
      // Java: this.mVArray[5] = 372324522;
      self.mVArray[5] = 372324522;
      // Java: this.mVArray[6] = -477237683;
      self.mVArray[6] = -477237683;
      // Java: this.mVArray[7] = -1325724082;
      self.mVArray[7] = -1325724082;
      // Java: this.mXOff = 0;
      self.mXOff = 0;
    }

    // Java: int doFinal(byte[] var1, int var2)
    pub fn doFinal(&mut self, var1: &mut [u8], var2: i32) -> i32 {
      // Java: this.finish();
      self.base.finish();
      // Java: Pack.intToBigEndian(this.mVArray, var1, var2);
      Pack::intToBigEndian_slice(&self.mVArray, var1, var2);
      // Java: this.reset();
      self.reset();
      // Java: return 32;
      32
    }

    // Java: void processWord(byte[] var1, int var2)
    pub fn processWord(&mut self, var1: &[u8], var2: i32) {
      // Java: int var10000 = (var1[var2] & 255) << 24;
      let mut var2 = var2 as usize;
      let mut var10000: i32 = ((var1[var2] as i32) & 255) << 24;
      // Java: ++var2;
      var2 += 1;
      // Java: var10000 |= (var1[var2] & 255) << 16;
      var10000 |= ((var1[var2] as i32) & 255) << 16;
      // Java: ++var2;
      var2 += 1;
      // Java: var10000 |= (var1[var2] & 255) << 8;
      var10000 |= ((var1[var2] as i32) & 255) << 8;
      // Java: ++var2;
      var2 += 1;
      // Java: int var3 = var10000 | var1[var2] & 255;
      let var3 = var10000 | (var1[var2] as i32) & 255;
      // Java: this.mInWords[this.mXOff] = var3;
      self.mInWords[self.mXOff as usize] = var3;
      // Java: ++this.mXOff;
      self.mXOff += 1;
      // Java: if (this.mXOff >= 16)
      if self.mXOff >= 16 {
         // Java: this.processBlock();
         self.processBlock();
      }
    }

    // Java: void processLength(long var1)
    pub fn processLength(&mut self, var1: i64) {
      // Java: if (this.mXOff > 14)
      if self.mXOff > 14 {
         // Java: this.mInWords[this.mXOff] = 0;
         self.mInWords[self.mXOff as usize] = 0;
         // Java: ++this.mXOff;
         self.mXOff += 1;
         // Java: this.processBlock();
         self.processBlock();
      }
      // Java: while(this.mXOff < 14)
      while self.mXOff < 14 {
         // Java: this.mInWords[this.mXOff] = 0;
         self.mInWords[self.mXOff as usize] = 0;
         // Java: ++this.mXOff;
         self.mXOff += 1;
      }
      // Java: this.mInWords[this.mXOff++] = (int)(var1 >>> 32);
      self.mInWords[self.mXOff as usize] = ((var1 as u64) >> 32) as i32;
      self.mXOff += 1;
      // Java: this.mInWords[this.mXOff++] = (int)var1;
      self.mInWords[self.mXOff as usize] = var1 as i32;
      self.mXOff += 1;
    }

    // Java: int p0(int var1)
    fn p0(&self, var1: i32) -> i32 {
      // Java: int var2 = var1 << 9 | var1 >>> 23;
      let var2 = (var1 as u32).rotate_left(9) as i32;
      // Java: int var3 = var1 << 17 | var1 >>> 15;
      let var3 = (var1 as u32).rotate_left(17) as i32;
      // Java: return var1 ^ var2 ^ var3;
      var1 ^ var2 ^ var3
    }

    // Java: int p1(int var1)
    fn p1(&self, var1: i32) -> i32 {
      // Java: int var2 = var1 << 15 | var1 >>> 17;
      let var2 = (var1 as u32).rotate_left(15) as i32;
      // Java: int var3 = var1 << 23 | var1 >>> 9;
      let var3 = (var1 as u32).rotate_left(23) as i32;
      // Java: return var1 ^ var2 ^ var3;
      var1 ^ var2 ^ var3
    }

    // Java: int ff0(int var1, int var2, int var3)
    fn ff0(&self, var1: i32, var2: i32, var3: i32) -> i32 {
      // Java: return var1 ^ var2 ^ var3;
      var1 ^ var2 ^ var3
    }

    // Java: int ff1(int var1, int var2, int var3)
    fn ff1(&self, var1: i32, var2: i32, var3: i32) -> i32 {
      // Java: return var1 & var2 | var1 & var3 | var2 & var3;
      (var1 & var2) | (var1 & var3) | (var2 & var3)
    }

    // Java: int gg0(int var1, int var2, int var3)
    fn gg0(&self, var1: i32, var2: i32, var3: i32) -> i32 {
      // Java: return var1 ^ var2 ^ var3;
      var1 ^ var2 ^ var3
    }

    // Java: int gg1(int var1, int var2, int var3)
    fn gg1(&self, var1: i32, var2: i32, var3: i32) -> i32 {
      // Java: return var1 & var2 | ~var1 & var3;
      (var1 & var2) | (!var1 & var3)
    }

    // Java: void processBlock()
    pub fn processBlock(&mut self) {
      let t = get_t();
      // Java: for(int var1 = 0; var1 < 16; ++var1)
      for var1 in 0..16usize {
         // Java: this.mWArray[var1] = this.mInWords[var1];
         self.mWArray[var1] = self.mInWords[var1];
      }
      // Java: for(int var19 = 16; var19 < 68; ++var19)
      for var19 in 16..68usize {
         // Java: int var2 = this.mWArray[var19 - 3];
         let var2 = self.mWArray[var19 - 3];
         // Java: int var3 = var2 << 15 | var2 >>> 17;
         let var3 = (var2 as u32).rotate_left(15) as i32;
         // Java: int var4 = this.mWArray[var19 - 13];
         let var4 = self.mWArray[var19 - 13];
         // Java: int var5 = var4 << 7 | var4 >>> 25;
         let var5 = (var4 as u32).rotate_left(7) as i32;
         // Java: this.mWArray[var19] = this.p1(this.mWArray[var19 - 16] ^ this.mWArray[var19 - 9] ^ var3) ^ var5 ^ this.mWArray[var19 - 6];
         self.mWArray[var19] = self.p1(self.mWArray[var19 - 16] ^ self.mWArray[var19 - 9] ^ var3) ^ var5 ^ self.mWArray[var19 - 6];
      }
      // Java: int var20 = this.mVArray[0];
      let mut var20 = self.mVArray[0];
      // Java: int var2 = this.mVArray[1];
      let mut var2 = self.mVArray[1];
      // Java: int var3 = this.mVArray[2];
      let mut var3 = self.mVArray[2];
      // Java: int var4 = this.mVArray[3];
      let mut var4 = self.mVArray[3];
      // Java: int var5 = this.mVArray[4];
      let mut var5 = self.mVArray[4];
      // Java: int var6 = this.mVArray[5];
      let mut var6 = self.mVArray[5];
      // Java: int var7 = this.mVArray[6];
      let mut var7 = self.mVArray[6];
      // Java: int var8 = this.mVArray[7];
      let mut var8 = self.mVArray[7];
      // Java: for(int var9 = 0; var9 < 16; ++var9)
      for var9 in 0..16usize {
         // Java: int var10 = var20 << 12 | var20 >>> 20;
         let var10 = (var20 as u32).rotate_left(12) as i32;
         // Java: int var11 = var10 + var5 + T[var9];
         let var11 = var10.wrapping_add(var5).wrapping_add(t[var9]);
         // Java: int var12 = var11 << 7 | var11 >>> 25;
         let var12 = (var11 as u32).rotate_left(7) as i32;
         // Java: int var13 = var12 ^ var10;
         let var13 = var12 ^ var10;
         // Java: int var14 = this.mWArray[var9];
         let var14 = self.mWArray[var9];
         // Java: int var15 = var14 ^ this.mWArray[var9 + 4];
         let var15 = var14 ^ self.mWArray[var9 + 4];
         // Java: int var16 = this.ff0(var20, var2, var3) + var4 + var13 + var15;
         let var16 = self.ff0(var20, var2, var3).wrapping_add(var4).wrapping_add(var13).wrapping_add(var15);
         // Java: int var17 = this.gg0(var5, var6, var7) + var8 + var12 + var14;
         let var17 = self.gg0(var5, var6, var7).wrapping_add(var8).wrapping_add(var12).wrapping_add(var14);
         // Java: var4 = var3;
         var4 = var3;
         // Java: var3 = var2 << 9 | var2 >>> 23;
         var3 = (var2 as u32).rotate_left(9) as i32;
         // Java: var2 = var20;
         var2 = var20;
         // Java: var20 = var16;
         var20 = var16;
         // Java: var8 = var7;
         var8 = var7;
         // Java: var7 = var6 << 19 | var6 >>> 13;
         var7 = (var6 as u32).rotate_left(19) as i32;
         // Java: var6 = var5;
         var6 = var5;
         // Java: var5 = this.p0(var17);
         var5 = self.p0(var17);
      }
      // Java: for(int var25 = 16; var25 < 64; ++var25)
      for var25 in 16..64usize {
         // Java: int var10 = var20 << 12 | var20 >>> 20;
         let var10 = (var20 as u32).rotate_left(12) as i32;
         // Java: int var11 = var10 + var5 + T[var25];
         let var11 = var10.wrapping_add(var5).wrapping_add(t[var25]);
         // Java: int var12 = var11 << 7 | var11 >>> 25;
         let var12 = (var11 as u32).rotate_left(7) as i32;
         // Java: int var13 = var12 ^ var10;
         let var13 = var12 ^ var10;
         // Java: int var14 = this.mWArray[var25];
         let var14 = self.mWArray[var25];
         // Java: int var15 = var14 ^ this.mWArray[var25 + 4];
         let var15 = var14 ^ self.mWArray[var25 + 4];
         // Java: int var16 = this.ff1(var20, var2, var3) + var4 + var13 + var15;
         let var16 = self.ff1(var20, var2, var3).wrapping_add(var4).wrapping_add(var13).wrapping_add(var15);
         // Java: int var17 = this.gg1(var5, var6, var7) + var8 + var12 + var14;
         let var17 = self.gg1(var5, var6, var7).wrapping_add(var8).wrapping_add(var12).wrapping_add(var14);
         // Java: var4 = var3;
         var4 = var3;
         // Java: var3 = var2 << 9 | var2 >>> 23;
         var3 = (var2 as u32).rotate_left(9) as i32;
         // Java: var2 = var20;
         var2 = var20;
         // Java: var20 = var16;
         var20 = var16;
         // Java: var8 = var7;
         var8 = var7;
         // Java: var7 = var6 << 19 | var6 >>> 13;
         var7 = (var6 as u32).rotate_left(19) as i32;
         // Java: var6 = var5;
         var6 = var5;
         // Java: var5 = this.p0(var17);
         var5 = self.p0(var17);
      }
      // Java: int[] var10000 = this.mVArray;
      // Java: var10000[0] ^= var20;
      self.mVArray[0] ^= var20;
      // Java: var10000 = this.mVArray;
      // Java: var10000[1] ^= var2;
      self.mVArray[1] ^= var2;
      // Java: var10000 = this.mVArray;
      // Java: var10000[2] ^= var3;
      self.mVArray[2] ^= var3;
      // Java: var10000 = this.mVArray;
      // Java: var10000[3] ^= var4;
      self.mVArray[3] ^= var4;
      // Java: var10000 = this.mVArray;
      // Java: var10000[4] ^= var5;
      self.mVArray[4] ^= var5;
      // Java: var10000 = this.mVArray;
      // Java: var10000[5] ^= var6;
      self.mVArray[5] ^= var6;
      // Java: var10000 = this.mVArray;
      // Java: var10000[6] ^= var7;
      self.mVArray[6] ^= var7;
      // Java: var10000 = this.mVArray;
      // Java: var10000[7] ^= var8;
      self.mVArray[7] ^= var8;
      // Java: this.mXOff = 0;
      self.mXOff = 0;
    }

}
