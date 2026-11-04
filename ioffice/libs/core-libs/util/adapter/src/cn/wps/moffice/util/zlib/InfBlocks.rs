// Java field: int MANY
pub(crate) const MANY: i32 = 1440;
// Java field: int[] inflate_mask
static inflate_mask: std::sync::OnceLock<Vec<i32>> = std::sync::OnceLock::new();
// Java field: int[] border
static border: std::sync::OnceLock<Vec<i32>> = std::sync::OnceLock::new();
// Java field: int TYPE
pub(crate) const TYPE: i32 = 0;
// Java field: int LENS
pub(crate) const LENS: i32 = 1;
// Java field: int STORED
pub(crate) const STORED: i32 = 2;
// Java field: int TABLE
pub(crate) const TABLE: i32 = 3;
// Java field: int BTREE
pub(crate) const BTREE: i32 = 4;
// Java field: int DTREE
pub(crate) const DTREE: i32 = 5;
// Java field: int CODES
pub(crate) const CODES: i32 = 6;
// Java field: int DRY
pub(crate) const DRY: i32 = 7;
// Java field: int DONE
pub(crate) const DONE: i32 = 8;
// Java field: int BAD
pub(crate) const BAD: i32 = 9;

// Java: public class InfBlocks
pub struct InfBlocks {
    // Java field: int mode
    mode: i32,
    // Java field: int left
    left: i32,
    // Java field: int table
    table: i32,
    // Java field: int index
    index: i32,
    // Java field: int[] blens
    blens: Option<Vec<i32>>,
    // Java field: int[] bb
    bb: Option<Vec<i32>>,
    // Java field: int[] tb
    tb: Option<Vec<i32>>,
    // Java field: InfCodes codes
    codes: Option<Box<dyn std::any::Any>>,
    // Java field: int last
    last: i32,
    // Java field: int bitk
    bitk: i32,
    // Java field: int bitb
    bitb: i32,
    // Java field: int[] hufts
    hufts: Option<Vec<i32>>,
    // Java field: byte[] window
    window: Option<Vec<u8>>,
    // Java field: int end
    end: i32,
    // Java field: int read
    read: i32,
    // Java field: int write
    write: i32,
    // Java field: Object checkfn
    checkfn: Option<Box<dyn std::any::Any>>,
    // Java field: long check
    check: i64,
    // Java field: InfTree inftree
    inftree: Option<Box<dyn std::any::Any>>,
}

// Java: public class InfBlocks
impl InfBlocks {
    // Java: void reset(ZStream z, long[] c)
    pub fn reset(&mut self, _z: &mut dyn std::any::Any, _c: Option<&mut [i64]>) {
      // Java: if (c != null)
      {
         // Java: c[0] = this.check;
      }
      // Java: if (this.mode != 4 && this.mode == 5)
      {
      }
      // Java: if (this.mode == 6)
      {
         // Java: this.codes.free(z);
      }
      // Java: this.mode = 0;
      // Java: this.bitk = 0;
      // Java: this.bitb = 0;
      // Java: this.read = this.write = 0;
      // Java: if (this.checkfn != null)
      {
         // Java: z.adler = this.check = z._adler.adler32(0L, (byte[])null, 0, 0);
      }
    }

