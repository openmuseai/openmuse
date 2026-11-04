#![allow(non_snake_case, non_upper_case_globals, dead_code, unused_variables, unused_imports)]

// Java: import cn.wps.moffice.util.KSLog

// Java field: String TAG
pub(crate) const TAG: &str = "cn.wps.graphics.data.decoder.RleDecoder";

// Java: public class RleDecoder
pub struct RleDecoder {
}

// Java: public class RleDecoder
impl RleDecoder {
    // Java: RleDecoder()
    pub fn new() -> Self {
      // Java: super();
      RleDecoder {}
    }

    // Java: static byte[] rle8Decoder(byte[] data, int offset, int srcCount, int arraySize, int width, byte[] dst)
    pub fn rle8Decoder(data: &[u8], offset: i32, srcCount: i32, arraySize: i32, width: i32, dst: Option<Vec<u8>>) -> Vec<u8> {
      // Java: if (dst == null)
      let mut dst = if dst.is_none()
      {
         // Java: dst = new byte[arraySize];
         vec![0u8; arraySize as usize]
      } else {
         dst.unwrap()
      };
      // Java: int count = 0;
      let mut count: i32 = 0;
      // Java: int dstCount = 0;
      let mut dstCount: i32 = 0;
      // Java: try
      let _result: Result<(), ()> = (|| -> Result<(), ()> {
      {
         // Java: while(count < srcCount && dstCount < arraySize)
         while count < srcCount && dstCount < arraySize
         {
            // Java: int pre_byte = data[offset + count++] & 255;
            let mut pre_byte: i32 = (data[(offset + count) as usize]) as i32 & 255;
            count += 1;
            // Java: byte cur_byte = data[offset + count++];
            let cur_byte: u8 = data[(offset + count) as usize];
            count += 1;
            // Java: if (pre_byte != 0)
            if pre_byte != 0
            {
               // Java: for(int i = 0; i < pre_byte; ++i)
               for _i in 0..pre_byte
               {
                  // Java: dst[dstCount++] = cur_byte;
                  if (dstCount as usize) < dst.len() { dst[dstCount as usize] = cur_byte; }
                  dstCount += 1;
               }
            }
            // Java: else
            else
            {
               // Java: pre_byte = cur_byte & 255;
               pre_byte = cur_byte as i32 & 255;
               // Java: if (pre_byte == 0 && dstCount % width != 0)
               if pre_byte == 0 && dstCount % width != 0
               {
                  // Java: dstCount = dstCount + width - dstCount % width;
                  dstCount = dstCount + width - dstCount % width;
               }
               // Java: else
               else
               {
                  // Java: if (pre_byte == 1)
                  if pre_byte == 1
                  {
                     // Java: break;
                     return Ok(());
                  }
                  // Java: if (pre_byte == 2)
                  if pre_byte == 2
                  {
                     // Java: int x = data[offset + count++] & 255;
                     let x: i32 = data[(offset + count) as usize] as i32 & 255;
                     count += 1;
                     // Java: int y = data[offset + count++] & 255;
                     let y: i32 = data[(offset + count) as usize] as i32 & 255;
                     count += 1;
                     // Java: dstCount += x * width + y;
                     dstCount += x * width + y;
                  }
                  // Java: else if (pre_byte >= 3)
                  else if pre_byte >= 3
                  {
                     // Java: for(int repeat = pre_byte; repeat-- > 0; dst[dstCount++] = data[offset + count++])
                     let mut repeat = pre_byte;
                     while { let old = repeat; repeat -= 1; old } > 0
                     {
                        if (dstCount as usize) < dst.len() { dst[dstCount as usize] = data[(offset + count) as usize]; }
                        dstCount += 1;
                        count += 1;
                     }
                     // Java: count += pre_byte % 2;
                     count += pre_byte % 2;
                  }
               }
            }
         }
      }
      Ok(())
      })();
      // Java: catch (Exception e)
      if _result.is_err()
      {
         // Java: KSLog.w(TAG, "rle8Decoder", e);
      }
      // Java: return dst;
      dst
    }

