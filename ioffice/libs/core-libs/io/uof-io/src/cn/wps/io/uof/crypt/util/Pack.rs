// Java: public class Pack
pub struct Pack {
}

// Java: public class Pack
impl Pack {
    // Java: Pack()
    pub fn Pack(&self) {
      // Java: super();
    }

    // Java: static short bigEndianToShort(byte[] var0, int var1)
    pub fn bigEndianToShort(var0: &[u8], var1: i32) -> i16 {
      // Java: int var2 = (var0[var1] & 255) << 8;
      let mut var1 = var1 as usize;
      let mut var2: i32 = ((var0[var1] as i32) & 255) << 8;
      // Java: ++var1;
      var1 += 1;
      // Java: var2 |= var0[var1] & 255;
      var2 |= (var0[var1] as i32) & 255;
      // Java: return (short)var2;
      var2 as i16
    }

    // Java: static int bigEndianToInt(byte[] var0, int var1)
    pub fn bigEndianToInt(var0: &[u8], var1: i32) -> i32 {
      // Java: int var2 = var0[var1] << 24;
      let mut var1 = var1 as usize;
      let mut var2: i32 = (var0[var1] as i8 as i32) << 24;
      // Java: ++var1;
      var1 += 1;
      // Java: var2 |= (var0[var1] & 255) << 16;
      var2 |= ((var0[var1] as i32) & 255) << 16;
      // Java: ++var1;
      var1 += 1;
      // Java: var2 |= (var0[var1] & 255) << 8;
      var2 |= ((var0[var1] as i32) & 255) << 8;
      // Java: ++var1;
      var1 += 1;
      // Java: var2 |= var0[var1] & 255;
      var2 |= (var0[var1] as i32) & 255;
      // Java: return var2;
      var2
    }

    // Java: static void bigEndianToInt(byte[] var0, int var1, int[] var2)
    pub fn bigEndianToInt_slice(var0: &[u8], mut var1: i32, var2: &mut [i32]) {
      // Java: for(int var3 = 0; var3 < var2.length; ++var3)
      for var3 in 0..var2.len() {
         // Java: var2[var3] = bigEndianToInt(var0, var1);
         var2[var3] = Self::bigEndianToInt(var0, var1);
         // Java: var1 += 4;
         var1 += 4;
      }
    }

    // Java: static byte[] intToBigEndian(int var0)
    pub fn intToBigEndian(var0: i32) -> Vec<u8> {
      // Java: byte[] var1 = new byte[4];
      let mut var1 = vec![0u8; 4];
      // Java: intToBigEndian(var0, var1, 0);
      Self::intToBigEndian_into(var0, &mut var1, 0);
      // Java: return var1;
      var1
    }

    // Java: static void intToBigEndian(int var0, byte[] var1, int var2)
    pub fn intToBigEndian_into(var0: i32, var1: &mut [u8], var2: i32) {
      // Java: var1[var2] = (byte)(var0 >>> 24);
      let mut var2 = var2 as usize;
      var1[var2] = ((var0 as u32) >> 24) as u8;
      // Java: ++var2;
      var2 += 1;
      // Java: var1[var2] = (byte)(var0 >>> 16);
      var1[var2] = ((var0 as u32) >> 16) as u8;
      // Java: ++var2;
      var2 += 1;
      // Java: var1[var2] = (byte)(var0 >>> 8);
      var1[var2] = ((var0 as u32) >> 8) as u8;
      // Java: ++var2;
      var2 += 1;
      // Java: var1[var2] = (byte)var0;
      var1[var2] = var0 as u8;
    }

    // Java: static byte[] intToBigEndian(int[] var0)
    pub fn intToBigEndian_from_slice(var0: &[i32]) -> Vec<u8> {
      // Java: byte[] var1 = new byte[4 * var0.length];
      let mut var1 = vec![0u8; 4 * var0.len()];
      // Java: intToBigEndian(var0, var1, 0);
      Self::intToBigEndian_slice(var0, &mut var1, 0);
      // Java: return var1;
      var1
    }

