// Java field: int[] inflate_mask
static inflate_mask: std::sync::OnceLock<Vec<i32>> = std::sync::OnceLock::new();
// Java field: int START
pub(crate) const START: i32 = 0;
// Java field: int LEN
pub(crate) const LEN: i32 = 1;
// Java field: int LENEXT
pub(crate) const LENEXT: i32 = 2;
// Java field: int DIST
pub(crate) const DIST: i32 = 3;
// Java field: int DISTEXT
pub(crate) const DISTEXT: i32 = 4;
// Java field: int COPY
pub(crate) const COPY: i32 = 5;
// Java field: int LIT
pub(crate) const LIT: i32 = 6;
// Java field: int WASH
pub(crate) const WASH: i32 = 7;
// Java field: int END
pub(crate) const END: i32 = 8;
// Java field: int BADCODE
pub(crate) const BADCODE: i32 = 9;

// Java: public class InfCodes
pub struct InfCodes {
    // Java field: int mode
    mode: i32,
    // Java field: int len
    len: i32,
    // Java field: int[] tree
    tree: Option<Vec<i32>>,
    // Java field: int tree_index
    tree_index: i32,
    // Java field: int need
    need: i32,
    // Java field: int lit
    lit: i32,
    // Java field: int get
    get: i32,
    // Java field: int dist
    dist: i32,
    // Java field: byte lbits
    lbits: i8,
    // Java field: byte dbits
    dbits: i8,
    // Java field: int[] ltree
    ltree: Option<Vec<i32>>,
    // Java field: int ltree_index
    ltree_index: i32,
    // Java field: int[] dtree
    dtree: Option<Vec<i32>>,
    // Java field: int dtree_index
    dtree_index: i32,
}

// Java: public class InfCodes
impl InfCodes {
    // Java: void init(int bl, int bd, int[] tl, int tl_index, int[] td, int td_index, ZStream z)
    pub fn init(&mut self, bl: i32, bd: i32, tl: &[i32], tl_index: i32, td: &[i32], td_index: i32, _z: &dyn std::any::Any) {
      // Java: this.mode = 0;
      self.mode = 0;
      // Java: this.lbits = (byte)bl;
      self.lbits = bl as i8;
      // Java: this.dbits = (byte)bd;
      self.dbits = bd as i8;
      // Java: this.ltree = tl;
      self.ltree = Some(tl.to_vec());
      // Java: this.ltree_index = tl_index;
      self.ltree_index = tl_index;
      // Java: this.dtree = td;
      self.dtree = Some(td.to_vec());
      // Java: this.dtree_index = td_index;
      self.dtree_index = td_index;
      // Java: this.tree = null;
      self.tree = None;
    }

