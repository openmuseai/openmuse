use super::StaticTree::StaticTree;

// Java field: int MAX_BITS
pub(crate) const MAX_BITS: i32 = 15;
// Java field: int LITERALS
pub(crate) const LITERALS: i32 = 256;
// Java field: int LENGTH_CODES
pub(crate) const LENGTH_CODES: i32 = 29;
// Java field: int L_CODES
pub(crate) const L_CODES: i32 = 286;
// Java field: int HEAP_SIZE
pub(crate) const HEAP_SIZE: i32 = 573;
// Java field: int MAX_BL_BITS
pub(crate) const MAX_BL_BITS: i32 = 7;
// Java field: int END_BLOCK
pub(crate) const END_BLOCK: i32 = 256;
// Java field: int REP_3_6
pub(crate) const REP_3_6: i32 = 16;
// Java field: int REPZ_3_10
pub(crate) const REPZ_3_10: i32 = 17;
// Java field: int REPZ_11_138
pub(crate) const REPZ_11_138: i32 = 18;
// Java field: int[] extra_lbits
static EXTRA_LBITS: std::sync::OnceLock<Vec<i32>> = std::sync::OnceLock::new();
// Java field: int[] extra_dbits
static EXTRA_DBITS: std::sync::OnceLock<Vec<i32>> = std::sync::OnceLock::new();
// Java field: int[] extra_blbits
static EXTRA_BLBITS: std::sync::OnceLock<Vec<i32>> = std::sync::OnceLock::new();
// Java field: byte[] bl_order
static BL_ORDER: std::sync::OnceLock<Vec<u8>> = std::sync::OnceLock::new();
// Java field: int Buf_size
pub(crate) const Buf_size: i32 = 16;
// Java field: int DIST_CODE_LEN
pub(crate) const DIST_CODE_LEN: i32 = 512;
// Java field: byte[] _dist_code
static _DIST_CODE: std::sync::OnceLock<Vec<u8>> = std::sync::OnceLock::new();
// Java field: byte[] _length_code
static _LENGTH_CODE: std::sync::OnceLock<Vec<u8>> = std::sync::OnceLock::new();
// Java field: int[] base_length
static BASE_LENGTH: std::sync::OnceLock<Vec<i32>> = std::sync::OnceLock::new();
// Java field: int[] base_dist
static BASE_DIST: std::sync::OnceLock<Vec<i32>> = std::sync::OnceLock::new();

pub(crate) fn get_extra_lbits() -> &'static Vec<i32> {
    EXTRA_LBITS.get_or_init(|| vec![0, 0, 0, 0, 0, 0, 0, 0, 1, 1, 1, 1, 2, 2, 2, 2, 3, 3, 3, 3, 4, 4, 4, 4, 5, 5, 5, 5, 0])
}

pub(crate) fn get_extra_dbits() -> &'static Vec<i32> {
    EXTRA_DBITS.get_or_init(|| vec![0, 0, 0, 0, 1, 1, 2, 2, 3, 3, 4, 4, 5, 5, 6, 6, 7, 7, 8, 8, 9, 9, 10, 10, 11, 11, 12, 12, 13, 13])
}

pub(crate) fn get_extra_blbits() -> &'static Vec<i32> {
    EXTRA_BLBITS.get_or_init(|| vec![0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 2, 3, 7])
}

// Java: public class Tree
pub struct Tree {
    // Java field: short[] dyn_tree
    pub(crate) dyn_tree: Option<Vec<i16>>,
    // Java field: int max_code
    pub(crate) max_code: i32,
    // Java field: StaticTree stat_desc
    pub(crate) stat_desc: Option<StaticTree>,
}

// Java: public class Tree
impl Tree {
    // Java: static int d_code(int dist)
    pub fn d_code(dist: i32) -> i32 {
      // Java: return dist < 256 ? _dist_code[dist] : _dist_code[256 + (dist >>> 7)];
      todo!()
    }

    // Java: void gen_bitlen(Deflate s)
    pub fn gen_bitlen(&mut self, _s: &mut dyn std::any::Any) {
      // Java: short[] tree = this.dyn_tree;
      // Java: short[] stree = this.stat_desc.static_tree;
      // Java: int[] extra = this.stat_desc.extra_bits;
      // Java: int base = this.stat_desc.extra_base;
      // Java: int max_length = this.stat_desc.max_length;
      // Java: int overflow = 0;
      // Java: for(int bits = 0; bits <= 15; ++bits)
      {
         // Java: s.bl_count[bits] = 0;
      }
      // Java: tree[s.heap[s.heap_max] * 2 + 1] = 0;
      // Java: int h;
      // Java: for(h = s.heap_max + 1; h < 573; ++h)
      {
         // Java: int n = s.heap[h];
         // Java: int var15 = tree[tree[n * 2 + 1] * 2 + 1] + 1;
         // Java: if (var15 > max_length)
         {
            // Java: var15 = max_length;
            // Java: ++overflow;
         }
         // Java: tree[n * 2 + 1] = (short)var15;
         // Java: if (n <= this.max_code)
         {
            // Java: ++s.bl_count[var15];
            // Java: int xbits = 0;
            // Java: if (n >= base)
            {
               // Java: xbits = extra[n - base];
            }
            // Java: short f = tree[n * 2];
            // Java: s.opt_len += f * (var15 + xbits);
            // Java: if (stree != null)
            {
               // Java: s.static_len += f * (stree[n * 2 + 1] + xbits);
            }
         }
      }
      // Java: if (overflow != 0)
      {
         // Java: do
         {
            // Java: int var16;
            // Java: for(var16 = max_length - 1; s.bl_count[var16] == 0; --var16)
            {
            }
            // Java: --s.bl_count[var16];
            // Java: short[] var19 = s.bl_count;
            // Java: var19[var16 + 1] = (short)(var19[var16 + 1] + 2);
            // Java: --s.bl_count[max_length];
            // Java: overflow -= 2;
         }
         // Java: for(int var17 = max_length; var17 != 0; --var17)
         {
            // Java: int n = s.bl_count[var17];
            // Java: while(n != 0)
            {
               // Java: --h;
               // Java: int m = s.heap[h];
               // Java: if (m <= this.max_code)
               {
                  // Java: if (tree[m * 2 + 1] != var17)
                  {
                     // Java: s.opt_len = (int)((long)s.opt_len + ((long)var17 - (long)tree[m * 2 + 1]) * (long)tree[m * 2]);
                     // Java: tree[m * 2 + 1] = (short)var17;
                  }
                  // Java: --n;
               }
            }
         }
      }
      todo!()
    }

