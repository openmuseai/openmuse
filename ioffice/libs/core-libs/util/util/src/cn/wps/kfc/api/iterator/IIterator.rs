// Java: public interface IIterator
pub trait IIterator {
    // Java: abstract boolean hasNext()
    fn hasNext(&self) -> bool;

    // Java: abstract void remove()
    fn remove(&mut self);
}
