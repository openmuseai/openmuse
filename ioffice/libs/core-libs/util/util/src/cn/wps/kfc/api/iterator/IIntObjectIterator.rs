// Java: public interface IIntObjectIterator extends IIterator
pub trait IIntObjectIterator<V> {
    // Java: int key()
    fn key(&self) -> i32;

    // Java: V value()
    fn value(&self) -> &V;

    // Java: V setValue(V var1)
    fn setValue(&mut self, val: V) -> V;

    // Java: void advance()
    fn advance(&mut self);

    // From IIterator
    fn hasNext(&self) -> bool;
    fn remove(&mut self);
}
