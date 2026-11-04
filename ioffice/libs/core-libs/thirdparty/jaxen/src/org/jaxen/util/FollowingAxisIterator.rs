// Java: import java.util.Iterator
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
use super::DescendantOrSelfAxisIterator::DescendantOrSelfAxisIterator;

// Java: public class FollowingAxisIterator implements Iterator
pub struct FollowingAxisIterator {
    // Java field: Object contextNode
    contextNode: Option<Object>,
    // Java field: Navigator navigator
    navigator: Box<dyn Navigator>,
    // Java field: Iterator siblings
    siblings: NodeIterator,
    // Java field: Iterator currentSibling
    pub(crate) currentSibling: NodeIterator,
}

// Java: public class FollowingAxisIterator implements Iterator
impl FollowingAxisIterator {
    // Java: FollowingAxisIterator(Object contextNode, Navigator navigator)
    pub fn new(contextNode: Object, navigator: Box<dyn Navigator>) -> Self {
        // Java: super();
        // Java: this.contextNode = contextNode;
        // Java: this.navigator = navigator;
        // Java: this.siblings = navigator.getFollowingSiblingAxisIterator(contextNode);
        let siblings = navigator.getFollowingSiblingAxisIterator(&contextNode)
            .unwrap_or_else(|_| Box::new(std::iter::empty()));
        // Java: this.currentSibling = JaxenConstants.EMPTY_ITERATOR;
        let currentSibling: NodeIterator = Box::new(std::iter::empty());
        FollowingAxisIterator {
            contextNode: Some(contextNode),
            navigator,
            siblings,
            currentSibling,
        }
    }

    // Java: boolean goForward()
    fn goForward(&mut self) -> bool {
        // Java: while(true)
        loop {
            // Java: if (!this.siblings.hasNext())
            match self.siblings.next() {
                None => {
                    // Java: if (this.goUp())
                    if self.goUp() {
                        // Java: continue;
                        continue;
                    }
                    // Java: return false;
                    return false;
                }
                Some(next_sibling) => {
                    // Java: Object nextSibling = this.siblings.next();
                    // Java: this.currentSibling = new DescendantOrSelfAxisIterator(nextSibling, this.navigator);
                    self.currentSibling = Box::new(DescendantOrSelfAxisIterator::new(
                        next_sibling,
                        self.clone_navigator(),
                    ));
                    // Java: return true;
                    return true;
                }
            }
        }
    }

    // Java: boolean goUp()
    fn goUp(&mut self) -> bool {
        // Java: if (this.contextNode != null && !this.navigator.isDocument(this.contextNode))
        let should_go_up = if let Some(ref ctx) = self.contextNode {
            !self.navigator.isDocument(ctx)
        } else {
            false
        };

        if should_go_up {
            // Java: try
            let ctx = self.contextNode.as_ref().unwrap().clone();
            // Java: this.contextNode = this.navigator.getParentNode(this.contextNode);
            match self.navigator.getParentNode(&ctx) {
                Ok(parent) => {
                    // Java: if (this.contextNode != null && !this.navigator.isDocument(this.contextNode))
                    if !self.navigator.isDocument(&parent) {
                        self.contextNode = Some(parent.clone());
                        // Java: this.siblings = this.navigator.getFollowingSiblingAxisIterator(this.contextNode);
                        self.siblings = self.navigator.getFollowingSiblingAxisIterator(&parent)
                            .unwrap_or_else(|_| Box::new(std::iter::empty()));
                        // Java: return true;
                        true
                    }
                    // Java: else
                    else {
                        self.contextNode = Some(parent);
                        // Java: return false;
                        false
                    }
                }
                // Java: catch (UnsupportedAxisException e)
                Err(_) => {
                    self.contextNode = None;
                    // Java: throw new JaxenRuntimeException(e);
                    false
                }
            }
        }
        // Java: else
        else {
            // Java: return false;
            false
        }
    }

    // Java: boolean hasNext()
    pub fn hasNext(&mut self) -> bool {
        // Java: while(true)
        loop {
            // Java: if (!this.currentSibling.hasNext())
            if self.current_sibling_exhausted() {
                // Java: if (this.goForward())
                if self.goForward() {
                    // Java: continue;
                    continue;
                }
                // Java: return false;
                return false;
            }
            // Java: return true;
            return true;
        }
    }

    fn current_sibling_exhausted(&self) -> bool {
        self.currentSibling.size_hint().1 == Some(0)
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
        else {
            // Java: return this.currentSibling.next();
            self.currentSibling.next().unwrap()
        }
    }

    // Java: void remove()
    pub fn remove(&self) {
        // Java: throw new UnsupportedOperationException();
        panic!("UnsupportedOperationException");
    }
}

impl Iterator for FollowingAxisIterator {
    type Item = Object;

    fn next(&mut self) -> Option<Self::Item> {
        if self.hasNext() {
            Some(self.currentSibling.next().unwrap())
        } else {
            None
        }
    }
}
