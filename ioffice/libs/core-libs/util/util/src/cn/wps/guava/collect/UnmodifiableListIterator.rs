#![allow(non_snake_case, dead_code, unused_variables)]

use std::any::Any;
use super::UnmodifiableIterator::UnmodifiableIterator;

// Java: public class UnmodifiableListIterator extends UnmodifiableIterator implements ListIterator
pub trait UnmodifiableListIterator: UnmodifiableIterator {
    // Java: UnmodifiableListIterator()
    // Java: super();

    fn has_previous(&self) -> bool;
    fn previous(&mut self) -> Option<Self::Item>;
    fn next_index(&self) -> i32;
    fn previous_index(&self) -> i32;

    // Java: void add(E e)
    fn add(&self, _e: &dyn Any) {
        // Java: throw new UnsupportedOperationException();
        panic!("UnsupportedOperationException: add not supported");
    }

    // Java: void set(E e)
    fn set(&self, _e: &dyn Any) {
        // Java: throw new UnsupportedOperationException();
        panic!("UnsupportedOperationException: set not supported");
    }
}
