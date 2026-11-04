use super::ZStream::ZStream;
use super::Tree::Tree;
use super::StaticTree::StaticTree;

// Java field: int MAX_MEM_LEVEL
const MAX_MEM_LEVEL: i32 = 9;
// Java field: int Z_DEFAULT_COMPRESSION
const Z_DEFAULT_COMPRESSION: i32 = -1;
// Java field: int MAX_WBITS
const MAX_WBITS: i32 = 15;
// Java field: int DEF_MEM_LEVEL
const DEF_MEM_LEVEL: i32 = 8;
// Java field: int STORED
const STORED: i32 = 0;
// Java field: int FAST
const FAST: i32 = 1;
// Java field: int SLOW
const SLOW: i32 = 2;
// Java field: Config[] config_table
static config_table: std::sync::OnceLock<Vec<Config>> = std::sync::OnceLock::new();
// Java field: String[] z_errmsg
static z_errmsg: std::sync::OnceLock<Vec<String>> = std::sync::OnceLock::new();
// Java field: int NeedMore
const NeedMore: i32 = 0;
// Java field: int BlockDone
const BlockDone: i32 = 1;
// Java field: int FinishStarted
const FinishStarted: i32 = 2;
// Java field: int FinishDone
const FinishDone: i32 = 3;
// Java field: int PRESET_DICT
const PRESET_DICT: i32 = 32;
// Java field: int INIT_STATE
const INIT_STATE: i32 = 42;
// Java field: int BUSY_STATE
const BUSY_STATE: i32 = 113;
// Java field: int FINISH_STATE
const FINISH_STATE: i32 = 666;
// Java field: int Z_DEFLATED
const Z_DEFLATED: i32 = 8;
// Java field: int STORED_BLOCK
const STORED_BLOCK: i32 = 0;
// Java field: int STATIC_TREES
const STATIC_TREES: i32 = 1;
// Java field: int DYN_TREES
const DYN_TREES: i32 = 2;
// Java field: int Z_BINARY
const Z_BINARY: i32 = 0;
// Java field: int Z_ASCII
const Z_ASCII: i32 = 1;
// Java field: int Z_UNKNOWN
const Z_UNKNOWN: i32 = 2;
// Java field: int Buf_size
const Buf_size: i32 = 16;
// Java field: int REP_3_6
const REP_3_6: i32 = 16;
// Java field: int REPZ_3_10
const REPZ_3_10: i32 = 17;
// Java field: int REPZ_11_138
const REPZ_11_138: i32 = 18;
// Java field: int MIN_MATCH
const MIN_MATCH: i32 = 3;
// Java field: int MAX_MATCH
const MAX_MATCH: i32 = 258;
// Java field: int MIN_LOOKAHEAD
const MIN_LOOKAHEAD: i32 = 262;
// Java field: int MAX_BITS
const MAX_BITS: i32 = 15;
// Java field: int D_CODES
const D_CODES: i32 = 30;
// Java field: int BL_CODES
const BL_CODES: i32 = 19;
// Java field: int LENGTH_CODES
const LENGTH_CODES: i32 = 29;
// Java field: int LITERALS
const LITERALS: i32 = 256;
// Java field: int L_CODES
const L_CODES: i32 = 286;
// Java field: int HEAP_SIZE
const HEAP_SIZE: i32 = 573;
// Java field: int END_BLOCK
const END_BLOCK: i32 = 256;

// Java: public class Deflate
pub struct Deflate {
    // Java field: ZStream strm
    strm: Option<ZStream>,
    // Java field: int status
    status: i32,
    // Java field: byte[] pending_buf
    pending_buf: Option<Vec<u8>>,
    // Java field: int pending_buf_size
    pending_buf_size: i32,
    // Java field: int pending_out
    pending_out: i32,
    // Java field: int pending
    pending: i32,
    // Java field: int noheader
    noheader: i32,
    // Java field: byte data_type
    data_type: i8,
    // Java field: byte method
    method: i8,
    // Java field: int last_flush
    last_flush: i32,
    // Java field: int w_size
    w_size: i32,
    // Java field: int w_bits
    w_bits: i32,
    // Java field: int w_mask
    w_mask: i32,
    // Java field: byte[] window
    window: Option<Vec<u8>>,
    // Java field: int window_size
    window_size: i32,
    // Java field: short[] prev
    prev: Option<Vec<i16>>,
    // Java field: short[] head
    head: Option<Vec<i16>>,
    // Java field: int ins_h
    ins_h: i32,
    // Java field: int hash_size
    hash_size: i32,
    // Java field: int hash_bits
    hash_bits: i32,
    // Java field: int hash_mask
    hash_mask: i32,
    // Java field: int hash_shift
    hash_shift: i32,
    // Java field: int block_start
    block_start: i32,
    // Java field: int match_length
    match_length: i32,
    // Java field: int prev_match
    prev_match: i32,
    // Java field: int match_available
    match_available: i32,
    // Java field: int strstart
    strstart: i32,
    // Java field: int match_start
    match_start: i32,
    // Java field: int lookahead
    lookahead: i32,
    // Java field: int prev_length
    prev_length: i32,
    // Java field: int max_chain_length
    max_chain_length: i32,
    // Java field: int max_lazy_match
    max_lazy_match: i32,
    // Java field: int level
    level: i32,
    // Java field: int strategy
    strategy: i32,
    // Java field: int good_match
    good_match: i32,
    // Java field: int nice_match
    nice_match: i32,
    // Java field: short[] dyn_ltree
    dyn_ltree: Option<Vec<i16>>,
    // Java field: short[] dyn_dtree
    dyn_dtree: Option<Vec<i16>>,
    // Java field: short[] bl_tree
    bl_tree: Option<Vec<i16>>,
    // Java field: Tree l_desc
    l_desc: Option<Tree>,
    // Java field: Tree d_desc
    d_desc: Option<Tree>,
    // Java field: Tree bl_desc
    bl_desc: Option<Tree>,
    // Java field: short[] bl_count
    bl_count: Option<Vec<i16>>,
    // Java field: int[] heap
    heap: Option<Vec<i32>>,
    // Java field: int heap_len
    heap_len: i32,
    // Java field: int heap_max
    heap_max: i32,
    // Java field: byte[] depth
    depth: Option<Vec<u8>>,
    // Java field: int l_buf
    l_buf: i32,
    // Java field: int lit_bufsize
    lit_bufsize: i32,
    // Java field: int last_lit
    last_lit: i32,
    // Java field: int d_buf
    d_buf: i32,
    // Java field: int opt_len
    opt_len: i32,
    // Java field: int static_len
    static_len: i32,
    // Java field: int matches
    matches: i32,
    // Java field: int last_eob_len
    last_eob_len: i32,
    // Java field: short bi_buf
    bi_buf: i16,
    // Java field: int bi_valid
    bi_valid: i32,
}