    // Java: static void intToBigEndian(int[] var0, byte[] var1, int var2)
    pub fn intToBigEndian_slice(var0: &[i32], var1: &mut [u8], mut var2: i32) {
      // Java: for(int var3 = 0; var3 < var0.length; ++var3)
      for var3 in 0..var0.len() {
         // Java: intToBigEndian(var0[var3], var1, var2);
         Self::intToBigEndian_into(var0[var3], var1, var2);
         // Java: var2 += 4;
         var2 += 4;
      }
    }

    // Java: static long bigEndianToLong(byte[] var0, int var1)
    pub fn bigEndianToLong(var0: &[u8], var1: i32) -> i64 {
      // Java: int var2 = bigEndianToInt(var0, var1);
      let var2 = Self::bigEndianToInt(var0, var1);
      // Java: int var3 = bigEndianToInt(var0, var1 + 4);
      let var3 = Self::bigEndianToInt(var0, var1 + 4);
      // Java: return ((long)var2 & 4294967295L) << 32 | (long)var3 & 4294967295L;
      ((var2 as i64) & 0xFFFFFFFF) << 32 | (var3 as i64) & 0xFFFFFFFF
    }

    // Java: static void bigEndianToLong(byte[] var0, int var1, long[] var2)
    pub fn bigEndianToLong_slice(var0: &[u8], mut var1: i32, var2: &mut [i64]) {
      // Java: for(int var3 = 0; var3 < var2.length; ++var3)
      for var3 in 0..var2.len() {
         // Java: var2[var3] = bigEndianToLong(var0, var1);
         var2[var3] = Self::bigEndianToLong(var0, var1);
         // Java: var1 += 8;
         var1 += 8;
      }
    }

    // Java: static byte[] longToBigEndian(long var0)
    pub fn longToBigEndian(var0: i64) -> Vec<u8> {
      // Java: byte[] var2 = new byte[8];
      let mut var2 = vec![0u8; 8];
      // Java: longToBigEndian(var0, var2, 0);
      Self::longToBigEndian_into(var0, &mut var2, 0);
      // Java: return var2;
      var2
    }

    // Java: static void longToBigEndian(long var0, byte[] var2, int var3)
    pub fn longToBigEndian_into(var0: i64, var2: &mut [u8], var3: i32) {
      // Java: intToBigEndian((int)(var0 >>> 32), var2, var3);
      Self::intToBigEndian_into(((var0 as u64) >> 32) as i32, var2, var3);
      // Java: intToBigEndian((int)(var0 & 4294967295L), var2, var3 + 4);
      Self::intToBigEndian_into((var0 & 0xFFFFFFFF) as i32, var2, var3 + 4);
    }

    // Java: static byte[] longToBigEndian(long[] var0)
    pub fn longToBigEndian_from_slice(var0: &[i64]) -> Vec<u8> {
      // Java: byte[] var1 = new byte[8 * var0.length];
      let mut var1 = vec![0u8; 8 * var0.len()];
      // Java: longToBigEndian(var0, var1, 0);
      Self::longToBigEndian_slice(var0, &mut var1, 0);
      // Java: return var1;
      var1
    }

    // Java: static void longToBigEndian(long[] var0, byte[] var1, int var2)
    pub fn longToBigEndian_slice(var0: &[i64], var1: &mut [u8], mut var2: i32) {
      // Java: for(int var3 = 0; var3 < var0.length; ++var3)
      for var3 in 0..var0.len() {
         // Java: longToBigEndian(var0[var3], var1, var2);
         Self::longToBigEndian_into(var0[var3], var1, var2);
         // Java: var2 += 8;
         var2 += 8;
      }
    }

    // Java: static void longToBigEndian(long var0, byte[] var2, int var3, int var4)
    pub fn longToBigEndian_into_len(mut var0: i64, var2: &mut [u8], var3: i32, var4: i32) {
      // Java: for(int var5 = var4 - 1; var5 >= 0; --var5)
      for var5 in (0..var4).rev() {
         // Java: var2[var5 + var3] = (byte)((int)(var0 & 255L));
         var2[(var5 + var3) as usize] = (var0 & 255) as u8;
         // Java: var0 >>>= 8;
         var0 = ((var0 as u64) >> 8) as i64;
      }
    }

