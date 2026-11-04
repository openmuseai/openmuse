// Java: import java.util.HashSet
// Java: import java.util.Iterator
// Java: import java.util.LinkedList
// Java: import java.util.NoSuchElementException
// Java: import java.util.Set
// Java: import org.jaxen.Navigator
use crate::org::jaxen::Navigator::Navigator;
use crate::org::jaxen::{Object, NodeIterator};
use std::collections::VecDeque;

// Java: public class StackedIterator implements Iterator
pub struct StackedIterator {
    // Java field: LinkedList iteratorStack
    pub(crate) iteratorStack: VecDeque<NodeIterator>,
    // Java field: Navigator navigator
    pub(crate) navigator: Option<Box<dyn Navigator>>,
    // Java field: Set created
    pub(crate) created: Vec<usize>,
}

// Java: public class StackedIterator implements Iterator
impl StackedIterator {
    // Java: StackedIterator(Object contextNode, Navigator navigator)
    pub fn new(_contextNode: &Object, navigator: Box<dyn Navigator>) -> Self {
      // Java: super();
      // Java: this.init(contextNode, navigator);
      StackedIterator {
         iteratorStack: VecDeque::new(),
         navigator: Some(navigator),
         created: Vec::new(),
      }
    }

    // Java: StackedIterator()
    pub fn new_empty() -> Self {
      // Java: super();
      StackedIterator {
         iteratorStack: VecDeque::new(),
         navigator: None,
         created: Vec::new(),
      }
    }

    // Java: void init(Object contextNode, Navigator navigator)
    pub fn init(&mut self, _contextNode: &Object, navigator: Box<dyn Navigator>) {
      // Java: this.navigator = navigator;
      self.navigator = Some(navigator);
    }

    // Java: Iterator internalCreateIterator(Object contextNode)
    pub fn internalCreateIterator(&mut self, _contextNode: &Object) -> Option<NodeIterator> {
      // Java: if (this.created.contains(contextNode))
      // Java: return null;
      // Java: else
      // Java: this.created.add(contextNode);
      // Java: return this.createIterator(contextNode);
      todo!("StackedIterator::internalCreateIterator")
    }

    // Java: boolean hasNext()
    pub fn hasNext(&mut self) -> bool {
      // Java: Iterator curIter = this.currentIterator();
      // Java: return curIter == null ? false : curIter.hasNext();
      self.currentIterator_check()
    }

    // Java: Object next()
    pub fn next_item(&mut self) -> Option<Object> {
      // Java: if (!this.hasNext())
      // Java: throw new NoSuchElementException();
      // Java: else
      // Java: Iterator curIter = this.currentIterator();
      // Java: Object object = curIter.next();
      // Java: this.pushIterator(this.internalCreateIterator(object));
      // Java: return object;
      todo!("StackedIterator::next_item")
    }

    // Java: void remove()
    pub fn remove(&self) {
      // Java: throw new UnsupportedOperationException();
      panic!("UnsupportedOperationException");
    }

    // Java: abstract Iterator createIterator(Object var1)

    // Java: void pushIterator(Iterator iter)
    pub fn pushIterator(&mut self, iter: Option<NodeIterator>) {
      // Java: if (iter != null)
      if let Some(i) = iter {
         // Java: this.iteratorStack.addFirst(iter);
         self.iteratorStack.push_front(i);
      }
    }

    // Java: Iterator currentIterator()
    fn currentIterator_check(&mut self) -> bool {
      // Java: while(this.iteratorStack.size() > 0)
      // Java: Iterator curIter = (Iterator)this.iteratorStack.getFirst();
      // Java: if (curIter.hasNext())
      // Java: return curIter;
      // Java: this.iteratorStack.removeFirst();
      // Java: return null;
      !self.iteratorStack.is_empty()
    }

    // Java: Navigator getNavigator()
    pub fn getNavigator(&self) -> Option<&dyn Navigator> {
      // Java: return this.navigator;
      self.navigator.as_deref()
    }
}