    // Java: int proc(ZStream z, int r)
    pub fn proc(&mut self, _z: &mut dyn std::any::Any, _r: i32) -> i32 {
      // Java: int p = z.next_in_index;
      // Java: int n = z.avail_in;
      // Java: int b = this.bitb;
      // Java: int k = this.bitk;
      // Java: int q = this.write;
      // Java: int m = q < this.read ? this.read - q - 1 : this.end - q;
      // Java: label335:
      // Java: while(true)
      {
         // Java: label328:
         // Java: while(true)
         {
            // Java: label326:
            {
               // Java: label405:
               {
                  // Java: switch (this.mode)
                  {
                     // Java: case 0:
                        // Java: while(k < 3)
                        {
                           // Java: if (n == 0)
                           {
                              // Java: this.bitb = b;
                              // Java: this.bitk = k;
                              // Java: z.avail_in = n;
                              // Java: z.total_in += (long)(p - z.next_in_index);
                              // Java: z.next_in_index = p;
                              // Java: this.write = q;
                              // Java: return this.inflate_flush(z, r);
                           }
                           // Java: r = 0;
                           // Java: --n;
                           // Java: b |= (z.next_in[p++] & 255) << k;
                           // Java: k += 8;
                        }
                        // Java: int t = b & 7;
                        // Java: this.last = t & 1;
                        // Java: switch (t >>> 1)
                        {
                           // Java: case 0:
                              // Java: b >>>= 3;
                              // Java: k -= 3;
                              // Java: t = k & 7;
                              // Java: b >>>= t;
                              // Java: k -= t;
                              // Java: this.mode = 1;
                              // Java: continue;
                           // Java: case 1:
                              // Java: int[] bl = new int[1];
                              // Java: int[] bd = new int[1];
                              // Java: int[][] tl = new int[1][];
                              // Java: int[][] td = new int[1][];
                              // Java: InfTree.inflate_trees_fixed(bl, bd, tl, td, z);
                              // Java: this.codes.init(bl[0], bd[0], tl[0], 0, td[0], 0, z);
                              // Java: b >>>= 3;
                              // Java: k -= 3;
                              // Java: this.mode = 6;
                              // Java: continue;
                           // Java: case 2:
                              // Java: b >>>= 3;
                              // Java: k -= 3;
                              // Java: this.mode = 3;
                              // Java: continue;
                           // Java: case 3:
                              // Java: b >>>= 3;
                              // Java: k -= 3;
                              // Java: this.mode = 9;
                              // Java: z.msg = "invalid block type";
                              // Java: r = -3;
                              // Java: this.bitb = b;
                              // Java: this.bitk = k;
                              // Java: z.avail_in = n;
                              // Java: z.total_in += (long)(p - z.next_in_index);
                              // Java: z.next_in_index = p;
                              // Java: this.write = q;
                              // Java: return this.inflate_flush(z, r);
                        }
                     // Java: case 1:
                        // Java: while(k < 32)
                        {
                           // Java: if (n == 0)
                           {
                              // Java: this.bitb = b;
                              // Java: this.bitk = k;
                              // Java: z.avail_in = n;
                              // Java: z.total_in += (long)(p - z.next_in_index);
                              // Java: z.next_in_index = p;
                              // Java: this.write = q;
                              // Java: return this.inflate_flush(z, r);
                           }
                           // Java: r = 0;
                           // Java: --n;
                           // Java: b |= (z.next_in[p++] & 255) << k;
                           // Java: k += 8;
                        }
                        // Java: if ((~b >>> 16 & '\uffff') != (b & '\uffff'))
                        {
                           // Java: this.mode = 9;
                           // Java: z.msg = "invalid stored block lengths";
                           // Java: r = -3;
                           // Java: this.bitb = b;
                           // Java: this.bitk = k;
                           // Java: z.avail_in = n;
                           // Java: z.total_in += (long)(p - z.next_in_index);
                           // Java: z.next_in_index = p;
                           // Java: this.write = q;
                           // Java: return this.inflate_flush(z, r);
                        }
                        // Java: this.left = b & '\uffff';
                        // Java: k = 0;
                        // Java: b = 0;
                        // Java: this.mode = this.left != 0 ? 2 : (this.last != 0 ? 7 : 0);
                        // Java: continue;
                     // Java: case 2:
                        // Java: if (n == 0)
                        {
                           // Java: this.bitb = b;
                           // Java: this.bitk = k;
                           // Java: z.avail_in = n;
                           // Java: z.total_in += (long)(p - z.next_in_index);
                           // Java: z.next_in_index = p;
                           // Java: this.write = q;
                           // Java: return this.inflate_flush(z, r);
                        }
                        // Java: if (m == 0)
                        {
                           // Java: if (q == this.end && this.read != 0)
                           {
                              // Java: q = 0;
                              // Java: m = q < this.read ? this.read - q - 1 : this.end - q;
                           }
                           // Java: if (m == 0)
                           {
                              // Java: this.write = q;
                              // Java: r = this.inflate_flush(z, r);
                              // Java: q = this.write;
                              // Java: m = q < this.read ? this.read - q - 1 : this.end - q;
                              // Java: if (q == this.end && this.read != 0)
                              {
                                 // Java: q = 0;
                                 // Java: m = q < this.read ? this.read - q - 1 : this.end - q;
                              }
                              // Java: if (m == 0)
                              {
                                 // Java: this.bitb = b;
                                 // Java: this.bitk = k;
                                 // Java: z.avail_in = n;
                                 // Java: z.total_in += (long)(p - z.next_in_index);
                                 // Java: z.next_in_index = p;
                                 // Java: this.write = q;
                                 // Java: return this.inflate_flush(z, r);
                              }
                           }
                        }
                        // Java: r = 0;
                        // Java: t = this.left;
                        // Java: if (t > n)
                        {
                           // Java: t = n;
                        }
                        // Java: if (t > m)
                        {
                           // Java: t = m;
                        }
                        // Java: System.arraycopy(z.next_in, p, this.window, q, t);
                        // Java: p += t;
                        // Java: n -= t;
                        // Java: q += t;
                        // Java: m -= t;
                        // Java: if ((this.left -= t) == 0)
                        {
                           // Java: this.mode = this.last != 0 ? 7 : 0;
                        }
                        // Java: continue;
                     // Java: case 3:
                        // Java: while(k < 14)
                        {
                           // Java: if (n == 0)
                           {
                              // Java: this.bitb = b;
                              // Java: this.bitk = k;
                              // Java: z.avail_in = n;
                              // Java: z.total_in += (long)(p - z.next_in_index);
                              // Java: z.next_in_index = p;
                              // Java: this.write = q;
                              // Java: return this.inflate_flush(z, r);
                           }
                           // Java: r = 0;
                           // Java: --n;
                           // Java: b |= (z.next_in[p++] & 255) << k;
                           // Java: k += 8;
                        }
                        // Java: // int t;
                        // Java: this.table = t = b & 16383;
                        // Java: if ((t & 31) > 29 || (t >> 5 & 31) > 29)
                        {
                           // Java: this.mode = 9;
                           // Java: z.msg = "too many length or distance symbols";
                           // Java: r = -3;
                           // Java: this.bitb = b;
                           // Java: this.bitk = k;
                           // Java: z.avail_in = n;
                           // Java: z.total_in += (long)(p - z.next_in_index);
                           // Java: z.next_in_index = p;
                           // Java: this.write = q;
                           // Java: return this.inflate_flush(z, r);
                        }
                        // Java: t = 258 + (t & 31) + (t >> 5 & 31);
                        // Java: if (this.blens != null && this.blens.length >= t)
                        {
                           // Java: for(int i = 0; i < t; ++i)
                           {
                              // Java: this.blens[i] = 0;
                           }
                        }
                        // Java: else
                        {
                           // Java: this.blens = new int[t];
                        }
                        // Java: b >>>= 14;
                        // Java: k -= 14;
                        // Java: this.index = 0;
                        // Java: this.mode = 4;
                     // Java: case 4:
                        // Java: break;
                     // Java: case 5:
                        // Java: break label405;
                     // Java: case 6:
                        // Java: break label326;
                     // Java: case 7:
                        // Java: break label328;
                     // Java: case 8:
                        // Java: break label335;
                     // Java: case 9:
                        // Java: r = -3;
                        // Java: this.bitb = b;
                        // Java: this.bitk = k;
                        // Java: z.avail_in = n;
                        // Java: z.total_in += (long)(p - z.next_in_index);
                        // Java: z.next_in_index = p;
                        // Java: this.write = q;
                        // Java: return this.inflate_flush(z, r);
                     // Java: default:
                        // Java: r = -2;
                        // Java: this.bitb = b;
                        // Java: this.bitk = k;
                        // Java: z.avail_in = n;
                        // Java: z.total_in += (long)(p - z.next_in_index);
                        // Java: z.next_in_index = p;
                        // Java: this.write = q;
                        // Java: return this.inflate_flush(z, r);
                  }
                  // Java: while(this.index < 4 + (this.table >>> 10))
                  {
                     // Java: while(k < 3)
                     {
                        // Java: if (n == 0)
                        {
                           // Java: this.bitb = b;
                           // Java: this.bitk = k;
                           // Java: z.avail_in = n;
                           // Java: z.total_in += (long)(p - z.next_in_index);
                           // Java: z.next_in_index = p;
                           // Java: this.write = q;
                           // Java: return this.inflate_flush(z, r);
                        }
                        // Java: r = 0;
                        // Java: --n;
                        // Java: b |= (z.next_in[p++] & 255) << k;
                        // Java: k += 8;
                     }
                     // Java: this.blens[border[this.index++]] = b & 7;
                     // Java: b >>>= 3;
                     // Java: k -= 3;
                  }
                  // Java: while(this.index < 19)
                  {
                     // Java: this.blens[border[this.index++]] = 0;
                  }
                  // Java: this.bb[0] = 7;
                  // Java: int t = this.inftree.inflate_trees_bits(this.blens, this.bb, this.tb, this.hufts, z);
                  // Java: if (t != 0)
                  {
                     // Java: if (t == -3)
                     {
                        // Java: this.blens = null;
                        // Java: this.mode = 9;
                     }
                     // Java: this.bitb = b;
                     // Java: this.bitk = k;
                     // Java: z.avail_in = n;
                     // Java: z.total_in += (long)(p - z.next_in_index);
                     // Java: z.next_in_index = p;
                     // Java: this.write = q;
                     // Java: return this.inflate_flush(z, t);
                  }
                  // Java: this.index = 0;
                  // Java: this.mode = 5;
               }
               // Java: while(true)
               {
                  // Java: int t = this.table;
                  // Java: if (this.index >= 258 + (t & 31) + (t >> 5 & 31))
                  {
                     // Java: this.tb[0] = -1;
                     // Java: int[] bl = new int[1];
                     // Java: int[] bd = new int[1];
                     // Java: int[] tl = new int[1];
                     // Java: int[] td = new int[1];
                     // Java: bl[0] = 9;
                     // Java: bd[0] = 6;
                     // Java: t = this.table;
                     // Java: t = this.inftree.inflate_trees_dynamic(257 + (t & 31), 1 + (t >> 5 & 31), this.blens, bl, bd, tl, td, this.hufts, z);
                     // Java: if (t != 0)
                     {
                        // Java: if (t == -3)
                        {
                           // Java: this.blens = null;
                           // Java: this.mode = 9;
                        }
                        // Java: this.bitb = b;
                        // Java: this.bitk = k;
                        // Java: z.avail_in = n;
                        // Java: z.total_in += (long)(p - z.next_in_index);
                        // Java: z.next_in_index = p;
                        // Java: this.write = q;
                        // Java: return this.inflate_flush(z, t);
                     }
                     // Java: this.codes.init(bl[0], bd[0], this.hufts, tl[0], this.hufts, td[0], z);
                     // Java: this.mode = 6;
                     // Java: break;
                  }
                  // Java: for(t = this.bb[0]; k < t; k += 8)
                  {
                     // Java: if (n == 0)
                     {
                        // Java: this.bitb = b;
                        // Java: this.bitk = k;
                        // Java: z.avail_in = n;
                        // Java: z.total_in += (long)(p - z.next_in_index);
                        // Java: z.next_in_index = p;
                        // Java: this.write = q;
                        // Java: return this.inflate_flush(z, r);
                     }
                     // Java: r = 0;
                     // Java: --n;
                     // Java: b |= (z.next_in[p++] & 255) << k;
                  }
                  // Java: if (this.tb[0] == -1)
                  {
                  }
                  // Java: t = this.hufts[(this.tb[0] + (b & inflate_mask[t])) * 3 + 1];
                  // Java: int c = this.hufts[(this.tb[0] + (b & inflate_mask[t])) * 3 + 2];
                  // Java: if (c < 16)
                  {
                     // Java: b >>>= t;
                     // Java: k -= t;
                     // Java: this.blens[this.index++] = c;
                  }
                  // Java: else
                  {
                     // Java: int i = c == 18 ? 7 : c - 14;
                     // Java: int j;
                     // Java: for(j = c == 18 ? 11 : 3; k < t + i; k += 8)
                     {
                        // Java: if (n == 0)
                        {
                           // Java: this.bitb = b;
                           // Java: this.bitk = k;
                           // Java: z.avail_in = n;
                           // Java: z.total_in += (long)(p - z.next_in_index);
                           // Java: z.next_in_index = p;
                           // Java: this.write = q;
                           // Java: return this.inflate_flush(z, r);
                        }
                        // Java: r = 0;
                        // Java: --n;
                        // Java: b |= (z.next_in[p++] & 255) << k;
                     }
                     // Java: b >>>= t;
                     // Java: k -= t;
                     // Java: j += b & inflate_mask[i];
                     // Java: b >>>= i;
                     // Java: k -= i;
                     // Java: i = this.index;
                     // Java: t = this.table;
                     // Java: if (i + j > 258 + (t & 31) + (t >> 5 & 31) || c == 16 && i < 1)
                     {
                        // Java: this.blens = null;
                        // Java: this.mode = 9;
                        // Java: z.msg = "invalid bit length repeat";
                        // Java: r = -3;
                        // Java: this.bitb = b;
                        // Java: this.bitk = k;
                        // Java: z.avail_in = n;
                        // Java: z.total_in += (long)(p - z.next_in_index);
                        // Java: z.next_in_index = p;
                        // Java: this.write = q;
                        // Java: return this.inflate_flush(z, r);
                     }
                     // Java: c = c == 16 ? this.blens[i - 1] : 0;
                     // Java: do
                     {
                        // Java: this.blens[i++] = c;
                        // Java: --j;
                     }
                     // Java: this.index = i;
                  }
               }
            }
            // Java: this.bitb = b;
            // Java: this.bitk = k;
            // Java: z.avail_in = n;
            // Java: z.total_in += (long)(p - z.next_in_index);
            // Java: z.next_in_index = p;
            // Java: this.write = q;
            // Java: if ((r = this.codes.proc(this, z, r)) != 1)
            {
               // Java: return this.inflate_flush(z, r);
            }
            // Java: r = 0;
            // Java: this.codes.free(z);
            // Java: p = z.next_in_index;
            // Java: n = z.avail_in;
            // Java: b = this.bitb;
            // Java: k = this.bitk;
            // Java: q = this.write;
            // Java: m = q < this.read ? this.read - q - 1 : this.end - q;
            // Java: if (this.last != 0)
            {
               // Java: this.mode = 7;
               // Java: break;
            }
            // Java: this.mode = 0;
         }
         // Java: this.write = q;
         // Java: r = this.inflate_flush(z, r);
         // Java: q = this.write;
         // Java: if (q < this.read)
         {
            // Java: int var10000 = this.read - q - 1;
         }
         // Java: else
         {
            // Java: int var52 = this.end - q;
         }
         // Java: if (this.read != this.write)
         {
            // Java: this.bitb = b;
            // Java: this.bitk = k;
            // Java: z.avail_in = n;
            // Java: z.total_in += (long)(p - z.next_in_index);
            // Java: z.next_in_index = p;
            // Java: this.write = q;
            // Java: return this.inflate_flush(z, r);
         }
         // Java: this.mode = 8;
         // Java: break;
      }
      // Java: r = 1;
      // Java: this.bitb = b;
      // Java: this.bitk = k;
      // Java: z.avail_in = n;
      // Java: z.total_in += (long)(p - z.next_in_index);
      // Java: z.next_in_index = p;
      // Java: this.write = q;
      // Java: return this.inflate_flush(z, r);
      todo!()
    }