// Java: public class Config
pub struct Config {
    // Java field: int good_length
    pub(crate) good_length: i32,
    // Java field: int max_lazy
    pub(crate) max_lazy: i32,
    // Java field: int nice_length
    pub(crate) nice_length: i32,
    // Java field: int max_chain
    pub(crate) max_chain: i32,
    // Java field: int func
    pub(crate) func: i32,
}

// Java: public class Deflate
impl Deflate {
    pub fn new() -> Self {
        Deflate {
            strm: None,
            status: 0,
            pending_buf: None,
            pending_buf_size: 0,
            pending_out: 0,
            pending: 0,
            noheader: 0,
            data_type: 0,
            method: 0,
            last_flush: 0,
            w_size: 0,
            w_bits: 0,
            w_mask: 0,
            window: None,
            window_size: 0,
            prev: None,
            head: None,
            ins_h: 0,
            hash_size: 0,
            hash_bits: 0,
            hash_mask: 0,
            hash_shift: 0,
            block_start: 0,
            match_length: 0,
            prev_match: 0,
            match_available: 0,
            strstart: 0,
            match_start: 0,
            lookahead: 0,
            prev_length: 0,
            max_chain_length: 0,
            max_lazy_match: 0,
            level: 0,
            strategy: 0,
            good_match: 0,
            nice_match: 0,
            dyn_ltree: None,
            dyn_dtree: None,
            bl_tree: None,
            l_desc: None,
            d_desc: None,
            bl_desc: None,
            bl_count: None,
            heap: None,
            heap_len: 0,
            heap_max: 0,
            depth: None,
            l_buf: 0,
            lit_bufsize: 0,
            last_lit: 0,
            d_buf: 0,
            opt_len: 0,
            static_len: 0,
            matches: 0,
            last_eob_len: 0,
            bi_buf: 0,
            bi_valid: 0,
        }
    }

    // Java: void lm_init()
    pub fn lm_init(&mut self) {
      // Java: this.window_size = 2 * this.w_size;

      // Java: this.head[this.hash_size - 1] = 0;

      // Java: for(int i = 0; i < this.hash_size - 1; ++i)
  
      {
         // Java: this.head[i] = 0;
   
      }
      // Java: this.max_lazy_match = config_table[this.level].max_lazy;

      // Java: this.good_match = config_table[this.level].good_length;

      // Java: this.nice_match = config_table[this.level].nice_length;

      // Java: this.max_chain_length = config_table[this.level].max_chain;

      // Java: this.strstart = 0;

      // Java: this.block_start = 0;

      // Java: this.lookahead = 0;

      // Java: this.match_length = this.prev_length = 2;

      // Java: this.match_available = 0;

      // Java: this.ins_h = 0;

    }

    // Java: void tr_init()
    pub fn tr_init(&mut self) {
      // Java: this.l_desc.dyn_tree = this.dyn_ltree;

      // Java: this.l_desc.stat_desc = StaticTree.static_l_desc;

      // Java: this.d_desc.dyn_tree = this.dyn_dtree;

      // Java: this.d_desc.stat_desc = StaticTree.static_d_desc;

      // Java: this.bl_desc.dyn_tree = this.bl_tree;

      // Java: this.bl_desc.stat_desc = StaticTree.static_bl_desc;

      // Java: this.bi_buf = 0;

      // Java: this.bi_valid = 0;

      // Java: this.last_eob_len = 8;

      // Java: this.init_block();

    }

    // Java: void init_block()
    pub fn init_block(&mut self) {
      // Java: for(int i = 0; i < 286; ++i)
  
      {
         // Java: this.dyn_ltree[i * 2] = 0;
   
      }
      // Java: for(int i = 0; i < 30; ++i)
  
      {
         // Java: this.dyn_dtree[i * 2] = 0;
   
      }
      // Java: for(int i = 0; i < 19; ++i)
  
      {
         // Java: this.bl_tree[i * 2] = 0;
   
      }
      // Java: this.dyn_ltree[512] = 1;

      // Java: this.opt_len = this.static_len = 0;

      // Java: this.last_lit = this.matches = 0;

    }

    // Java: void pqdownheap(short[] tree, int k)
    pub fn pqdownheap(&mut self, _tree: &[i16], _k: i32) {
      // Java: int v = this.heap[k];

      // Java: for(int j = k << 1; j <= this.heap_len; j <<= 1)
  
      {
         // Java: if (j < this.heap_len && smaller(tree, this.heap[j + 1], this.heap[j], this.depth))
      
         {
            // Java: ++j;
      
         }
         // Java: if (smaller(tree, v, this.heap[j], this.depth))
      
         {
            // Java: break;
      
         }
         // Java: this.heap[k] = this.heap[j];
   
         // Java: k = j;
   
      }
      // Java: this.heap[k] = v;

    }

    // Java: static boolean smaller(short[] tree, int n, int m, byte[] depth)
    pub fn smaller(_tree: &[i16], _n: i32, _m: i32, _depth: &[u8]) -> bool {
      // Java: short tn2 = tree[n * 2];

      // Java: short tm2 = tree[m * 2];

      // Java: return tn2 < tm2 || tn2 == tm2 && depth[n] <= depth[m];

      todo!()
    }

    // Java: void scan_tree(short[] tree, int max_code)
    pub fn scan_tree(&mut self, _tree: &mut [i16], _max_code: i32) {
      // Java: int prevlen = -1;

      // Java: int nextlen = tree[1];

      // Java: int count = 0;

      // Java: int max_count = 7;

      // Java: int min_count = 4;

      // Java: if (nextlen == 0)
  
      {
         // Java: max_count = 138;
   
         // Java: min_count = 3;
   
      }
      // Java: tree[(max_code + 1) * 2 + 1] = -1;

      // Java: for(int n = 0; n <= max_code; ++n)
  
      {
         // Java: int curlen = nextlen;
   
         // Java: nextlen = tree[(n + 1) * 2 + 1];
   
         // Java: ++count;
   
         // Java: if (count >= max_count || curlen != nextlen)
      
         {
            // Java: if (count < min_count)
          
            {
               // Java: short[] var10000 = this.bl_tree;
         
               // Java: var10000[curlen * 2] = (short)(var10000[curlen * 2] + count);
         
            }
            // Java: else if (curlen != 0)
          
            {
               // Java: if (curlen != prevlen)
              
               {
                  // Java: ++this.bl_tree[curlen * 2];
            
               }
               // Java: ++this.bl_tree[32];
         
            }
            // Java: else if (count <= 10)
          
            {
               // Java: ++this.bl_tree[34];
         
            }
            // Java: else
          
            {
               // Java: ++this.bl_tree[36];
         
            }
            // Java: count = 0;
      
            // Java: prevlen = curlen;
      
            // Java: if (nextlen == 0)
          
            {
               // Java: max_count = 138;
         
               // Java: min_count = 3;
         
            }
            // Java: else if (curlen == nextlen)
          
            {
               // Java: max_count = 6;
         
               // Java: min_count = 3;
         
            }
            // Java: else
          
            {
               // Java: max_count = 7;
         
               // Java: min_count = 4;
         
            }
         }
      }
    }

