// Java field: int MAX_CAPACITY
pub(crate) const MAX_CAPACITY: i32 = 10;

// Java: public class PoolableObjectFactory
pub trait PoolableObjectFactory {
    type Item;

    // Java: PoolableObjectFactory()
    // Java: super();

    // Java: abstract T newObject()
    fn newObject(&self) -> Self::Item;

    // Java: abstract void passivateObject(T var1)
    fn passivateObject(&self, var1: &mut Self::Item);

    // Java: int maxPooled()
    fn maxPooled(&self) -> i32 {
      // Java: return 10;
      10
    }

}
