use super::ConcurrentArrayQueue::ConcurrentArrayQueue;
use super::PoolableObjectFactory::PoolableObjectFactory;

// Java: public class ObjectPool
pub struct ObjectPool<F: PoolableObjectFactory> {
    // Java field: ConcurrentArrayQueue<T> mPool
    mPool: ConcurrentArrayQueue<F::Item>,
    // Java field: PoolableObjectFactory<T> mFactory
    mFactory: F,
}

// Java: public class ObjectPool
impl<F: PoolableObjectFactory> ObjectPool<F> {
    // Java: ObjectPool(PoolableObjectFactory<T> factory)
    pub fn new(factory: F) -> Self {
      // Java: super();
      // Java: this.mFactory = factory;
      // Java: this.mPool = new ConcurrentArrayQueue<T>(factory.maxPooled());
      let capacity = factory.maxPooled();
      ObjectPool {
          mPool: ConcurrentArrayQueue::new(capacity),
          mFactory: factory,
      }
    }

    // Java: T borrowObject()
    pub fn borrowObject(&self) -> F::Item {
      // Java: T ret = this.mPool.tryPop();
      let ret = self.mPool.tryPop();
      // Java: if (ret == null)
      match ret {
          None =>
      {
         // Java: ret = this.mFactory.newObject();
         self.mFactory.newObject()
      }
      // Java: return ret;
          Some(val) => val,
      }
    }

    // Java: T returnObject(T obj)
    pub fn returnObject(&self, obj: Option<F::Item>) {
      // Java: if (obj == null)
      let obj = match obj {
          None =>
      {
         // Java: return null;
         return;
      }
      // Java: else
          Some(o) => o,
      };
      {
         // Java: this.mFactory.passivateObject(obj);
         // Java: this.mPool.tryPush(obj);
         self.mPool.tryPush(obj);
         // Java: return null;
      }
    }

    // Java: void clear()
    pub fn clear(&self) {
      // Java: this.mPool.clear();
      self.mPool.clear();
    }

}
