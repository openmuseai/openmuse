use std::sync::Mutex;
use std::sync::OnceLock;

// Java field: String TAG
const TAG: &str = "";
// Java field: boolean DEBUG
const DEBUG: bool = false;
// Java field: int BUF_SIZE
const BUF_SIZE: usize = 8192;

struct PoolState {
    pool: Vec<DumpPicBuffer>,
    pool_size: i32,
    max_pool_size: i32,
    new_ins_count: i32,
}

fn get_pool() -> &'static Mutex<PoolState> {
    static POOL: OnceLock<Mutex<PoolState>> = OnceLock::new();
    POOL.get_or_init(|| Mutex::new(PoolState {
        pool: Vec::new(),
        pool_size: 0,
        max_pool_size: DEFAULT_MAX_POOL_SIZE,
        new_ins_count: 0,
    }))
}

// Java: public class DumpPicBuffer
pub struct DumpPicBuffer {
    // Java field: _mBuffer: &[u8]
    pub(crate) mBuffer: Vec<u8>,
    // Java field: DumpPicBuffer next
    // Java field: Object sPoolSync
    // Java field: DumpPicBuffer sPool
    // Java field: int sPoolSize
    // Java field: int DEFAULT_MAX_POOL_SIZE
    // Java field: int sMaxPoolSize
    // Java field: int sNewInsCount
}

pub const DEFAULT_MAX_POOL_SIZE: i32 = 2;

// Java: public class DumpPicBuffer
impl DumpPicBuffer {
    // Java: static void setMaxPoolSize(int size)
    pub fn setMaxPoolSize(size: i32) {
      // Java: sMaxPoolSize = size;
      let mut state = get_pool().lock().unwrap();
      state.max_pool_size = size;
      // Java: synchronized(sPoolSync)
      // Java: while(sPoolSize > sMaxPoolSize)
      while state.pool_size > state.max_pool_size
      {
         // Java: DumpPicBuffer m = sPool;
         // Java: sPool = m.next;
         // Java: m.next = null;
         state.pool.pop();
         // Java: --sPoolSize;
         state.pool_size -= 1;
      }
    }

    // Java: static void clearPool()
    pub fn clearPool() {
      // Java: synchronized(sPoolSync)
      let mut state = get_pool().lock().unwrap();
      // Java: while(null != sPool)
      while !state.pool.is_empty()
      {
         // Java: DumpPicBuffer m = sPool;
         // Java: sPool = m.next;
         // Java: m.next = null;
         state.pool.pop();
         // Java: --sPoolSize;
         state.pool_size -= 1;
      }
      // Java: sNewInsCount = 0;
      state.new_ins_count = 0;
    }

    // Java: static DumpPicBuffer obtain()
    pub fn obtain() -> DumpPicBuffer {
      // Java: synchronized(sPoolSync)
      {
        let mut state = get_pool().lock().unwrap();
        // Java: if (sPool != null)
        if !state.pool.is_empty()
        {
           // Java: DumpPicBuffer m = sPool;
           // Java: sPool = m.next;
           // Java: m.next = null;
           let m = state.pool.pop().unwrap();
           // Java: --sPoolSize;
           state.pool_size -= 1;
           // Java: return m;
           return m;
        }
      }
      // Java: return new DumpPicBuffer();
      DumpPicBuffer::new()
    }

    // Java: DumpPicBuffer()
    pub fn new() -> Self {
      // Java: super();
      Self {
        mBuffer: vec![0u8; BUF_SIZE],
      }
    }

    // Java: _getBuf: &[u8]()
    pub fn getBuf(&self) -> &[u8] {
      // Java: return this.mBuffer;
      &self.mBuffer
    }

    // Java: void recycle()
    pub fn recycle(self) {
      // Java: synchronized(sPoolSync)
      let mut state = get_pool().lock().unwrap();
      // Java: if (sPoolSize < sMaxPoolSize)
      if state.pool_size < state.max_pool_size
      {
         // Java: this.next = sPool;
         // Java: sPool = this;
         state.pool.push(self);
         // Java: ++sPoolSize;
         state.pool_size += 1;
      }
    }

}
