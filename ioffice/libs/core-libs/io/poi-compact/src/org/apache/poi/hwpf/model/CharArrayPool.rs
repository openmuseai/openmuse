#![allow(non_snake_case, dead_code, unused_variables, non_upper_case_globals)]

use std::sync::{Mutex, OnceLock};

// Java field: int sBufferSize
pub const sBufferSize: i32 = 256;

// Java field: CharArrayPool sInstance
pub fn sInstance() -> &'static Mutex<CharArrayPool> {
    static INSTANCE: OnceLock<Mutex<CharArrayPool>> = OnceLock::new();
    INSTANCE.get_or_init(|| Mutex::new(CharArrayPool::new(2)))
}

// Java: public class CharArrayPool
pub struct CharArrayPool {
    // Java field: Object mPoolSync
    mPoolSync: Mutex<()>,
    // Java field: char[][] mPool
    mPool: Option<Vec<Vec<char>>>,
    // Java field: int mMaxPoolSize
    mMaxPoolSize: i32,
    // Java field: int mPos
    mPos: i32,
}

// Java: public class CharArrayPool
impl CharArrayPool {
    // Java: CharArrayPool(int maxPoolSize)
    pub fn new(maxPoolSize: i32) -> Self {
      // Java: super();
      // Java: this.mMaxPoolSize = maxPoolSize;
      let mut s = CharArrayPool {
          mPoolSync: Mutex::new(()),
          mPool: None,
          mMaxPoolSize: maxPoolSize,
          mPos: 0,
      };
      // Java: this.initPool();
      s.initPool();
      s
    }

    // Java: char[] obtain(int len)
    pub fn obtain(&mut self, len: i32) -> Vec<char> {
      // Java: synchronized(this.mPoolSync)
      let _lock = self.mPoolSync.lock().unwrap();
      // Java: if (null == this.mPool)
      if self.mPool.is_none()
      {
         // Java: this.initPool();
         self.mPool = Some(vec![vec!['\0'; 256]; self.mMaxPoolSize as usize]);
         self.mPos = self.mMaxPoolSize;
      }
      // Java: if (this.mPos > 0 && len <= 256)
      if self.mPos > 0 && len <= 256
      {
         // Java: if (this.mPool[--this.mPos] == null)
         self.mPos -= 1;
         let pool = self.mPool.as_mut().unwrap();
         if pool[self.mPos as usize].is_empty()
         {
            // Java: this.mPool[this.mPos] = new char[256];
            pool[self.mPos as usize] = vec!['\0'; 256];
         }
         // Java: return this.mPool[this.mPos];
         return std::mem::replace(&mut pool[self.mPos as usize], Vec::new());
      }
      // Java: else
      {
         // Java: return new char[len];
         return vec!['\0'; len as usize];
      }
    }

    // Java: void give(char[] tag)
    pub fn give(&mut self, tag: Vec<char>) {
      // Java: if (null != tag)
      {
         // Java: synchronized(this.mPoolSync)
         let _lock = self.mPoolSync.lock().unwrap();
         // Java: if (this.mPos < this.mMaxPoolSize && tag.length == 256)
         if self.mPos < self.mMaxPoolSize && tag.len() == 256
         {
            // Java: this.mPool[this.mPos++] = tag;
            if let Some(ref mut pool) = self.mPool {
                pool[self.mPos as usize] = tag;
                self.mPos += 1;
            }
         }
      }
    }

    // Java: void dispose()
    pub fn dispose(&mut self) {
      // Java: synchronized(this.mPoolSync)
      let _lock = self.mPoolSync.lock().unwrap();
      // Java: this.mPool = null;
      self.mPool = None;
      // Java: this.mPos = 0;
      self.mPos = 0;
    }

    // Java: void initPool()
    fn initPool(&mut self) {
      // Java: this.mPool = new char[this.mMaxPoolSize][256];
      self.mPool = Some(vec![vec!['\0'; 256]; self.mMaxPoolSize as usize]);
      // Java: this.mPos = this.mMaxPoolSize;
      self.mPos = self.mMaxPoolSize;
    }
}