    // Java: int build_bl_tree()
    pub fn build_bl_tree(&mut self) -> i32 {
      // Java: this.scan_tree(this.dyn_ltree, this.l_desc.max_code);

      // Java: this.scan_tree(this.dyn_dtree, this.d_desc.max_code);

      // Java: this.bl_desc.build_tree(this);

      // Java: int max_blindex;

      // Java: for(max_blindex = 18; max_blindex >= 3 && this.bl_tree[Tree.bl_order[max_blindex] * 2 + 1] == 0; --max_blindex)
  
      {
      }
      // Java: this.opt_len += 3 * (max_blindex + 1) + 5 + 5 + 4;

      // Java: return max_blindex;

      todo!()
    }

    // Java: void send_all_trees(int lcodes, int dcodes, int blcodes)
    pub fn send_all_trees(&mut self, _lcodes: i32, _dcodes: i32, _blcodes: i32) {
      // Java: this.send_bits(lcodes - 257, 5);

      // Java: this.send_bits(dcodes - 1, 5);

      // Java: this.send_bits(blcodes - 4, 4);

      // Java: for(int rank = 0; rank < blcodes; ++rank)
  
      {
         // Java: this.send_bits(this.bl_tree[Tree.bl_order[rank] * 2 + 1], 3);
   
      }
      // Java: this.send_tree(this.dyn_ltree, lcodes - 1);

      // Java: this.send_tree(this.dyn_dtree, dcodes - 1);

    }

    // Java: void send_tree(short[] tree, int max_code)
    pub fn send_tree(&mut self, _tree: &[i16], _max_code: i32) {
      // Java: int prevlen = -1;

      // Java: int nextlen = tree[1];

      // Java: int count = 0;

      // Java: int max_count = 7;

      // Java: int min_count = 4;

      // Java: if (nextlen == 0)
  
      {
         // Java: max_count = 138;
   
         // Java: min_count = 3;
   
      }
      // Java: for(int n = 0; n <= max_code; ++n)
  
      {
         // Java: int curlen = nextlen;
   
         // Java: nextlen = tree[(n + 1) * 2 + 1];
   
         // Java: ++count;
   
         // Java: if (count >= max_count || curlen != nextlen)
      
         {
            // Java: if (count < min_count)
          
            {
               // Java: do
              
               {
                  // Java: this.send_code(curlen, this.bl_tree);
            
                  // Java: --count;
            
               }
            }
            // Java: else if (curlen != 0)
          
            {
               // Java: if (curlen != prevlen)
              
               {
                  // Java: this.send_code(curlen, this.bl_tree);
            
                  // Java: --count;
            
               }
               // Java: this.send_code(16, this.bl_tree);
         
               // Java: this.send_bits(count - 3, 2);
         
            }
            // Java: else if (count <= 10)
          
            {
               // Java: this.send_code(17, this.bl_tree);
         
               // Java: this.send_bits(count - 3, 3);
         
            }
            // Java: else
          
            {
               // Java: this.send_code(18, this.bl_tree);
         
               // Java: this.send_bits(count - 11, 7);
         
            }
            // Java: count = 0;
      
            // Java: prevlen = curlen;
      
            // Java: if (nextlen == 0)
          
            {
               // Java: max_count = 138;
         
               // Java: min_count = 3;
         
            }
            // Java: else if (curlen == nextlen)
          
            {
               // Java: max_count = 6;
         
               // Java: min_count = 3;
         
            }
            // Java: else
          
            {
               // Java: max_count = 7;
         
               // Java: min_count = 4;
         
            }
         }
      }
    }

    // Java: void put_byte(byte[] p, int start, int len)
    pub fn put_byte(&mut self, _p: &[u8], _start: i32, _len: i32) {
      // Java: System.arraycopy(p, start, this.pending_buf, this.pending, len);

      // Java: this.pending += len;

    }

    // Java: void put_byte(byte c)
    pub fn put_byte_single(&mut self, _c: u8) {
      // Java: this.pending_buf[this.pending++] = c;

    }

    // Java: void put_short(int w)
    pub fn put_short(&mut self, _w: i32) {
      // Java: this.put_byte((byte)w);

      // Java: this.put_byte((byte)(w >>> 8));

    }

    // Java: void putShortMSB(int b)
    pub fn putShortMSB(&mut self, _b: i32) {
      // Java: this.put_byte((byte)(b >> 8));

      // Java: this.put_byte((byte)b);

    }

    // Java: void send_code(int c, short[] tree)
    pub fn send_code(&mut self, _c: i32, _tree: &[i16]) {
      // Java: int c2 = c * 2;

      // Java: this.send_bits(tree[c2] & '\uffff', tree[c2 + 1] & '\uffff');

    }

    // Java: void send_bits(int value, int length)
    pub fn send_bits(&mut self, _value: i32, _length: i32) {
      // Java: if (this.bi_valid > 16 - length)
  
      {
         // Java: this.bi_buf = (short)(this.bi_buf | value << this.bi_valid & '\uffff');
   
         // Java: this.put_short(this.bi_buf);
   
         // Java: this.bi_buf = (short)(value >>> 16 - this.bi_valid);
   
         // Java: this.bi_valid += length - 16;
   
      }
      // Java: else
  
      {
         // Java: this.bi_buf = (short)(this.bi_buf | value << this.bi_valid & '\uffff');
   
         // Java: this.bi_valid += length;
   
      }
    }

    // Java: void _tr_align()
    pub fn _tr_align(&mut self) {
      // Java: this.send_bits(2, 3);

      // Java: this.send_code(256, StaticTree.static_ltree);

      // Java: this.bi_flush();

      // Java: if (1 + this.last_eob_len + 10 - this.bi_valid < 9)
  
      {
         // Java: this.send_bits(2, 3);
   
         // Java: this.send_code(256, StaticTree.static_ltree);
   
         // Java: this.bi_flush();
   
      }
      // Java: this.last_eob_len = 7;

    }

