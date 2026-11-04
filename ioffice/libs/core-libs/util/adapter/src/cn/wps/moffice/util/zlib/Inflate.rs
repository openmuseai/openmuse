// Java field: int PRESET_DICT
pub(crate) const PRESET_DICT: i32 = 32;
// Java field: int Z_DEFLATED
pub(crate) const Z_DEFLATED: i32 = 8;
// Java field: int METHOD
pub(crate) const METHOD: i32 = 0;
// Java field: int FLAG
pub(crate) const FLAG: i32 = 1;
// Java field: int DICT4
pub(crate) const DICT4: i32 = 2;
// Java field: int DICT3
pub(crate) const DICT3: i32 = 3;
// Java field: int DICT2
pub(crate) const DICT2: i32 = 4;
// Java field: int DICT1
pub(crate) const DICT1: i32 = 5;
// Java field: int DICT0
pub(crate) const DICT0: i32 = 6;
// Java field: int BLOCKS
pub(crate) const BLOCKS: i32 = 7;
// Java field: int CHECK4
pub(crate) const CHECK4: i32 = 8;
// Java field: int CHECK3
pub(crate) const CHECK3: i32 = 9;
// Java field: int CHECK2
pub(crate) const CHECK2: i32 = 10;
// Java field: int CHECK1
pub(crate) const CHECK1: i32 = 11;
// Java field: int DONE
pub(crate) const DONE: i32 = 12;
// Java field: int BAD
pub(crate) const BAD: i32 = 13;
// Java field: byte[] mark
static mark: std::sync::OnceLock<std::sync::Mutex<Vec<u8>>> = std::sync::OnceLock::new();

// Java: public class Inflate
pub struct Inflate {
    // Java field: int mode
    mode: i32,
    // Java field: int method
    method: i32,
    // Java field: long[] was
    was: Option<Vec<i64>>,
    // Java field: long need
    need: i64,
    // Java field: int marker
    marker: i32,
    // Java field: int nowrap
    nowrap: i32,
    // Java field: int wbits
    wbits: i32,
    // Java field: InfBlocks blocks
    blocks: Option<Box<dyn std::any::Any>>,
}

// Java field: byte[] mark
static MARK: &[u8] = &[0, 0, 0xFF, 0xFF];

// Java: public class Inflate
impl Inflate {
    // Java: int inflateReset(ZStream z)
    pub fn inflateReset(&mut self, _z: &mut dyn std::any::Any) -> i32 {
      // Java: if (z != null && z.istate != null)
      {
         // Java: z.total_in = z.total_out = 0L;
         // Java: z.msg = null;
         // Java: z.istate.mode = z.istate.nowrap != 0 ? 7 : 0;
         // Java: z.istate.blocks.reset(z, (long[])null);
         // Java: return 0;
      }
      // Java: else
      {
         // Java: return -2;
      }
      todo!()
    }

    // Java: int inflateEnd(ZStream z)
    pub fn inflateEnd(&mut self, _z: &mut dyn std::any::Any) -> i32 {
      // Java: if (this.blocks != null)
      {
         // Java: this.blocks.free(z);
      }
      // Java: this.blocks = null;
      // Java: return 0;
      todo!()
    }

    // Java: int inflateInit(ZStream z, int w)
    pub fn inflateInit(&mut self, _z: &mut dyn std::any::Any, _w: i32) -> i32 {
      // Java: z.msg = null;
      // Java: this.blocks = null;
      // Java: this.nowrap = 0;
      // Java: if (w < 0)
      {
         // Java: w = -w;
         // Java: this.nowrap = 1;
      }
      // Java: if (w >= 8 && w <= 15)
      {
         // Java: this.wbits = w;
         // Java: z.istate.blocks = new InfBlocks(z, z.istate.nowrap != 0 ? null : this, 1 << w);
         // Java: this.inflateReset(z);
         // Java: return 0;
      }
      // Java: else
      {
         // Java: this.inflateEnd(z);
         // Java: return -2;
      }
      todo!()
    }

