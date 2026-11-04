// Java: import java.util.Iterator

// Java: public class SingleIterator implements Iterator
pub struct SingleIterator<T> {
    // Java field: boolean first
    pub(crate) first: bool,
    // Java field: T object
    object: Option<T>,
}

// Java: public class SingleIterator implements Iterator
impl<T> SingleIterator<T> {
    // Java: SingleIterator(T object)
    pub fn new(object: T) -> Self {
        // Java: super();
        // Java: this.object = object;
        SingleIterator {
            first: true,
            object: Some(object),
        }
    }

    // Java: boolean hasNext()
    pub fn hasNext(&self) -> bool {
        // Java: return this.first;
        self.first
    }

    // Java: T next()
    pub fn next_item(&mut self) -> Option<T> {
        // Java: T answer = this.object;
        // Java: this.object = null;
        // Java: this.first = false;
        // Java: return answer;
        if self.first {
            self.first = false;
            self.object.take()
        } else {
            None
        }
    }

    // Java: void remove()
    pub fn remove(&self) {
        // Java: throw new UnsupportedOperationException("remove() is not supported by this iterator");
        panic!("remove() is not supported by this iterator");
    }
}

impl<T> Iterator for SingleIterator<T> {
    type Item = T;

    fn next(&mut self) -> Option<T> {
        self.next_item()
    }
}