    // Java: boolean _tr_tally(int dist, int lc)
    pub fn _tr_tally(&mut self, _dist: i32, _lc: i32) -> bool {
      // Java: this.pending_buf[this.d_buf + this.last_lit * 2] = (byte)(dist >>> 8);

      // Java: this.pending_buf[this.d_buf + this.last_lit * 2 + 1] = (byte)dist;

      // Java: this.pending_buf[this.l_buf + this.last_lit] = (byte)lc;

      // Java: ++this.last_lit;

      // Java: if (dist == 0)
  
      {
         // Java: ++this.dyn_ltree[lc * 2];
   
      }
      // Java: else
  
      {
         // Java: ++this.matches;
   
         // Java: --dist;
   
         // Java: ++this.dyn_ltree[(Tree._length_code[lc] + 256 + 1) * 2];
   
         // Java: ++this.dyn_dtree[Tree.d_code(dist) * 2];
   
      }
      // Java: if ((this.last_lit & 8191) == 0 && this.level > 2)
  
      {
         // Java: int out_length = this.last_lit * 8;
   
         // Java: int in_length = this.strstart - this.block_start;
   
         // Java: for(int dcode = 0; dcode < 30; ++dcode)
      
         {
            // Java: out_length = (int)((long)out_length + (long)this.dyn_dtree[dcode * 2] * (5L + (long)Tree.extra_dbits[dcode]));
      
         }
         // Java: out_length >>>= 3;
   
         // Java: if (this.matches < this.last_lit / 2 && out_length < in_length / 2)
      
         {
            // Java: return true;
      
         }
      }
      // Java: return this.last_lit == this.lit_bufsize - 1;

      todo!()
    }

    // Java: void compress_block(short[] ltree, short[] dtree)
    pub fn compress_block(&mut self, _ltree: &[i16], _dtree: &[i16]) {
      // Java: int lx = 0;

      // Java: if (this.last_lit != 0)
  
      {
         // Java: do
      
         {
            // Java: int dist = this.pending_buf[this.d_buf + lx * 2] << 8 & '\uff00' | this.pending_buf[this.d_buf + lx * 2 + 1] & 255;
      
            // Java: int lc = this.pending_buf[this.l_buf + lx] & 255;
      
            // Java: ++lx;
      
            // Java: if (dist == 0)
          
            {
               // Java: this.send_code(lc, ltree);
         
            }
            // Java: else
          
            {
               // Java: int code = Tree._length_code[lc];
         
               // Java: this.send_code(code + 256 + 1, ltree);
         
               // Java: int extra = Tree.extra_lbits[code];
         
               // Java: if (extra != 0)
              
               {
                  // Java: lc -= Tree.base_length[code];
            
                  // Java: this.send_bits(lc, extra);
            
               }
               // Java: --dist;
         
               // Java: code = Tree.d_code(dist);
         
               // Java: this.send_code(code, dtree);
         
               // Java: extra = Tree.extra_dbits[code];
         
               // Java: if (extra != 0)
              
               {
                  // Java: dist -= Tree.base_dist[code];
            
                  // Java: this.send_bits(dist, extra);
            
               }
            }
         }
      }
      // Java: this.send_code(256, ltree);

      // Java: this.last_eob_len = ltree[513];

    }

    // Java: void set_data_type()
    pub fn set_data_type(&mut self) {
      // Java: int n = 0;

      // Java: int ascii_freq = 0;

      // Java: int bin_freq;

      // Java: for(bin_freq = 0; n < 7; ++n)
  
      {
         // Java: bin_freq += this.dyn_ltree[n * 2];
   
      }
      // Java: while(n < 128)
  
      {
         // Java: ascii_freq += this.dyn_ltree[n * 2];
   
         // Java: ++n;
   
      }
      // Java: while(n < 256)
  
      {
         // Java: bin_freq += this.dyn_ltree[n * 2];
   
         // Java: ++n;
   
      }
      // Java: this.data_type = (byte)(bin_freq > ascii_freq >>> 2 ? 0 : 1);

    }

    // Java: void bi_flush()
    pub fn bi_flush(&mut self) {
      // Java: if (this.bi_valid == 16)
  
      {
         // Java: this.put_short(this.bi_buf);
   
         // Java: this.bi_buf = 0;
   
         // Java: this.bi_valid = 0;
   
      }
      // Java: else if (this.bi_valid >= 8)
  
      {
         // Java: this.put_byte((byte)this.bi_buf);
   
         // Java: this.bi_buf = (short)(this.bi_buf >>> 8);
   
         // Java: this.bi_valid -= 8;
   
      }
    }

    // Java: void bi_windup()
    pub fn bi_windup(&mut self) {
      // Java: if (this.bi_valid > 8)
  
      {
         // Java: this.put_short(this.bi_buf);
   
      }
      // Java: else if (this.bi_valid > 0)
  
      {
         // Java: this.put_byte((byte)this.bi_buf);
   
      }
      // Java: this.bi_buf = 0;

      // Java: this.bi_valid = 0;

    }

    // Java: void copy_block(int buf, int len, boolean header)
    pub fn copy_block(&mut self, _buf: i32, _len: i32, _header: bool) {
      // Java: this.bi_windup();

      // Java: this.last_eob_len = 8;

      // Java: if (header)
  
      {
         // Java: this.put_short((short)len);
   
         // Java: this.put_short((short)(~len));
   
      }
      // Java: this.put_byte(this.window, buf, len);

    }

    // Java: void flush_block_only(boolean eof)
    pub fn flush_block_only(&mut self, _eof: bool) {
      // Java: this._tr_flush_block(this.block_start >= 0 ? this.block_start : -1, this.strstart - this.block_start, eof);

      // Java: this.block_start = this.strstart;

      // Java: this.strm.flush_pending();

    }

    // Java: int deflate_stored(int flush)
    pub fn deflate_stored(&mut self, _flush: i32) -> i32 {
      // Java: int max_block_size = 65535;

      // Java: if (max_block_size > this.pending_buf_size - 5)
  
      {
         // Java: max_block_size = this.pending_buf_size - 5;
   
      }
      // Java: while(true)
  
      {
         // Java: if (this.lookahead <= 1)
      
         {
            // Java: this.fill_window();
      
            // Java: if (this.lookahead == 0 && flush == 0)
          
            {
               // Java: return 0;
         
            }
            // Java: if (this.lookahead == 0)
          
            {
               // Java: this.flush_block_only(flush == 4);
         
               // Java: if (this.strm.avail_out == 0)
              
               {
                  // Java: return flush == 4 ? 2 : 0;
            
               }
               // Java: return flush == 4 ? 3 : 1;
         
            }
         }
         // Java: this.strstart += this.lookahead;
   
         // Java: this.lookahead = 0;
   
         // Java: int max_start = this.block_start + max_block_size;
   
         // Java: if (this.strstart == 0 || this.strstart >= max_start)
      
         {
            // Java: this.lookahead = this.strstart - max_start;
      
            // Java: this.strstart = max_start;
      
            // Java: this.flush_block_only(false);
      
            // Java: if (this.strm.avail_out == 0)
          
            {
               // Java: return 0;
         
            }
         }
         // Java: if (this.strstart - this.block_start >= this.w_size - 262)
      
         {
            // Java: this.flush_block_only(false);
      
            // Java: if (this.strm.avail_out == 0)
          
            {
               // Java: return 0;
         
            }
         }
      }
      todo!()
    }

