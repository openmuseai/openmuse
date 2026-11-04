#![allow(non_snake_case, dead_code, unused_variables)]

use std::any::Any;

// Java: public class UnmodifiableIterator implements Iterator
pub trait UnmodifiableIterator {
    type Item;

    // Java: UnmodifiableIterator()
    // Java: super();

    fn has_next(&self) -> bool;
    fn next(&mut self) -> Option<Self::Item>;

    // Java: void remove()
    fn remove(&self) {
        // Java: throw new UnsupportedOperationException();
        panic!("UnsupportedOperationException: remove not supported");
    }
}