    // Java: static byte[] rle4Decoder(byte[] data, int offset, int srcCount, int arraySize, int width, byte[] dst)
    pub fn rle4Decoder(data: &[u8], offset: i32, srcCount: i32, arraySize: i32, width: i32, dst: Option<Vec<u8>>) -> Vec<u8> {
      // Java: if (dst == null)
      let mut dst = if dst.is_none()
      {
         // Java: dst = new byte[arraySize];
         vec![0u8; arraySize as usize]
      } else {
         dst.unwrap()
      };
      // Java: int count = 0;
      let mut count: i32 = 0;
      // Java: int dstCount = 0;
      let mut dstCount: i32 = 0;
      // Java: try
      let _result: Result<(), ()> = (|| -> Result<(), ()> {
      {
         // Java: while(count < srcCount && dstCount < arraySize)
         while count < srcCount && dstCount < arraySize
         {
            // Java: int pre_byte = data[offset + count++] & 255;
            let mut pre_byte: i32 = data[(offset + count) as usize] as i32 & 255;
            count += 1;
            // Java: byte cur_byte = data[offset + count++];
            let cur_byte: u8 = data[(offset + count) as usize];
            count += 1;
            // Java: if (pre_byte == 0)
            if pre_byte == 0
            {
               // Java: pre_byte = cur_byte & 255;
               pre_byte = cur_byte as i32 & 255;
               // Java: if (pre_byte == 0 && dstCount % width != 0)
               if pre_byte == 0 && dstCount % width != 0
               {
                  // Java: dstCount = dstCount + width - dstCount % width;
                  dstCount = dstCount + width - dstCount % width;
               }
               // Java: else
               else
               {
                  // Java: if (pre_byte == 1)
                  if pre_byte == 1
                  {
                     // Java: break;
                     return Ok(());
                  }
                  // Java: if (pre_byte == 2)
                  if pre_byte == 2
                  {
                     // Java: int x = data[offset + count++] & 255;
                     let x: i32 = data[(offset + count) as usize] as i32 & 255;
                     count += 1;
                     // Java: int y = data[offset + count++] & 255;
                     let y: i32 = data[(offset + count) as usize] as i32 & 255;
                     count += 1;
                     // Java: dstCount += x * width + y;
                     dstCount += x * width + y;
                  }
                  // Java: else if (pre_byte >= 3)
                  else if pre_byte >= 3
                  {
                     // Java: int repeat = pre_byte;
                     let repeat: i32 = pre_byte;
                     // Java: int j = 0;
                     let mut j: i32 = 0;
                     // Java: while(j < repeat)
                     while j < repeat
                     {
                        // Java: byte Byte = data[offset + count++];
                        let byte_val: u8 = data[(offset + count) as usize];
                        count += 1;
                        // Java: dst[dstCount++] = (byte)(Byte >> 4);
                        if (dstCount as usize) < dst.len() { dst[dstCount as usize] = byte_val >> 4; }
                        dstCount += 1;
                        // Java: ++j;
                        j += 1;
                        // Java: if (j < repeat)
                        if j < repeat
                        {
                           // Java: dst[dstCount++] = (byte)(Byte & 15);
                           if (dstCount as usize) < dst.len() { dst[dstCount as usize] = byte_val & 15; }
                           dstCount += 1;
                           // Java: ++j;
                           j += 1;
                        }
                     }
                     // Java: count += (pre_byte * 4 + 15) / 16 * 2 - (pre_byte + 1) / 2;
                     count += (pre_byte * 4 + 15) / 16 * 2 - (pre_byte + 1) / 2;
                  }
               }
            }
            // Java: else
            else
            {
               // Java: byte pre = (byte)(cur_byte >> 4);
               let pre: u8 = cur_byte >> 4;
               // Java: byte pos = (byte)(cur_byte & 15);
               let pos: u8 = cur_byte & 15;
               // Java: int i = 0;
               let mut i: i32 = 0;
               // Java: for(i = 0; i + 1 < pre_byte; i += 2)
               i = 0;
               while i + 1 < pre_byte
               {
                  // Java: dst[dstCount++] = pre;
                  if (dstCount as usize) < dst.len() { dst[dstCount as usize] = pre; }
                  dstCount += 1;
                  // Java: dst[dstCount++] = pos;
                  if (dstCount as usize) < dst.len() { dst[dstCount as usize] = pos; }
                  dstCount += 1;
                  i += 2;
               }
               // Java: if (i < pre_byte)
               if i < pre_byte
               {
                  // Java: dst[dstCount++] = pre;
                  if (dstCount as usize) < dst.len() { dst[dstCount as usize] = pre; }
                  dstCount += 1;
               }
            }
         }
      }
      Ok(())
      })();
      // Java: catch (Exception e)
      if _result.is_err()
      {
         // Java: KSLog.w(TAG, "rle8Decoder", e);
      }
      // Java: return dst;
      dst
    }

}
