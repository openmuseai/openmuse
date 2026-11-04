#![allow(non_snake_case, non_upper_case_globals, dead_code)]

use std::sync::{Mutex, OnceLock};

// Java field: String TAG
const TAG: Option<&str> = None;
// Java field: int BUFFER_SIZE
pub(crate) const BUFFER_SIZE: i32 = 16384;
// Java field: Object sPoolSync
// Java field: int MaxPoolSize
const MaxPoolSize: i32 = 8;

struct PoolState {
    // Java field: BufferBytes sPool
    sPool: Vec<BufferBytes>,
    // Java field: int sPoolSize
    sPoolSize: i32,
    // Java field: int sNewSize
    sNewSize: i32,
}

fn get_pool_state() -> &'static Mutex<PoolState> {
    static POOL: OnceLock<Mutex<PoolState>> = OnceLock::new();
    POOL.get_or_init(|| {
        Mutex::new(PoolState {
            sPool: Vec::new(),
            sPoolSize: 0,
            sNewSize: 0,
        })
    })
}

// Java: public class BufferBytes
pub struct BufferBytes {
    // Java field: byte[] bytes
    pub(crate) bytes: Vec<u8>,
    // Java field: BufferBytes next
}

// Java: public class BufferBytes
impl BufferBytes {
    // Java: BufferBytes()
    pub fn new() -> Self {
        // Java: super();
        // Java: this.bytes = new byte[16384];
        BufferBytes {
            bytes: vec![0u8; BUFFER_SIZE as usize],
        }
    }

    // Java: static void clearPool()
    pub fn clearPool() {
        // Java: synchronized(sPoolSync)
        let mut pool = get_pool_state().lock().unwrap();
        // Java: while(null != sPool)
        while let Some(_m) = pool.sPool.pop() {
            // Java: BufferBytes m = sPool;
            // Java: sPool = m.next;
            // Java: m.next = null;
            // Java: --sPoolSize;
            pool.sPoolSize -= 1;
        }
        // Java: sNewSize = 0;
        pool.sNewSize = 0;
    }

    // Java: static BufferBytes obtain()
    pub fn obtain() -> BufferBytes {
        // Java: synchronized(sPoolSync)
        {
            let mut pool = get_pool_state().lock().unwrap();
            // Java: if (sPool != null)
            if let Some(m) = pool.sPool.pop() {
                // Java: BufferBytes m = sPool;
                // Java: sPool = m.next;
                // Java: m.next = null;
                // Java: --sPoolSize;
                pool.sPoolSize -= 1;
                // Java: return m;
                return m;
            }
        }
        // Java: return new BufferBytes();
        BufferBytes::new()
    }

    // Java: void recycle()
    pub fn recycle(self) {
        // Java: synchronized(sPoolSync)
        let mut pool = get_pool_state().lock().unwrap();
        // Java: if (sPoolSize < 8)
        if pool.sPoolSize < MaxPoolSize {
            // Java: this.next = sPool;
            // Java: sPool = this;
            pool.sPool.push(self);
            // Java: ++sPoolSize;
            pool.sPoolSize += 1;
        }
    }
}
