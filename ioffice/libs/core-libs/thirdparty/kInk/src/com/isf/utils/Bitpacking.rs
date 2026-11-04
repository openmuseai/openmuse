#![allow(non_snake_case, dead_code, unused_variables, unused_imports)]

// Java: import cn.wps.base.assertion.KSAssert
use super::BytesReader::BytesReader;

// Java: public class Bitpacking
pub struct Bitpacking {
}

// Java: public class Bitpacking
impl Bitpacking {
    // Java: Bitpacking()
    pub fn Bitpacking() -> Self {
      // Java: super();
      Self {}
    }

    // Java: static void deflate()
    pub fn deflate() {
    }

    // Java: static void inflate(BytesReader reader, int cbBits, int[] out)
    pub fn inflate(reader: &mut BytesReader, cb_bits: i32, out: &mut [i32]) {
      // Java: KSAssert.assertNotNull("reader should not be null!", reader);
      // Java: byte srcData = (byte)reader.getByte();
      let mut src_data = reader.getByte() as u8;
      // Java: int dstData = 0;
      let mut dst_data: i32 = 0;
      // Java: int srcRemainData = 8;
      let mut src_remain_data: i32 = 8;
      // Java: int dstBitPos = cbBits;
      let mut dst_bit_pos: i32 = cb_bits;
      // Java: int i = 0;
      let mut i: usize = 0;
      // Java: while(true)
      loop
      {
         // Java: int needDataLen = dstBitPos < srcRemainData ? dstBitPos : srcRemainData;
         let need_data_len = if dst_bit_pos < src_remain_data { dst_bit_pos } else { src_remain_data };
         // Java: int unpackData = unpack(srcData, srcRemainData - 1, needDataLen);
         let unpack_data = Self::unpack(src_data, src_remain_data - 1, need_data_len);
         // Java: dstData = pack(dstData, dstBitPos, unpackData, needDataLen);
         dst_data = Self::pack(dst_data, dst_bit_pos, unpack_data, need_data_len);
         // Java: srcRemainData -= needDataLen;
         src_remain_data -= need_data_len;
         // Java: dstBitPos -= needDataLen;
         dst_bit_pos -= need_data_len;
         // Java: if (0 == dstBitPos)
         if 0 == dst_bit_pos
         {
            // Java: out[i++] = dstData;
            out[i] = dst_data;
            i += 1;
            // Java: dstData = 0;
            dst_data = 0;
            // Java: dstBitPos = cbBits;
            dst_bit_pos = cb_bits;
            // Java: if (i >= out.length)
            if i >= out.len()
            {
               // Java: return;
               return;
            }
         }
         // Java: if (0 == srcRemainData)
         if 0 == src_remain_data
         {
            // Java: srcData = (byte)reader.getByte();
            src_data = reader.getByte() as u8;
            // Java: srcRemainData = 8;
            src_remain_data = 8;
         }
      }
    }

    // Java: static int unpack(byte in, int pos, int len)
    pub fn unpack(r#in: u8, pos: i32, len: i32) -> i32 {
      // Java: KSAssert.assertNotNull("len - pos <= 8 should be true!", len - pos <= 8);
      // Java: KSAssert.assertNotNull("0 <= len - pos should be true!", 0 <= len - pos);
      // Java: int mask = 0;
      let mut mask: i32 = 0;
      // Java: for(int i = 0; i < len; ++i)
      for i in 0..len
      {
         // Java: int offset = pos - i;
         let offset = pos - i;
         // Java: mask |= 1 << offset;
         mask |= 1 << offset;
      }
      // Java: int offset = pos - len + 1;
      let offset = pos - len + 1;
      // Java: return (in & mask) >> offset & 255;
      ((r#in as i32) & mask) >> offset & 255
    }

    // Java: static int pack(int in, int pos, int value, int len)
    pub fn pack(r#in: i32, pos: i32, value: i32, len: i32) -> i32 {
      // Java: KSAssert.assertTrue("len <= pos should be true!", len <= pos);
      // Java: int offset = pos - len;
      let offset = pos - len;
      // Java: return in | value << offset;
      r#in | (value << offset)
    }

}
