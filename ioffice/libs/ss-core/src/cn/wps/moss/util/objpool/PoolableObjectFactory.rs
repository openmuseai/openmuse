// java public abstract class PoolableObjectFactory<T> {
pub trait PoolableObjectFactory<T>{
    // java public abstract T newObject();
    fn newObject(&self)->T;
    // java public abstract void passivateObject(T var1);
    fn passivateObject(&self,var1:&T);
    // java public int maxPooled() { return 20; }
    fn maxPooled(&self)->i32{20}
}
