#![allow(non_snake_case, dead_code)]

use super::IIterator::IIterator;

// Java: public interface IIntIterator extends IIterator
// LLM Rust Pattern: Java interface extends -> Rust trait supertrait (no Deref needed; parent: IIterator is the supertrait bound)
// impl Deref for IIntIterator not needed: trait inheritance via supertrait bound
pub trait IIntIterator: IIterator {
    // Java: abstract int next()
    fn next(&mut self) -> i32;
}