    // Java: void _tr_stored_block(int buf, int stored_len, boolean eof)
    pub fn _tr_stored_block(&mut self, _buf: i32, _stored_len: i32, _eof: bool) {
      // Java: this.send_bits(0 + (eof ? 1 : 0), 3);

      // Java: this.copy_block(buf, stored_len, true);

    }

    // Java: void _tr_flush_block(int buf, int stored_len, boolean eof)
    pub fn _tr_flush_block(&mut self, _buf: i32, _stored_len: i32, _eof: bool) {
      // Java: int max_blindex = 0;

      // Java: int opt_lenb;

      // Java: int static_lenb;

      // Java: if (this.level > 0)
  
      {
         // Java: if (this.data_type == 2)
      
         {
            // Java: this.set_data_type();
      
         }
         // Java: this.l_desc.build_tree(this);
   
         // Java: this.d_desc.build_tree(this);
   
         // Java: max_blindex = this.build_bl_tree();
   
         // Java: opt_lenb = this.opt_len + 3 + 7 >>> 3;
   
         // Java: static_lenb = this.static_len + 3 + 7 >>> 3;
   
         // Java: if (static_lenb <= opt_lenb)
      
         {
            // Java: opt_lenb = static_lenb;
      
         }
      }
      // Java: else
  
      {
         // Java: opt_lenb = static_lenb = stored_len + 5;
   
      }
      // Java: if (stored_len + 4 <= opt_lenb && buf != -1)
  
      {
         // Java: this._tr_stored_block(buf, stored_len, eof);
   
      }
      // Java: else if (static_lenb == opt_lenb)
  
      {
         // Java: this.send_bits(2 + (eof ? 1 : 0), 3);
   
         // Java: this.compress_block(StaticTree.static_ltree, StaticTree.static_dtree);
   
      }
      // Java: else
  
      {
         // Java: this.send_bits(4 + (eof ? 1 : 0), 3);
   
         // Java: this.send_all_trees(this.l_desc.max_code + 1, this.d_desc.max_code + 1, max_blindex + 1);
   
         // Java: this.compress_block(this.dyn_ltree, this.dyn_dtree);
   
      }
      // Java: this.init_block();

      // Java: if (eof)
  
      {
         // Java: this.bi_windup();
   
      }
    }

    // Java: void fill_window()
    pub fn fill_window(&mut self) {
      // Java: do
  
      {
         // Java: int more = this.window_size - this.lookahead - this.strstart;
   
         // Java: if (more == 0 && this.strstart == 0 && this.lookahead == 0)
      
         {
            // Java: more = this.w_size;
      
         }
         // Java: else if (more == -1)
      
         {
            // Java: --more;
      
         }
         // Java: else if (this.strstart >= this.w_size + this.w_size - 262)
      
         {
            // Java: System.arraycopy(this.window, this.w_size, this.window, 0, this.w_size);
      
            // Java: this.match_start -= this.w_size;
      
            // Java: this.strstart -= this.w_size;
      
            // Java: this.block_start -= this.w_size;
      
            // Java: int n = this.hash_size;
      
            // Java: int p = n;
      
            // Java: do
          
            {
               // Java: --p;
         
               // Java: int m = this.head[p] & '\uffff';
         
               // Java: this.head[p] = m >= this.w_size ? (short)(m - this.w_size) : 0;
         
               // Java: --n;
         
            }
            // Java: n = this.w_size;
      
            // Java: p = n;
      
            // Java: do
          
            {
               // Java: --p;
         
               // Java: int var7 = this.prev[p] & '\uffff';
         
               // Java: this.prev[p] = var7 >= this.w_size ? (short)(var7 - this.w_size) : 0;
         
               // Java: --n;
         
            }
            // Java: more += this.w_size;
      
         }
         // Java: if (this.strm.avail_in == 0)
      
         {
            // Java: return;
      
         }
         // Java: int n = this.strm.read_buf(this.window, this.strstart + this.lookahead, more);
   
         // Java: this.lookahead += n;
   
         // Java: if (this.lookahead >= 3)
      
         {
            // Java: this.ins_h = this.window[this.strstart] & 255;
      
            // Java: this.ins_h = (this.ins_h << this.hash_shift ^ this.window[this.strstart + 1] & 255) & this.hash_mask;
      
         }
      }
    }

    // Java: int deflate_fast(int flush)
    pub fn deflate_fast(&mut self, _flush: i32) -> i32 {
      // Java: int hash_head = 0;

      // Java: while(true)
  
      {
         // Java: if (this.lookahead < 262)
      
         {
            // Java: this.fill_window();
      
            // Java: if (this.lookahead < 262 && flush == 0)
          
            {
               // Java: return 0;
         
            }
            // Java: if (this.lookahead == 0)
          
            {
               // Java: this.flush_block_only(flush == 4);
         
               // Java: if (this.strm.avail_out == 0)
              
               {
                  // Java: if (flush == 4)
                  
                  {
                     // Java: return 2;
               
                  }
                  // Java: return 0;
            
               }
               // Java: return flush == 4 ? 3 : 1;
         
            }
         }
         // Java: if (this.lookahead >= 3)
      
         {
            // Java: this.ins_h = (this.ins_h << this.hash_shift ^ this.window[this.strstart + 2] & 255) & this.hash_mask;
      
            // Java: hash_head = this.head[this.ins_h] & '\uffff';
      
            // Java: this.prev[this.strstart & this.w_mask] = this.head[this.ins_h];
      
            // Java: this.head[this.ins_h] = (short)this.strstart;
      
         }
         // Java: if ((long)hash_head != 0L && (this.strstart - hash_head & '\uffff') <= this.w_size - 262 && this.strategy != 2)
      
         {
            // Java: this.match_length = this.longest_match(hash_head);
      
         }
         // Java: boolean bflush;
   
         // Java: if (this.match_length < 3)
      
         {
            // Java: bflush = this._tr_tally(0, this.window[this.strstart] & 255);
      
            // Java: --this.lookahead;
      
            // Java: ++this.strstart;
      
         }
         // Java: else
      
         {
            // Java: bflush = this._tr_tally(this.strstart - this.match_start, this.match_length - 3);
      
            // Java: this.lookahead -= this.match_length;
      
            // Java: if (this.match_length <= this.max_lazy_match && this.lookahead >= 3)
          
            {
               // Java: --this.match_length;
         
               // Java: do
              
               {
                  // Java: ++this.strstart;
            
                  // Java: this.ins_h = (this.ins_h << this.hash_shift ^ this.window[this.strstart + 2] & 255) & this.hash_mask;
            
                  // Java: hash_head = this.head[this.ins_h] & '\uffff';
            
                  // Java: this.prev[this.strstart & this.w_mask] = this.head[this.ins_h];
            
                  // Java: this.head[this.ins_h] = (short)this.strstart;
            
               }
               // Java: ++this.strstart;
         
            }
            // Java: else
          
            {
               // Java: this.strstart += this.match_length;
         
               // Java: this.match_length = 0;
         
               // Java: this.ins_h = this.window[this.strstart] & 255;
         
               // Java: this.ins_h = (this.ins_h << this.hash_shift ^ this.window[this.strstart + 1] & 255) & this.hash_mask;
         
            }
         }
         // Java: if (bflush)
      
         {
            // Java: this.flush_block_only(false);
      
            // Java: if (this.strm.avail_out == 0)
          
            {
               // Java: return 0;
         
            }
         }
      }
      todo!()
    }