    // Java: int inflate(ZStream z, int f)
    pub fn inflate(&mut self, _z: &mut dyn std::any::Any, _f: i32) -> i32 {
      // Java: if (z != null && z.istate != null && z.next_in != null)
      {
         // Java: f = f == 4 ? -5 : 0;
         // Java: int r = -5;
         // Java: while(true)
         {
            // Java: switch (z.istate.mode)
            {
               // Java: case 0:
                  // Java: if (z.avail_in == 0)
                  {
                     // Java: return r;
                  }
                  // Java: r = f;
                  // Java: --z.avail_in;
                  // Java: ++z.total_in;
                  // Java: if (((z.istate.method = z.next_in[z.next_in_index++]) & 15) != 8)
                  {
                     // Java: z.istate.mode = 13;
                     // Java: z.msg = "unknown compression method";
                     // Java: z.istate.marker = 5;
                     // Java: break;
                  }
                  // Java: else if ((z.istate.method >> 4) + 8 > z.istate.wbits)
                  {
                     // Java: z.istate.mode = 13;
                     // Java: z.msg = "invalid window size";
                     // Java: z.istate.marker = 5;
                     // Java: break;
                  }
                  // Java: else
                  {
                     // Java: z.istate.mode = 1;
                  }
               // Java: case 1:
                  // Java: if (z.avail_in == 0)
                  {
                     // Java: return r;
                  }
                  // Java: r = f;
                  // Java: --z.avail_in;
                  // Java: ++z.total_in;
                  // Java: int b = z.next_in[z.next_in_index++] & 255;
                  // Java: if (((z.istate.method << 8) + b) % 31 != 0)
                  {
                     // Java: z.istate.mode = 13;
                     // Java: z.msg = "incorrect header check";
                     // Java: z.istate.marker = 5;
                     // Java: break;
                  }
                  // Java: else if ((b & 32) == 0)
                  {
                     // Java: z.istate.mode = 7;
                     // Java: break;
                  }
                  // Java: else
                  {
                     // Java: z.istate.mode = 2;
                  }
               // Java: case 2:
                  // Java: if (z.avail_in == 0)
                  {
                     // Java: return r;
                  }
                  // Java: r = f;
                  // Java: --z.avail_in;
                  // Java: ++z.total_in;
                  // Java: z.istate.need = (long)((z.next_in[z.next_in_index++] & 255) << 24) & 4278190080L;
                  // Java: z.istate.mode = 3;
               // Java: case 3:
                  // Java: if (z.avail_in == 0)
                  {
                     // Java: return r;
                  }
                  // Java: r = f;
                  // Java: --z.avail_in;
                  // Java: ++z.total_in;
                  // Java: Inflate var8 = z.istate;
                  // Java: var8.need += (long)((z.next_in[z.next_in_index++] & 255) << 16) & 16711680L;
                  // Java: z.istate.mode = 4;
               // Java: case 4:
                  // Java: if (z.avail_in == 0)
                  {
                     // Java: return r;
                  }
                  // Java: r = f;
                  // Java: --z.avail_in;
                  // Java: ++z.total_in;
                  // Java: Inflate var9 = z.istate;
                  // Java: var9.need += (long)((z.next_in[z.next_in_index++] & 255) << 8) & 65280L;
                  // Java: z.istate.mode = 5;
               // Java: case 5:
                  // Java: if (z.avail_in == 0)
                  {
                     // Java: return r;
                  }
                  // Java: --z.avail_in;
                  // Java: ++z.total_in;
                  // Java: Inflate var10 = z.istate;
                  // Java: var10.need += (long)z.next_in[z.next_in_index++] & 255L;
                  // Java: z.adler = z.istate.need;
                  // Java: z.istate.mode = 6;
                  // Java: return 2;
               // Java: case 6:
                  // Java: z.istate.mode = 13;
                  // Java: z.msg = "need dictionary";
                  // Java: z.istate.marker = 0;
                  // Java: return -2;
               // Java: case 7:
                  // Java: r = z.istate.blocks.proc(z, r);
                  // Java: if (r == -3)
                  {
                     // Java: z.istate.mode = 13;
                     // Java: z.istate.marker = 0;
                     // Java: break;
                  }
                  // Java: else
                  {
                     // Java: if (r == 0)
                     {
                        // Java: r = f;
                     }
                     // Java: if (r != 1)
                     {
                        // Java: return r;
                     }
                     // Java: r = f;
                     // Java: z.istate.blocks.reset(z, z.istate.was);
                     // Java: if (z.istate.nowrap != 0)
                     {
                        // Java: z.istate.mode = 12;
                        // Java: break;
                     }
                     // Java: else
                     {
                        // Java: z.istate.mode = 8;
                     }
                  }
               // Java: case 8:
                  // Java: if (z.avail_in == 0)
                  {
                     // Java: return r;
                  }
                  // Java: r = f;
                  // Java: --z.avail_in;
                  // Java: ++z.total_in;
                  // Java: z.istate.need = (long)((z.next_in[z.next_in_index++] & 255) << 24) & 4278190080L;
                  // Java: z.istate.mode = 9;
               // Java: case 9:
                  // Java: if (z.avail_in == 0)
                  {
                     // Java: return r;
                  }
                  // Java: r = f;
                  // Java: --z.avail_in;
                  // Java: ++z.total_in;
                  // Java: Inflate var10000 = z.istate;
                  // Java: var10000.need += (long)((z.next_in[z.next_in_index++] & 255) << 16) & 16711680L;
                  // Java: z.istate.mode = 10;
               // Java: case 10:
                  // Java: if (z.avail_in == 0)
                  {
                     // Java: return r;
                  }
                  // Java: r = f;
                  // Java: --z.avail_in;
                  // Java: ++z.total_in;
                  // Java: Inflate var6 = z.istate;
                  // Java: var6.need += (long)((z.next_in[z.next_in_index++] & 255) << 8) & 65280L;
                  // Java: z.istate.mode = 11;
               // Java: case 11:
                  // Java: if (z.avail_in == 0)
                  {
                     // Java: return r;
                  }
                  // Java: r = f;
                  // Java: --z.avail_in;
                  // Java: ++z.total_in;
                  // Java: Inflate var7 = z.istate;
                  // Java: var7.need += (long)z.next_in[z.next_in_index++] & 255L;
                  // Java: if ((int)z.istate.was[0] != (int)z.istate.need)
                  {
                     // Java: z.istate.mode = 13;
                     // Java: z.msg = "incorrect data check";
                     // Java: z.istate.marker = 5;
                     // Java: break;
                  }
                  // Java: else
                  {
                     // Java: z.istate.mode = 12;
                  }
               // Java: case 12:
                  // Java: return 1;
               // Java: case 13:
                  // Java: return -3;
               // Java: default:
                  // Java: return -2;
            }
         }
      }
      // Java: else
      {
         // Java: return -2;
      }
      todo!()
    }

