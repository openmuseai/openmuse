// Java: import java.util.ArrayList
// Java: import java.util.Iterator
// Java: import java.util.ListIterator
// Java: import java.util.NoSuchElementException
// Java: import org.jaxen.JaxenConstants
use crate::org::jaxen::JaxenConstants;
// Java: import org.jaxen.JaxenRuntimeException
use crate::org::jaxen::JaxenRuntimeException::JaxenRuntimeException;
// Java: import org.jaxen.Navigator
use crate::org::jaxen::Navigator::Navigator;
// Java: import org.jaxen.UnsupportedAxisException
use crate::org::jaxen::UnsupportedAxisException::UnsupportedAxisException;
use crate::org::jaxen::{Object, NodeIterator};
use super::PrecedingSiblingAxisIterator::PrecedingSiblingAxisIterator;

struct ListIter {
    items: Vec<Object>,
    pos: usize,
}

impl ListIter {
    fn new(items: Vec<Object>) -> Self {
        let pos = items.len();
        ListIter { items, pos }
    }

    fn empty() -> Self {
        ListIter { items: Vec::new(), pos: 0 }
    }

    fn has_previous(&self) -> bool {
        self.pos > 0
    }

    fn previous(&mut self) -> Object {
        self.pos -= 1;
        self.items[self.pos].clone()
    }
}

// Java: public class PrecedingAxisIterator implements Iterator
pub struct PrecedingAxisIterator {
    // Java field: Iterator ancestorOrSelf
    ancestorOrSelf: NodeIterator,
    // Java field: Iterator precedingSibling
    precedingSibling: NodeIterator,
    // Java field: ListIterator childrenOrSelf
    childrenOrSelf: ListIter,
    // Java field: ArrayList stack
    stack: Vec<ListIter>,
    // Java field: Navigator navigator
    navigator: Box<dyn Navigator>,
}

// Java: public class PrecedingAxisIterator implements Iterator
impl PrecedingAxisIterator {
    // Java: PrecedingAxisIterator(Object contextNode, Navigator navigator)
    pub fn new(contextNode: &Object, navigator: Box<dyn Navigator>) -> Self {
        // Java: super();
        // Java: this.navigator = navigator;
        // Java: this.ancestorOrSelf = navigator.getAncestorOrSelfAxisIterator(contextNode);
        let ancestorOrSelf = navigator.getAncestorOrSelfAxisIterator(contextNode)
            .unwrap_or_else(|_| Box::new(std::iter::empty()));
        // Java: this.precedingSibling = JaxenConstants.EMPTY_ITERATOR;
        let precedingSibling: NodeIterator = Box::new(std::iter::empty());
        // Java: this.childrenOrSelf = JaxenConstants.EMPTY_LIST_ITERATOR;
        let childrenOrSelf = ListIter::empty();
        // Java: this.stack = new ArrayList();
        PrecedingAxisIterator {
            ancestorOrSelf,
            precedingSibling,
            childrenOrSelf,
            stack: Vec::new(),
            navigator,
        }
    }

    // Java: boolean hasNext()
    pub fn hasNext(&mut self) -> bool {
        // Java: try
        // Java: while(!this.childrenOrSelf.hasPrevious())
        while !self.childrenOrSelf.has_previous() {
            // Java: if (this.stack.isEmpty())
            if self.stack.is_empty() {
                // Java: while(!this.precedingSibling.hasNext())
                loop {
                    if let Some(_) = self.peek_preceding_sibling() {
                        break;
                    }
                    // Java: if (!this.ancestorOrSelf.hasNext())
                    match self.ancestorOrSelf.next() {
                        None => {
                            // Java: return false;
                            return false;
                        }
                        Some(context_node) => {
                            // Java: Object contextNode = this.ancestorOrSelf.next();
                            // Java: this.precedingSibling = new PrecedingSiblingAxisIterator(contextNode, this.navigator);
                            self.precedingSibling = Box::new(
                                PrecedingSiblingAxisIterator::new(context_node, self.clone_navigator())
                            );
                        }
                    }
                }
                // Java: Object node = this.precedingSibling.next();
                let node = self.precedingSibling.next().unwrap();
                // Java: this.childrenOrSelf = this.childrenOrSelf(node);
                self.childrenOrSelf = self.make_children_or_self(&node);
            }
            // Java: else
            else {
                // Java: this.childrenOrSelf = (ListIterator)this.stack.remove(this.stack.size() - 1);
                self.childrenOrSelf = self.stack.pop().unwrap();
            }
        }
        // Java: return true;
        true
        // Java: catch (UnsupportedAxisException e)
        // Java: throw new JaxenRuntimeException(e);
    }

    fn peek_preceding_sibling(&mut self) -> Option<()> {
        if self.precedingSibling.size_hint().1 == Some(0) {
            None
        } else {
            Some(())
        }
    }

    // Java: ListIterator childrenOrSelf(Object node)
    fn make_children_or_self(&self, node: &Object) -> ListIter {
        // Java: try
        // Java: ArrayList reversed = new ArrayList();
        let mut reversed = Vec::new();
        // Java: reversed.add(node);
        reversed.push(node.clone());
        // Java: Iterator childAxisIterator = this.navigator.getChildAxisIterator(node);
        // Java: if (childAxisIterator != null)
        if let Ok(mut child_iter) = self.navigator.getChildAxisIterator(node) {
            // Java: while(childAxisIterator.hasNext())
            while let Some(child) = child_iter.next() {
                // Java: reversed.add(childAxisIterator.next());
                reversed.push(child);
            }
        }
        // Java: return reversed.listIterator(reversed.size());
        ListIter::new(reversed)
        // Java: catch (UnsupportedAxisException e)
        // Java: throw new JaxenRuntimeException(e);
    }

    fn clone_navigator(&self) -> Box<dyn Navigator> {
        unimplemented!("Navigator cloning not yet supported")
    }

    // Java: Object next()
    pub fn next_node(&mut self) -> Object {
        // Java: if (!this.hasNext())
        if !self.hasNext() {
            // Java: throw new NoSuchElementException();
            panic!("NoSuchElementException");
        }
        // Java: else
        // Java: while(true)
        loop {
            // Java: Object result = this.childrenOrSelf.previous();
            let result = self.childrenOrSelf.previous();
            // Java: if (!this.childrenOrSelf.hasPrevious())
            if !self.childrenOrSelf.has_previous() {
                // Java: return result;
                return result;
            }
            // Java: this.stack.add(this.childrenOrSelf);
            let old = std::mem::replace(&mut self.childrenOrSelf, ListIter::empty());
            self.stack.push(old);
            // Java: this.childrenOrSelf = this.childrenOrSelf(result);
            self.childrenOrSelf = self.make_children_or_self(&result);
        }
    }

    // Java: void remove()
    pub fn remove(&self) {
        // Java: throw new UnsupportedOperationException();
        panic!("UnsupportedOperationException");
    }
}

impl Iterator for PrecedingAxisIterator {
    type Item = Object;

    fn next(&mut self) -> Option<Self::Item> {
        if self.hasNext() {
            Some(self.next_node())
        } else {
            None
        }
    }
}
