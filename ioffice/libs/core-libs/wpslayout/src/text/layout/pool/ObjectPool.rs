// Java: public class ObjectPool
pub struct ObjectPool {
    // Java field: ConcurrentArrayQueue<T> mPool
    mPool: ConcurrentArrayQueue<T>,
    // Java field: PoolableObjectFactory<T> mFactory
    mFactory: PoolableObjectFactory<T>,
}

// Java: public class ObjectPool
impl ObjectPool {
    // Java: ObjectPool(PoolableObjectFactory<T> factory)
    pub fn ObjectPool(&self, PoolableObjectFactory<T> factory) {
      // Java: super();
      // TODO_RUST_CODE
      // Java: this.mFactory = factory;
      // TODO_RUST_CODE
      // Java: this.mPool = new ConcurrentArrayQueue<T>(factory.maxPooled());
      // TODO_RUST_CODE
    }

    // Java: T borrowObject()
    pub fn borrowObject(&self) -> T {
      // Java: T ret = this.mPool.tryPop();
      // TODO_RUST_CODE
      // Java: if (ret == null)
        // TODO_RUST_CODE
      {
         // Java: ret = this.mFactory.newObject();
         // TODO_RUST_CODE
      }
      // Java: return ret;
      // TODO_RUST_CODE
    }

    // Java: T returnObject(T obj)
    pub fn returnObject(&self, T obj) -> T {
      // Java: if (obj == null)
        // TODO_RUST_CODE
      {
         // Java: return null;
         // TODO_RUST_CODE
      }
      // Java: else
        // TODO_RUST_CODE
      {
         // Java: this.mFactory.passivateObject(obj);
         // TODO_RUST_CODE
         // Java: this.mPool.tryPush(obj);
         // TODO_RUST_CODE
         // Java: return null;
         // TODO_RUST_CODE
      }
    }

    // Java: void clear()
    pub fn clear(&self) {
      // Java: this.mPool.clear();
      // TODO_RUST_CODE
    }

}
