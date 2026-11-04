// Java: import java.util.Iterator
// Java: import java.util.NoSuchElementException
use crate::org::jaxen::Object;

// Java: public class SingleObjectIterator implements Iterator
pub struct SingleObjectIterator {
    // Java field: Object object
    pub(crate) object: Object,
    // Java field: boolean seen
    seen: bool,
}

// Java: public class SingleObjectIterator implements Iterator
impl SingleObjectIterator {
    // Java: SingleObjectIterator(Object object)
    pub fn new(object: Object) -> Self {
        // Java: super();
        // Java: this.object = object;
        // Java: this.seen = false;
        SingleObjectIterator {
            object,
            seen: false,
        }
    }

    // Java: boolean hasNext()
    pub fn hasNext(&self) -> bool {
        // Java: return !this.seen;
        !self.seen
    }

    // Java: Object next()
    pub fn next_object(&mut self) -> Object {
        // Java: if (this.hasNext())
        if self.hasNext() {
            // Java: this.seen = true;
            self.seen = true;
            // Java: return this.object;
            self.object.clone()
        }
        // Java: else
        else {
            // Java: throw new NoSuchElementException();
            panic!("NoSuchElementException");
        }
    }

    // Java: void remove()
    pub fn remove(&self) {
        // Java: throw new UnsupportedOperationException();
        panic!("UnsupportedOperationException");
    }
}

impl Iterator for SingleObjectIterator {
    type Item = Object;

    fn next(&mut self) -> Option<Self::Item> {
        if self.hasNext() {
            Some(self.next_object())
        } else {
            None
        }
    }
}