    // Java: static short littleEndianToShort(byte[] var0, int var1)
    pub fn littleEndianToShort(var0: &[u8], var1: i32) -> i16 {
      // Java: int var2 = var0[var1] & 255;
      let mut var1 = var1 as usize;
      let mut var2: i32 = (var0[var1] as i32) & 255;
      // Java: ++var1;
      var1 += 1;
      // Java: var2 |= (var0[var1] & 255) << 8;
      var2 |= ((var0[var1] as i32) & 255) << 8;
      // Java: return (short)var2;
      var2 as i16
    }

    // Java: static int littleEndianToInt(byte[] var0, int var1)
    pub fn littleEndianToInt(var0: &[u8], var1: i32) -> i32 {
      // Java: int var2 = var0[var1] & 255;
      let mut var1 = var1 as usize;
      let mut var2: i32 = (var0[var1] as i32) & 255;
      // Java: ++var1;
      var1 += 1;
      // Java: var2 |= (var0[var1] & 255) << 8;
      var2 |= ((var0[var1] as i32) & 255) << 8;
      // Java: ++var1;
      var1 += 1;
      // Java: var2 |= (var0[var1] & 255) << 16;
      var2 |= ((var0[var1] as i32) & 255) << 16;
      // Java: ++var1;
      var1 += 1;
      // Java: var2 |= var0[var1] << 24;
      var2 |= (var0[var1] as i8 as i32) << 24;
      // Java: return var2;
      var2
    }

    // Java: static void littleEndianToInt(byte[] var0, int var1, int[] var2)
    pub fn littleEndianToInt_slice(var0: &[u8], mut var1: i32, var2: &mut [i32]) {
      // Java: for(int var3 = 0; var3 < var2.length; ++var3)
      for var3 in 0..var2.len() {
         // Java: var2[var3] = littleEndianToInt(var0, var1);
         var2[var3] = Self::littleEndianToInt(var0, var1);
         // Java: var1 += 4;
         var1 += 4;
      }
    }

    // Java: static void littleEndianToInt(byte[] var0, int var1, int[] var2, int var3, int var4)
    pub fn littleEndianToInt_slice_offset(var0: &[u8], mut var1: i32, var2: &mut [i32], var3: i32, var4: i32) {
      // Java: for(int var5 = 0; var5 < var4; ++var5)
      for var5 in 0..var4 {
         // Java: var2[var3 + var5] = littleEndianToInt(var0, var1);
         var2[(var3 + var5) as usize] = Self::littleEndianToInt(var0, var1);
         // Java: var1 += 4;
         var1 += 4;
      }
    }

    // Java: static int[] littleEndianToInt(byte[] var0, int var1, int var2)
    pub fn littleEndianToInt_new(var0: &[u8], mut var1: i32, var2: i32) -> Vec<i32> {
      // Java: int[] var3 = new int[var2];
      let mut var3 = vec![0i32; var2 as usize];
      // Java: for(int var4 = 0; var4 < var3.length; ++var4)
      for var4 in 0..var3.len() {
         // Java: var3[var4] = littleEndianToInt(var0, var1);
         var3[var4] = Self::littleEndianToInt(var0, var1);
         // Java: var1 += 4;
         var1 += 4;
      }
      // Java: return var3;
      var3
    }

    // Java: static byte[] shortToLittleEndian(short var0)
    pub fn shortToLittleEndian(var0: i16) -> Vec<u8> {
      // Java: byte[] var1 = new byte[2];
      let mut var1 = vec![0u8; 2];
      // Java: shortToLittleEndian(var0, var1, 0);
      Self::shortToLittleEndian_into(var0, &mut var1, 0);
      // Java: return var1;
      var1
    }

