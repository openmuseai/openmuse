use super::IIterator::IIterator;

// Java: public interface IDoubleIterator extends IIterator
pub trait IDoubleIterator: IIterator {
    // Java: abstract double next()
    fn next(&mut self) -> f64;
}
