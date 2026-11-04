#![allow(non_snake_case, dead_code, unused_variables, unused_imports)]

// Java: import cn.wps.base.assertion.KSAssert
// Java: import cn.wps.moffice.util.KSLog
// Java: import com.isf.utils.BitArray
use crate::com::isf::utils::BitArray::BitArray;
// Java: import java.io.IOException
// Java: import java.io.OutputStream

// Java: public class BitWriter
pub struct BitWriter {
    // Java field: String TAG
    // Java field: OutputStream output
    output: Vec<u8>,
    // Java field: int currentByte
    currentByte: i32,
    // Java field: int numBitsInCurrentByte
    numBitsInCurrentByte: i32,
}

// Java: public class BitWriter
impl BitWriter {
    // Java: BitWriter(OutputStream out)
    pub fn BitWriter() -> Self {
      // Java: super();
      // Java: KSAssert.assertNotNull("out should not be null!", out);
      // Java: this.output = out;
      // Java: this.currentByte = 0;
      // Java: this.numBitsInCurrentByte = 0;
      Self {
          output: Vec::new(),
          currentByte: 0,
          numBitsInCurrentByte: 0,
      }
    }

    // Java: void write(boolean in)
    pub fn write_bit(&mut self, r#in: bool) {
      // Java: int b = in ? 1 : 0;
      let b = if r#in { 1 } else { 0 };
      // Java: this.currentByte = this.currentByte << 1 | b;
      self.currentByte = self.currentByte << 1 | b;
      // Java: ++this.numBitsInCurrentByte;
      self.numBitsInCurrentByte += 1;
      // Java: if (8 == this.numBitsInCurrentByte)
      if 8 == self.numBitsInCurrentByte
      {
         // Java: this.output.write(this.currentByte);
         self.output.push(self.currentByte as u8);
         // Java: this.numBitsInCurrentByte = 0;
         self.numBitsInCurrentByte = 0;
      }
    }

    // Java: void write(BitArray bitArray)
    pub fn write_bit_array(&mut self, bit_array: &BitArray) {
      // Java: KSAssert.assertNotNull("bitArray should not be null!", bitArray);
      // Java: int len = bitArray.length();
      let len = bit_array.length();
      // Java: for(int i = 0; i < len; ++i)
      for i in 0..len
      {
         // Java: this.write(bitArray.get(i));
         self.write_bit(bit_array.get(i));
      }
    }

    // Java: void close()
    pub fn close(&mut self) {
      // Java: try
      {
         // Java: while(true)
         loop
         {
            // Java: if (0 != this.numBitsInCurrentByte)
            if 0 != self.numBitsInCurrentByte
            {
               // Java: this.write(false);
               self.write_bit(false);
            }
            // Java: else
            else
            {
               // Java: this.output.close();
               // Java: break;
               break;
            }
         }
      }
      // Java: catch (IOException e)
      {
         // Java: KSLog.e(TAG, "IOException", e);
      }
    }

    pub fn get_output(&self) -> &[u8] {
        &self.output
    }

}
