#![allow(non_snake_case, dead_code, unused_variables, unused_imports)]

// Java: import cn.wps.base.assertion.KSAssert
use super::BytesReader::BytesReader;

// Java: public class BitReader
pub struct BitReader<'a> {
    // Java field: BytesReader mReader
    pub(crate) mReader: &'a mut BytesReader,
    // Java field: int mNextBits
    pub(crate) mNextBits: i32,
    // Java field: int mNumBitsRemaining
    pub(crate) mNumBitsRemaining: i32,
}

// Java: public class BitReader
impl<'a> BitReader<'a> {
    // Java: BitReader(BytesReader in)
    pub fn BitReader(reader: &'a mut BytesReader) -> Self {
      // Java: super();
      // Java: KSAssert.assertNotNull("in should not be null!", in);
      // Java: this.mReader = in;
      // Java: this.mNumBitsRemaining = 0;
      Self {
          mReader: reader,
          mNextBits: 0,
          mNumBitsRemaining: 0,
      }
    }

    // Java: boolean getBit()
    pub fn getBit(&mut self) -> bool {
      // Java: KSAssert.assertNotNull("mReader should not be null!", this.mReader);
      // Java: if (this.mNumBitsRemaining == 0)
      if self.mNumBitsRemaining == 0
      {
         // Java: this.mNextBits = this.mReader.getByte();
         self.mNextBits = self.mReader.getByte();
         // Java: this.mNumBitsRemaining = 8;
         self.mNumBitsRemaining = 8;
      }
      // Java: --this.mNumBitsRemaining;
      self.mNumBitsRemaining -= 1;
      // Java: return 1 == (this.mNextBits >>> this.mNumBitsRemaining & 1);
      1 == ((self.mNextBits as u32 >> self.mNumBitsRemaining as u32) & 1) as i32
    }

    // Java: int getBits(int amount)
    pub fn getBits(&mut self, amount: i32) -> i32 {
      // Java: KSAssert.assertTrue("amount <= 32 should be true!", amount <= 32);
      // Java: byte bitIndex = 0;
      let mut bit_index: i32 = 0;
      // Java: int value;
      // Java: for(value = 0; !this.isEOF() && bitIndex < amount; ++bitIndex)
      let mut value: i32 = 0;
      while !self.isEOF() && bit_index < amount
      {
         // Java: boolean bit = this.getBit();
         let bit = self.getBit();
         // Java: if (bit)
         if bit
         {
            // Java: value |= 1 << amount - bitIndex - 1;
            value |= 1 << (amount - bit_index - 1);
         }
         bit_index += 1;
      }
      // Java: return value;
      value
    }

    // Java: boolean isEOF()
    pub fn isEOF(&self) -> bool {
      // Java: KSAssert.assertNotNull("mReader should not be null!", this.mReader);
      // Java: return this.mReader.isEOF();
      self.mReader.isEOF()
    }

    // Java: void close()
    pub fn close(&mut self) {
      // Java: KSAssert.assertNotNull("mReader should not be null!", this.mReader);
      // Java: this.mReader.close();
      self.mReader.close();
    }

}
