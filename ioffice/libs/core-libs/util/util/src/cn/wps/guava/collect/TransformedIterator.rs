#![allow(non_snake_case, dead_code, unused_variables, unused_imports)]

use std::any::Any;

// Java: import java.util.Iterator

// Java: public class TransformedIterator implements Iterator
pub struct TransformedIterator {
    // Java field: Iterator<? extends F> backingIterator
    pub backingIterator: Option<Box<dyn Any>>,
}

// Java: public class TransformedIterator implements Iterator
impl TransformedIterator {
    // Java: TransformedIterator(Iterator<? extends F> backingIterator)
    pub fn new(backingIterator: Option<Box<dyn Any>>) -> Self {
        // Java: super();
        // Java: this.backingIterator = (Iterator)Preconditions.checkNotNull(backingIterator);
        TransformedIterator { backingIterator }
    }

    // Java: abstract abstract T transform(F var1)
    pub fn transform(&self, _var1: &dyn Any) -> Option<Box<dyn Any>> {
        None
    }

    // Java: boolean hasNext()
    pub fn hasNext(&self) -> bool {
        // Java: return this.backingIterator.hasNext();
        false
    }

    // Java: T next()
    pub fn next(&self) -> Option<Box<dyn Any>> {
        // Java: return (T)this.transform(this.backingIterator.next());
        None
    }

    // Java: void remove()
    pub fn remove(&self) {
        // Java: this.backingIterator.remove();
    }
}
