// Java: import java.util.ArrayList
// Java: import java.util.Iterator
// Java: import java.util.List
// Java: import java.util.NoSuchElementException
use crate::org::jaxen::{Object, NodeIterator};

// Java: public class LinkedIterator implements Iterator
pub struct LinkedIterator {
    // Java field: List iterators
    pub(crate) iterators: Vec<NodeIterator>,
    // Java field: int cur
    pub(crate) cur: usize,
}

// Java: public class LinkedIterator implements Iterator
impl LinkedIterator {
    // Java: LinkedIterator()
    pub fn new() -> Self {
      // Java: super();
      LinkedIterator {
         iterators: Vec::new(),
         cur: 0,
      }
    }

    // Java: void addIterator(Iterator i)
    pub fn addIterator(&mut self, i: NodeIterator) {
      // Java: this.iterators.add(i);
      self.iterators.push(i);
    }

    // Java: boolean hasNext()
    pub fn hasNext(&mut self) -> bool {
      // Java: boolean has = false;
      // Java: if (this.cur < this.iterators.size())
      if self.cur < self.iterators.len() {
         // Java: has = ((Iterator)this.iterators.get(this.cur)).hasNext();
         // Java: if (!has && this.cur < this.iterators.size())
         // Java: ++this.cur;
         // Java: has = this.hasNext();
         return true;
      }
      // Java: else
      // Java: has = false;
      // Java: return has;
      false
    }

    // Java: Object next()
    pub fn next_item(&mut self) -> Option<Object> {
      // Java: if (!this.hasNext())
      // Java: throw new NoSuchElementException();
      // Java: else
      // Java: return ((Iterator)this.iterators.get(this.cur)).next();
      while self.cur < self.iterators.len() {
         if let Some(item) = self.iterators[self.cur].next() {
            return Some(item);
         }
         self.cur += 1;
      }
      None
    }

    // Java: void remove()
    pub fn remove(&self) {
      // Java: throw new UnsupportedOperationException();
      panic!("UnsupportedOperationException");
    }
}

impl Iterator for LinkedIterator {
    type Item = Object;

    fn next(&mut self) -> Option<Object> {
      self.next_item()
    }
}