    // Java: int inflateSetDictionary(ZStream z, byte[] dictionary, int dictLength)
    pub fn inflateSetDictionary(&mut self, _z: &mut dyn std::any::Any, _dictionary: &[u8], _dictLength: i32) -> i32 {
      // Java: int index = 0;
      // Java: int length = dictLength;
      // Java: if (z != null && z.istate != null && z.istate.mode == 6)
      {
         // Java: if (z._adler.adler32(1L, dictionary, 0, dictLength) != z.adler)
         {
            // Java: return -3;
         }
         // Java: else
         {
            // Java: z.adler = z._adler.adler32(0L, (byte[])null, 0, 0);
            // Java: if (dictLength >= 1 << z.istate.wbits)
            {
               // Java: length = (1 << z.istate.wbits) - 1;
               // Java: index = dictLength - length;
            }
            // Java: z.istate.blocks.set_dictionary(dictionary, index, length);
            // Java: z.istate.mode = 7;
            // Java: return 0;
         }
      }
      // Java: else
      {
         // Java: return -2;
      }
      todo!()
    }

    // Java: int inflateSync(ZStream z)
    pub fn inflateSync(&mut self, _z: &mut dyn std::any::Any) -> i32 {
      // Java: if (z != null && z.istate != null)
      {
         // Java: if (z.istate.mode != 13)
         {
            // Java: z.istate.mode = 13;
            // Java: z.istate.marker = 0;
         }
         // Java: int n;
         // Java: if ((n = z.avail_in) == 0)
         {
            // Java: return -5;
         }
         // Java: else
         {
            // Java: int p = z.next_in_index;
            // Java: int m;
            // Java: for(m = z.istate.marker; n != 0 && m < 4; --n)
            {
               // Java: if (z.next_in[p] == mark[m])
               {
                  // Java: ++m;
               }
               // Java: else if (z.next_in[p] != 0)
               {
                  // Java: m = 0;
               }
               // Java: else
               {
                  // Java: m = 4 - m;
               }
               // Java: ++p;
            }
            // Java: z.total_in += (long)(p - z.next_in_index);
            // Java: z.next_in_index = p;
            // Java: z.avail_in = n;
            // Java: z.istate.marker = m;
            // Java: if (m != 4)
            {
               // Java: return -3;
            }
            // Java: else
            {
               // Java: long r = z.total_in;
               // Java: long w = z.total_out;
               // Java: this.inflateReset(z);
               // Java: z.total_in = r;
               // Java: z.total_out = w;
               // Java: z.istate.mode = 7;
               // Java: return 0;
            }
         }
      }
      // Java: else
      {
         // Java: return -2;
      }
      todo!()
    }

    // Java: int inflateSyncPoint(ZStream z)
    pub fn inflateSyncPoint(&self, _z: &dyn std::any::Any) -> i32 {
      // Java: return z != null && z.istate != null && z.istate.blocks != null ? z.istate.blocks.sync_point() : -2;
      todo!()
    }

}
