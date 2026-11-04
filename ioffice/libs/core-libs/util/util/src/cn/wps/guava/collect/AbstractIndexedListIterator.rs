#![allow(non_snake_case, dead_code, unused_variables)]

use std::any::Any;

// Java: public class AbstractIndexedListIterator extends UnmodifiableListIterator
pub struct AbstractIndexedListIterator {
    // Java field: int size
    size: i32,
    // Java field: int position
    pub(crate) position: i32,
    get_fn: Box<dyn Fn(i32) -> Box<dyn Any>>,
}

// Java: public class AbstractIndexedListIterator extends UnmodifiableListIterator
impl AbstractIndexedListIterator {
    // Java: abstract E get(int var1)

    // Java: AbstractIndexedListIterator(int size)
    pub fn new(size: i32, get_fn: Box<dyn Fn(i32) -> Box<dyn Any>>) -> Self {
        // Java: this(size, 0);
        Self::new_with_position(size, 0, get_fn)
    }

    // Java: AbstractIndexedListIterator(int size, int position)
    pub fn new_with_position(size: i32, position: i32, get_fn: Box<dyn Fn(i32) -> Box<dyn Any>>) -> Self {
        // Java: super();
        // Java: Preconditions.checkPositionIndex(position, size);
        assert!(position >= 0 && position <= size, "index out of range");
        Self {
            // Java: this.size = size;
            size,
            // Java: this.position = position;
            position,
            get_fn,
        }
    }

    // Java: boolean hasNext()
    pub fn hasNext(&self) -> bool {
        // Java: return this.position < this.size;
        self.position < self.size
    }

    // Java: E next()
    pub fn next(&mut self) -> Box<dyn Any> {
        // Java: if (!this.hasNext())
        if !self.hasNext()
        {
            // Java: throw new NoSuchElementException();
            panic!("NoSuchElementException");
        }
        // Java: else
        {
            // Java: return (E)this.get(this.position++);
            let result = (self.get_fn)(self.position);
            self.position += 1;
            result
        }
    }

    // Java: int nextIndex()
    pub fn nextIndex(&self) -> i32 {
        // Java: return this.position;
        self.position
    }

    // Java: boolean hasPrevious()
    pub fn hasPrevious(&self) -> bool {
        // Java: return this.position > 0;
        self.position > 0
    }

    // Java: E previous()
    pub fn previous(&mut self) -> Box<dyn Any> {
        // Java: if (!this.hasPrevious())
        if !self.hasPrevious()
        {
            // Java: throw new NoSuchElementException();
            panic!("NoSuchElementException");
        }
        // Java: else
        {
            // Java: return (E)this.get(--this.position);
            self.position -= 1;
            (self.get_fn)(self.position)
        }
    }

    // Java: int previousIndex()
    pub fn previousIndex(&self) -> i32 {
        // Java: return this.position - 1;
        self.position - 1
    }
}
