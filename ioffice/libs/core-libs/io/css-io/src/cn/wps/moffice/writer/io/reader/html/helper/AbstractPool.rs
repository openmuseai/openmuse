#![allow(dead_code, non_snake_case, non_upper_case_globals, non_camel_case_types)]

// Java: import cn.wps.moffice.util.KSLog
use std::sync::Mutex;

// Java: public class AbstractPool
pub struct AbstractPool {
    // Java field: Object mPoolSync
    mPoolSync: Mutex<()>,
    // Java field: Object[] mPool
    mPool: Option<Vec<Option<Box<dyn std::any::Any>>>>,
    // Java field: Class<E> mCls
    // Java field: int mMaxPoolSize
    mMaxPoolSize: i32,
    // Java field: int mPos
    mPos: i32,
}

// Java: public class AbstractPool
impl AbstractPool {
    // Java: AbstractPool(Class<E> cls, int maxPoolSize)
    pub fn new(maxPoolSize: i32) -> Self {
      // Java: super();
      // Java: this.mCls = cls;
      // Java: this.mMaxPoolSize = maxPoolSize;
      AbstractPool {
          mPoolSync: Mutex::new(()),
          mPool: None,
          mMaxPoolSize: maxPoolSize,
          mPos: 0,
      }
    }

    // Java: abstract String tag()
    pub fn tag(&self) -> String {
      todo!()
    }

    // Java: E obtain()
    pub fn obtain(&self) -> Option<Box<dyn std::any::Any>> {
      // Java: synchronized(this.mPoolSync)
      let _lock = self.mPoolSync.lock().unwrap();
      {
         // Java: return (E)this.newInstance();
         todo!()
      }
    }

    // Java: void give(E tag)
    pub fn give(&self, _tag: Box<dyn std::any::Any>) {
      // Java: synchronized(this.mPoolSync)
      let _lock = self.mPoolSync.lock().unwrap();
      {
         // Java: ;
      }
    }

    // Java: void dispose()
    pub fn dispose(&mut self) {
      // Java: synchronized(this.mPoolSync)
      let _lock = self.mPoolSync.lock().unwrap();
      {
         // Java: this.mPool = null;
         self.mPool = None;
         // Java: this.mPos = 0;
         self.mPos = 0;
      }
    }

    // Java: void initPool()
    pub fn initPool(&mut self) {
      // Java: this.mPool = new Object[this.mMaxPoolSize];
      self.mPool = Some(Vec::with_capacity(self.mMaxPoolSize as usize));
      // Java: this.mPos = this.mMaxPoolSize;
      self.mPos = self.mMaxPoolSize;
      // Java: for(int i = 0; i < this.mMaxPoolSize; ++i)
      for _i in 0..self.mMaxPoolSize
      {
         // Java: this.mPool[i] = this.newInstance();
         todo!()
      }
    }

    // Java: E newInstance()
    pub fn newInstance(&self) -> Option<Box<dyn std::any::Any>> {
      // Java: try
      // Java: return (E)this.mCls.newInstance();
      // Java: catch (InstantiationException e)
      // Java: KSLog.e(this.tag(), "InstantiationException", e);
      // Java: catch (IllegalAccessException e)
      // Java: KSLog.e(this.tag(), "IllegalAccessException", e);
      // Java: return null;
      todo!()
    }

}
