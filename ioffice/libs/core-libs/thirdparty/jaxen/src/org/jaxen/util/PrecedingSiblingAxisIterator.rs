// Java: import java.util.Iterator
// Java: import java.util.LinkedList
// Java: import java.util.NoSuchElementException
// Java: import org.jaxen.JaxenConstants
use crate::org::jaxen::JaxenConstants;
// Java: import org.jaxen.Navigator
use crate::org::jaxen::Navigator::Navigator;
// Java: import org.jaxen.UnsupportedAxisException
use crate::org::jaxen::UnsupportedAxisException::UnsupportedAxisException;
use crate::org::jaxen::{Object, NodeIterator};
use std::sync::Arc;
use std::collections::VecDeque;

// Java: public class PrecedingSiblingAxisIterator implements Iterator
pub struct PrecedingSiblingAxisIterator {
    // Java field: Object contextNode
    contextNode: Object,
    // Java field: Navigator navigator
    navigator: Box<dyn Navigator>,
    // Java field: Iterator siblingIter
    pub(crate) siblingIter: VecDeque<Object>,
    // Java field: Object nextObj
    nextObj: Option<Object>,
}

// Java: public class PrecedingSiblingAxisIterator implements Iterator
impl PrecedingSiblingAxisIterator {
    // Java: PrecedingSiblingAxisIterator(Object contextNode, Navigator navigator)
    pub fn new(contextNode: Object, navigator: Box<dyn Navigator>) -> Self {
        // Java: super();
        // Java: this.contextNode = contextNode;
        // Java: this.navigator = navigator;
        let mut instance = PrecedingSiblingAxisIterator {
            contextNode,
            navigator,
            siblingIter: VecDeque::new(),
            nextObj: None,
        };
        // Java: this.init();
        instance.init();
        // Java: if (this.siblingIter.hasNext())
        if !instance.siblingIter.is_empty() {
            // Java: this.nextObj = this.siblingIter.next();
            instance.nextObj = instance.siblingIter.pop_front();
        }
        instance
    }

    // Java: void init()
    fn init(&mut self) {
        // Java: Object parent = this.navigator.getParentNode(this.contextNode);
        let parent = self.navigator.getParentNode(&self.contextNode).ok();
        // Java: if (parent != null)
        if let Some(parent) = parent {
            // Java: Iterator childIter = this.navigator.getChildAxisIterator(parent);
            let mut child_iter = self.navigator.getChildAxisIterator(&parent)
                .unwrap_or_else(|_| Box::new(std::iter::empty()));
            // Java: LinkedList siblings = new LinkedList();
            let mut siblings = VecDeque::new();
            // Java: while(childIter.hasNext())
            while let Some(each_child) = child_iter.next() {
                // Java: Object eachChild = childIter.next();
                // Java: if (eachChild.equals(this.contextNode))
                if Arc::ptr_eq(&each_child, &self.contextNode) {
                    // Java: break;
                    break;
                }
                // Java: siblings.addFirst(eachChild);
                siblings.push_front(each_child);
            }
            // Java: this.siblingIter = siblings.iterator();
            self.siblingIter = siblings;
        }
        // Java: else
        else {
            // Java: this.siblingIter = JaxenConstants.EMPTY_ITERATOR;
            self.siblingIter = VecDeque::new();
        }
    }

    // Java: boolean hasNext()
    pub fn hasNext(&self) -> bool {
        // Java: return this.nextObj != null;
        self.nextObj.is_some()
    }

    // Java: Object next()
    pub fn next_node(&mut self) -> Object {
        // Java: if (!this.hasNext())
        if !self.hasNext() {
            // Java: throw new NoSuchElementException();
            panic!("NoSuchElementException");
        }
        // Java: else
        // Java: Object obj = this.nextObj;
        let obj = self.nextObj.clone().unwrap();
        // Java: if (this.siblingIter.hasNext())
        if !self.siblingIter.is_empty() {
            // Java: this.nextObj = this.siblingIter.next();
            self.nextObj = self.siblingIter.pop_front();
        }
        // Java: else
        else {
            // Java: this.nextObj = null;
            self.nextObj = None;
        }
        // Java: return obj;
        obj
    }

    // Java: void remove()
    pub fn remove(&self) {
        // Java: throw new UnsupportedOperationException();
        panic!("UnsupportedOperationException");
    }
}

impl Iterator for PrecedingSiblingAxisIterator {
    type Item = Object;

    fn next(&mut self) -> Option<Self::Item> {
        if self.hasNext() {
            Some(self.next_node())
        } else {
            None
        }
    }
}
