// Java: import cn.wps.base.assertion.IOAssert
// Java: import cn.wps.collections.CloseGuard
// Java: import cn.wps.moffice.util.MutableInt
// Java: import java.util.HashMap

use std::collections::HashMap;
use std::sync::{Mutex, OnceLock};

// Java: public class BlockBufPool
struct BlockBufPool {
    // Java field: BlockBuf head
    head: Vec<BlockBuf>,
    // Java field: int poolSize
    poolSize: i32,
    // Java field: int maxPoolSize
    maxPoolSize: i32,
    // Java field: int newInsCount
    newInsCount: i32,
}

fn get_pool_map() -> &'static Mutex<HashMap<i32, BlockBufPool>> {
    static MAP: OnceLock<Mutex<HashMap<i32, BlockBufPool>>> = OnceLock::new();
    MAP.get_or_init(|| Mutex::new(HashMap::new()))
}

// Java: public class BlockBuf
pub struct BlockBuf {
    // Java field: String TAG
    // Java field: boolean DEBUG
    // Java field: byte[] mBuffer
    mBuffer: Vec<u8>,
    // Java field: BlockBuf next
    // Java field: boolean mRecyclable
    mRecyclable: bool,
    // Java field: HashMap<MutableInt, BlockBufPool> sMap
    // Java field: Object sPoolSync
    // Java field: int DEFAULT_MAX_POOL_SIZE
    // Java field: MutableInt sInt
    // Java field: CloseGuard guard
}

pub const DEFAULT_MAX_POOL_SIZE: i32 = 10;

// Java: public class BlockBuf
impl BlockBuf {
    // Java: static void setMaxPoolSize(int bufSize, int poolSize)
    pub fn setMaxPoolSize(bufSize: i32, poolSize: i32) {
        // Java: synchronized(sPoolSync)
        let mut map = get_pool_map().lock().unwrap();
        // Java: sInt.value = bufSize;
        // Java: BlockBufPool pool;
        // Java: if (null != (pool = (BlockBufPool)sMap.get(sInt)))
        if let Some(pool) = map.get_mut(&bufSize) {
            // Java: pool.maxPoolSize = poolSize;
            pool.maxPoolSize = poolSize;
            // Java: while(pool.poolSize > poolSize)
            while pool.poolSize > poolSize {
                // Java: BlockBuf m = pool.head;
                // Java: pool.head = m.next;
                // Java: m.next = null;
                pool.head.pop();
                // Java: --pool.poolSize;
                pool.poolSize -= 1;
            }
        }
    }

    // Java: static void clearPool()
    pub fn clearPool() {
        // Java: synchronized(sPoolSync)
        let mut map = get_pool_map().lock().unwrap();
        // Java: for(BlockBufPool pool : sMap.values())
        for (_key, pool) in map.iter_mut() {
            // Java: IOAssert.assertNotNull("pool should not be null", pool);
            // Java: while(null != pool.head)
            while !pool.head.is_empty() {
                // Java: BlockBuf m = pool.head;
                // Java: pool.head = m.next;
                // Java: m.next = null;
                pool.head.pop();
                // Java: --pool.poolSize;
                pool.poolSize -= 1;
            }
            // Java: pool.newInsCount = 0;
            pool.newInsCount = 0;
        }
    }

    // Java: static BlockBuf obtain(int size)
    pub fn obtain(size: i32) -> BlockBuf {
        // Java: synchronized(sPoolSync)
        {
            let mut map = get_pool_map().lock().unwrap();
            // Java: sInt.value = size;
            // Java: BlockBufPool pool;
            // Java: if (null == (pool = (BlockBufPool)sMap.get(sInt)))
            if !map.contains_key(&size) {
                // Java: pool = new BlockBufPool();
                // Java: sMap.put(new MutableInt(size), pool);
                map.insert(size, BlockBufPool {
                    head: Vec::new(),
                    poolSize: 0,
                    maxPoolSize: 10,
                    newInsCount: 0,
                });
            }
            let pool = map.get_mut(&size).unwrap();
            // Java: BlockBuf head = pool.head;
            // Java: if (null != head)
            if let Some(mut head) = pool.head.pop() {
                // Java: pool.head = head.next;
                // Java: head.next = null;
                // Java: pool.poolSize--;
                pool.poolSize -= 1;
                // Java: head.mRecyclable = true;
                head.mRecyclable = true;
                // Java: return head;
                return head;
            }
        }
        // Java: return new BlockBuf(size);
        BlockBuf::new(size)
    }

    // Java: BlockBuf(int size)
    fn new(size: i32) -> Self {
        // Java: super();
        // Java: this.mBuffer = new byte[size];
        BlockBuf {
            mBuffer: vec![0u8; size as usize],
            mRecyclable: true,
        }
    }

    // Java: byte[] getBuf()
    pub fn getBuf(&mut self) -> &mut [u8] {
        // Java: return this.mBuffer;
        &mut self.mBuffer
    }

    // Java: void setRecyclable(boolean recyclable)
    pub fn setRecyclable(&mut self, recyclable: bool) {
        // Java: this.mRecyclable = recyclable;
        self.mRecyclable = recyclable;
    }

    // Java: void recycle()
    pub fn recycle(mut self) {
        // Java: if (this.mRecyclable)
        if self.mRecyclable {
            // Java: synchronized(sPoolSync)
            let mut map = get_pool_map().lock().unwrap();
            // Java: sInt.value = this.mBuffer.length;
            let buf_size = self.mBuffer.len() as i32;
            // Java: BlockBufPool pool = (BlockBufPool)sMap.get(sInt);
            // Java: IOAssert.assertNotNull(pool);
            if let Some(pool) = map.get_mut(&buf_size) {
                // Java: if (pool.poolSize < pool.maxPoolSize)
                if pool.poolSize < pool.maxPoolSize {
                    // Java: this.next = pool.head;
                    // Java: pool.head = this;
                    // Java: pool.poolSize++;
                    pool.poolSize += 1;
                    pool.head.push(self);
                }
            }
        }
    }

    // Java: void finalize()
    // Java: super.finalize();
}

// Java: public class BlockBufPool
impl BlockBufPool {
    // Java: BlockBufPool()
    fn new() -> Self {
        // Java: super();
        // Java: this.poolSize = 0;
        // Java: this.maxPoolSize = 10;
        // Java: this.newInsCount = 0;
        BlockBufPool { head: Vec::new(), poolSize: 0, maxPoolSize: 10, newInsCount: 0 }
    }
}