    // Java: void build_tree(Deflate s)
    pub fn build_tree(&mut self, _s: &mut dyn std::any::Any) {
      // Java: short[] tree = this.dyn_tree;
      // Java: short[] stree = this.stat_desc.static_tree;
      // Java: int elems = this.stat_desc.elems;
      // Java: int max_code = -1;
      // Java: s.heap_len = 0;
      // Java: s.heap_max = 573;
      // Java: for(int n = 0; n < elems; ++n)
      {
         // Java: if (tree[n * 2] != 0)
         {
            // Java: int[] var10000 = s.heap;
            // Java: int var10001 = ++s.heap_len;
            // Java: max_code = n;
            // Java: var10000[var10001] = n;
            // Java: s.depth[n] = 0;
         }
         // Java: else
         {
            // Java: tree[n * 2 + 1] = 0;
         }
      }
      // Java: while(s.heap_len < 2)
      {
         // Java: int[] var12 = s.heap;
         // Java: int var13 = ++s.heap_len;
         // Java: int var10002;
         // Java: if (max_code < 2)
         {
            // Java: ++max_code;
            // Java: var10002 = max_code;
         }
         // Java: else
         {
            // Java: var10002 = 0;
         }
         // Java: int node = var12[var13] = var10002;
         // Java: tree[node * 2] = 1;
         // Java: s.depth[node] = 0;
         // Java: --s.opt_len;
         // Java: if (stree != null)
         {
            // Java: s.static_len -= stree[node * 2 + 1];
         }
      }
      // Java: this.max_code = max_code;
      // Java: for(int var9 = s.heap_len / 2; var9 >= 1; --var9)
      {
         // Java: s.pqdownheap(tree, var9);
      }
      // Java: int node = elems;
      // Java: do
      {
         // Java: int var10 = s.heap[1];
         // Java: s.heap[1] = s.heap[s.heap_len--];
         // Java: s.pqdownheap(tree, 1);
         // Java: int m = s.heap[1];
         // Java: s.heap[--s.heap_max] = var10;
         // Java: s.heap[--s.heap_max] = m;
         // Java: tree[node * 2] = (short)(tree[var10 * 2] + tree[m * 2]);
         // Java: s.depth[node] = (byte)(Math.max(s.depth[var10], s.depth[m]) + 1);
         // Java: tree[var10 * 2 + 1] = tree[m * 2 + 1] = (short)node;
         // Java: s.heap[1] = node++;
         // Java: s.pqdownheap(tree, 1);
      }
      // Java: s.heap[--s.heap_max] = s.heap[1];
      // Java: this.gen_bitlen(s);
      // Java: gen_codes(tree, max_code, s.bl_count);
      todo!()
    }

    // Java: static void gen_codes(short[] tree, int max_code, short[] bl_count)
    pub fn gen_codes(_tree: &mut [i16], _max_code: i32, _bl_count: &[i16]) {
      // Java: short[] next_code = new short[16];
      // Java: short code = 0;
      // Java: for(int bits = 1; bits <= 15; ++bits)
      {
         // Java: next_code[bits] = code = (short)(code + bl_count[bits - 1] << 1);
      }
      // Java: for(int n = 0; n <= max_code; ++n)
      {
         // Java: int len = tree[n * 2 + 1];
         // Java: if (len != 0)
         {
            // Java: tree[n * 2] = (short)bi_reverse(next_code[len]++, len);
         }
      }
      todo!()
    }

    // Java: static int bi_reverse(int code, int len)
    pub fn bi_reverse(code: i32, len: i32) -> i32 {
      // Java: int res = 0;
      let mut res = 0i32;
      let mut code = code;
      let mut len = len;
      // Java: do
      loop {
         // Java: res |= code & 1;
         res |= code & 1;
         // Java: code >>>= 1;
         code = ((code as u32) >> 1) as i32;
         // Java: res <<= 1;
         res <<= 1;
         // Java: --len;
         len -= 1;
         if len <= 0 { break; }
      }
      // Java: return res >>> 1;
      ((res as u32) >> 1) as i32
    }

}
