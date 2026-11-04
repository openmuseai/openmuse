use super::BlockCipher::BlockCipher;
use super::StreamCipher::StreamCipher;
use super::CipherParameters::CipherParameters;

// Java: public class StreamBlockCipher implements BlockCipher, StreamCipher
pub struct StreamBlockCipher {
    // Java field: BlockCipher mCipher
    pub(crate) mCipher: Box<dyn BlockCipher>,
    pub(crate) calculateByteFn: fn(&mut StreamBlockCipher, u8) -> u8,
}

// Java: public class StreamBlockCipher implements BlockCipher, StreamCipher
impl StreamBlockCipher {
    // Java: StreamBlockCipher(BlockCipher var1)
    pub fn StreamBlockCipher(var1: Box<dyn BlockCipher>, calc_fn: fn(&mut StreamBlockCipher, u8) -> u8) -> Self {
      // Java: super();
      // Java: this.mCipher = var1;
      StreamBlockCipher { mCipher: var1, calculateByteFn: calc_fn }
    }

    // Java: BlockCipher getUnderlyingCipher()
    pub fn getUnderlyingCipher(&self) -> &dyn BlockCipher {
      // Java: return this.mCipher;
      &*self.mCipher
    }

    // Java: byte returnByte(byte var1)
    pub fn returnByte(&mut self, var1: u8) -> u8 {
      // Java: return this.calculateByte(var1);
      self.calculateByte(var1)
    }

    // Java: int processBytes(byte[] var1, int var2, int var3, byte[] var4, int var5)
    pub fn processBytes(&mut self, var1: &[u8], var2: i32, var3: i32, var4: &mut [u8], var5: i32) -> i32 {
      // Java: if (var2 + var3 > var1.length)
      if var2 + var3 > var1.len() as i32 {
         // Java: throw new DataLengthException("input buffer too small");
         panic!("input buffer too small");
      }
      // Java: else if (var5 + var3 > var4.length)
      else if var5 + var3 > var4.len() as i32 {
         // Java: throw new OutputLengthException("output buffer too short");
         panic!("output buffer too short");
      }
      // Java: else
      {
         // Java: int var6 = var2;
         let mut var6 = var2;
         // Java: int var7 = var2 + var3;
         let var7 = var2 + var3;
         // Java: for(int var8 = var5; var6 < var7; var4[var8++] = this.calculateByte(var1[var6++]))
         let mut var8 = var5;
         while var6 < var7 {
            var4[var8 as usize] = self.calculateByte(var1[var6 as usize]);
            var8 += 1;
            var6 += 1;
         }
         // Java: return var3;
         var3
      }
    }

    // Java: abstract byte calculateByte(byte var1)
    pub fn calculateByte(&mut self, var1: u8) -> u8 {
      (self.calculateByteFn)(self, var1)
    }

}