    // Java: int deflate_slow(int flush)
    pub fn deflate_slow(&mut self, _flush: i32) -> i32 {
      // Java: int hash_head = 0;

      // Java: while(true)
  
      {
         // Java: if (this.lookahead < 262)
      
         {
            // Java: this.fill_window();
      
            // Java: if (this.lookahead < 262 && flush == 0)
          
            {
               // Java: return 0;
         
            }
            // Java: if (this.lookahead == 0)
          
            {
               // Java: if (this.match_available != 0)
              
               {
                  // Java: this._tr_tally(0, this.window[this.strstart - 1] & 255);
            
                  // Java: this.match_available = 0;
            
               }
               // Java: this.flush_block_only(flush == 4);
         
               // Java: if (this.strm.avail_out == 0)
              
               {
                  // Java: if (flush == 4)
                  
                  {
                     // Java: return 2;
               
                  }
                  // Java: return 0;
            
               }
               // Java: return flush == 4 ? 3 : 1;
         
            }
         }
         // Java: if (this.lookahead >= 3)
      
         {
            // Java: this.ins_h = (this.ins_h << this.hash_shift ^ this.window[this.strstart + 2] & 255) & this.hash_mask;
      
            // Java: hash_head = this.head[this.ins_h] & '\uffff';
      
            // Java: this.prev[this.strstart & this.w_mask] = this.head[this.ins_h];
      
            // Java: this.head[this.ins_h] = (short)this.strstart;
      
         }
         // Java: this.prev_length = this.match_length;
   
         // Java: this.prev_match = this.match_start;
   
         // Java: this.match_length = 2;
   
         // Java: if (hash_head != 0 && this.prev_length < this.max_lazy_match && (this.strstart - hash_head & '\uffff') <= this.w_size - 262)
      
         {
            // Java: if (this.strategy != 2)
          
            {
               // Java: this.match_length = this.longest_match(hash_head);
         
            }
            // Java: if (this.match_length <= 5 && (this.strategy == 1 || this.match_length == 3 && this.strstart - this.match_start > 4096))
          
            {
               // Java: this.match_length = 2;
         
            }
         }
         // Java: if (this.prev_length >= 3 && this.match_length <= this.prev_length)
      
         {
            // Java: int max_insert = this.strstart + this.lookahead - 3;
      
            // Java: boolean bflush = this._tr_tally(this.strstart - 1 - this.prev_match, this.prev_length - 3);
      
            // Java: this.lookahead -= this.prev_length - 1;
      
            // Java: this.prev_length -= 2;
      
            // Java: do
          
            {
               // Java: if (++this.strstart <= max_insert)
              
               {
                  // Java: this.ins_h = (this.ins_h << this.hash_shift ^ this.window[this.strstart + 2] & 255) & this.hash_mask;
            
                  // Java: hash_head = this.head[this.ins_h] & '\uffff';
            
                  // Java: this.prev[this.strstart & this.w_mask] = this.head[this.ins_h];
            
                  // Java: this.head[this.ins_h] = (short)this.strstart;
            
               }
            }
            // Java: this.match_available = 0;
      
            // Java: this.match_length = 2;
      
            // Java: ++this.strstart;
      
            // Java: if (bflush)
          
            {
               // Java: this.flush_block_only(false);
         
               // Java: if (this.strm.avail_out == 0)
              
               {
                  // Java: return 0;
            
               }
            }
         }
         // Java: else if (this.match_available != 0)
      
         {
            // Java: boolean bflush = this._tr_tally(0, this.window[this.strstart - 1] & 255);
      
            // Java: if (bflush)
          
            {
               // Java: this.flush_block_only(false);
         
            }
            // Java: ++this.strstart;
      
            // Java: --this.lookahead;
      
            // Java: if (this.strm.avail_out == 0)
          
            {
               // Java: return 0;
         
            }
         }
         // Java: else
      
         {
            // Java: this.match_available = 1;
      
            // Java: ++this.strstart;
      
            // Java: --this.lookahead;
      
         }
      }
      todo!()
    }

