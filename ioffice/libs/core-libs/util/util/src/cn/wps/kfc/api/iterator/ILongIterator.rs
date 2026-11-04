// Java: public interface ILongIterator extends IIterator
pub trait ILongIterator {
    // Java: abstract long next()
    fn next(&mut self) -> i64;
    fn hasNext(&self) -> bool;
    fn remove(&mut self);
}