    // Java: static void shortToLittleEndian(short var0, byte[] var1, int var2)
    pub fn shortToLittleEndian_into(var0: i16, var1: &mut [u8], var2: i32) {
      // Java: var1[var2] = (byte)var0;
      let mut var2 = var2 as usize;
      var1[var2] = var0 as u8;
      // Java: ++var2;
      var2 += 1;
      // Java: var1[var2] = (byte)(var0 >>> 8);
      var1[var2] = ((var0 as u16) >> 8) as u8;
    }

    // Java: static byte[] shortToBigEndian(short var0)
    pub fn shortToBigEndian(var0: i16) -> Vec<u8> {
      // Java: byte[] var1 = new byte[2];
      let mut var1 = vec![0u8; 2];
      // Java: shortToBigEndian(var0, var1, 0);
      Self::shortToBigEndian_into(var0, &mut var1, 0);
      // Java: return var1;
      var1
    }

    // Java: static void shortToBigEndian(short var0, byte[] var1, int var2)
    pub fn shortToBigEndian_into(var0: i16, var1: &mut [u8], var2: i32) {
      // Java: var1[var2] = (byte)(var0 >>> 8);
      let mut var2 = var2 as usize;
      var1[var2] = ((var0 as u16) >> 8) as u8;
      // Java: ++var2;
      var2 += 1;
      // Java: var1[var2] = (byte)var0;
      var1[var2] = var0 as u8;
    }

    // Java: static byte[] intToLittleEndian(int var0)
    pub fn intToLittleEndian(var0: i32) -> Vec<u8> {
      // Java: byte[] var1 = new byte[4];
      let mut var1 = vec![0u8; 4];
      // Java: intToLittleEndian(var0, var1, 0);
      Self::intToLittleEndian_into(var0, &mut var1, 0);
      // Java: return var1;
      var1
    }

    // Java: static void intToLittleEndian(int var0, byte[] var1, int var2)
    pub fn intToLittleEndian_into(var0: i32, var1: &mut [u8], var2: i32) {
      // Java: var1[var2] = (byte)var0;
      let mut var2 = var2 as usize;
      var1[var2] = var0 as u8;
      // Java: ++var2;
      var2 += 1;
      // Java: var1[var2] = (byte)(var0 >>> 8);
      var1[var2] = ((var0 as u32) >> 8) as u8;
      // Java: ++var2;
      var2 += 1;
      // Java: var1[var2] = (byte)(var0 >>> 16);
      var1[var2] = ((var0 as u32) >> 16) as u8;
      // Java: ++var2;
      var2 += 1;
      // Java: var1[var2] = (byte)(var0 >>> 24);
      var1[var2] = ((var0 as u32) >> 24) as u8;
    }

    // Java: static byte[] intToLittleEndian(int[] var0)
    pub fn intToLittleEndian_from_slice(var0: &[i32]) -> Vec<u8> {
      // Java: byte[] var1 = new byte[4 * var0.length];
      let mut var1 = vec![0u8; 4 * var0.len()];
      // Java: intToLittleEndian(var0, var1, 0);
      Self::intToLittleEndian_slice(var0, &mut var1, 0);
      // Java: return var1;
      var1
    }

    // Java: static void intToLittleEndian(int[] var0, byte[] var1, int var2)
    pub fn intToLittleEndian_slice(var0: &[i32], var1: &mut [u8], mut var2: i32) {
      // Java: for(int var3 = 0; var3 < var0.length; ++var3)
      for var3 in 0..var0.len() {
         // Java: intToLittleEndian(var0[var3], var1, var2);
         Self::intToLittleEndian_into(var0[var3], var1, var2);
         // Java: var2 += 4;
         var2 += 4;
      }
    }

    // Java: static long littleEndianToLong(byte[] var0, int var1)
    pub fn littleEndianToLong(var0: &[u8], var1: i32) -> i64 {
      // Java: int var2 = littleEndianToInt(var0, var1);
      let var2 = Self::littleEndianToInt(var0, var1);
      // Java: int var3 = littleEndianToInt(var0, var1 + 4);
      let var3 = Self::littleEndianToInt(var0, var1 + 4);
      // Java: return ((long)var3 & 4294967295L) << 32 | (long)var2 & 4294967295L;
      ((var3 as i64) & 0xFFFFFFFF) << 32 | (var2 as i64) & 0xFFFFFFFF
    }