    // Java: int longest_match(int cur_match)
    pub fn longest_match(&mut self, _cur_match: i32) -> i32 {
      // Java: int chain_length = this.max_chain_length;

      // Java: int scan = this.strstart;

      // Java: int best_len = this.prev_length;

      // Java: int limit = this.strstart > this.w_size - 262 ? this.strstart - (this.w_size - 262) : 0;

      // Java: int nice_match = this.nice_match;

      // Java: int wmask = this.w_mask;

      // Java: int strend = this.strstart + 258;

      // Java: byte scan_end1 = this.window[scan + best_len - 1];

      // Java: byte scan_end = this.window[scan + best_len];

      // Java: if (this.prev_length >= this.good_match)
  
      {
         // Java: chain_length >>= 2;
   
      }
      // Java: if (nice_match > this.lookahead)
  
      {
         // Java: nice_match = this.lookahead;
   
      }
      // Java: do
  
      {
         // Java: if (this.window[cur_match + best_len] == scan_end && this.window[cur_match + best_len - 1] == scan_end1 && this.window[cur_match] == this.window[scan])
      
         {
            // Java: int match = cur_match + 1;
      
            // Java: if (this.window[match] == this.window[scan + 1])
          
            {
               // Java: scan += 2;
         
               // Java: ++match;
         
               // Java: byte var28;
         
               // Java: do
              
               {
                  // Java: ++scan;
            
                  // Java: var28 = this.window[scan];
            
                  // Java: ++match;
            
                  // Java: if (var28 != this.window[match])
                  
                  {
                     // Java: break;
               
                  }
                  // Java: ++scan;
            
                  // Java: var28 = this.window[scan];
            
                  // Java: ++match;
            
                  // Java: if (var28 != this.window[match])
                  
                  {
                     // Java: break;
               
                  }
                  // Java: ++scan;
            
                  // Java: var28 = this.window[scan];
            
                  // Java: ++match;
            
                  // Java: if (var28 != this.window[match])
                  
                  {
                     // Java: break;
               
                  }
                  // Java: ++scan;
            
                  // Java: var28 = this.window[scan];
            
                  // Java: ++match;
            
                  // Java: if (var28 != this.window[match])
                  
                  {
                     // Java: break;
               
                  }
                  // Java: ++scan;
            
                  // Java: var28 = this.window[scan];
            
                  // Java: ++match;
            
                  // Java: if (var28 != this.window[match])
                  
                  {
                     // Java: break;
               
                  }
                  // Java: ++scan;
            
                  // Java: var28 = this.window[scan];
            
                  // Java: ++match;
            
                  // Java: if (var28 != this.window[match])
                  
                  {
                     // Java: break;
               
                  }
                  // Java: ++scan;
            
                  // Java: var28 = this.window[scan];
            
                  // Java: ++match;
            
                  // Java: if (var28 != this.window[match])
                  
                  {
                     // Java: break;
               
                  }
                  // Java: ++scan;
            
                  // Java: var28 = this.window[scan];
            
                  // Java: ++match;
            
               }
               // Java: int len = 258 - (strend - scan);
         
               // Java: scan = strend - 258;
         
               // Java: if (len > best_len)
              
               {
                  // Java: this.match_start = cur_match;
            
                  // Java: best_len = len;
            
                  // Java: if (len >= nice_match)
                  
                  {
                     // Java: break;
               
                  }
                  // Java: scan_end1 = this.window[scan + len - 1];
            
                  // Java: scan_end = this.window[scan + len];
            
               }
            }
         }
         // Java: if ((cur_match = this.prev[cur_match & wmask] & '\uffff') <= limit)
      
         {
            // Java: break;
      
         }
         // Java: --chain_length;
   
      }
      // Java: return best_len <= this.lookahead ? best_len : this.lookahead;

      todo!()
    }

    // Java: int deflateInit(ZStream strm, int level, int bits)
    pub fn deflateInit(&mut self, _strm: &mut ZStream, _level: i32, _bits: i32) -> i32 {
      // Java: return this.deflateInit2(strm, level, 8, bits, 8, 0);

      todo!()
    }

    // Java: int deflateInit(ZStream strm, int level)
    pub fn deflateInit_with_level(&mut self, _strm: &mut ZStream, _level: i32) -> i32 {
      // Java: return this.deflateInit(strm, level, 15);

      todo!()
    }

    // Java: int deflateInit2(ZStream strm, int level, int method, int windowBits, int memLevel, int strategy)
    pub fn deflateInit2(&mut self, _strm: &mut ZStream, _level: i32, _method: i32, _windowBits: i32, _memLevel: i32, _strategy: i32) -> i32 {
      // Java: int noheader = 0;

      // Java: strm.msg = null;

      // Java: if (level == -1)
  
      {
         // Java: level = 6;
   
      }
      // Java: if (windowBits < 0)
  
      {
         // Java: noheader = 1;
   
         // Java: windowBits = -windowBits;
   
      }
      // Java: if (memLevel >= 1 && memLevel <= 9 && method == 8 && windowBits >= 9 && windowBits <= 15 && level >= 0 && level <= 9 && strategy >= 0 && strategy <= 2)
  
      {
         // Java: strm.dstate = this;
   
         // Java: this.noheader = noheader;
   
         // Java: this.w_bits = windowBits;
   
         // Java: this.w_size = 1 << this.w_bits;
   
         // Java: this.w_mask = this.w_size - 1;
   
         // Java: this.hash_bits = memLevel + 7;
   
         // Java: this.hash_size = 1 << this.hash_bits;
   
         // Java: this.hash_mask = this.hash_size - 1;
   
         // Java: this.hash_shift = (this.hash_bits + 3 - 1) / 3;
   
         // Java: this.window = new byte[this.w_size * 2];
   
         // Java: this.prev = new short[this.w_size];
   
         // Java: this.head = new short[this.hash_size];
   
         // Java: this.lit_bufsize = 1 << memLevel + 6;
   
         // Java: this.pending_buf = new byte[this.lit_bufsize * 4];
   
         // Java: this.pending_buf_size = this.lit_bufsize * 4;
   
         // Java: this.d_buf = this.lit_bufsize / 2;
   
         // Java: this.l_buf = 3 * this.lit_bufsize;
   
         // Java: this.level = level;
   
         // Java: this.strategy = strategy;
   
         // Java: this.method = (byte)method;
   
         // Java: return this.deflateReset(strm);
   
      }
      // Java: else
  
      {
         // Java: return -2;
   
      }
      todo!()
    }

    // Java: int deflateReset(ZStream strm)
    pub fn deflateReset(&mut self, _strm: &mut ZStream) -> i32 {
      // Java: strm.total_in = strm.total_out = 0L;

      // Java: strm.msg = null;

      // Java: strm.data_type = 2;

      // Java: this.pending = 0;

      // Java: this.pending_out = 0;

      // Java: if (this.noheader < 0)
  
      {
         // Java: this.noheader = 0;
   
      }
      // Java: this.status = this.noheader != 0 ? 113 : 42;

      // Java: strm.adler = strm._adler.adler32(0L, (byte[])null, 0, 0);

      // Java: this.last_flush = 0;

      // Java: this.tr_init();

      // Java: this.lm_init();

      // Java: return 0;

      todo!()
    }

    // Java: int deflateEnd()
    pub fn deflateEnd(&mut self) -> i32 {
      // Java: if (this.status != 42 && this.status != 113 && this.status != 666)
  
      {
         // Java: return -2;
   
      }
      // Java: else
  
      {
         // Java: this.pending_buf = null;
   
         // Java: this.head = null;
   
         // Java: this.prev = null;
   
         // Java: this.window = null;
   
         // Java: return this.status == 113 ? -3 : 0;
   
      }
      todo!()
    }

    // Java: int deflateParams(ZStream strm, int _level, int _strategy)
    pub fn deflateParams(&mut self, _strm: &mut ZStream, __level: i32, __strategy: i32) -> i32 {
      // Java: int err = 0;

      // Java: if (_level == -1)
  
      {
         // Java: _level = 6;
   
      }
      // Java: if (_level >= 0 && _level <= 9 && _strategy >= 0 && _strategy <= 2)
  
      {
         // Java: if (config_table[this.level].func != config_table[_level].func && strm.total_in != 0L)
      
         {
            // Java: err = strm.deflate(1);
      
         }
         // Java: if (this.level != _level)
      
         {
            // Java: this.level = _level;
      
            // Java: this.max_lazy_match = config_table[this.level].max_lazy;
      
            // Java: this.good_match = config_table[this.level].good_length;
      
            // Java: this.nice_match = config_table[this.level].nice_length;
      
            // Java: this.max_chain_length = config_table[this.level].max_chain;
      
         }
         // Java: this.strategy = _strategy;
   
         // Java: return err;
   
      }
      // Java: else
  
      {
         // Java: return -2;
   
      }
      todo!()
    }