    // Java: void free(ZStream z)
    pub fn free(&mut self, _z: &dyn std::any::Any) {
      // Java: this.reset(z, (long[])null);
      // Java: this.window = null;
      // Java: this.hufts = null;
    }

    // Java: void set_dictionary(byte[] d, int start, int n)
    pub fn set_dictionary(&mut self, _d: &[u8], _start: i32, _n: i32) {
      // Java: System.arraycopy(d, start, this.window, 0, n);
      // Java: this.read = this.write = n;
    }

    // Java: int sync_point()
    pub fn sync_point(&self) -> i32 {
      // Java: return this.mode == 1 ? 1 : 0;
      todo!()
    }

    // Java: int inflate_flush(ZStream z, int r)
    pub fn inflate_flush(&mut self, _z: &mut dyn std::any::Any, _r: i32) -> i32 {
      // Java: int p = z.next_out_index;
      // Java: int q = this.read;
      // Java: int n = (q <= this.write ? this.write : this.end) - q;
      // Java: if (n > z.avail_out)
      {
         // Java: n = z.avail_out;
      }
      // Java: if (n != 0 && r == -5)
      {
         // Java: r = 0;
      }
      // Java: z.avail_out -= n;
      // Java: z.total_out += (long)n;
      // Java: if (this.checkfn != null)
      {
         // Java: z.adler = this.check = z._adler.adler32(this.check, this.window, q, n);
      }
      // Java: System.arraycopy(this.window, q, z.next_out, p, n);
      // Java: p += n;
      // Java: q += n;
      // Java: if (q == this.end)
      {
         // Java: q = 0;
         // Java: if (this.write == this.end)
         {
            // Java: this.write = 0;
         }
         // Java: n = this.write - q;
         // Java: if (n > z.avail_out)
         {
            // Java: n = z.avail_out;
         }
         // Java: if (n != 0 && r == -5)
         {
            // Java: r = 0;
         }
         // Java: z.avail_out -= n;
         // Java: z.total_out += (long)n;
         // Java: if (this.checkfn != null)
         {
            // Java: z.adler = this.check = z._adler.adler32(this.check, this.window, q, n);
         }
         // Java: System.arraycopy(this.window, q, z.next_out, p, n);
         // Java: p += n;
         // Java: q += n;
      }
      // Java: z.next_out_index = p;
      // Java: this.read = q;
      // Java: return r;
      todo!()
    }

}
