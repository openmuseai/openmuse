// Java: import java.util.ArrayList
// Java: import java.util.Iterator
// Java: import java.util.NoSuchElementException
// Java: import org.jaxen.JaxenRuntimeException
use crate::org::jaxen::JaxenRuntimeException::JaxenRuntimeException;
// Java: import org.jaxen.Navigator
use crate::org::jaxen::Navigator::Navigator;
// Java: import org.jaxen.UnsupportedAxisException
use crate::org::jaxen::UnsupportedAxisException::UnsupportedAxisException;
use crate::org::jaxen::{Object, NodeIterator};

// Java: public class DescendantAxisIterator implements Iterator
pub struct DescendantAxisIterator {
    // Java field: ArrayList stack
    stack: Vec<NodeIterator>,
    // Java field: Iterator children
    children: NodeIterator,
    // Java field: Navigator navigator
    navigator: Box<dyn Navigator>,
}

// Java: public class DescendantAxisIterator implements Iterator
impl DescendantAxisIterator {
    // Java: DescendantAxisIterator(Object contextNode, Navigator navigator)
    pub fn new(contextNode: &Object, navigator: Box<dyn Navigator>) -> Self {
        // Java: this(navigator, navigator.getChildAxisIterator(contextNode));
        let children = navigator.getChildAxisIterator(contextNode)
            .unwrap_or_else(|_| Box::new(std::iter::empty()));
        Self::new_with_iterator(navigator, children)
    }

    // Java: DescendantAxisIterator(Navigator navigator, Iterator iterator)
    pub fn new_with_iterator(navigator: Box<dyn Navigator>, iterator: NodeIterator) -> Self {
        // Java: super();
        // Java: this.stack = new ArrayList();
        // Java: this.navigator = navigator;
        // Java: this.children = iterator;
        DescendantAxisIterator {
            stack: Vec::new(),
            children: iterator,
            navigator,
        }
    }

    // Java: boolean hasNext()
    pub fn hasNext(&mut self) -> bool {
        // Java: while(!this.children.hasNext())
        while !self.children_has_next() {
            // Java: if (this.stack.isEmpty())
            if self.stack.is_empty() {
                // Java: return false;
                return false;
            }
            // Java: this.children = (Iterator)this.stack.remove(this.stack.size() - 1);
            self.children = self.stack.pop().unwrap();
        }
        // Java: return true;
        true
    }

    fn children_has_next(&self) -> bool {
        self.children.size_hint().0 > 0 || self.children.size_hint().1 != Some(0)
    }

    // Java: Object next()
    pub fn next_node(&mut self) -> Object {
        // Java: try
        // Java: if (this.hasNext())
        if self.hasNext() {
            // Java: Object node = this.children.next();
            let node = self.children.next().unwrap();
            // Java: this.stack.add(this.children);
            let old_children = std::mem::replace(
                &mut self.children,
                Box::new(std::iter::empty()),
            );
            self.stack.push(old_children);
            // Java: this.children = this.navigator.getChildAxisIterator(node);
            self.children = self.navigator.getChildAxisIterator(&node)
                .unwrap_or_else(|_| Box::new(std::iter::empty()));
            // Java: return node;
            node
        }
        // Java: else
        else {
            // Java: throw new NoSuchElementException();
            panic!("NoSuchElementException");
        }
        // Java: catch (UnsupportedAxisException e)
        // Java: throw new JaxenRuntimeException(e);
    }

    // Java: void remove()
    pub fn remove(&self) {
        // Java: throw new UnsupportedOperationException();
        panic!("UnsupportedOperationException");
    }
}

impl Iterator for DescendantAxisIterator {
    type Item = Object;

    fn next(&mut self) -> Option<Self::Item> {
        if self.hasNext() {
            Some(self.next_node())
        } else {
            None
        }
    }
}