    // Java: static void littleEndianToLong(byte[] var0, int var1, long[] var2)
    pub fn littleEndianToLong_slice(var0: &[u8], mut var1: i32, var2: &mut [i64]) {
      // Java: for(int var3 = 0; var3 < var2.length; ++var3)
      for var3 in 0..var2.len() {
         // Java: var2[var3] = littleEndianToLong(var0, var1);
         var2[var3] = Self::littleEndianToLong(var0, var1);
         // Java: var1 += 8;
         var1 += 8;
      }
    }

    // Java: static void littleEndianToLong(byte[] var0, int var1, long[] var2, int var3, int var4)
    pub fn littleEndianToLong_slice_offset(var0: &[u8], mut var1: i32, var2: &mut [i64], var3: i32, var4: i32) {
      // Java: for(int var5 = 0; var5 < var4; ++var5)
      for var5 in 0..var4 {
         // Java: var2[var3 + var5] = littleEndianToLong(var0, var1);
         var2[(var3 + var5) as usize] = Self::littleEndianToLong(var0, var1);
         // Java: var1 += 8;
         var1 += 8;
      }
    }

    // Java: static byte[] longToLittleEndian(long var0)
    pub fn longToLittleEndian(var0: i64) -> Vec<u8> {
      // Java: byte[] var2 = new byte[8];
      let mut var2 = vec![0u8; 8];
      // Java: longToLittleEndian(var0, var2, 0);
      Self::longToLittleEndian_into(var0, &mut var2, 0);
      // Java: return var2;
      var2
    }

    // Java: static void longToLittleEndian(long var0, byte[] var2, int var3)
    pub fn longToLittleEndian_into(var0: i64, var2: &mut [u8], var3: i32) {
      // Java: intToLittleEndian((int)(var0 & 4294967295L), var2, var3);
      Self::intToLittleEndian_into((var0 & 0xFFFFFFFF) as i32, var2, var3);
      // Java: intToLittleEndian((int)(var0 >>> 32), var2, var3 + 4);
      Self::intToLittleEndian_into(((var0 as u64) >> 32) as i32, var2, var3 + 4);
    }

    // Java: static byte[] longToLittleEndian(long[] var0)
    pub fn longToLittleEndian_from_slice(var0: &[i64]) -> Vec<u8> {
      // Java: byte[] var1 = new byte[8 * var0.length];
      let mut var1 = vec![0u8; 8 * var0.len()];
      // Java: longToLittleEndian(var0, var1, 0);
      Self::longToLittleEndian_slice(var0, &mut var1, 0);
      // Java: return var1;
      var1
    }

    // Java: static void longToLittleEndian(long[] var0, byte[] var1, int var2)
    pub fn longToLittleEndian_slice(var0: &[i64], var1: &mut [u8], mut var2: i32) {
      // Java: for(int var3 = 0; var3 < var0.length; ++var3)
      for var3 in 0..var0.len() {
         // Java: longToLittleEndian(var0[var3], var1, var2);
         Self::longToLittleEndian_into(var0[var3], var1, var2);
         // Java: var2 += 8;
         var2 += 8;
      }
    }

    // Java: static void longToLittleEndian(long[] var0, int var1, int var2, byte[] var3, int var4)
    pub fn longToLittleEndian_slice_offset(var0: &[i64], var1: i32, var2: i32, var3: &mut [u8], mut var4: i32) {
      // Java: for(int var5 = 0; var5 < var2; ++var5)
      for var5 in 0..var2 {
         // Java: longToLittleEndian(var0[var1 + var5], var3, var4);
         Self::longToLittleEndian_into(var0[(var1 + var5) as usize], var3, var4);
         // Java: var4 += 8;
         var4 += 8;
      }
    }

}
