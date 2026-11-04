// Java: import java.util.Iterator
// Java: import java.util.NoSuchElementException
// Java: import org.jaxen.JaxenConstants
use crate::org::jaxen::JaxenConstants;
// Java: import org.jaxen.Navigator
use crate::org::jaxen::Navigator::Navigator;
// Java: import org.jaxen.UnsupportedAxisException
use crate::org::jaxen::UnsupportedAxisException::UnsupportedAxisException;
use crate::org::jaxen::{Object, NodeIterator};
use std::sync::Arc;

// Java: public class FollowingSiblingAxisIterator implements Iterator
pub struct FollowingSiblingAxisIterator {
    // Java field: Object contextNode
    contextNode: Option<Object>,
    // Java field: Navigator navigator
    navigator: Box<dyn Navigator>,
    // Java field: Iterator siblingIter
    pub(crate) siblingIter: NodeIterator,
}

// Java: public class FollowingSiblingAxisIterator implements Iterator
impl FollowingSiblingAxisIterator {
    // Java: FollowingSiblingAxisIterator(Object contextNode, Navigator navigator)
    pub fn new(contextNode: Object, navigator: Box<dyn Navigator>) -> Self {
        // Java: super();
        // Java: this.contextNode = contextNode;
        // Java: this.navigator = navigator;
        let mut instance = FollowingSiblingAxisIterator {
            contextNode: Some(contextNode),
            navigator,
            siblingIter: Box::new(std::iter::empty()),
        };
        // Java: this.init();
        instance.init();
        instance
    }

    // Java: void init()
    fn init(&mut self) {
        // Java: Object parent = this.navigator.getParentNode(this.contextNode);
        let parent = self.contextNode.as_ref()
            .and_then(|ctx| self.navigator.getParentNode(ctx).ok());
        // Java: if (parent != null)
        if let Some(parent) = parent {
            // Java: this.siblingIter = this.navigator.getChildAxisIterator(parent);
            self.siblingIter = self.navigator.getChildAxisIterator(&parent)
                .unwrap_or_else(|_| Box::new(std::iter::empty()));
            // Java: while(this.siblingIter.hasNext())
            while let Some(each_child) = self.siblingIter.next() {
                // Java: Object eachChild = this.siblingIter.next();
                // Java: if (eachChild.equals(this.contextNode))
                if let Some(ref ctx) = self.contextNode {
                    if Arc::ptr_eq(&each_child, ctx) {
                        // Java: break;
                        break;
                    }
                }
            }
        }
        // Java: else
        else {
            // Java: this.siblingIter = JaxenConstants.EMPTY_ITERATOR;
            self.siblingIter = Box::new(std::iter::empty());
        }
    }

    // Java: boolean hasNext()
    pub fn hasNext(&self) -> bool {
        // Java: return this.siblingIter.hasNext();
        self.siblingIter.size_hint().1 != Some(0)
    }

    // Java: Object next()
    pub fn next_node(&mut self) -> Object {
        // Java: return this.siblingIter.next();
        self.siblingIter.next().expect("NoSuchElementException")
    }

    // Java: void remove()
    pub fn remove(&self) {
        // Java: throw new UnsupportedOperationException();
        panic!("UnsupportedOperationException");
    }
}

impl Iterator for FollowingSiblingAxisIterator {
    type Item = Object;

    fn next(&mut self) -> Option<Self::Item> {
        self.siblingIter.next()
    }
}
