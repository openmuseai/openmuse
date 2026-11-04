// Java: public interface ILongObjectIterator extends IIterator
pub trait ILongObjectIterator<V> {
    // Java: abstract long key()
    fn key(&self) -> i64;
    // Java: abstract V value()
    fn value(&self) -> &V;
    // Java: abstract V setValue(V var1)
    fn setValue(&mut self, val: V) -> V;
    // Java: abstract void advance()
    fn advance(&mut self);
}
