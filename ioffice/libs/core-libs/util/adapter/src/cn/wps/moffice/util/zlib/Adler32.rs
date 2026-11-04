// Java field: int BASE
pub(crate) const BASE: i32 = 65521;
// Java field: int NMAX
pub(crate) const NMAX: i32 = 5552;

// Java: public class Adler32
pub struct Adler32 {
}

// Java: public class Adler32
impl Adler32 {
    // Java: long adler32(long adler, byte[] buf, int index, int len)
    pub fn adler32(&self, _adler: i64, _buf: &[u8], _index: i32, _len: i32) -> i64 {
      // Java: if (buf == null)
      {
         // Java: return 1L;
      }
      // Java: else
      {
         // Java: long s1 = adler & 65535L;
         // Java: long s2;
         // Java: for(s2 = adler >> 16 & 65535L; len > 0; s2 %= 65521L)
         {
            // Java: int k = len < 5552 ? len : 5552;
            // Java: for(len -= k; k >= 16; k -= 16)
            {
               // Java: long var26 = s1 + (long)(buf[index++] & 255);
               // Java: s2 += var26;
               // Java: long var27 = var26 + (long)(buf[index++] & 255);
               // Java: s2 += var27;
               // Java: long var28 = var27 + (long)(buf[index++] & 255);
               // Java: s2 += var28;
               // Java: long var29 = var28 + (long)(buf[index++] & 255);
               // Java: s2 += var29;
               // Java: long var30 = var29 + (long)(buf[index++] & 255);
               // Java: s2 += var30;
               // Java: long var31 = var30 + (long)(buf[index++] & 255);
               // Java: s2 += var31;
               // Java: long var32 = var31 + (long)(buf[index++] & 255);
               // Java: s2 += var32;
               // Java: long var33 = var32 + (long)(buf[index++] & 255);
               // Java: s2 += var33;
               // Java: long var34 = var33 + (long)(buf[index++] & 255);
               // Java: s2 += var34;
               // Java: long var35 = var34 + (long)(buf[index++] & 255);
               // Java: s2 += var35;
               // Java: long var36 = var35 + (long)(buf[index++] & 255);
               // Java: s2 += var36;
               // Java: long var37 = var36 + (long)(buf[index++] & 255);
               // Java: s2 += var37;
               // Java: long var38 = var37 + (long)(buf[index++] & 255);
               // Java: s2 += var38;
               // Java: long var39 = var38 + (long)(buf[index++] & 255);
               // Java: s2 += var39;
               // Java: long var40 = var39 + (long)(buf[index++] & 255);
               // Java: s2 += var40;
               // Java: s1 = var40 + (long)(buf[index++] & 255);
               // Java: s2 += s1;
            }
            // Java: if (k != 0)
            {
               // Java: do
               {
                  // Java: s1 += (long)(buf[index++] & 255);
                  // Java: s2 += s1;
                  // Java: --k;
               }
            }
            // Java: s1 %= 65521L;
         }
         // Java: return s2 << 16 | s1;
      }
      todo!()
    }

}
