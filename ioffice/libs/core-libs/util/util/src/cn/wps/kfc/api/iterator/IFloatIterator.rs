use super::IIterator::IIterator;

// Java: public interface IFloatIterator extends IIterator
pub trait IFloatIterator: IIterator {
    // Java: abstract float next()
    fn next(&mut self) -> f32;
}