    // Java: int deflateSetDictionary(ZStream strm, byte[] dictionary, int dictLength)
    pub fn deflateSetDictionary(&mut self, _strm: &mut ZStream, _dictionary: &[u8], _dictLength: i32) -> i32 {
      // Java: int length = dictLength;

      // Java: int index = 0;

      // Java: if (dictionary != null && this.status == 42)
  
      {
         // Java: strm.adler = strm._adler.adler32(strm.adler, dictionary, 0, dictLength);
   
         // Java: if (dictLength < 3)
      
         {
            // Java: return 0;
      
         }
         // Java: else
      
         {
            // Java: if (dictLength > this.w_size - 262)
          
            {
               // Java: length = this.w_size - 262;
         
               // Java: index = dictLength - length;
         
            }
            // Java: System.arraycopy(dictionary, index, this.window, 0, length);
      
            // Java: this.strstart = length;
      
            // Java: this.block_start = length;
      
            // Java: this.ins_h = this.window[0] & 255;
      
            // Java: this.ins_h = (this.ins_h << this.hash_shift ^ this.window[1] & 255) & this.hash_mask;
      
            // Java: for(int n = 0; n <= length - 3; ++n)
          
            {
               // Java: this.ins_h = (this.ins_h << this.hash_shift ^ this.window[n + 2] & 255) & this.hash_mask;
         
               // Java: this.prev[n & this.w_mask] = this.head[this.ins_h];
         
               // Java: this.head[this.ins_h] = (short)n;
         
            }
            // Java: return 0;
      
         }
      }
      // Java: else
  
      {
         // Java: return -2;
   
      }
      todo!()
    }

    // Java: int deflate(ZStream strm, int flush)
    pub fn deflate(&mut self, _strm: &mut ZStream, _flush: i32) -> i32 {
      // Java: if (flush <= 4 && flush >= 0)
  
      {
         // Java: if (strm.next_out != null && (strm.next_in != null || strm.avail_in == 0) && (this.status != 666 || flush == 4))
      
         {
            // Java: if (strm.avail_out == 0)
          
            {
               // Java: strm.msg = z_errmsg[7];
         
               // Java: return -5;
         
            }
            // Java: else
          
            {
               // Java: this.strm = strm;
         
               // Java: int old_flush = this.last_flush;
         
               // Java: this.last_flush = flush;
         
               // Java: if (this.status == 42)
              
               {
                  // Java: int header = 8 + (this.w_bits - 8 << 4) << 8;
            
                  // Java: int level_flags = (this.level - 1 & 255) >> 1;
            
                  // Java: if (level_flags > 3)
                  
                  {
                     // Java: level_flags = 3;
               
                  }
                  // Java: header |= level_flags << 6;
            
                  // Java: if (this.strstart != 0)
                  
                  {
                     // Java: header |= 32;
               
                  }
                  // Java: header += 31 - header % 31;
            
                  // Java: this.status = 113;
            
                  // Java: this.putShortMSB(header);
            
                  // Java: if (this.strstart != 0)
                  
                  {
                     // Java: this.putShortMSB((int)(strm.adler >>> 16));
               
                     // Java: this.putShortMSB((int)(strm.adler & 65535L));
               
                  }
                  // Java: strm.adler = strm._adler.adler32(0L, (byte[])null, 0, 0);
            
               }
               // Java: if (this.pending != 0)
              
               {
                  // Java: strm.flush_pending();
            
                  // Java: if (strm.avail_out == 0)
                  
                  {
                     // Java: this.last_flush = -1;
               
                     // Java: return 0;
               
                  }
               }
               // Java: else if (strm.avail_in == 0 && flush <= old_flush && flush != 4)
              
               {
                  // Java: strm.msg = z_errmsg[7];
            
                  // Java: return -5;
            
               }
               // Java: if (this.status == 666 && strm.avail_in != 0)
              
               {
                  // Java: strm.msg = z_errmsg[7];
            
                  // Java: return -5;
            
               }
               // Java: else
              
               {
                  // Java: if (strm.avail_in != 0 || this.lookahead != 0 || flush != 0 && this.status != 666)
                  
                  {
                     // Java: int bstate = -1;
               
                     // Java: switch (config_table[this.level].func)
                      
                     {
                        // Java: case 0:
                  
                           // Java: bstate = this.deflate_stored(flush);
                     
                           // Java: break;
                     
                        // Java: case 1:
                  
                           // Java: bstate = this.deflate_fast(flush);
                     
                           // Java: break;
                     
                        // Java: case 2:
                  
                           // Java: bstate = this.deflate_slow(flush);
                     
                     }
                     // Java: if (bstate == 2 || bstate == 3)
                      
                     {
                        // Java: this.status = 666;
                  
                     }
                     // Java: if (bstate == 0 || bstate == 2)
                      
                     {
                        // Java: if (strm.avail_out == 0)
                          
                        {
                           // Java: this.last_flush = -1;
                     
                        }
                        // Java: return 0;
                  
                     }
                     // Java: if (bstate == 1)
                      
                     {
                        // Java: if (flush == 1)
                          
                        {
                           // Java: this._tr_align();
                     
                        }
                        // Java: else
                          
                        {
                           // Java: this._tr_stored_block(0, 0, false);
                     
                           // Java: if (flush == 3)
                              
                           {
                              // Java: for(int i = 0; i < this.hash_size; ++i)
                                  
                              {
                                 // Java: this.head[i] = 0;
                           
                              }
                           }
                        }
                        // Java: strm.flush_pending();
                  
                        // Java: if (strm.avail_out == 0)
                          
                        {
                           // Java: this.last_flush = -1;
                     
                           // Java: return 0;
                     
                        }
                     }
                  }
                  // Java: if (flush != 4)
                  
                  {
                     // Java: return 0;
               
                  }
                  // Java: else if (this.noheader != 0)
                  
                  {
                     // Java: return 1;
               
                  }
                  // Java: else
                  
                  {
                     // Java: this.putShortMSB((int)(strm.adler >>> 16));
               
                     // Java: this.putShortMSB((int)(strm.adler & 65535L));
               
                     // Java: strm.flush_pending();
               
                     // Java: this.noheader = -1;
               
                     // Java: return this.pending != 0 ? 0 : 1;
               
                  }
               }
            }
         }
         // Java: else
      
         {
            // Java: strm.msg = z_errmsg[4];
      
            // Java: return -2;
      
         }
      }
      // Java: else
  
      {
         // Java: return -2;
   
      }
      todo!()
    }

}

// Java: public class Config
impl Config {
}
