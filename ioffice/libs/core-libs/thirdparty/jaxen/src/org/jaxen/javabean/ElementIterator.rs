// Java: import java.util.Iterator
use crate::org::jaxen::Object;
use super::Element::Element;

// Java: public class ElementIterator implements Iterator
pub struct ElementIterator {
    // Java field: Element parent
    pub(crate) parent: Option<Box<Element>>,
    // Java field: String name
    pub(crate) name: String,
    // Java field: Iterator iterator
    pub(crate) items: Vec<Object>,
    pub(crate) index: usize,
}

// Java: public class ElementIterator implements Iterator
impl ElementIterator {
    // Java: ElementIterator(Element parent, String name, Iterator iterator)
    pub fn new(parent: Option<Box<Element>>, name: String, items: Vec<Object>) -> Self {
      // Java: super();
      // Java: this.parent = parent;
      // Java: this.name = name;
      // Java: this.iterator = iterator;
      ElementIterator { parent, name, items, index: 0 }
    }

    // Java: boolean hasNext()
    pub fn hasNext(&self) -> bool {
      // Java: return this.iterator.hasNext();
      self.index < self.items.len()
    }

    // Java: Object next()
    pub fn next_element(&mut self) -> Option<Object> {
      // Java: return new Element(this.parent, this.name, this.iterator.next());
      if self.index < self.items.len() {
         let obj = self.items[self.index].clone();
         self.index += 1;
         Some(std::sync::Arc::new(Element::new(None, self.name.clone(), obj)))
      } else {
         None
      }
    }

    // Java: void remove()
    pub fn remove(&self) {
      // Java: throw new UnsupportedOperationException();
      panic!("UnsupportedOperationException");
    }
}

impl Iterator for ElementIterator {
    type Item = Object;

    fn next(&mut self) -> Option<Object> {
      self.next_element()
    }
}
