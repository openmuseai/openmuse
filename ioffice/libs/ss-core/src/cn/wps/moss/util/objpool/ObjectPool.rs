use super::{ConcurrentArrayQueue::ConcurrentArrayQueue,PoolableObjectFactory::PoolableObjectFactory};
// java public class ObjectPool<T> {
pub struct ObjectPool<T:Clone,F:PoolableObjectFactory<T>>{
    // java private final ConcurrentArrayQueue<T> pool;
    pool:ConcurrentArrayQueue<T>,
    // java private final PoolableObjectFactory<T> factory;
    factory:F,
}
impl<T:Clone,F:PoolableObjectFactory<T>> ObjectPool<T,F>{
    // java public ObjectPool(PoolableObjectFactory<T> factory) { this.factory = factory; this.pool = new ConcurrentArrayQueue<T>(factory.maxPooled()); }
    pub fn ObjectPool(factory:F)->Self{let pool=ConcurrentArrayQueue::ConcurrentArrayQueue(factory.maxPooled());Self{pool,factory}}
    // java public T borrowObject() { T ret = this.pool.tryPop(); if (ret == null) { ret = this.factory.newObject(); } return ret; }
    pub fn borrowObject(&self)->T{self.pool.tryPop().unwrap_or_else(||self.factory.newObject())}
    // java public T returnObject(T obj) {
    pub fn returnObject(&self,obj:Option<T>)->Option<T>{
        // java if (obj == null) { return null; } else {
        let Some(obj)=obj else{return None;};
        // java this.factory.passivateObject(obj);
        self.factory.passivateObject(&obj);
        // java try { this.pool.tryPush(obj); } catch (Exception var3) { } return null;
        let _=std::panic::catch_unwind(std::panic::AssertUnwindSafe(||self.pool.tryPush(Some(obj))));None
    }
    // java public void clear() { this.pool.clear(); }
    pub fn clear(&self){self.pool.clear();}
}