    // Java: int proc(InfBlocks s, ZStream z, int r)
    pub fn proc(&mut self, _s: &mut dyn std::any::Any, _z: &mut dyn std::any::Any, _r: i32) -> i32 {
      // Java: int b = 0;
      // Java: int k = 0;
      // Java: int p = 0;
      // Java: p = z.next_in_index;
      // Java: int n = z.avail_in;
      // Java: b = s.bitb;
      // Java: k = s.bitk;
      // Java: int q = s.write;
      // Java: int m = q < s.read ? s.read - q - 1 : s.end - q;
      // Java: while(true)
      {
         // Java: switch (this.mode)
         {
            // Java: case 0:
               // Java: if (m >= 258 && n >= 10)
               {
                  // Java: s.bitb = b;
                  // Java: s.bitk = k;
                  // Java: z.avail_in = n;
                  // Java: z.total_in += (long)(p - z.next_in_index);
                  // Java: z.next_in_index = p;
                  // Java: s.write = q;
                  // Java: r = this.inflate_fast(this.lbits, this.dbits, this.ltree, this.ltree_index, this.dtree, this.dtree_index, s, z);
                  // Java: p = z.next_in_index;
                  // Java: n = z.avail_in;
                  // Java: b = s.bitb;
                  // Java: k = s.bitk;
                  // Java: q = s.write;
                  // Java: m = q < s.read ? s.read - q - 1 : s.end - q;
                  // Java: if (r != 0)
                  {
                     // Java: this.mode = r == 1 ? 7 : 9;
                     // Java: break;
                  }
               }
               // Java: this.need = this.lbits;
               // Java: this.tree = this.ltree;
               // Java: this.tree_index = this.ltree_index;
               // Java: this.mode = 1;
            // Java: case 1:
               // Java: int j;
               // Java: for(j = this.need; k < j; k += 8)
               {
                  // Java: if (n == 0)
                  {
                     // Java: s.bitb = b;
                     // Java: s.bitk = k;
                     // Java: z.avail_in = n;
                     // Java: z.total_in += (long)(p - z.next_in_index);
                     // Java: z.next_in_index = p;
                     // Java: s.write = q;
                     // Java: return s.inflate_flush(z, r);
                  }
                  // Java: r = 0;
                  // Java: --n;
                  // Java: b |= (z.next_in[p++] & 255) << k;
               }
               // Java: int tindex = (this.tree_index + (b & inflate_mask[j])) * 3;
               // Java: b >>>= this.tree[tindex + 1];
               // Java: k -= this.tree[tindex + 1];
               // Java: int e = this.tree[tindex];
               // Java: if (e == 0)
               {
                  // Java: this.lit = this.tree[tindex + 2];
                  // Java: this.mode = 6;
               }
               // Java: else if ((e & 16) != 0)
               {
                  // Java: this.get = e & 15;
                  // Java: this.len = this.tree[tindex + 2];
                  // Java: this.mode = 2;
               }
               // Java: else if ((e & 64) == 0)
               {
                  // Java: this.need = e;
                  // Java: this.tree_index = tindex / 3 + this.tree[tindex + 2];
               }
               // Java: else
               {
                  // Java: if ((e & 32) == 0)
                  {
                     // Java: this.mode = 9;
                     // Java: z.msg = "invalid literal/length code";
                     // Java: r = -3;
                     // Java: s.bitb = b;
                     // Java: s.bitk = k;
                     // Java: z.avail_in = n;
                     // Java: z.total_in += (long)(p - z.next_in_index);
                     // Java: z.next_in_index = p;
                     // Java: s.write = q;
                     // Java: return s.inflate_flush(z, r);
                  }
                  // Java: this.mode = 7;
               }
               // Java: break;
            // Java: case 2:
               // Java: // int j;
               // Java: for(j = this.get; k < j; k += 8)
               {
                  // Java: if (n == 0)
                  {
                     // Java: s.bitb = b;
                     // Java: s.bitk = k;
                     // Java: z.avail_in = n;
                     // Java: z.total_in += (long)(p - z.next_in_index);
                     // Java: z.next_in_index = p;
                     // Java: s.write = q;
                     // Java: return s.inflate_flush(z, r);
                  }
                  // Java: r = 0;
                  // Java: --n;
                  // Java: b |= (z.next_in[p++] & 255) << k;
               }
               // Java: this.len += b & inflate_mask[j];
               // Java: b >>= j;
               // Java: k -= j;
               // Java: this.need = this.dbits;
               // Java: this.tree = this.dtree;
               // Java: this.tree_index = this.dtree_index;
               // Java: this.mode = 3;
            // Java: case 3:
               // Java: // int j;
               // Java: for(j = this.need; k < j; k += 8)
               {
                  // Java: if (n == 0)
                  {
                     // Java: s.bitb = b;
                     // Java: s.bitk = k;
                     // Java: z.avail_in = n;
                     // Java: z.total_in += (long)(p - z.next_in_index);
                     // Java: z.next_in_index = p;
                     // Java: s.write = q;
                     // Java: return s.inflate_flush(z, r);
                  }
                  // Java: r = 0;
                  // Java: --n;
                  // Java: b |= (z.next_in[p++] & 255) << k;
               }
               // Java: tindex = (this.tree_index + (b & inflate_mask[j])) * 3;
               // Java: b >>= this.tree[tindex + 1];
               // Java: k -= this.tree[tindex + 1];
               // Java: e = this.tree[tindex];
               // Java: if ((e & 16) != 0)
               {
                  // Java: this.get = e & 15;
                  // Java: this.dist = this.tree[tindex + 2];
                  // Java: this.mode = 4;
               }
               // Java: else
               {
                  // Java: if ((e & 64) != 0)
                  {
                     // Java: this.mode = 9;
                     // Java: z.msg = "invalid distance code";
                     // Java: r = -3;
                     // Java: s.bitb = b;
                     // Java: s.bitk = k;
                     // Java: z.avail_in = n;
                     // Java: z.total_in += (long)(p - z.next_in_index);
                     // Java: z.next_in_index = p;
                     // Java: s.write = q;
                     // Java: return s.inflate_flush(z, r);
                  }
                  // Java: this.need = e;
                  // Java: this.tree_index = tindex / 3 + this.tree[tindex + 2];
               }
               // Java: break;
            // Java: case 4:
               // Java: // int j;
               // Java: for(j = this.get; k < j; k += 8)
               {
                  // Java: if (n == 0)
                  {
                     // Java: s.bitb = b;
                     // Java: s.bitk = k;
                     // Java: z.avail_in = n;
                     // Java: z.total_in += (long)(p - z.next_in_index);
                     // Java: z.next_in_index = p;
                     // Java: s.write = q;
                     // Java: return s.inflate_flush(z, r);
                  }
                  // Java: r = 0;
                  // Java: --n;
                  // Java: b |= (z.next_in[p++] & 255) << k;
               }
               // Java: this.dist += b & inflate_mask[j];
               // Java: b >>= j;
               // Java: k -= j;
               // Java: this.mode = 5;
            // Java: case 5:
               // Java: int f;
               // Java: for(f = q - this.dist; f < 0; f += s.end)
               {
               }
               // Java: for(; this.len != 0; --this.len)
               {
                  // Java: if (m == 0)
                  {
                     // Java: if (q == s.end && s.read != 0)
                     {
                        // Java: q = 0;
                        // Java: m = q < s.read ? s.read - q - 1 : s.end - q;
                     }
                     // Java: if (m == 0)
                     {
                        // Java: s.write = q;
                        // Java: r = s.inflate_flush(z, r);
                        // Java: q = s.write;
                        // Java: m = q < s.read ? s.read - q - 1 : s.end - q;
                        // Java: if (q == s.end && s.read != 0)
                        {
                           // Java: q = 0;
                           // Java: m = q < s.read ? s.read - q - 1 : s.end - q;
                        }
                        // Java: if (m == 0)
                        {
                           // Java: s.bitb = b;
                           // Java: s.bitk = k;
                           // Java: z.avail_in = n;
                           // Java: z.total_in += (long)(p - z.next_in_index);
                           // Java: z.next_in_index = p;
                           // Java: s.write = q;
                           // Java: return s.inflate_flush(z, r);
                        }
                     }
                  }
                  // Java: s.window[q++] = s.window[f++];
                  // Java: --m;
                  // Java: if (f == s.end)
                  {
                     // Java: f = 0;
                  }
               }
               // Java: this.mode = 0;
               // Java: break;
            // Java: case 6:
               // Java: if (m == 0)
               {
                  // Java: if (q == s.end && s.read != 0)
                  {
                     // Java: q = 0;
                     // Java: m = q < s.read ? s.read - q - 1 : s.end - q;
                  }
                  // Java: if (m == 0)
                  {
                     // Java: s.write = q;
                     // Java: r = s.inflate_flush(z, r);
                     // Java: q = s.write;
                     // Java: m = q < s.read ? s.read - q - 1 : s.end - q;
                     // Java: if (q == s.end && s.read != 0)
                     {
                        // Java: q = 0;
                        // Java: m = q < s.read ? s.read - q - 1 : s.end - q;
                     }
                     // Java: if (m == 0)
                     {
                        // Java: s.bitb = b;
                        // Java: s.bitk = k;
                        // Java: z.avail_in = n;
                        // Java: z.total_in += (long)(p - z.next_in_index);
                        // Java: z.next_in_index = p;
                        // Java: s.write = q;
                        // Java: return s.inflate_flush(z, r);
                     }
                  }
               }
               // Java: r = 0;
               // Java: s.window[q++] = (byte)this.lit;
               // Java: --m;
               // Java: this.mode = 0;
               // Java: break;
            // Java: case 7:
               // Java: if (k > 7)
               {
                  // Java: k -= 8;
                  // Java: ++n;
                  // Java: --p;
               }
               // Java: s.write = q;
               // Java: r = s.inflate_flush(z, r);
               // Java: q = s.write;
               // Java: if (q < s.read)
               {
                  // Java: int var10000 = s.read - q - 1;
               }
               // Java: else
               {
                  // Java: int var29 = s.end - q;
               }
               // Java: if (s.read != s.write)
               {
                  // Java: s.bitb = b;
                  // Java: s.bitk = k;
                  // Java: z.avail_in = n;
                  // Java: z.total_in += (long)(p - z.next_in_index);
                  // Java: z.next_in_index = p;
                  // Java: s.write = q;
                  // Java: return s.inflate_flush(z, r);
               }
               // Java: this.mode = 8;
            // Java: case 8:
               // Java: r = 1;
               // Java: s.bitb = b;
               // Java: s.bitk = k;
               // Java: z.avail_in = n;
               // Java: z.total_in += (long)(p - z.next_in_index);
               // Java: z.next_in_index = p;
               // Java: s.write = q;
               // Java: return s.inflate_flush(z, r);
            // Java: case 9:
               // Java: r = -3;
               // Java: s.bitb = b;
               // Java: s.bitk = k;
               // Java: z.avail_in = n;
               // Java: z.total_in += (long)(p - z.next_in_index);
               // Java: z.next_in_index = p;
               // Java: s.write = q;
               // Java: return s.inflate_flush(z, r);
            // Java: default:
               // Java: r = -2;
               // Java: s.bitb = b;
               // Java: s.bitk = k;
               // Java: z.avail_in = n;
               // Java: z.total_in += (long)(p - z.next_in_index);
               // Java: z.next_in_index = p;
               // Java: s.write = q;
               // Java: return s.inflate_flush(z, r);
         }
      }
      todo!()
    }

    // Java: void free(ZStream z)
    pub fn free(&mut self, _z: &dyn std::any::Any) {
    }

    // Java: int inflate_fast(int bl, int bd, int[] tl, int tl_index, int[] td, int td_index, InfBlocks s, ZStream z)
    pub fn inflate_fast(&mut self, _bl: i32, _bd: i32, _tl: &[i32], _tl_index: i32, _td: &[i32], _td_index: i32, _s: &mut dyn std::any::Any, _z: &mut dyn std::any::Any) -> i32 {
      // Java: int p = z.next_in_index;
      // Java: int n = z.avail_in;
      // Java: int b = s.bitb;
      // Java: int k = s.bitk;
      // Java: int q = s.write;
      // Java: int m = q < s.read ? s.read - q - 1 : s.end - q;
      // Java: int ml = inflate_mask[bl];
      // Java: int md = inflate_mask[bd];
      // Java: label141:
      // Java: do
      {
         // Java: while(k < 20)
         {
            // Java: --n;
            // Java: b |= (z.next_in[p++] & 255) << k;
            // Java: k += 8;
         }
         // Java: int t = b & ml;
         // Java: int[] tp = tl;
         // Java: int tp_index = tl_index;
         // Java: int tp_index_t_3 = (tl_index + t) * 3;
         // Java: int e;
         // Java: if ((e = tl[tp_index_t_3]) == 0)
         {
            // Java: b >>= tl[tp_index_t_3 + 1];
            // Java: k -= tl[tp_index_t_3 + 1];
            // Java: s.window[q++] = (byte)tl[tp_index_t_3 + 2];
            // Java: --m;
         }
         // Java: else
         {
            // Java: while(true)
            {
               // Java: b >>= tp[tp_index_t_3 + 1];
               // Java: k -= tp[tp_index_t_3 + 1];
               // Java: if ((e & 16) != 0)
               {
                  // Java: e &= 15;
                  // Java: int c = tp[tp_index_t_3 + 2] + (b & inflate_mask[e]);
                  // Java: b >>= e;
                  // Java: for(k -= e; k < 15; k += 8)
                  {
                     // Java: --n;
                     // Java: b |= (z.next_in[p++] & 255) << k;
                  }
                  // Java: t = b & md;
                  // Java: tp = td;
                  // Java: tp_index = td_index;
                  // Java: tp_index_t_3 = (td_index + t) * 3;
                  // Java: e = td[tp_index_t_3];
                  // Java: while(true)
                  {
                     // Java: b >>= tp[tp_index_t_3 + 1];
                     // Java: k -= tp[tp_index_t_3 + 1];
                     // Java: if ((e & 16) != 0)
                     {
                        // Java: for(e &= 15; k < e; k += 8)
                        {
                           // Java: --n;
                           // Java: b |= (z.next_in[p++] & 255) << k;
                        }
                        // Java: int d = tp[tp_index_t_3 + 2] + (b & inflate_mask[e]);
                        // Java: b >>= e;
                        // Java: k -= e;
                        // Java: m -= c;
                        // Java: int r;
                        // Java: if (q >= d)
                        {
                           // Java: r = q - d;
                           // Java: if (q - r > 0 && 2 > q - r)
                           {
                              // Java: s.window[q++] = s.window[r++];
                              // Java: s.window[q++] = s.window[r++];
                              // Java: c -= 2;
                           }
                           // Java: else
                           {
                              // Java: System.arraycopy(s.window, r, s.window, q, 2);
                              // Java: q += 2;
                              // Java: r += 2;
                              // Java: c -= 2;
                           }
                        }
                        // Java: else
                        {
                           // Java: r = q - d;
                           // Java: do
                           {
                              // Java: r += s.end;
                           }
                           // Java: e = s.end - r;
                           // Java: if (c > e)
                           {
                              // Java: c -= e;
                              // Java: if (q - r > 0 && e > q - r)
                              {
                                 // Java: do
                                 {
                                    // Java: s.window[q++] = s.window[r++];
                                    // Java: --e;
                                 }
                              }
                              // Java: else
                              {
                                 // Java: System.arraycopy(s.window, r, s.window, q, e);
                                 // Java: q += e;
                                 // Java: int var10000 = r + e;
                                 // Java: e = 0;
                              }
                              // Java: r = 0;
                           }
                        }
                        // Java: if (q - r > 0 && c > q - r)
                        {
                           // Java: while(true)
                           {
                              // Java: s.window[q++] = s.window[r++];
                              // Java: --c;
                              // Java: if (c == 0)
                              {
                                 // Java: continue label141;
                              }
                           }
                        }
                        // Java: else
                        {
                           // Java: System.arraycopy(s.window, r, s.window, q, c);
                           // Java: q += c;
                           // Java: int var62 = r + c;
                           // Java: c = 0;
                           // Java: continue label141;
                        }
                     }
                     // Java: if ((e & 64) != 0)
                     {
                        // Java: z.msg = "invalid distance code";
                        // Java: c = z.avail_in - n;
                        // Java: c = k >> 3 < c ? k >> 3 : c;
                        // Java: n += c;
                        // Java: p -= c;
                        // Java: k -= c << 3;
                        // Java: s.bitb = b;
                        // Java: s.bitk = k;
                        // Java: z.avail_in = n;
                        // Java: z.total_in += (long)(p - z.next_in_index);
                        // Java: z.next_in_index = p;
                        // Java: s.write = q;
                        // Java: return -3;
                     }
                     // Java: int var27 = t + tp[tp_index_t_3 + 2];
                     // Java: t = var27 + (b & inflate_mask[e]);
                     // Java: tp_index_t_3 = (tp_index + t) * 3;
                     // Java: e = tp[tp_index_t_3];
                  }
               }
               // Java: if ((e & 64) != 0)
               {
                  // Java: if ((e & 32) != 0)
                  {
                     // Java: int c = z.avail_in - n;
                     // Java: c = k >> 3 < c ? k >> 3 : c;
                     // Java: n += c;
                     // Java: p -= c;
                     // Java: k -= c << 3;
                     // Java: s.bitb = b;
                     // Java: s.bitk = k;
                     // Java: z.avail_in = n;
                     // Java: z.total_in += (long)(p - z.next_in_index);
                     // Java: z.next_in_index = p;
                     // Java: s.write = q;
                     // Java: return 1;
                  }
                  // Java: z.msg = "invalid literal/length code";
                  // Java: int c = z.avail_in - n;
                  // Java: c = k >> 3 < c ? k >> 3 : c;
                  // Java: n += c;
                  // Java: p -= c;
                  // Java: k -= c << 3;
                  // Java: s.bitb = b;
                  // Java: s.bitk = k;
                  // Java: z.avail_in = n;
                  // Java: z.total_in += (long)(p - z.next_in_index);
                  // Java: z.next_in_index = p;
                  // Java: s.write = q;
                  // Java: return -3;
               }
               // Java: int var25 = t + tp[tp_index_t_3 + 2];
               // Java: t = var25 + (b & inflate_mask[e]);
               // Java: tp_index_t_3 = (tp_index + t) * 3;
               // Java: if ((e = tp[tp_index_t_3]) == 0)
               {
                  // Java: b >>= tp[tp_index_t_3 + 1];
                  // Java: k -= tp[tp_index_t_3 + 1];
                  // Java: s.window[q++] = (byte)tp[tp_index_t_3 + 2];
                  // Java: --m;
                  // Java: break;
               }
            }
         }
      }
      // Java: int c = z.avail_in - n;
      // Java: c = k >> 3 < c ? k >> 3 : c;
      // Java: n += c;
      // Java: p -= c;
      // Java: k -= c << 3;
      // Java: s.bitb = b;
      // Java: s.bitk = k;
      // Java: z.avail_in = n;
      // Java: z.total_in += (long)(p - z.next_in_index);
      // Java: z.next_in_index = p;
      // Java: s.write = q;
      // Java: return 0;
      todo!()
    }

}
